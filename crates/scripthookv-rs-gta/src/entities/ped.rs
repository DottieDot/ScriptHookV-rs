use scripthookv::types::Ped as NativePed;
use crate::natives::*;

use super::Entity;

pub struct Ped {
  handle: NativePed
}

impl Ped {
  
}

impl Entity for Ped {
  /// Gets the underlying entity handle.
  fn handle(&self) -> scripthookv::types::Entity {
    self.handle
  }
}

#[derive(Debug)]
pub struct NotAPedError {
  handle: i32
}

impl std::fmt::Display for NotAPedError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "handle {} is not a valid ped handle", self.handle)
  }
}

impl TryFrom<i32> for Ped {
  type Error = NotAPedError;
  
  fn try_from(handle: i32) -> Result<Self, Self::Error> {
    unsafe {
      if entity::does_entity_exist(handle) != 0 && !entity::is_entity_a_ped(handle) != 0 {
        Ok(Self { handle })
      }
      else {
        Err(Self::Error { handle })
      }
    }
  }
}
