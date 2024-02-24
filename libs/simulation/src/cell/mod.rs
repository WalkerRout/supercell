
use crate::*;

use rand::RngCore;

mod cubecell;

pub use self::{
  cubecell::*,
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CellStatus {
  Alive,
  Decaying,
  Dead,
}

pub trait Cell {
  fn from_position(position: (u16, u16, u16)) -> Self;
  fn randomize_health(&mut self, rng: &mut impl RngCore);
  fn update(&mut self, rules: &Rules, cells: &[impl Cell]);
  fn status(&self) -> CellStatus;
}
