use std::fmt::Display;

pub struct Point {
  pub x: i32,
  pub y: i32
}

impl PartialEq for Point {
  fn eq(&self, other: &Self) -> bool {
      self.x == other.x && self.y == other.y
  }
}

impl Clone for Point {
  fn clone(&self) -> Self {
      Self { x: self.x.clone(), y: self.y.clone() }
  }
}

impl Copy for Point {
  
}

impl Display for Point {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      write!(f, "[ x: {}, y: {} ]", &self.x, &self.y)
  }
}