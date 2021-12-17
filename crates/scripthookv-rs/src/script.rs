use std::time::Duration;
use winapi::shared::minwindef::HINSTANCE;
use std::os::raw::c_void;

use shv_bindings::{scriptWait, scriptRegister, scriptUnregister, scriptRegisterAdditionalThread};

/// Pauses script execution for the specified duration.
/// 
/// ```
/// /* wait 100ms */
/// script_wait(Duration::from_millis(100))
/// ```
pub fn script_wait(duration: Duration) {
  unsafe {
    scriptWait(duration.as_millis() as u32)
  }
}

/// Pauses script execution for 1 game tick.
/// 
/// ```
/// loop {
///   /* run code in a loop */
/// 
///   script_yield();
/// }
/// ```
pub fn script_yield() {
  script_wait(Duration::from_millis(0))
}

/// Used for registering your main script function.
/// 
/// ```
/// extern "C" fn script_main() {
///   /* script logic... */
/// }
/// 
/// /* in DllMain */
/// register_script(instance, script_main);
/// ```
pub fn register_script(module: HINSTANCE, entrypoint: extern "C" fn()) {
  unsafe {
    scriptRegister(module as *const c_void, entrypoint)
  }
}

/// Used for registering secondary script functions.
/// 
/// ```
/// extern "C" fn background_script() {
///   /* do some tasks in the background */
/// }
/// 
/// /* in DllMain */
/// register_additional_script_thread(instance, background_script);
/// ```
pub fn register_additional_script_thread(module: HINSTANCE, entrypoint: extern "C" fn()) {
  unsafe {
    scriptRegisterAdditionalThread(module as *const c_void, entrypoint)
  }
}

/// Removes all script created with the given module instance.
/// 
/// ```
/// /* in DllMain */
/// remove_script(instance);
/// ```
pub fn remove_script(module: HINSTANCE) {
  unsafe {
    scriptUnregister(module as *const c_void)
  }
}
