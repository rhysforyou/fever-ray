use crate::color::Color;
use crate::vector::Vector3;

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
