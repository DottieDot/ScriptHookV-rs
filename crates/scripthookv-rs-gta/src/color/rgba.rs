use super::{Color, RGB};

#[derive(Debug, Copy, Clone)]
pub struct RGBA {
  pub r: u8,
  pub g: u8,
  pub b: u8,
  pub a: u8
}

impl RGBA {
  #[inline]
  #[must_use]
  pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
    Self {
      r,
      g,
      b,
      a
    }
  }
}

impl From<Color> for RGBA {
  #[inline]
  #[must_use]
  fn from(color: Color) -> Self {
    color.representation
  }
}

impl From<RGB> for RGBA {
  #[inline]
  #[must_use]
  fn from(rgb: RGB) -> Self {
    Self {
      r: rgb.r,
      g: rgb.g,
      b: rgb.b,
      a: 255
    }
  }
}
