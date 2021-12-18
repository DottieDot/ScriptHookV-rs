use scripthookv::types::{Object as NativeObject, Vector3, Hash};
use crate::natives::*;

use super::Entity;

pub struct Object {
  handle: NativeObject
}

impl Object {
  /// Creates a new object in the world.
  pub fn create(model: Hash, coords: Vector3, dynamic: bool) -> Result<Self, CreateObjectError> {
    unsafe {
      let handle = object::create_object(model, coords, false, false, dynamic);
      Object::try_from(handle)
        .map_err(|_| CreateObjectError { model })
    }
  }
  
  /// Tries finding the closest object of the given type.
  pub fn from_closest_of_type(coords: Vector3, radius: f32, model: Hash, ignore_mission_entities: bool) -> Option<Self> {
    unsafe {
      let handle = object::get_closest_object_of_type(coords, radius, model, ignore_mission_entities, false, false);
      Object::try_from(handle).ok()
    }
  }
  
  /// Align the object with the ground and returns wether this was successful or not.
  pub fn place_on_ground_properly(&self) -> bool {
    unsafe {
      object::place_object_on_ground_properly(self.handle())
    }
  }
  
  /// Moves an object to the target at the specified speed.
  /// 
  /// Returns true when the object has reached its destination.
  /// Has to be called every tick until the destination has been reached.
  pub fn slide_to(&self, to: Vector3, speed: Vector3, collisions: bool) -> bool {
    unsafe {
      object::slide_object(self.handle(), to, speed, collisions)
    }
  }
  
  /// Makes the object targetable.
  pub fn set_targetable(&self, targetable: bool) {
    unsafe {
      object::set_object_targettable(self.handle(), targetable)
    }
  }
  
  /// Makes vehicles avoid the object.
  /// 
  /// Overrides a flag on the object which determines if the object should be avoided by a vehicle in task CTaskVehicleGoToPointWithAvoidanceAutomobile.
  pub fn set_for_vehicles_to_avoid(&self, avoid: bool) {
    unsafe {
      object::set_object_force_vehicles_to_avoid(self.handle(), avoid)
    }
  }
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
      if entity::does_entity_exist(handle) && !entity::is_entity_an_object(handle) {
        Ok(Self { handle })
      }
      else {
        Err(Self::Error { handle })
      }
    }
  }
}

#[derive(Debug)]
pub struct CreateObjectError {
  model: Hash
}

impl std::fmt::Display for CreateObjectError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "failed to create object with model {:#010x}", self.model)
  }
}
