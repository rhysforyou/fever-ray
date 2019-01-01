extern crate image;
#[macro_use]
extern crate serde_derive;
extern crate serde;

pub mod color;
pub mod point;
pub mod rendering;
pub mod scene;
pub mod vector;

pub use crate::color::Color;
pub use crate::point::Point3;
pub use crate::rendering::{Config, Intersectable, Intersection, Ray};
pub use crate::scene::{Material, Object, Scene, Sphere};
pub use crate::vector::Vector3;

use image::{DynamicImage, GenericImage};

pub fn render(config: &Config) -> DynamicImage {
  let mut image = DynamicImage::new_rgb8(config.width, config.height);
  for x in 0..config.width {
    for y in 0..config.height {
      let ray = Ray::create_prime(x, y, config);
      let intersection = config.scene.trace(&ray);
      let color = match intersection {
        Some(ref i) => get_color(&config, &ray, &i),
        None => config.scene.sky.color,
      };

      image.put_pixel(x, y, color.to_rgba());
    }
  }

  image
}

fn get_color(config: &Config, ray: &Ray, intersection: &Intersection) -> Color {
  let hit_point = ray.origin + (ray.direction * intersection.distance);
  let surface_normal = intersection.object.surface_normal(&hit_point);
  let direction_to_light = -config.scene.light.direction;
  let light_power =
    (surface_normal.dot(&direction_to_light) as f32).max(0.0) * config.scene.light.intensity;
  let light_reflected = intersection.object.material().albedo / std::f32::consts::PI;

  let color = intersection.object.material().color.clone()
    * config.scene.light.color.clone()
    * light_power
    * light_reflected
    + Color {
      red: 0.01,
      green: 0.01,
      blue: 0.02,
    };

  color.clamp()
}
