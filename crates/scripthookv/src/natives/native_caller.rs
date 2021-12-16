use std::mem::size_of;
use shv_bindings::{nativeInit, nativePush64, nativeCall};

pub type NativeHash = u64;

pub fn native_init(hash: NativeHash) {
  unsafe {
    nativeInit(hash)
  }
}

pub unsafe fn native_push<T: Copy>(value: &T) {
  let size = size_of::<T>();
  let chunks = (size / 8) + ((size % 8).clamp(0, 1));

  for i in 0..chunks {
    let ptr    = (value as *const T) as *const u64;
    let buffer = *ptr.add(i);

    nativePush64(buffer);
  }
}

pub unsafe fn native_call<T: Copy>() -> T {
  let result_pointer = nativeCall();
  *(result_pointer as *mut T)
}
