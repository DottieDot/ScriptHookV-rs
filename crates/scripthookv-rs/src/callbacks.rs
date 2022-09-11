use crate::scripting_backend::{KeyboardHandler, PresentCallback, BACKEND};

/// Wrapper for presentCallbackRegister
#[inline]
pub(crate) fn register_present_callback(cb: PresentCallback) {
  unsafe {
    BACKEND
      .get()
      .expect("runtime not set")
      .present_callback_register(cb)
  }
}

/// Wrapper for presentCallbackUnregister
#[inline]
pub(crate) fn remove_present_callback(cb: PresentCallback) {
  unsafe {
    BACKEND
      .get()
      .expect("runtime not set")
      .present_callback_unregister(cb)
  }
}

/// Wrapper for register_keyboard_handler
#[inline]
pub(crate) fn register_keyboard_handler(handler: KeyboardHandler) {
  unsafe {
    BACKEND
      .get()
      .expect("runtime not set")
      .keyboard_handler_register(handler)
  }
}

/// Wrapper for remove_keyboard_handler
#[inline]
pub(crate) fn remove_keyboard_handler(handler: KeyboardHandler) {
  unsafe {
    BACKEND
      .get()
      .expect("runtime not set")
      .keyboard_handler_unregister(handler)
  }
}
