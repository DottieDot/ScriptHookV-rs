use shv_bindings::{
  keyboardHandlerRegister, keyboardHandlerUnregister, presentCallbackRegister,
  presentCallbackUnregister, KeyboardHandler, PresentCallback
};

/// Wrapper for presentCallbackRegister
#[inline]
pub(crate) fn register_present_callback(cb: PresentCallback) {
  unsafe { presentCallbackRegister(cb) }
}

/// Wrapper for presentCallbackUnregister
#[inline]
pub(crate) fn remove_present_callback(cb: PresentCallback) {
  unsafe { presentCallbackUnregister(cb) }
}

/// Wrapper for register_keyboard_handler
#[inline]
pub(crate) fn register_keyboard_handler(handler: KeyboardHandler) {
  unsafe { keyboardHandlerRegister(handler) }
}

/// Wrapper for remove_keyboard_handler
#[inline]
pub(crate) fn remove_keyboard_handler(handler: KeyboardHandler) {
  unsafe { keyboardHandlerUnregister(handler) }
}
