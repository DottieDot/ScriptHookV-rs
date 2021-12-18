use scripthookv::types::Vehicle as NativeVehicle;
use crate::natives::*;

use super::Entity;

pub struct Vehicle {
  handle: NativeVehicle
}

impl Vehicle {
  
}

impl Entity for Vehicle {
  /// Gets the underlying entity handle.
  fn handle(&self) -> scripthookv::types::Entity {
    self.handle
  }
}

#[derive(Debug)]
pub struct NotAVehicleHandleError {
  handle: i32
}

impl std::fmt::Display for NotAVehicleHandleError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "handle {} is not a valid vehicle handle", self.handle)
  }
}

impl TryFrom<i32> for Vehicle {
  type Error = NotAVehicleHandleError;
  
  fn try_from(handle: i32) -> Result<Self, Self::Error> {
    unsafe {
      if entity::does_entity_exist(handle) != 0 && !entity::is_entity_a_vehicle(handle) != 0 {
        Ok(Self { handle })
      }
      else {
        Err(Self::Error { handle })
      }
    }
  }
}
