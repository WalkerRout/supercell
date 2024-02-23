
use crate::*;

use rand::prelude::*;

use rayon::prelude::*;

#[derive(Debug, Clone, PartialEq)]
pub struct World {
  pub rules: Rules,
  pub cells: Vec<Cell>,
}

impl World {
  pub fn new(dims: u16) -> (Self, Self) {
    let rules = Rules::new(dims);
    let mut rng = thread_rng();
    let cells = {
      let mut cells = Vec::with_capacity((dims*dims*dims) as usize);
      for i in 0..dims {
        for j in 0..dims {
          for k in 0..dims {
            let mut cell = Cell::new((i, j, k));
            cell.health.health_ticks = rng.gen_range(0..HEALTH);
            cells.push(cell);
          }
        }
      }
      cells
    };
    let previous = Self { cells, rules, };
    let world = previous.clone();
    (previous, world)
  }

  pub fn update(&mut self, previous: &mut Self) {
    *previous = self.clone();

    let threads = 8;
    let mut chunk_size = self.cells.len() / threads;
    if chunk_size == 0 {
      chunk_size = 4;
    }
    
    self.cells
      .par_chunks_mut(chunk_size)
      .for_each(|chunk| {
        chunk
          .iter_mut()
          .for_each(|cell| {
            cell.clear_neighbours();
            cell.update_neighbours(&self.rules, &previous.cells);
            // update health status based on previous neighbours
            cell.update_health(&self.rules);
          });
      });
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use rstest::*;

  mod world {
    use super::*;

    #[rstest]
    fn new() {
      let (previous, world) = World::new(3);
      assert_eq!(previous, world);
    }

    #[rstest]
    fn update() {
      let (mut previous, mut world) = World::new(2);
      world.update(&mut previous);
      assert_ne!(world.cells, previous.cells);
    }
  }
}