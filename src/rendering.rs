use crate::point::Point3;
use crate::scene::{Object, Scene, Sphere};
use crate::vector::Vector3;

pub struct Config {
  pub width: u32,
  pub height: u32,
  pub fov: f64,
  pub scene: Scene,
}

pub struct Ray {
  pub origin: Point3,
  pub direction: Vector3,
}

impl Ray {
  pub fn create_prime(x: u32, y: u32, config: &Config) -> Ray {
    assert!(config.width > config.height);
    let fov_adjustment = (config.fov.to_radians() / 2.0).tan();
    let aspect_ratio = (config.width as f64) / (config.height as f64);
    let sensor_x =
      (((x as f64 + 0.5) / config.width as f64) * 2.0 - 1.0) * aspect_ratio * fov_adjustment;
    let sensor_y = 1.0 - ((y as f64 + 0.5) / config.height as f64) * 2.0 * fov_adjustment;

    Ray {
      origin: Point3::zero(),
      direction: Vector3 {
        x: sensor_x,
        y: sensor_y,
        z: -1.0,
      }
      .normalize(),
    }
  }
}

pub trait Intersectable {
  fn intersected_by(&self, ray: &Ray) -> bool;
}

impl Intersectable for Object {
  fn intersected_by(&self, ray: &Ray) -> bool {
    match self {
      Object::Sphere(ref sphere) => sphere.intersected_by(ray),
    }
  }
}

impl Intersectable for Sphere {
  fn intersected_by(&self, ray: &Ray) -> bool {
    //Create a line segment between the ray origin and the center of the sphere
    let l: Vector3 = self.center - ray.origin;
    //Use l as a hypotenuse and find the length of the adjacent side
    let adj2 = l.dot(&ray.direction);
    //Find the length-squared of the opposite side
    //This is equivalent to (but faster than) (l.length() * l.length()) - (adj2 * adj2)
    let d2 = l.dot(&l) - (adj2 * adj2);
    //If that length-squared is less than radius squared, the ray intersects the sphere
    d2 < (self.radius * self.radius)
  }
}
