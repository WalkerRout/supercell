
#[derive(Debug, Clone, PartialEq)]
pub struct Rules {
  pub dims: u16, // cube dimensions
  pub neighbours: Vec<u8>, // list of valid neighbours
  pub offsets: Vec<(i8, i8, i8)>, // list of offsets to get nearby neighbours
}

impl Rules {
  pub fn new(dims: u16) -> Self {
    assert!(dims > 0);
    Rules {
      dims,
      .. Default::default()
    }
  }
}

impl Default for Rules {
  fn default() -> Self {
    let dims = 6; // in all directions
    let neighbours = vec![3, 5];
    let offsets = vec![
      (1, 0, 0),
      (-1, 0, 0),
      (0, 1, 0),
      (0, -1, 0),
      (0, 0, 1),
      (0, 0, -1),
    ];

    Self {
      dims,
      neighbours,
      offsets,
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use rstest::*;

  mod rules {
    use super::*;
    
    #[rstest]
    fn default() {
      let rules = Rules::default();
      assert_eq!(rules.dims, 6);
      assert_eq!(&rules.neighbours, &[3, 5]);
      assert_eq!(&rules.offsets, &[
        (1, 0, 0),
        (-1, 0, 0),
        (0, 1, 0),
        (0, -1, 0),
        (0, 0, 1),
        (0, 0, -1),
      ]);
    }

    #[rstest]
    #[case(1)]
    #[case(2)]
    #[case(3)]
    #[case(10)]
    #[case(20)]
    #[case(50)]
    #[should_panic]
    #[case::panic(0)]
    fn new(#[case] dims: u16) {
      let rules = Rules::new(dims);
      assert_eq!(rules.dims, dims);
    }
  }
}
