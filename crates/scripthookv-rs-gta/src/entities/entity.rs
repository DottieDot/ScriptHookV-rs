use scripthookv::types::{Entity as NativeEntity, Vector3};

use crate::natives::*;

pub trait Entity {
  /// Gets the underlying entity handle.
  fn handle(&self) -> NativeEntity;

  /// Checks if the entity exists.
  fn exists(&self) -> bool {
    unsafe {
      entity::does_entity_exist(self.handle()) != 0
    }
  }

  // Checks if the entity is dead.
  fn is_dead(&self) -> bool {
    unsafe {
      entity::is_entity_dead(self.handle(), 0) != 0
    }
  }

  // Checks if the entity is alive.
  fn is_alive(&self) -> bool {
    !self.is_dead()
  }

  /// Returns the current entity opacity.
  fn opacity(&self) -> i32 {
    unsafe {
      entity::get_entity_alpha(self.handle())
    }
  }

  /// Sets the opacity of the entity.
  /// 
  /// Changes only take effect in increments of 51 (20%).
  fn set_opacity(&self, opacity: i32) {
    if opacity == 255 {
      self.reset_opacity()
    }
    else {
      unsafe {
        entity::set_entity_alpha(self.handle(), opacity, 0);
      }
    }
  }

  /// Resets the opacity of the entity to opaque.
  fn reset_opacity(&self) {
    unsafe {
      entity::reset_entity_alpha(self.handle());
    }
  }


  /// Checks if the entity is persistent.
  fn persistent(&self) -> bool {
    unsafe {
      entity::is_entity_a_mission_entity(self.handle()) != 0
    }
  }

  /// Makes the entity persistent.
  fn persist(&self) {
    unsafe {
      entity::set_entity_as_mission_entity(self.handle(), true.into(), true.into());
    }
  }

  /// Marks the entity as no longer needed.
  fn no_longer_needed(&self) {
    unsafe {
      let mut handle = self.handle();
      entity::set_entity_as_no_longer_needed(&mut handle);
    }
  }

  /// Freezes the position of the entity.
  fn freeze_position(&self, toggle: bool) {
    unsafe {
      entity::freeze_entity_position(self.handle(), toggle.into());
    }
  }

  /// Gets the current entity health.
  /// 
  /// Standard health ranges:
  ///     Ped: 0 to 200
  /// Vehicle: 0 to 1000
  ///  Object: 0 to 1000
  /// 
  /// For peds a health level below 100 is considered fatal.
  fn health(&self) -> i32 {
    unsafe {
      entity::get_entity_health(self.handle())
    }
  }

  /// Sets the health of the entity.
  /// 
  /// Standard health ranges:
  ///     Ped: 0 to 200
  /// Vehicle: 0 to 1000
  ///  Object: 0 to 1000
  /// 
  /// For peds a health level below 100 is considered fatal.
  fn set_health(&self, health: i32) {
    unsafe {
      entity::set_entity_health(self.handle(), health, 0);
    }
  }

  /// Gets the max health of the entity.
  /// 
  /// Standard health ranges:
  ///     Ped: 0 to 200
  /// Vehicle: 0 to 1000
  ///  Object: 0 to 1000
  /// 
  /// For peds a health level below 100 is considered fatal.
  fn max_health(&self) -> i32 {
    unsafe {
      entity::get_entity_max_health(self.handle())
    }
  }

  /// Sets the max health of the entity.
  /// 
  /// Standard health ranges:
  ///     Ped: 0 to 200
  /// Vehicle: 0 to 1000
  ///  Object: 0 to 1000
  /// 
  /// For peds a health level below 100 is considered fatal.
  fn set_max_health(&self, max_health: i32) {
    unsafe {
      entity::set_entity_max_health(self.handle(), max_health);
    }
  }

  /// Gets the current location of the entity.
  fn position(&self) -> Vector3 {
    unsafe {
      entity::get_entity_coords(self.handle(), false.into())
    }
  }

  /// Sets the entity to a new position.
  /// 
  /// Might change the position slightly to better accommodate for terrain and nearby infrastructure.
  /// If this is not desired use `Entity::set_position_no_offset` instead.
  fn set_position(&self, coords: Vector3, clear_area: bool) {
    unsafe {
      entity::set_entity_coords(self.handle(), coords, false.into(), false.into(), false.into(), clear_area.into());
    }
  }

  /// Sets the entity to a new exact position.
  fn set_position_no_offset(&self, coords: Vector3) {
    unsafe {
      entity::set_entity_coords_no_offset(self.handle(), coords, false.into(), false.into(), false.into());
    }
  }

  /// Gets the current entity rotation.
  /// 
  /// Uses rotation order 2.
  fn rotation(&self) -> Vector3 {
    unsafe {
      entity::get_entity_rotation(self.handle(), 2)
    }
  }

  /// Sets the entity rotation.
  /// 
  /// uses rotation order 2.
  fn set_rotation(&self, rotation: Vector3) {
    unsafe {
      entity::set_entity_rotation(self.handle(), rotation.x, rotation.y, rotation.z, 2, 1);
    }
  }

  /// Gets the current entity heading.
  fn heading(&self) -> f32 {
    unsafe {
      entity::get_entity_heading(self.handle())
    }
  }

  /// Sets the heading of the entity.
  fn set_heading(&self, heading: f32) {
    unsafe {
      entity::set_entity_heading(self.handle(), heading);
    }
  }

  /// Gets the heading of the entity physics.
  /// 
  /// Tends to be more accurate than `Entity::heading` especially when a ped is in ragdoll.
  fn physics_heading(&self) -> f32 {
    unsafe {
      entity::_get_entity_physics_heading(self.handle())
    }
  }

  /// Gets how far the entity is submerged
  /// 
  /// A value of 1 means that the entity is fully submerged.
  fn submersion_level(&self) -> f32 {
    unsafe {
      entity::get_entity_submerged_level(self.handle())
    }
  }

  /// Gets the distance between the entity and the ground.
  fn height_above_ground(&self) -> f32 {
    unsafe {
      entity::get_entity_height_above_ground(self.handle())
    }
  }

  /// Gets the world coordinates for of a relative offset to this entity.
  fn get_offsetted_position(&self, offset: Vector3) -> Vector3 {
    unsafe {
      entity::get_offset_from_entity_in_world_coords(self.handle(), offset)
    }
  }

  /// Gets the relative position to this entity for world coords.
  fn get_position_offset(&self, world_coords: Vector3) -> Vector3 {
    unsafe {
      entity::get_offset_from_entity_given_world_coords(self.handle(), world_coords)
    }
  }

  /// Gets the entity's speed.
  /// 
  /// Speed is in m/s.
  fn speed(&self) -> f32 {
    unsafe {
      entity::get_entity_speed(self.handle())
    }
  }

  /// Sets the entity's speed.
  /// 
  /// Speed is in m/s.
  fn set_speed(&self, speed: f32) {
    let velocity = self.velocity().normalized();
    self.set_velocity(velocity * speed)
  }

  /// Gets the velocity of the entity.
  fn velocity(&self) -> Vector3 {
    unsafe {
      entity::get_entity_velocity(self.handle())
    }
  }

  /// Sets the velocity of the entity.
  fn set_velocity(&self, velocity: Vector3) {
    unsafe {
      entity::set_entity_velocity(self.handle(), velocity);
    }
  }

  /// Sets the max speed of the entity.
  /// 
  /// Speed is in m/s.
  fn set_max_speed(&self, max_speed: f32) {
    unsafe {
      entity::set_entity_max_speed(self.handle(), max_speed);
    }
  }
}
