use std::time::Duration;

use shv_bindings::{scriptRegister, scriptRegisterAdditionalThread, scriptUnregister, scriptWait};

use crate::ModuleHandle;

/// Pauses script execution for the specified duration.
///
/// ```
/// /* wait 100ms */
/// script_wait(Duration::from_millis(100))
/// ```
#[inline]
pub fn script_wait(duration: Duration) {
  unsafe { scriptWait(duration.as_millis() as u32) }
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
#[inline]
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
#[inline]
pub(crate) fn register_script(module: ModuleHandle, entrypoint: extern "C" fn()) {
  unsafe { scriptRegister(module, entrypoint) }
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
#[inline]
pub(crate) fn register_additional_script_thread(module: ModuleHandle, entrypoint: extern "C" fn()) {
  unsafe { scriptRegisterAdditionalThread(module, entrypoint) }
}

/// Removes all script created with the given module instance.
///
/// ```
/// /* in DllMain */
/// remove_script(instance);
/// ```
#[inline]
pub(crate) fn remove_script(module: ModuleHandle) {
  unsafe { scriptUnregister(module) }
}
