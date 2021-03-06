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
use rayon::prelude::*;

pub fn render(config: &Config) -> DynamicImage {
  let mut image = DynamicImage::new_rgb8(config.width, config.height);

  let samples: Vec<(u32, u32)> = (0..config.width)
    .flat_map(move |x| (0..config.height).map(move |y| (x, y)))
    .collect();

  let pixels: Vec<(u32, u32, Color)> = samples
    .par_iter()
    .map(|(x, y)| {
      let ray = Ray::create_prime(*x, *y, config);
      let intersection = config.scene.trace(&ray);
      (
        *x,
        *y,
        intersection
          .map(|i| get_color(&config, &ray, &i))
          .unwrap_or(config.scene.sky.color),
      )
    })
    .collect();

  for (x, y, color) in pixels {
    image.put_pixel(x, y, color.to_rgba());
  }

  image
}

fn get_color(config: &Config, ray: &Ray, intersection: &Intersection) -> Color {
  let hit_point = ray.origin + (ray.direction * intersection.distance);
  let surface_normal = intersection.object.surface_normal(&hit_point);

  let mut color = Color::black();

  for light in &config.scene.lights {
    let direction_to_light = light.direction(&hit_point);

    let light_power: f32;

    if let Some(direction_to_light) = direction_to_light {
      let shadow_ray = Ray {
        origin: hit_point + (surface_normal * config.shadow_bias),
        direction: direction_to_light,
      };
      let shadow_intersection = config.scene.trace(&shadow_ray);
      let in_light = shadow_intersection.is_none()
        || shadow_intersection.unwrap().distance > light.distance(&hit_point);

      let light_intensity = if in_light {
        light.intensity(&hit_point)
      } else {
        0.0
      };

      light_power = (surface_normal.dot(&direction_to_light) as f32).max(0.0) * light_intensity;
    } else {
      light_power = light.intensity(&hit_point);
    }

    let light_reflected = intersection.object.material().albedo / std::f32::consts::PI;

    let light_color = light.color() * light_power * light_reflected;
    color = color + (intersection.object.material().color * light_color);
  }

  color.clamp()
}
