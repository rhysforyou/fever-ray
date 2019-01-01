use crate::point::Point3;
use crate::scene::{Object, Plane, Scene, Sphere};
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
  fn intersection(&self, ray: &Ray) -> Option<f64>;
  fn surface_normal(&self, hit_point: &Point3) -> Vector3;
}

impl Intersectable for Object {
  fn intersection(&self, ray: &Ray) -> Option<f64> {
    match self {
      Object::Sphere(ref sphere) => sphere.intersection(ray),
      Object::Plane(ref plane) => plane.intersection(ray),
    }
  }
  fn surface_normal(&self, hit_point: &Point3) -> Vector3 {
    match self {
      Object::Sphere(ref sphere) => sphere.surface_normal(hit_point),
      Object::Plane(ref plane) => plane.surface_normal(hit_point),
    }
  }
}

impl Intersectable for Sphere {
  fn intersection(&self, ray: &Ray) -> Option<f64> {
    let l: Vector3 = self.center - ray.origin;
    let adj = l.dot(&ray.direction);
    let d2 = l.dot(&l) - (adj * adj);
    let radius2 = self.radius * self.radius;
    if d2 > radius2 {
      return None;
    }
    let thc = (radius2 - d2).sqrt();
    let t0 = adj - thc;
    let t1 = adj + thc;

    if t0 < 0.0 && t1 < 0.0 {
      return None;
    }

    let distance = if t0 < t1 { t0 } else { t1 };
    Some(distance)
  }

  fn surface_normal(&self, hit_point: &Point3) -> Vector3 {
    (*hit_point - self.center).normalize()
  }
}

impl Intersectable for Plane {
  fn intersection(&self, ray: &Ray) -> Option<f64> {
    let normal = &self.normal;
    let denom = normal.dot(&ray.direction);
    if denom > 1e-6 {
      let v = self.origin - ray.origin;
      let distance = v.dot(&normal) / denom;
      if distance >= 0.0 {
        return Some(distance);
      }
    }

    None
  }

  fn surface_normal(&self, _: &Point3) -> Vector3 {
    -self.normal
  }
}

pub struct Intersection<'a> {
  pub distance: f64,
  pub object: &'a Object,
}

impl<'a> Intersection<'a> {
  pub fn new<'b>(distance: f64, object: &'b Object) -> Intersection<'b> {
    Intersection { distance, object }
  }
}

impl Scene {
  pub fn trace(&self, ray: &Ray) -> Option<Intersection> {
    self
      .objects
      .iter()
      .filter_map(|s| s.intersection(ray).map(|d| Intersection::new(d, s)))
      .min_by(|i1, i2| i1.distance.partial_cmp(&i2.distance).unwrap())
  }
}
