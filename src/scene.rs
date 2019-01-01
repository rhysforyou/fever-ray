use crate::point::Point3;

pub struct Color {
  pub red: u8,
  pub green: u8,
  pub blue: u8,
}

pub struct Material {
  pub color: Color,
}

pub struct Sphere {
  pub center: Point3,
  pub radius: f64,
  pub material: Material,
}

pub struct Scene {
  pub width: u32,
  pub height: u32,
  pub fov: f64,
  pub objects: Vec<Sphere>,
}
