use crate::chunk::{CHUNK_WIDTH, CHUNK_WIDTH_E2, CHUNK_WIDTH_E3};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Point {
  n: u16,
}

impl Point {
  pub fn new(x: u8, y: u8, z: u8) -> Self {
    if x >= CHUNK_WIDTH {
      panic!("x is out of range")
    }
    if y >= CHUNK_WIDTH {
      panic!("y is out of range")
    }
    if z >= CHUNK_WIDTH {
      panic!("z is out of range")
    }
    Point {
      n: ((x as usize) * CHUNK_WIDTH_E2 + (y as usize) * (CHUNK_WIDTH as usize) + (z as usize))
        as u16,
    }
  }

  pub fn raw_n(self) -> u16 {
    self.n
  }

  #[allow(clippy::cast_lossless)]
  pub fn x(self) -> u8 {
    ((self.n / (CHUNK_WIDTH_E2 as u16)) % (CHUNK_WIDTH as u16)) as u8
  }

  #[allow(clippy::cast_lossless)]
  pub fn y(self) -> u8 {
    ((self.n / (CHUNK_WIDTH as u16)) % (CHUNK_WIDTH as u16)) as u8
  }

  #[allow(clippy::cast_lossless)]
  pub fn z(self) -> u8 {
    (self.n % (CHUNK_WIDTH as u16)) as u8
  }

  pub fn increment(&mut self) -> bool {
    if self.n == CHUNK_WIDTH_E3 as u16 - 1 {
      false
    } else {
      self.n += 1;
      true
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_point_splitting() {
    let p = Point::new(0, 0, 0);
    assert_eq!(p.x(), 0);
    assert_eq!(p.y(), 0);
    assert_eq!(p.z(), 0);

    let p = Point::new(1, 2, 3);
    assert_eq!(p.x(), 1);
    assert_eq!(p.y(), 2);
    assert_eq!(p.z(), 3);

    let p = Point::new(8, 9, 7);
    assert_eq!(p.x(), 8);
    assert_eq!(p.y(), 9);
    assert_eq!(p.z(), 7);

    let p = Point::new(15, 15, 15);
    assert_eq!(p.x(), 15);
    assert_eq!(p.y(), 15);
    assert_eq!(p.z(), 15);
  }
}
