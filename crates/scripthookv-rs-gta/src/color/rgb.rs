use super::{Color, RGBA};

#[derive(Debug, Copy, Clone)]
pub struct RGB {
  pub r: u8,
  pub g: u8,
  pub b: u8
}

impl RGB {
  #[inline]
  #[must_use]
  pub fn new(r: u8, g: u8, b: u8) -> Self {
    Self { r, g, b }
  }
}

impl From<Color> for RGB {
  #[inline]
  #[must_use]
  fn from(color: Color) -> Self {
    color.representation.into()
  }
}

impl From<RGBA> for RGB {
  #[inline]
  #[must_use]
  fn from(rgba: RGBA) -> Self {
    Self {
      r: rgba.r,
      g: rgba.g,
      b: rgba.b
    }
  }
}
