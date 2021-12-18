use scripthookv::types::Entity as NativeEntity;
use crate::natives;

mod entity;
mod vehicle;
mod ped;
mod object;

pub use entity::*;
pub use vehicle::*;
pub use ped::*;
pub use object::*;

/// The provided handle is not an entity handle.
#[derive(Debug)]
pub struct NotAnEntityHandleError {
  handle: i32
}

impl std::fmt::Display for NotAnEntityHandleError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "handle {} is not a valid entity handle", self.handle)
  }
}

/// The entity handle itself was valid but no implementation to represent it exists.
#[derive(Debug)]
pub struct UnrecognizedEntityTypeError {
  handle     : i32,
  entity_type: i32
}

impl std::fmt::Display for UnrecognizedEntityTypeError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "the entity type {} for entity handle {} is not recognized", self.entity_type, self.handle)
  }
}

/// The error returned when converting a handle to an entity fails.
pub enum EntityFromHandleError {
  NotAnEntityHandle(NotAnEntityHandleError),
  UnrecognizedEntity(UnrecognizedEntityTypeError)
}

/// Creates an entity struct for a given handle.
/// 
/// Creates the appropriate entity struct for the handle and returns an error if the handle is invalid.
pub fn entity_from_handle(handle: NativeEntity) -> Result<Box<dyn Entity>, EntityFromHandleError> {
  unsafe {
    if natives::entity::does_entity_exist(handle) {
      let entity_type = natives::entity::get_entity_type(handle);
      match entity_type {
        1 => Ok(Box::new(Ped::try_from(handle).expect("Ped::try_from failed"))),         // Ped
        2 => Ok(Box::new(Vehicle::try_from(handle).expect("Vehicle::try_from failed"))), // Vehicle
        3 => Ok(Box::new(Object::try_from(handle).expect("Object::try_from failed"))),   // Object
        _ => Err(EntityFromHandleError::UnrecognizedEntity(UnrecognizedEntityTypeError { handle, entity_type }))
      }
    }
    else {
      Err(EntityFromHandleError::NotAnEntityHandle(NotAnEntityHandleError { handle }))
    }
  }
}
