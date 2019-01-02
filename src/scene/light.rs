use crate::color::Color;
use crate::point::Point3;
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
pub struct PointLight {
  pub color: Color,
  pub intensity: f32,
  pub position: Point3,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum Light {
  Directional(DirectionalLight),
  Ambient(AmbientLight),
  Point(PointLight),
}

impl Light {
  pub fn color(&self) -> Color {
    match self {
      Light::Directional(l) => l.color,
      Light::Ambient(l) => l.color,
      Light::Point(l) => l.color,
    }
  }

  pub fn intensity(&self, hit_point: &Point3) -> f32 {
    match self {
      Light::Directional(l) => l.intensity,
      Light::Ambient(l) => l.intensity,
      Light::Point(l) => {
        let r2 = (l.position - *hit_point).norm() as f32;
        l.intensity / (4.0 * ::std::f32::consts::PI * r2)
      }
    }
  }
  pub fn direction(&self, hit_point: &Point3) -> Option<Vector3> {
    match self {
      Light::Directional(l) => Some(-l.direction),
      Light::Ambient(_) => None,
      Light::Point(l) => Some((l.position - *hit_point).normalize()),
    }
  }

  pub fn distance(&self, hit_point: &Point3) -> f64 {
    match self {
      Light::Directional(_) => ::std::f64::INFINITY,
      Light::Ambient(_) => ::std::f64::INFINITY,
      Light::Point(ref l) => (l.position - *hit_point).length(),
    }
  }
}
