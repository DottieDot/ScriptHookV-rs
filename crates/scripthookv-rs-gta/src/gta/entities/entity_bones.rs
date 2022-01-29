use std::ffi::CString;

use super::EntityBone;

use crate::natives::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EntityBones {
  entity_handle: i32
}

impl EntityBones {
  #[inline]
  #[must_use]
  pub(super) fn new(entity_handle: i32) -> Self {
    Self { entity_handle }
  }

  #[inline]
  #[must_use]
  pub fn bone_count(&self) -> i32 {
    unsafe { entity::_get_entity_bone_count(self.entity_handle) }
  }

  #[inline]
  #[must_use]
  pub fn has_bone_with_name(&self, bone_name: &str) -> bool {
    self.get_bone_index_by_name(bone_name) != -1
  }

  #[inline]
  #[must_use]
  pub fn is_bone_index_valid(&self, bone_index: i32) -> bool {
    bone_index >= 0 && bone_index < self.bone_count()
  }

  #[inline]
  #[must_use]
  pub fn get_bone(&self, bone_index: i32) -> Option<EntityBone> {
    self
      .is_bone_index_valid(bone_index)
      .then(|| EntityBone::new(self.entity_handle, bone_index))
  }

  #[inline]
  #[must_use]
  pub fn get_bone_by_name(&self, bone_name: &str) -> Option<EntityBone> {
    self.get_bone(self.get_bone_index_by_name(bone_name))
  }

  #[inline]
  #[must_use]
  fn get_bone_index_by_name(&self, bone_name: &str) -> i32 {
    let cstring = CString::new(bone_name).expect("CString::new failed");
    unsafe { entity::get_entity_bone_index_by_name(self.entity_handle, cstring.as_ptr()) }
  }
}

pub struct EntityBonesIter {
  bones:   EntityBones,
  current: i32
}

impl Iterator for EntityBonesIter {
  type Item = EntityBone;

  #[inline]
  #[must_use]
  fn next(&mut self) -> Option<Self::Item> {
    let index = self.current;

    if let Some(bone) = self.bones.get_bone(index) {
      self.current += 1;
      Some(bone)
    } else {
      None
    }
  }
}
