use shv_bindings::scriptUnregister;

use crate::ModuleHandle;

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
