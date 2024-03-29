use std::{
  fmt::Display,
  ops::{Add, Mul},
  ops::{Div, Sub}
};

/// A vector2 representation that can be used with natives.
///
/// To make it compatible with natives every field is 8 byte aligned.
#[repr(C, align(1))]
#[derive(Clone, Copy, Debug)]
pub struct Vector2 {
  pub x:    f32,
  _pad0x04: u32,
  pub y:    f32,
  _pad0x08: u32
}

impl Vector2 {
  /// Creates a new Vector2.
  #[inline]
  #[must_use]
  pub fn new(x: f32, y: f32) -> Self {
    Self {
      x,
      y,
      _pad0x04: 0,
      _pad0x08: 0
    }
  }

  /// Creates a new Vector2 initialized with 0.
  #[inline]
  #[must_use]
  pub fn zero() -> Self {
    Self::new(0f32, 0f32)
  }

  /// Gets the length of the vector.
  #[must_use]
  pub fn length(self) -> f32 {
    (self.x.powf(2f32) + self.y.powf(2f32)).sqrt()
  }

  /// Creates a normalized copy of the vector.
  #[must_use]
  pub fn normalized(self) -> Self {
    let length = self.length();
    if length == 0f32 {
      return Vector2::zero();
    }

    let length_ratio = 1f32 / length;
    Vector2::new(self.x * length_ratio, self.y * length_ratio)
  }

  /// Gets the distance to another vector.
  #[inline]
  #[must_use]
  pub fn distance_to(self, to: Vector2) -> f32 {
    (to - self).length()
  }

  /// Gets the dot product with another vector.
  #[inline]
  #[must_use]
  pub fn dot(self, rhs: Vector2) -> f32 {
    self.x * rhs.x + self.y * rhs.y
  }
}

impl Display for Vector2 {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "({}, {})", self.x, self.y)
  }
}

impl Add<Vector2> for Vector2 {
  type Output = Vector2;

  #[inline]
  #[must_use]
  fn add(self, rhs: Vector2) -> Self::Output {
    Vector2::new(self.x + rhs.x, self.y + rhs.y)
  }
}

impl Sub<Vector2> for Vector2 {
  type Output = Vector2;

  #[inline]
  #[must_use]
  fn sub(self, rhs: Vector2) -> Self::Output {
    Vector2::new(self.x - rhs.x, self.y - rhs.y)
  }
}

impl Mul<f32> for Vector2 {
  type Output = Vector2;

  #[inline]
  #[must_use]
  fn mul(self, rhs: f32) -> Self::Output {
    Vector2::new(self.x * rhs, self.y * rhs)
  }
}

impl Div<f32> for Vector2 {
  type Output = Vector2;

  fn div(self, rhs: f32) -> Self::Output {
    Vector2::new(self.x / rhs, self.y / rhs)
  }
}

impl Div<Vector2> for Vector2 {
  type Output = Vector2;

  fn div(self, rhs: Vector2) -> Self::Output {
    Vector2::new(self.x / rhs.x, self.y / rhs.y)
  }
}

impl PartialEq<Vector2> for Vector2 {
  #[inline]
  #[must_use]
  fn eq(&self, other: &Vector2) -> bool {
    self.x == other.x && self.y == other.y
  }
}

impl PartialOrd<Vector2> for Vector2 {
  #[inline]
  #[must_use]
  fn partial_cmp(&self, other: &Vector2) -> Option<std::cmp::Ordering> {
    match self.x.partial_cmp(&other.x) {
      Some(core::cmp::Ordering::Equal) => {}
      ord => return ord
    }
    self.y.partial_cmp(&other.y)
  }
}
