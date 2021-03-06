use crate::vector::Vector3;
use std::ops::{Add, Sub};

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct Point3 {
  pub x: f64,
  pub y: f64,
  pub z: f64,
}
impl Point3 {
  pub fn zero() -> Point3 {
    Point3::from_one(0.0)
  }

  pub fn from_one(v: f64) -> Point3 {
    Point3 { x: v, y: v, z: v }
  }
}

impl Add<Vector3> for Point3 {
  type Output = Point3;

  fn add(self, other: Vector3) -> Point3 {
    Point3 {
      x: self.x + other.x,
      y: self.y + other.y,
      z: self.z + other.z,
    }
  }
}
impl Add<Point3> for Vector3 {
  type Output = Point3;

  fn add(self, other: Point3) -> Point3 {
    other + self
  }
}

impl Sub<Vector3> for Point3 {
  type Output = Point3;

  fn sub(self, other: Vector3) -> Point3 {
    Point3 {
      x: self.x - other.x,
      y: self.y - other.y,
      z: self.z - other.z,
    }
  }
}
impl Sub<Point3> for Vector3 {
  type Output = Point3;

  fn sub(self, other: Point3) -> Point3 {
    other - self
  }
}

impl Sub<Point3> for Point3 {
  type Output = Vector3;

  fn sub(self, other: Point3) -> Vector3 {
    Vector3 {
      x: self.x - other.x,
      y: self.y - other.y,
      z: self.z - other.z,
    }
  }
}
