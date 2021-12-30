use crate::{natives::*, Model};
use scripthookv::types::{Object as NativeObject, Vector3};

use super::Entity;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Object {
  handle: NativeObject
}

impl Object {
  /// Creates a new object in the world.
  #[inline]
  #[must_use]
  pub fn create(model: Model, coords: Vector3, dynamic: bool) -> Result<Self, CreateObjectError> {
    unsafe {
      let handle = object::create_object(model.hash(), coords, false, false, dynamic);
      Object::try_from(handle).map_err(|_| CreateObjectError { model })
    }
  }

  /// Creates a new object in the world at an exact position.
  #[inline]
  #[must_use]
  pub fn create_no_offset(
    model: Model,
    coords: Vector3,
    dynamic: bool
  ) -> Result<Self, CreateObjectError> {
    unsafe {
      let handle = object::create_object_no_offset(model.hash(), coords, false, false, dynamic);
      Object::try_from(handle).map_err(|_| CreateObjectError { model })
    }
  }

  /// Tries finding the closest object of the given type.
  #[inline]
  #[must_use]
  pub fn from_closest_of_type(
    coords: Vector3,
    radius: f32,
    model: Model,
    ignore_mission_entities: bool
  ) -> Option<Self> {
    unsafe {
      object::get_closest_object_of_type(
        coords,
        radius,
        model.hash(),
        ignore_mission_entities,
        false,
        false
      )
      .try_into()
      .ok()
    }
  }

  /// Align the object with the ground and returns wether this was successful or not.
  #[inline]
  pub fn place_on_ground_properly(&self) -> bool {
    unsafe { object::place_object_on_ground_properly(self.handle()) }
  }

  /// Moves an object to the target at the specified speed.
  ///
  /// Returns true when the object has reached its destination.
  /// Has to be called every tick until the destination has been reached.
  #[inline]
  #[must_use]
  pub fn slide_to(&self, to: Vector3, speed: Vector3, collisions: bool) -> bool {
    unsafe { object::slide_object(self.handle(), to, speed, collisions) }
  }

  /// Makes the object targetable.
  #[inline]
  pub fn set_targetable(&self, targetable: bool) {
    unsafe { object::set_object_targettable(self.handle(), targetable) }
  }

  /// Makes vehicles avoid the object.
  ///
  /// Overrides a flag on the object which determines if the object should be avoided by a vehicle in task CTaskVehicleGoToPointWithAvoidanceAutomobile.
  #[inline]
  pub fn set_for_vehicles_to_avoid(&self, avoid: bool) {
    unsafe { object::set_object_force_vehicles_to_avoid(self.handle(), avoid) }
  }
}

impl Entity for Object {
  /// Gets the underlying entity handle.
  #[inline]
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
      } else {
        Err(Self::Error { handle })
      }
    }
  }
}

#[derive(Debug)]
pub struct CreateObjectError {
  model: Model
}

impl std::fmt::Display for CreateObjectError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "failed to create object with model {}", self.model)
  }
}

impl Into<i32> for Object {
  #[inline]
  #[must_use]
  fn into(self) -> i32 {
    self.handle()
  }
}
