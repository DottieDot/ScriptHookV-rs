use shv_bindings::{PresentCallback, KeyboardHandler, presentCallbackRegister, presentCallbackUnregister, keyboardHandlerRegister, keyboardHandlerUnregister};


pub fn register_present_callback(cb: PresentCallback) {
  unsafe {
    presentCallbackRegister(cb)
  }
}

pub fn remove_present_callback(cb: PresentCallback) {
  unsafe {
    presentCallbackUnregister(cb)
  }
}

pub fn register_keyboard_handler(handler: KeyboardHandler) {
  unsafe {
    keyboardHandlerRegister(handler)
  }
}

pub fn remove_keyboard_handler(handler: KeyboardHandler) {
  unsafe {
    keyboardHandlerUnregister(handler)
  }
}
