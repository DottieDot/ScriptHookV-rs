pub type Void = usize;
pub type Any = usize;
pub type Bool = u32;
pub type Hash = u32;
pub type Entity = i32;
pub type Player = i32;
pub type FireId = i32;
pub type Ped = i32;
pub type Vehicle = i32;
pub type Cam = i32;
pub type CarGenerator = i32;
pub type Group = i32;
pub type Train = i32;
pub type Pickup = i32;
pub type Object = i32;
pub type Weapon = i32;
pub type Interior = i32;
pub type Blip = i32;
pub type Texture = i32;
pub type TextureDict = i32;
pub type CoverPoint = i32;
pub type Camera = i32;
pub type TaskSequence = i32;
pub type ColourIndex = i32;
pub type Sphere = i32;
pub type ScrHandle = i32;

/// A vector4 representation that can be used with natives.
///
/// To make it compatible with natives every field is 8 byte aligned.
#[repr(C, align(1))]
#[derive(Clone, Copy, Debug)]
pub struct Vector4 {
  pub x:    f32,
  _pad0x04: u32,
  pub y:    f32,
  _pad0x08: u32,
  pub z:    f32,
  _pad0x10: u32,
  pub w:    f32,
  _pad0x18: u32
}

mod matrix;
mod vector2;
mod vector3;

pub use matrix::*;
pub use vector2::*;
pub use vector3::*;
