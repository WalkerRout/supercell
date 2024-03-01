
use crate::*;

use rand::RngCore;

mod cubecell;

pub use self::{
  cubecell::*,
};

pub type Index = (u16, u16, u16);

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CellStatus {
  Alive,
  Decaying,
  Dead,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct HealthStatus {
  pub max_health: u8,
  pub curr_health: u8,
  pub min_health: u8,
}

pub trait Cell {
  fn from_index(index: Index) -> Self;
  fn randomize_health(&mut self, rng: &mut impl RngCore);
  fn update(&mut self, rules: &Rules, cells: &[impl Cell]);
  fn status(&self) -> (CellStatus, HealthStatus);
}
