use std::fmt::Display;

pub type Void         = usize;
pub type Any          = usize;
pub type Bool         = u32;
pub type Hash         = u32;
pub type Entity       = i32;
pub type Player       = i32;
pub type FireId       = i32;
pub type Ped          = i32;
pub type Vehicle      = i32;
pub type Cam          = i32;
pub type CarGenerator = i32;
pub type Group        = i32;
pub type Train        = i32;
pub type Pickup       = i32;
pub type Object       = i32;
pub type Weapon       = i32;
pub type Interior     = i32;
pub type Blip         = i32;
pub type Texture      = i32;
pub type TextureDict  = i32;
pub type CoverPoint   = i32;
pub type Camera       = i32;
pub type TaskSequence = i32;
pub type ColourIndex  = i32;
pub type Sphere       = i32;
pub type ScrHandle    = i32;

#[repr(C, align(1))]
#[derive(Clone, Copy, Debug)]
pub struct Vector4 {
  pub x   : f32,
  _pad0x04: u32,
  pub y   : f32,
  _pad0x08: u32,
  pub z   : f32,
  _pad0x10: u32,
  pub w   : f32,
  _pad0x18: u32
}

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

  pub fn zero() -> Self {
    Self::new(0f32, 0f32, 0f32)
  }
}

impl Display for Vector3 {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "({}, {}, {})", self.x, self.y, self.z)
  }
}

#[repr(C, align(1))]
#[derive(Clone, Copy, Debug)]
pub struct Vector2 {
  pub x   : f32,
  _pad0x04: u32,
  pub y   : f32,
  _pad0x08: u32
}
