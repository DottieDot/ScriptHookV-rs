use scripthookv::types::Vehicle as NativeVehicle;
use crate::natives::*;

use super::Entity;

#[must_use]
#[derive(Debug, Clone, Copy)]
pub struct Vehicle {
  handle: NativeVehicle
}

impl Vehicle {
  
}

impl Entity for Vehicle {
  /// Gets the underlying entity handle.
  #[inline]
  #[must_use]
  fn handle(&self) -> scripthookv::types::Entity {
    self.handle
  }
}

/// The given handle is not a vehicle handle.
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
      if entity::does_entity_exist(handle) && !entity::is_entity_a_vehicle(handle) {
        Ok(Self { handle })
      }
      else {
        Err(Self::Error { handle })
      }
    }
  }
}
