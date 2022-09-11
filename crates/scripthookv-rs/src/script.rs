use crate::{scripting_backend::BACKEND, ModuleHandle};

/// Removes all script created with the given module instance.
///
/// ```
/// /* in DllMain */
/// remove_script(instance);
/// ```
#[inline]
pub(crate) fn remove_script(module: ModuleHandle) {
  unsafe {
    BACKEND
      .get()
      .expect("runtime not set")
      .script_unregister(module)
  }
}
