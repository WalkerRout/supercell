
use crate::*;

use rand::prelude::*;

use rayon::prelude::*;

#[derive(Debug, Clone, PartialEq)]
pub struct World<C> {
  pub rules: Rules,
  pub cells: Vec<C>,
}

impl<C> World<C>
  where C: Cell {
  pub fn new(dims: u16) -> (Self, Self) 
    where C: Clone {
    let rules = Rules::new(dims);
    let cells = {
      let mut rng = thread_rng();
      let mut cells = Vec::with_capacity((dims*dims*dims) as usize);
      for i in 0..dims {
        for j in 0..dims {
          for k in 0..dims {
            let mut cell = C::from_index((i, j, k));
            cell.randomize_health(&mut rng);
            cells.push(cell);
          }
        }
      }
      cells
    };
    let previous = Self { rules, cells, };
    let world = previous.clone();
    (previous, world)
  }

  pub fn update(&mut self, previous: &mut Self) 
    where C: Clone + Send + Sync {
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
            cell.update(&self.rules, &previous.cells);
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
      let (previous, world) = World::<CubeCell>::new(3);
      assert_eq!(previous, world);
    }

    #[rstest]
    fn update() {
      let (mut previous, mut world) = World::<CubeCell>::new(2);
      world.update(&mut previous);
      assert_ne!(world.cells, previous.cells);
    }
  }
}
