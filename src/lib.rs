extern crate image;
#[macro_use]
extern crate serde_derive;
extern crate serde;

pub mod point;
pub mod rendering;
pub mod scene;
pub mod vector;

pub use crate::point::Point3;
pub use crate::rendering::{Config, Intersectable, Ray};
pub use crate::scene::{Color, Material, Object, Scene, Sphere};
pub use crate::vector::Vector3;

use image::{DynamicImage, GenericImage, Pixel, Rgba};

pub fn render(config: &Config) -> DynamicImage {
  let mut image = DynamicImage::new_rgb8(config.width, config.height);
  let black = Rgba::from_channels(0, 0, 0, 0);
  for x in 0..config.width {
    for y in 0..config.height {
      let ray = Ray::create_prime(x, y, config);
      let mut intersected_object = false;

      for object in config.scene.objects.iter() {
        if object.intersected_by(&ray) {
          image.put_pixel(x, y, to_rgba(&object.material().color));
          intersected_object = true;
          break;
        }
      }

      if !intersected_object {
        image.put_pixel(x, y, black);
      }
    }
  }

  image
}

fn to_rgba(color: &Color) -> Rgba<u8> {
  Rgba::from_channels(color.red, color.green, color.blue, 0)
}

#[cfg(test)]
mod tests {
  use crate::*;
  use image::{DynamicImage, GenericImageView};

  #[test]
  fn test_can_render_scene() {
    let config = Config {
      width: 800,
      height: 600,
      fov: 90.0,
      scene: Scene {
        objects: vec![Object::Sphere(Sphere {
          center: Point3 {
            x: 0.0,
            y: 0.0,
            z: -5.0,
          },
          radius: 1.0,
          material: Material {
            color: Color {
              red: 100,
              green: 255,
              blue: 100,
            },
          },
        })],
      },
    };

    let img: DynamicImage = render(&config);
    assert_eq!(config.width, img.width());
    assert_eq!(config.height, img.height());
  }
}
