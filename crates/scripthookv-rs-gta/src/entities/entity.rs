use scripthookv::types::{Entity as NativeEntity, Vector3};

use crate::natives::*;

use super::EntityBones;

#[must_use]
pub trait Entity {
  /// Gets the underlying entity handle.
  fn handle(&self) -> NativeEntity;

  /// Checks if the entity exists.
  #[inline]
  #[must_use]
  fn exists(&self) -> bool {
    unsafe { entity::does_entity_exist(self.handle()) }
  }

  // Checks if the entity is dead.
  #[inline]
  #[must_use]
  fn is_dead(&self) -> bool {
    unsafe { entity::is_entity_dead(self.handle(), false) }
  }

  // Checks if the entity is alive.
  #[inline]
  #[must_use]
  fn is_alive(&self) -> bool {
    !self.is_dead()
  }

  /// Returns the current entity opacity.
  #[inline]
  #[must_use]
  fn opacity(&self) -> i32 {
    unsafe { entity::get_entity_alpha(self.handle()) }
  }

  /// Sets the opacity of the entity.
  ///
  /// Changes only take effect in increments of 51 (20%).
  #[inline]
  fn set_opacity(&self, opacity: i32) {
    if opacity == 255 {
      self.reset_opacity()
    } else {
      unsafe { entity::set_entity_alpha(self.handle(), opacity, false) }
    }
  }

  /// Resets the opacity of the entity to opaque.
  #[inline]
  fn reset_opacity(&self) {
    unsafe { entity::reset_entity_alpha(self.handle()) }
  }

  /// Checks if the entity is persistent.
  #[inline]
  #[must_use]
  fn persistent(&self) -> bool {
    unsafe { entity::is_entity_a_mission_entity(self.handle()) }
  }

  /// Makes the entity persistent.
  #[inline]
  fn persist(&self) {
    unsafe { entity::set_entity_as_mission_entity(self.handle(), true, true) }
  }

  /// Marks the entity as no longer needed.
  #[inline]
  fn no_longer_needed(&self) {
    unsafe {
      let mut handle = self.handle();
      entity::set_entity_as_no_longer_needed(&mut handle)
    }
  }

  /// Freezes the position of the entity.
  #[inline]
  fn freeze_position(&self, toggle: bool) {
    unsafe { entity::freeze_entity_position(self.handle(), toggle) }
  }

  /// Gets the current entity health.
  ///
  /// Standard health ranges:
  ///     Ped: 0 to 200
  /// Vehicle: 0 to 1000
  ///  Object: 0 to 1000
  ///
  /// For peds a health level below 100 is considered fatal.
  #[inline]
  #[must_use]
  fn health(&self) -> i32 {
    unsafe { entity::get_entity_health(self.handle()) }
  }

  /// Sets the health of the entity.
  ///
  /// Standard health ranges:
  ///     Ped: 0 to 200
  /// Vehicle: 0 to 1000
  ///  Object: 0 to 1000
  ///
  /// For peds a health level below 100 is considered fatal.
  #[inline]
  fn set_health(&self, health: i32) {
    unsafe { entity::set_entity_health(self.handle(), health, 0) }
  }

  /// Gets the max health of the entity.
  ///
  /// Standard health ranges:
  ///     Ped: 0 to 200
  /// Vehicle: 0 to 1000
  ///  Object: 0 to 1000
  ///
  /// For peds a health level below 100 is considered fatal.
  #[inline]
  #[must_use]
  fn max_health(&self) -> i32 {
    unsafe { entity::get_entity_max_health(self.handle()) }
  }

  /// Sets the max health of the entity.
  ///
  /// Standard health ranges:
  ///     Ped: 0 to 200
  /// Vehicle: 0 to 1000
  ///  Object: 0 to 1000
  ///
  /// For peds a health level below 100 is considered fatal.
  #[inline]
  fn set_max_health(&self, max_health: i32) {
    unsafe { entity::set_entity_max_health(self.handle(), max_health) }
  }

  /// Gets the current location of the entity.
  #[inline]
  #[must_use]
  fn position(&self) -> Vector3 {
    unsafe { entity::get_entity_coords(self.handle(), false) }
  }

  /// Sets the entity to a new position.
  ///
  /// Might change the position slightly to better accommodate for terrain and nearby infrastructure.
  /// If this is not desired use `Entity::set_position_no_offset` instead.
  #[inline]
  fn set_position(&self, coords: Vector3, clear_area: bool) {
    unsafe { entity::set_entity_coords(self.handle(), coords, false, false, false, clear_area) }
  }

  /// Sets the entity to a new exact position.
  #[inline]
  fn set_position_no_offset(&self, coords: Vector3) {
    unsafe { entity::set_entity_coords_no_offset(self.handle(), coords, false, false, false) }
  }

  /// Gets the current entity rotation.
  ///
  /// Uses rotation order 2.
  #[inline]
  #[must_use]
  fn rotation(&self) -> Vector3 {
    unsafe { entity::get_entity_rotation(self.handle(), 2) }
  }

  /// Sets the entity rotation.
  ///
  /// uses rotation order 2.
  #[inline]
  fn set_rotation(&self, rotation: Vector3) {
    unsafe {
      entity::set_entity_rotation(self.handle(), rotation.x, rotation.y, rotation.z, 2, true)
    }
  }

  /// Gets the current entity heading.
  #[inline]
  #[must_use]
  fn heading(&self) -> f32 {
    unsafe { entity::get_entity_heading(self.handle()) }
  }

  /// Sets the heading of the entity.
  #[inline]
  fn set_heading(&self, heading: f32) {
    unsafe { entity::set_entity_heading(self.handle(), heading) }
  }

  /// Gets the heading of the entity physics.
  ///
  /// Tends to be more accurate than `Entity::heading` especially when a ped is in ragdoll.
  #[inline]
  #[must_use]
  fn physics_heading(&self) -> f32 {
    unsafe { entity::_get_entity_physics_heading(self.handle()) }
  }

  /// Gets how far the entity is submerged
  ///
  /// A value of 1 means that the entity is fully submerged.
  #[inline]
  #[must_use]
  fn submersion_level(&self) -> f32 {
    unsafe { entity::get_entity_submerged_level(self.handle()) }
  }

  /// Gets the distance between the entity and the ground.
  #[inline]
  #[must_use]
  fn height_above_ground(&self) -> f32 {
    unsafe { entity::get_entity_height_above_ground(self.handle()) }
  }

  /// Gets the world coordinates for of a relative offset to this entity.
  #[inline]
  #[must_use]
  fn get_offsetted_position(&self, offset: Vector3) -> Vector3 {
    unsafe { entity::get_offset_from_entity_in_world_coords(self.handle(), offset) }
  }

  /// Gets the relative position to this entity for world coords.
  #[inline]
  #[must_use]
  fn get_position_offset(&self, world_coords: Vector3) -> Vector3 {
    unsafe { entity::get_offset_from_entity_given_world_coords(self.handle(), world_coords) }
  }

  /// Gets the entity's speed.
  ///
  /// Speed is in m/s.
  #[inline]
  #[must_use]
  fn speed(&self) -> f32 {
    unsafe { entity::get_entity_speed(self.handle()) }
  }

  /// Sets the entity's speed.
  ///
  /// Speed is in m/s.
  #[inline]
  fn set_speed(&self, speed: f32) {
    let velocity = self.velocity().normalized();
    self.set_velocity(velocity * speed)
  }

  /// Gets the velocity of the entity.
  #[inline]
  #[must_use]
  fn velocity(&self) -> Vector3 {
    unsafe { entity::get_entity_velocity(self.handle()) }
  }

  /// Sets the velocity of the entity.
  #[inline]
  fn set_velocity(&self, velocity: Vector3) {
    unsafe { entity::set_entity_velocity(self.handle(), velocity) }
  }

  /// Sets the max speed of the entity.
  ///
  /// Speed is in m/s.
  #[inline]
  fn set_max_speed(&self, max_speed: f32) {
    unsafe { entity::set_entity_max_speed(self.handle(), max_speed) }
  }

  /// Checks if the entity is visible.
  #[inline]
  #[must_use]
  fn visible(&self) -> bool {
    unsafe { entity::is_entity_visible(self.handle()) }
  }

  /// Sets the visibility of the entity.
  #[inline]
  fn set_visible(&self, visible: bool) {
    unsafe { entity::set_entity_visible(self.handle(), visible, false) }
  }

  /// Makes the entity invincible.
  #[inline]
  fn set_invincible(&self, invincible: bool) {
    unsafe { entity::set_entity_invincible(self.handle(), invincible) }
  }

  /// Checks if collisions are enabled for the entity.
  #[inline]
  #[must_use]
  fn has_collisions(&self) -> bool {
    unsafe { !entity::get_entity_collision_disabled(self.handle()) }
  }

  /// Toggles collisions for the entity.
  #[inline]
  fn set_collisions(&self, enable: bool) {
    unsafe { entity::set_entity_collision(self.handle(), enable, true) }
  }

  #[inline]
  #[must_use]
  fn bones(&self) -> EntityBones {
    EntityBones::new(self.handle())
  }
}
