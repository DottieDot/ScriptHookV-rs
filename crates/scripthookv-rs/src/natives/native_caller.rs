use shv_bindings::{nativeCall, nativeInit, nativePush64};
use std::mem::size_of;

pub type NativeHash = u64;

/// Start a native call
#[inline]
pub fn native_init(hash: NativeHash) {
  unsafe { nativeInit(hash) }
}

/// Add a parameter to the current native call
/// 
/// # Safety
/// This function will split the provided struct up in chunks of 8 bytes and pass them as individual parameters.
#[inline]
pub unsafe fn native_push<T: Copy>(value: &T) {
  let size = size_of::<T>();
  let chunks = (size / 8) + ((size % 8).clamp(0, 1));

  for i in 0..chunks {
    let ptr = (value as *const T) as *const u64;
    let buffer = *ptr.add(i);

    nativePush64(buffer);
  }
}

/// Confirm the current native call and cast its result.
/// 
/// # Safety
/// This will function will convert the return value to `T` without any checks. It is the responsibility of the developer to verify if the return type is correct.
#[inline]
pub unsafe fn native_call<T: Copy>() -> T {
  let result_pointer = nativeCall();
  *(result_pointer as *mut T)
}
