use std::{ffi::CString, path::Path, time::Duration};

use shv_bindings::{createTexture, drawTexture};

use crate::types::Vector2;

/// Used for drawing custom textures.
pub struct Texture {
  id: i32
}

impl Texture {
  /// Creates a new texture from a file path.
  pub fn create(texture_file: &Path) -> Self {
    unsafe {
      let path = CString::new(
        texture_file
          .as_os_str()
          .to_str()
          .expect("Path includes invalid unicode")
      )
      .expect("CString::from failed");

      Self {
        id: createTexture(path.as_ptr())
      }
    }
  }

  /// Draws the texture
  #[inline]
  #[allow(clippy::too_many_arguments)]
  pub fn draw(
    &self,
    index: i32,
    level: i32,
    time: Duration,
    size: Vector2,
    center: Vector2,
    position: Vector2,
    rotation: f32,
    screen_height_scale_factor: f32,
    r: f32,
    g: f32,
    b: f32,
    a: f32
  ) {
    unsafe {
      drawTexture(
        self.id,
        index,
        level,
        time.as_millis() as i32,
        size.x,
        size.y,
        center.x,
        center.y,
        position.x,
        position.y,
        rotation,
        screen_height_scale_factor,
        r,
        g,
        b,
        a
      )
    }
  }
}
