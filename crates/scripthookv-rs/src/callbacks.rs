use shv_bindings::{PresentCallback, KeyboardHandler, presentCallbackRegister, presentCallbackUnregister, keyboardHandlerRegister, keyboardHandlerUnregister};


/// Wrapper for presentCallbackRegister
#[inline]
pub fn register_present_callback(cb: PresentCallback) {
  unsafe {
    presentCallbackRegister(cb)
  }
}

/// Wrapper for presentCallbackUnregister
#[inline]
pub fn remove_present_callback(cb: PresentCallback) {
  unsafe {
    presentCallbackUnregister(cb)
  }
}

/// Wrapper for register_keyboard_handler
#[inline]
pub fn register_keyboard_handler(handler: KeyboardHandler) {
  unsafe {
    keyboardHandlerRegister(handler)
  }
}

/// Wrapper for remove_keyboard_handler
#[inline]
pub fn remove_keyboard_handler(handler: KeyboardHandler) {
  unsafe {
    keyboardHandlerUnregister(handler)
  }
}
