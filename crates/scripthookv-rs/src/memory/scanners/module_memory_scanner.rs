use super::{MemoryScanner, ScannableNotFoundError};
use crate::{
  memory::{MemoryLocation, Scannable},
  try_winapi,
  winapi::WinApiError,
  ModuleHandle
};
use std::{ffi::c_void, mem::size_of, ptr};
use winapi::{
  shared::minwindef::HMODULE,
  um::{
    libloaderapi::GetModuleHandleA,
    processthreadsapi::GetCurrentProcess,
    psapi::{GetModuleInformation, MODULEINFO}
  }
};

fn get_module_size(module: ModuleHandle) -> Result<usize, WinApiError> {
  let mut module_info = MODULEINFO {
    EntryPoint:  ptr::null_mut(),
    SizeOfImage: 0,
    lpBaseOfDll: ptr::null_mut()
  };
  unsafe {
    try_winapi!(GetModuleInformation(
      GetCurrentProcess(),
      module as HMODULE,
      &mut module_info,
      size_of::<MODULEINFO>() as u32
    ));
    Ok(module_info.SizeOfImage as usize)
  }
}

pub struct ModuleMemoryScanner {
  module: ModuleHandle
}

impl MemoryScanner for ModuleMemoryScanner {
  fn scan(&self, scannable: &dyn Scannable) -> Result<MemoryLocation, ScannableNotFoundError> {
    let memory = self.module as *const u8;

    let bytes = &scannable.get_bytes();
    let mask = &scannable.get_mask();

    let module_size = get_module_size(self.module).map_err(|_| ScannableNotFoundError {})?;

    if module_size < scannable.len() {
      return Err(ScannableNotFoundError {});
    }

    let scan_range = module_size - scannable.len();
    for i in 0..scan_range {
      for j in 0..scannable.len() {
        unsafe {
          if ptr::read(memory.add(i + j)) != bytes[j] && mask[j] != '?' {
            break;
          }
        }

        if (j + 1) == scannable.len() {
          unsafe {
            return Ok(MemoryLocation::new(
              memory.offset(i.try_into().unwrap()) as usize
            ));
          }
        }
      }
    }

    Err(ScannableNotFoundError {})
  }
}

impl ModuleMemoryScanner {
  pub fn new(module: ModuleHandle) -> Self {
    Self { module }
  }
}

impl Default for ModuleMemoryScanner {
  fn default() -> Self {
    unsafe { ModuleMemoryScanner::new(GetModuleHandleA(ptr::null_mut()) as *const c_void) }
  }
}
