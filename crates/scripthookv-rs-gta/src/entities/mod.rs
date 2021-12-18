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

#[derive(Debug)]
pub struct NotAnEntityHandleError {
  handle: i32
}

impl std::fmt::Display for NotAnEntityHandleError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "handle {} is not a valid entity handle", self.handle)
  }
}

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

pub enum EntityFromHandleError {
  NotAnEntityHandle(NotAnEntityHandleError),
  UnrecognizedEntity(UnrecognizedEntityTypeError)
}

pub fn entity_from_handle(handle: NativeEntity) -> Result<Box<dyn Entity>, EntityFromHandleError> {
  unsafe {
    if natives::entity::does_entity_exist(handle) != 0 {
      let entity_type = natives::entity::get_entity_type(handle);
      match entity_type {
        1 => Ok(Box::new(Ped::try_from(handle).unwrap())),     // Ped
        2 => Ok(Box::new(Vehicle::try_from(handle).unwrap())), // Vehicle
        3 => Ok(Box::new(Object::try_from(handle).unwrap())),  // Object
        _ => Err(EntityFromHandleError::UnrecognizedEntity(UnrecognizedEntityTypeError { handle, entity_type }))
      }
    }
    else {
      Err(EntityFromHandleError::NotAnEntityHandle(NotAnEntityHandleError { handle }))
    }
  }
}
