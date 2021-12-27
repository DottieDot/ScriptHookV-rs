use super::{RGB, RGBA};

#[derive(Debug, Copy, Clone)]
pub struct Color {
  pub(crate) representation: RGBA
}

impl Color {}

impl From<RGBA> for Color {
  #[inline]
  #[must_use]
  fn from(value: RGBA) -> Self {
    Self {
      representation: value
    }
  }
}

impl From<RGB> for Color {
  #[inline]
  #[must_use]
  fn from(value: RGB) -> Self {
    Self {
      representation: value.into()
    }
  }
}
