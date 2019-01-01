use crate::point::Point3;
use crate::vector::Vector3;
use image::{Pixel, Rgba};
use std::ops;

const GAMMA: f32 = 2.2;

fn gamma_encode(linear: f32) -> f32 {
  linear.powf(1.0 / GAMMA)
}

fn gamma_decode(encoded: f32) -> f32 {
  encoded.powf(GAMMA)
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct Color {
  pub red: f32,
  pub green: f32,
  pub blue: f32,
}

impl Color {
  pub fn black() -> Color {
    Color {
      red: 0.0,
      green: 0.0,
      blue: 0.0,
    }
  }

  pub fn clamp(&self) -> Color {
    Color {
      red: self.red.min(1.0).max(0.0),
      blue: self.blue.min(1.0).max(0.0),
      green: self.green.min(1.0).max(0.0),
    }
  }

  pub fn to_rgba(&self) -> Rgba<u8> {
    Rgba::from_channels(
      (gamma_encode(self.red) * 255.0) as u8,
      (gamma_encode(self.green) * 255.0) as u8,
      (gamma_encode(self.blue) * 255.0) as u8,
      255,
    )
  }

  pub fn from_rgba(rgba: Rgba<u8>) -> Color {
    Color {
      red: gamma_decode((rgba.data[0] as f32) / 255.0),
      green: gamma_decode((rgba.data[1] as f32) / 255.0),
      blue: gamma_decode((rgba.data[2] as f32) / 255.0),
    }
  }
}

impl ops::Mul for Color {
  type Output = Color;

  fn mul(self, other: Color) -> Color {
    Color {
      red: self.red * other.red,
      blue: self.blue * other.blue,
      green: self.green * other.green,
    }
  }
}
impl ops::Mul<f32> for Color {
  type Output = Color;

  fn mul(self, other: f32) -> Color {
    Color {
      red: self.red * other,
      blue: self.blue * other,
      green: self.green * other,
    }
  }
}
impl ops::Mul<Color> for f32 {
  type Output = Color;
  fn mul(self, other: Color) -> Color {
    other * self
  }
}

impl ops::Add for Color {
  type Output = Color;
  fn add(self, other: Color) -> Color {
    Color {
      red: self.red + other.red,
      blue: self.blue + other.blue,
      green: self.green + other.green,
    }
  }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Material {
  pub color: Color,
  pub albedo: f32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Sphere {
  pub center: Point3,
  pub radius: f64,
  pub material: Material,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Plane {
  pub origin: Point3,
  pub normal: Vector3,
  pub material: Material,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum Object {
  Sphere(Sphere),
  Plane(Plane),
}

impl Object {
  pub fn material(&self) -> &Material {
    match *self {
      Object::Sphere(ref sphere) => &sphere.material,
      Object::Plane(ref plane) => &plane.material,
    }
  }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DirectionalLight {
  pub direction: Vector3,
  pub color: Color,
  pub intensity: f32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Scene {
  pub objects: Vec<Object>,
  pub light: DirectionalLight,
}
