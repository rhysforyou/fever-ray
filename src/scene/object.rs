use crate::point::Point3;
use crate::scene::Material;
use crate::vector::Vector3;

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
