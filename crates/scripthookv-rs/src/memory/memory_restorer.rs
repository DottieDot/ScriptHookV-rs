use super::MemoryLocation;
use std::ptr;

#[derive(Debug)]
struct ModifiedBytes {
  location: MemoryLocation,
  bytes:    Vec<u8>
}

#[derive(Debug)]
pub struct MemoryRestorer {
  modified_memory: Vec<ModifiedBytes>
}

impl MemoryRestorer {
  /// # Safety
  /// The caller has to ensure the validity of the given memory address.
  pub unsafe fn save_bytes(&mut self, location: MemoryLocation, count: usize) {
    let raw = location.cast::<*mut u8>();
    let bytes = Vec::from_raw_parts(raw, count, count);

    self.modified_memory.push(ModifiedBytes { location, bytes });
  }

  /// # Safety
  /// The caller has to ensure that the memory to be restored is still valid.
  pub unsafe fn restore_memory(&mut self) {
    for mem in &self.modified_memory {
      let src = mem.bytes.as_ptr();
      let dst = mem.location.cast::<*mut u8>();
      ptr::copy_nonoverlapping(src, dst, mem.bytes.len());
    }
    self.modified_memory.clear();
  }
}
