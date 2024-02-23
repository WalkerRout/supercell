
use crate::*;

pub const HEALTH: u8 = 13;
pub const MIN_HEALTH: u8 = 5;

#[derive(Debug, Clone, PartialEq)]
pub struct Health {
  pub health_ticks: u8,
  pub decay_ticks: u8,
}

impl Health {
  fn new(base: u8, decay_ticks: u8) -> Self {
    assert!(decay_ticks > 0);
    let health_ticks = if base.checked_add(decay_ticks).is_none() {
      u8::MAX
    } else {
      base + decay_ticks
    };
    Self {
      health_ticks,
      decay_ticks,
    }
  }

  fn update(&mut self, rules: &Rules, neighbours: u8) {
    if rules.neighbours.contains(&neighbours) {
      // branchless increment
      self.health_ticks += (self.is_dead() || self.health_ticks < u8::MAX) as u8;
    } else if self.health_ticks >= 1 {
      self.health_ticks -= 1;
    }
  }

  fn is_alive(&self) -> bool {
    self.health_ticks > self.decay_ticks
  }

  fn is_decaying(&self) -> bool {
    self.health_ticks > 0 && self.health_ticks <= self.decay_ticks
  }

  fn is_dead(&self) -> bool {
    self.health_ticks == 0
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Cell {
  pub neighbours: u8,
  pub health: Health,
  pub index: (u16, u16, u16),
}

impl Cell {
  pub fn new(index: (u16, u16, u16)) -> Self {
    Self {
      neighbours: 0,
      health: Health::new(HEALTH, MIN_HEALTH),
      index,
    }
  }

  pub fn update_neighbours(&mut self, rules: &Rules, cells: &[Cell]) {
    let check_pos = |pos: (u16, u16, u16), off: &(i8, i8, i8)| -> bool {
      let i = pos.0 as i32 - off.0 as i32;
      let j = pos.1 as i32 - off.1 as i32;
      let k = pos.2 as i32 - off.2 as i32;
      let pos = (i, j, k);
      pos.0 >= 0 && pos.0 < rules.dims as i32 &&
      pos.1 >= 0 && pos.1 < rules.dims as i32 &&
      pos.2 >= 0 && pos.2 < rules.dims as i32
    };

    for offset in &rules.offsets {
      if check_pos(self.index, offset) {
        let i = (self.index.0 as i32 - offset.0 as i32) as u16;
        let j = (self.index.1 as i32 - offset.1 as i32) as u16;
        let k = (self.index.2 as i32 - offset.2 as i32) as u16;
        let d = rules.dims;
        let pos = (i*d*d + j*d + k) as usize;
        // branchless increment
        self.neighbours += (cells[pos].is_alive()) as u8;
      }
    }
  }

  pub fn update_health(&mut self, rules: &Rules) {
    self.health.update(rules, self.neighbours);
  }

  pub fn clear_neighbours(&mut self) {
    self.neighbours = 0;
  }

  pub fn is_alive(&self) -> bool {
    self.health.is_alive()
  }

  pub fn is_decaying(&self) -> bool {
    self.health.is_decaying()
  }

  pub fn is_dead(&self) -> bool {
    self.health.is_dead()
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use rstest::*;

  mod health {
    use super::*;

    #[rstest]
    fn new() {
      let health = Health::new(15, 5);
      assert_eq!(health.health_ticks, 20);
      assert_eq!(health.decay_ticks, 5);
    }

    #[rstest]
    #[case(15, 5, 4, 15+5 + 1)]
    #[case(15, 5, 5, 15+5 - 1)]
    #[case(1, 1, 5, 1+1 - 1)]
    #[case(255, 5, 4, 255)]
    #[case(255, 5, 5, 255 - 1)]
    #[case(0, 5, 4, 0+5 + 1)]
    #[case(0, 5, 5, 0+5 - 1)]
    #[should_panic]
    #[case::panic(15, 0, 5, 15+0 - 1)]
    fn update(#[case] base: u8, #[case] decay: u8, #[case] neighbours: u8, #[case] expected_health_ticks: u8) {
      let mut rules = Rules::default();
      rules.neighbours = vec![4];
      let mut health = Health::new(base, decay);
      health.update(&rules, neighbours);
      assert_eq!(health.health_ticks, expected_health_ticks);
    }

    #[rstest]
    #[case(15)]
    #[case(255)]
    #[case(5+1)]
    pub fn is_alive(#[case] health_ticks: u8) {
      let mut health = Health::new(15, 5);
      health.health_ticks = health_ticks;
      assert!(health.is_alive());
    }

    #[rstest]
    #[case(5)]
    #[case(1)]
    pub fn is_decaying(#[case] health_ticks: u8) {
      let mut health = Health::new(15, 5);
      health.health_ticks = health_ticks;
      assert!(health.is_decaying());
    }

    #[rstest]
    #[case(0)]
    pub fn is_dead(#[case] health_ticks: u8) {
      let mut health = Health::new(15, 5);
      health.health_ticks = health_ticks;
      assert!(health.is_dead());
    }
  }

  mod cell {
    use super::*;

    #[rstest]
    fn new() {
      let cell = Cell::new((1, 2, 3));
      assert_eq!(cell.neighbours, 0);
      assert_eq!(cell.health, Health::new(HEALTH, MIN_HEALTH));
      assert_eq!(cell.index, (1, 2, 3));
    }

    #[rstest]
    fn update_neighbours() {
      let rules = Rules::new(2);
      let cells = vec![
        Cell::new((0, 0, 0)), Cell::new((0, 0, 1)),  Cell::new((0, 1, 0)), Cell::new((0, 1, 1)),
        Cell::new((1, 0, 0)), Cell::new((1, 0, 1)),  Cell::new((1, 1, 0)), Cell::new((1, 1, 1)),
      ];
      let mut cell = Cell::new((0, 0, 0));
      cell.update_neighbours(&rules, &cells);
      assert_eq!(cell.neighbours, 3);
    }

    #[rstest]
    #[case(4, 5, HEALTH+5 + 1)]
    #[case(5, 5, HEALTH+5 - 1)]
    fn update_health(#[case] neighbours: u8, #[case] decay_ticks: u8, #[case] expected_health_ticks: u8) {
      let mut rules = Rules::default();
      rules.neighbours = vec![4];
      let mut cell = Cell::new((1, 2, 3));
      cell.neighbours = neighbours;
      cell.update_health(&rules);
      assert_eq!(cell.health, Health { health_ticks: expected_health_ticks, decay_ticks, });
    }

    #[rstest]
    fn clear_neighbours() {
      let mut cell = Cell::new((1, 2, 3));
      cell.neighbours = 10;
      assert_eq!(cell.neighbours, 10);
      cell.clear_neighbours();
      assert_eq!(cell.neighbours, 0);
    }
  }
}
