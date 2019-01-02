use crate::color::Color;
use crate::point::Point3;
use crate::vector::Vector3;

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
pub struct AmbientLight {
  pub color: Color,
  pub intensity: f32,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum Light {
  Directional(DirectionalLight),
  Ambient(AmbientLight),
}

impl Light {
  pub fn color(&self) -> Color {
    match self {
      Light::Directional(l) => l.color,
      Light::Ambient(l) => l.color,
    }
  }

  pub fn intensity(&self) -> f32 {
    match self {
      Light::Directional(l) => l.intensity,
      Light::Ambient(l) => l.intensity,
    }
  }

  pub fn direction(&self) -> Option<Vector3> {
    match self {
      Light::Directional(l) => Some(-l.direction),
      Light::Ambient(_) => None,
    }
  }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Sky {
  pub color: Color,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Scene {
  pub sky: Sky,
  pub lights: Vec<Light>,
  pub objects: Vec<Object>,
}
