use std::ffi::c_void;

use scripthookv::memory::MemoryLocation;

use crate::global_memory::GET_SCRIPT_ENTITY_SAFE;

pub fn get_script_entity_safe(handle: i32) -> Option<MemoryLocation> {
  unsafe {
    let function = GET_SCRIPT_ENTITY_SAFE
      .get()
      .unwrap()
      .cast::<extern "C" fn(handle: i32) -> *mut c_void>();
    let address = function(handle);
    if !address.is_null() {
      Some(MemoryLocation::new(address as usize))
    } else {
      None
    }
  }
}
