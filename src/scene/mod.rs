pub mod light;
pub mod object;

use crate::color::Color;
pub use crate::scene::light::*;
pub use crate::scene::object::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct Material {
  pub color: Color,
  pub albedo: f32,
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
