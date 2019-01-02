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
      let color = intersection
        .map(|i| get_color(&config, &ray, &i))
        .unwrap_or(config.scene.sky.color);

      image.put_pixel(x, y, color.to_rgba());
    }
  }

  image
}

fn get_color(config: &Config, ray: &Ray, intersection: &Intersection) -> Color {
  let hit_point = ray.origin + (ray.direction * intersection.distance);
  let surface_normal = intersection.object.surface_normal(&hit_point);
  let direction_to_light = -config.scene.light.direction;

  let shadow_ray = Ray {
    origin: hit_point + (direction_to_light * config.shadow_bias),
    direction: direction_to_light,
  };
  let in_light = config.scene.trace(&shadow_ray).is_none();

  let light_intensity = if in_light {
    config.scene.light.intensity
  } else {
    0.0
  };

  let light_power = (surface_normal.dot(&direction_to_light) as f32).max(0.0) * light_intensity;
  let light_reflected = intersection.object.material().albedo / std::f32::consts::PI;

  let color = intersection.object.material().color.clone()
    * config.scene.light.color.clone()
    * light_power
    * light_reflected;

  color.clamp()
}
