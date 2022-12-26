use std::{ffi::CString, task::Poll};

use scripthookv::scripting::ScriptFuture;

use crate::natives::graphics;

#[derive(Clone)]
pub struct TextureDict {
  pub name: String,
  cstring:  CString
}

impl TextureDict {
  pub fn new(name: impl Into<String>) -> Self {
    let name: String = name.into();
    let cstring = CString::new(name.as_str()).unwrap();
    Self { name, cstring }
  }

  /// Requests the texture dictionary
  pub fn request(&self) {
    unsafe { graphics::request_streamed_texture_dict(self.cstring.as_ptr(), false) }
  }

  /// Checks if the texture dictionary has loaded
  pub fn loaded(&self) -> bool {
    unsafe { graphics::has_streamed_texture_dict_loaded(self.cstring.as_ptr()) }
  }

  /// Sets the texture dictionary as no longer needed
  pub fn no_longer_needed(&self) {
    unsafe { graphics::set_streamed_texture_dict_as_no_longer_needed(self.cstring.as_ptr()) }
  }

  /// Loads the texture dictionary asynchronously
  pub async fn load(&self) {
    self.request();
    ScriptFuture::new(move || {
      if !self.loaded() {
        Poll::Pending
      } else {
        Poll::Ready(())
      }
    })
    .await
  }
}
