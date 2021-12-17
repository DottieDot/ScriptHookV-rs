use std::{fmt::Display, ops::{Add, Mul}, ops::Sub};

/// A vector3 representation that can be used with natives.
/// 
/// To make it compatible with natives every field is 8 byte aligned.
#[repr(C, align(1))]
#[derive(Clone, Copy, Debug)]
pub struct Vector3 {
  pub x   : f32,
  _pad0x04: u32,
  pub y   : f32,
  _pad0x08: u32,
  pub z   : f32,
  _pad0x10: u32
}

impl Vector3 {
  /// Creates a new Vector3.
  pub fn new(x: f32, y: f32, z: f32) -> Self {
    Self {
      x,
      y,
      z,
      _pad0x04: 0,
      _pad0x08: 0,
      _pad0x10: 0
    }
  }
  
  /// Creates a new Vector3 initialized with 0.
  pub fn zero() -> Self {
    Self::new(0f32, 0f32, 0f32)
  }
  
  /// Gets the length of the vector.
  pub fn length(&self) -> f32 {
    (self.x.powf(2f32) + self.y.powf(2f32) + self.z.powf(2f32)).sqrt()
  }
  
  /// Creates a normalized copy of the vector.
  pub fn normalized(&self) -> Self {
    let length = self.length();
    if length == 0f32 {
      return Vector3::zero()
    }
    
    let length_ratio = 1f32 / length;
    Vector3::new(
      self.x * length_ratio,
      self.y * length_ratio,
      self.z * length_ratio
    )
  }
  
  /// Gets the distance to another vector.
  pub fn distance_to(&self, to: &Vector3) -> f32 {
    (*to - *self).length()
  }
  
  /// Gets the distance to another vector ignoring the z coordinate. 
  pub fn distance_to_2d(&self, to: &Vector3) -> f32 {
    let mut tmp = *to - *self;
    tmp.z = 0f32;
    tmp.length()
  }
  
  /// Gets the dot product with another vector.
  pub fn dot(&self, rhs: &Vector3) -> f32 { 
    self.x * rhs.x +
    self.y * rhs.y +
    self.z * rhs.z
  }
}

impl Display for Vector3 {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "({}, {}, {})", self.x, self.y, self.z)
  }
}

impl Add<Vector3> for Vector3 {
  type Output = Vector3;
  
  fn add(self, rhs: Vector3) -> Self::Output {
    Vector3::new(
      self.x + rhs.x,
      self.y + rhs.y,
      self.z + rhs.z
    )
  }
}

impl Sub<Vector3> for Vector3 {
  type Output = Vector3;
  
  fn sub(self, rhs: Vector3) -> Self::Output {
    Vector3::new(
      self.x - rhs.x,
      self.y - rhs.y,
      self.z - rhs.z
    )
  }
}

impl Mul<f32> for Vector3 {
  type Output = Vector3;
  
  fn mul(self, rhs: f32) -> Self::Output {
    Vector3::new(
      self.x * rhs,
      self.y * rhs,
      self.z * rhs
    )
  }
}

impl PartialEq<Vector3> for Vector3 {
  fn eq(&self, other: &Vector3) -> bool {
    self.x == other.x && 
    self.y == other.y && 
    self.z == other.z
  }
}

impl PartialOrd<Vector3> for Vector3 {
  fn partial_cmp(&self, other: &Vector3) -> Option<std::cmp::Ordering> {
    match self.x.partial_cmp(&other.x) {
      Some(core::cmp::Ordering::Equal) => {}
      ord => return ord,
    }
    match self.y.partial_cmp(&other.y) {
      Some(core::cmp::Ordering::Equal) => {}
      ord => return ord,
    }
    self.z.partial_cmp(&other.z)
  }
}
