use image::{Pixel, Rgba};
use std::ops;

/**
 * A color with red, green, and blue components.
 */
#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct Color {
  pub red: f32,
  pub green: f32,
  pub blue: f32,
}

impl Color {
  /** Pure black */
  pub fn black() -> Color {
    Color {
      red: 0.0,
      green: 0.0,
      blue: 0.0,
    }
  }

  /** Pure white */
  pub fn white() -> Color {
    Color {
      red: 1.0,
      green: 1.0,
      blue: 1.0,
    }
  }

  /** Clamp a color's red green, and blue channels to the range of 0.0 to 1.0 inclusive. */
  pub fn clamp(&self) -> Color {
    Color {
      red: self.red.min(1.0).max(0.0),
      blue: self.blue.min(1.0).max(0.0),
      green: self.green.min(1.0).max(0.0),
    }
  }

  pub fn to_rgba(&self) -> Rgba<u8> {
    Rgba::from_channels(
      (self.red * 255.0) as u8,
      (self.green * 255.0) as u8,
      (self.blue * 255.0) as u8,
      255,
    )
  }

  pub fn from_rgba(rgba: Rgba<u8>) -> Color {
    Color {
      red: (rgba.data[0] as f32) / 255.0,
      green: (rgba.data[1] as f32) / 255.0,
      blue: (rgba.data[2] as f32) / 255.0,
    }
  }

  /**
   * Creates a Color from a 32-bit hex value where each bye corresponds to
   * red, green, blue, and alpha respectively.
   *
   * ## Example
   *
   * ```rust
   * # use fever_ray::color::Color;
   * let pale_violet_red = 0xDB7093FF;
   * let color = Color::from_rgba_hex(pale_violet_red);
   * assert_eq!(color, Color { red: 0.858823529, green: 0.439215686, blue: 0.576470588 });
   * ```
   */
  pub fn from_rgba_hex(hex: u32) -> Color {
    Color {
      red: ((hex >> 24) as f32) / 255.0,
      green: (((hex & 0xFF0000) >> 16) as f32) / 255.0,
      blue: (((hex & 0xFF00) >> 8) as f32) / 255.0,
    }
  }

  /**
   * Creates a Color from a 24-bit hex value where each byte corresponds to
   * red, green, and blue respectively.
   *
   * ## Example
   *
   * ```rust
   * # use fever_ray::color::Color;
   * let pale_violet_red = 0xDB7093;
   * let color = Color::from_rgb_hex(pale_violet_red);
   * assert_eq!(color, Color { red: 0.858823529, green: 0.439215686, blue: 0.576470588 });
   * ```
   */
  pub fn from_rgb_hex(hex: u32) -> Color {
    Color {
      red: ((hex >> 16) as f32) / 255.0,
      green: (((hex & 0xFF00) >> 8) as f32) / 255.0,
      blue: ((hex & 0xFF) as f32) / 255.0,
    }
  }
}

impl ops::Mul for Color {
  type Output = Color;

  fn mul(self, other: Color) -> Color {
    Color {
      red: self.red * other.red,
      blue: self.blue * other.blue,
      green: self.green * other.green,
    }
  }
}

impl ops::Mul<f32> for Color {
  type Output = Color;

  fn mul(self, other: f32) -> Color {
    Color {
      red: self.red * other,
      blue: self.blue * other,
      green: self.green * other,
    }
  }
}

impl ops::Mul<Color> for f32 {
  type Output = Color;
  fn mul(self, other: Color) -> Color {
    other * self
  }
}

impl ops::Add for Color {
  type Output = Color;
  fn add(self, other: Color) -> Color {
    Color {
      red: self.red + other.red,
      blue: self.blue + other.blue,
      green: self.green + other.green,
    }
  }
}

impl PartialEq for Color {
  fn eq(&self, other: &Color) -> bool {
    self.red == other.red && self.blue == other.blue && self.green == other.green
  }
}

impl Eq for Color {}

#[cfg(test)]
mod tests {
  use crate::color::Color;

  #[test]
  fn test_from_rgba_hex() {
    let magenta = 0xFF00FF00;
    assert_eq!(
      Color::from_rgba_hex(magenta),
      Color {
        red: 1.0,
        green: 0.0,
        blue: 1.0
      }
    );
  }

  #[test]
  fn test_from_rgb_hex() {
    let magenta = 0xFF00FF;
    assert_eq!(
      Color::from_rgb_hex(magenta),
      Color {
        red: 1.0,
        green: 0.0,
        blue: 1.0
      }
    );
  }
}
