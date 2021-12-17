use std::{fmt::Display, ops::{Add, Mul}, ops::Sub};

#[repr(C, align(1))]
#[derive(Clone, Copy, Debug)]
pub struct Vector2 {
  pub x   : f32,
  _pad0x04: u32,
  pub y   : f32,
  _pad0x08: u32
}

impl Vector2 {
  pub fn new(x: f32, y: f32) -> Self {
    Self {
      x,
      y,
      _pad0x04: 0,
      _pad0x08: 0
    }
  }
  
  pub fn zero() -> Self {
    Self::new(0f32, 0f32)
  }
  
  pub fn length(&self) -> f32 {
    (self.x.powf(2f32) + self.y.powf(2f32)).sqrt()
  }
  
  pub fn normalized(&self) -> Self {
    let length = self.length();
    if length == 0f32 {
      return Vector2::zero()
    }
    
    let length_ratio = 1f32 / length;
    Vector2::new(
      self.x * length_ratio,
      self.y * length_ratio
    )
  }
  
  pub fn distance_to(&self, to: &Vector2) -> f32 {
    (*to - *self).length()
  }
  
  pub fn dot(&self, rhs: &Vector2) -> f32 { 
    self.x * rhs.x +
    self.y * rhs.y
  }
}

impl Display for Vector2 {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "({}, {})", self.x, self.y)
  }
}

impl Add<Vector2> for Vector2 {
  type Output = Vector2;
  
  fn add(self, rhs: Vector2) -> Self::Output {
    Vector2::new(
      self.x + rhs.x,
      self.y + rhs.y
    )
  }
}

impl Sub<Vector2> for Vector2 {
  type Output = Vector2;
  
  fn sub(self, rhs: Vector2) -> Self::Output {
    Vector2::new(
      self.x - rhs.x,
      self.y - rhs.y
    )
  }
}

impl Mul<f32> for Vector2 {
  type Output = Vector2;
  
  fn mul(self, rhs: f32) -> Self::Output {
    Vector2::new(
      self.x * rhs,
      self.y * rhs
    )
  }
}

impl PartialEq<Vector2> for Vector2 {
  fn eq(&self, other: &Vector2) -> bool {
    self.x == other.x && 
    self.y == other.y
  }
}

impl PartialOrd<Vector2> for Vector2 {
  fn partial_cmp(&self, other: &Vector2) -> Option<std::cmp::Ordering> {
    match self.x.partial_cmp(&other.x) {
      Some(core::cmp::Ordering::Equal) => {}
      ord => return ord,
    }
    self.y.partial_cmp(&other.y)
  }
}
