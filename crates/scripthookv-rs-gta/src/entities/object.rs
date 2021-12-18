use scripthookv::types::Object as NativeObject;
use crate::natives::*;

use super::Entity;

pub struct Object {
  handle: NativeObject
}

impl Object {
  
}

impl Entity for Object {
  /// Gets the underlying entity handle.
  fn handle(&self) -> scripthookv::types::Entity {
    self.handle
  }
}

/// The given handle is not an object handle.
#[derive(Debug)]
pub struct NotAnObjectError {
  handle: i32
}

impl std::fmt::Display for NotAnObjectError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "handle {} is not a valid object handle", self.handle)
  }
}

impl TryFrom<i32> for Object {
  type Error = NotAnObjectError;
  
  fn try_from(handle: i32) -> Result<Self, Self::Error> {
    unsafe {
      if entity::does_entity_exist(handle) != 0 && !entity::is_entity_an_object(handle) != 0 {
        Ok(Self { handle })
      }
      else {
        Err(Self::Error { handle })
      }
    }
  }
}
