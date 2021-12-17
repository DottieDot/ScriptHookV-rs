use std::time::Duration;
use winapi::shared::minwindef::HINSTANCE;
use std::os::raw::c_void;

use shv_bindings::{scriptWait, scriptRegister, scriptUnregister, scriptRegisterAdditionalThread};

pub fn script_wait(duration: Duration) {
  unsafe {
    scriptWait(duration.as_millis() as u32)
  }
}

pub fn script_yield() {
  script_wait(Duration::from_millis(0))
}

pub fn register_script(module: HINSTANCE, entrypoint: extern "C" fn()) {
  unsafe {
    scriptRegister(module as *const c_void, entrypoint)
  }
}

pub fn register_additional_script_thread(module: HINSTANCE, entrypoint: extern "C" fn()) {
  unsafe {
    scriptRegisterAdditionalThread(module as *const c_void, entrypoint)
  }
}

pub fn remove_script(module: HINSTANCE) {
  unsafe {
    scriptUnregister(module as *const c_void)
  }
}
