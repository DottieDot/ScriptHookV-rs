use scripthookv::types::Vector3;

use crate::natives::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EntityBone {
  entity_handle: i32,
  bone_index:    i32
}

impl EntityBone {
  #[inline]
  #[must_use]
  pub(super) fn new(entity_handle: i32, bone_index: i32) -> Self {
    Self {
      entity_handle: entity_handle,
      bone_index
    }
  }

  #[inline]
  #[must_use]
  pub fn position(&self) -> Vector3 {
    unsafe { entity::get_world_position_of_entity_bone(self.entity_handle, self.bone_index) }
  }
}
