use std::{ffi::CString, time::Duration};

use scripthookv::types::Vector3;
use thiserror::Error;

use crate::natives::*;

use super::entities::Entity;

#[derive(Copy, Clone)]
pub struct Cam {
  handle: i32
}

impl Cam {
  #[inline]
  pub fn create(
    name: &str,
    position: Vector3,
    rotation: Vector3,
    fov: f32
  ) -> Result<Self, FailedToCreateCamError> {
    let cstring = CString::new(name).expect("CString::new failed");
    let handle =
      unsafe { cam::create_cam_with_params(cstring.as_ptr(), position, rotation, fov, true, 2) };
    if unsafe { cam::does_cam_exist(handle) } {
      Ok(Self { handle })
    } else {
      Err(FailedToCreateCamError)
    }
  }

  #[inline]
  #[must_use]
  pub fn handle(&self) -> i32 {
    self.handle
  }

  #[inline]
  pub fn delete(&self) {
    if self.exists() {
      if self.is_active() {
        self.set_active(false)
      }
      unsafe { cam::destroy_cam(self.handle, false) }
    }
  }

  #[inline]
  #[must_use]
  pub fn exists(&self) -> bool {
    unsafe { cam::does_cam_exist(self.handle) }
  }

  #[inline]
  pub fn set_active(&self, toggle: bool) {
    unsafe { cam::set_cam_active(self.handle, toggle) }
  }

  #[inline]
  pub fn set_active_interpolated(
    &self,
    from: Cam,
    duration: Duration,
    ease_position: i32,
    ease_rotation: i32
  ) {
    unsafe {
      cam::set_cam_active_with_interp(
        self.handle,
        from.handle(),
        duration.as_millis() as i32,
        ease_position,
        ease_rotation
      )
    }
  }

  #[inline]
  #[must_use]
  pub fn is_interpolating(&self) -> bool {
    unsafe { cam::is_cam_interpolating(self.handle) }
  }

  #[inline]
  #[must_use]
  pub fn is_active(&self) -> bool {
    unsafe { cam::is_cam_active(self.handle) }
  }

  #[inline]
  #[must_use]
  pub fn is_rendering(&self) -> bool {
    unsafe { cam::is_cam_rendering(self.handle) }
  }

  #[inline]
  #[must_use]
  pub fn position(&self) -> Vector3 {
    unsafe { cam::get_cam_coord(self.handle) }
  }

  #[inline]
  pub fn set_position(&self, coords: Vector3) {
    unsafe { cam::set_cam_coord(self.handle, coords) }
  }

  #[inline]
  #[must_use]
  pub fn rotation(&self) -> Vector3 {
    unsafe { cam::get_cam_rot(self.handle, 2) }
  }

  #[inline]
  pub fn set_rotation(&self, rotation: Vector3) {
    unsafe { cam::set_cam_rot(self.handle, rotation, 2) }
  }

  #[inline]
  #[must_use]
  pub fn fov(&self) -> f32 {
    unsafe { cam::get_cam_fov(self.handle) }
  }

  #[inline]
  pub fn set_fov(&self, fov: f32) {
    unsafe { cam::set_cam_fov(self.handle, fov) }
  }

  #[inline]
  #[must_use]
  pub fn near_clip(&self) -> f32 {
    unsafe { cam::get_cam_near_clip(self.handle) }
  }

  #[inline]
  pub fn set_near_clip(&self, near_clip: f32) {
    unsafe { cam::set_cam_near_clip(self.handle, near_clip) }
  }

  #[inline]
  #[must_use]
  pub fn far_clip(&self) -> f32 {
    unsafe { cam::get_cam_far_clip(self.handle) }
  }

  #[inline]
  pub fn set_far_clip(&self, far_clip: f32) {
    unsafe { cam::set_cam_far_clip(self.handle, far_clip) }
  }

  #[inline]
  #[must_use]
  pub fn far_dof(&self) -> f32 {
    unsafe { cam::get_cam_far_dof(self.handle) }
  }

  #[inline]
  pub fn set_far_dof(&self, dof: f32) {
    unsafe { cam::set_cam_far_dof(self.handle, dof) }
  }

  #[inline]
  pub fn set_near_dof(&self, dof: f32) {
    unsafe { cam::set_cam_near_dof(self.handle, dof) }
  }

  #[inline]
  pub fn set_motion_blur_strength(&self, strength: f32) {
    unsafe { cam::set_cam_motion_blur_strength(self.handle, strength) }
  }

  #[inline]
  pub fn set_dof_strength(&self, strength: f32) {
    unsafe { cam::set_cam_dof_strength(self.handle, strength) }
  }

  #[inline]
  pub fn set_use_shallow_dof_node(&self, toggle: bool) {
    unsafe { cam::set_cam_use_shallow_dof_mode(self.handle, toggle) }
  }

  #[inline]
  pub fn point_at_position(&self, coords: Vector3) {
    unsafe { cam::point_cam_at_coord(self.handle, coords) }
  }

  #[inline]
  pub fn point_at_entity(&self, entity: &impl Entity, offset: Vector3) {
    unsafe {
      cam::point_cam_at_entity(
        self.handle,
        entity.handle(),
        offset.x,
        offset.y,
        offset.z,
        true
      )
    }
  }

  #[inline]
  pub fn stop_pointing(&self) {
    unsafe { cam::stop_cam_pointing(self.handle) }
  }

  #[inline]
  pub fn set_affects_aiming(&self, toggle: bool) {
    unsafe { cam::set_cam_affects_aiming(self.handle, toggle) }
  }

  #[inline]
  pub fn is_shaking(&self) -> bool {
    unsafe { cam::is_cam_shaking(self.handle) }
  }

  #[inline]
  pub fn shake(&self, shake_type: &str, amplitude: f32) {
    let cstring = CString::new(shake_type).expect("CString::new failed");
    unsafe { cam::shake_cam(self.handle, cstring.as_ptr(), amplitude) }
  }

  #[inline]
  pub fn stop_shaking(&self) {
    unsafe { cam::stop_cam_shaking(self.handle, true) }
  }

  #[inline]
  pub fn attach_to_entity(&self, entity: &impl Entity, offset: Vector3, relative: bool) {
    unsafe { cam::attach_cam_to_entity(self.handle, entity.handle(), offset, relative) }
  }
}

impl TryFrom<i32> for Cam {
  type Error = InvalidCamHandleError;

  fn try_from(handle: i32) -> Result<Self, Self::Error> {
    if unsafe { cam::does_cam_exist(handle) } {
      Ok(Cam { handle })
    } else {
      Err(InvalidCamHandleError { handle })
    }
  }
}

#[derive(Error, Debug)]
#[error("{handle} is not a cam handle")]
pub struct InvalidCamHandleError {
  handle: i32
}

#[derive(Error, Debug)]
#[error("Failed to create cam")]
pub struct FailedToCreateCamError;
