
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
    Self {
      health_ticks: base.checked_add(decay_ticks).expect("base + decay_ticks"),
      decay_ticks,
    }
  }

  fn update(&mut self, rules: &Rules, neighbours: u8) {
    if rules.neighbours.contains(&neighbours) {
      if self.is_dead() || self.health_ticks < 255 {
        self.health_ticks += 1;
      }
    } else if self.health_ticks >= 1 {
      self.health_ticks -= 1;
    }
  }

  #[allow(unused)]
  fn health(&self) -> u8 {
    self.health_ticks - self.decay_ticks
  }

  fn is_alive(&self) -> bool {
    self.health_ticks > self.decay_ticks
  }

  #[allow(unused)]
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
        if cells[pos].is_alive() {
          self.neighbours += 1;
        }
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

  // pretend it all works... im lazy
}