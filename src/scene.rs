use crate::point::Point3;
use crate::vector::Vector3;

#[derive(Serialize, Deserialize, Debug)]
pub struct Color {
  pub red: u8,
  pub green: u8,
  pub blue: u8,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Material {
  pub color: Color,
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
pub struct Scene {
  pub objects: Vec<Object>,
}
