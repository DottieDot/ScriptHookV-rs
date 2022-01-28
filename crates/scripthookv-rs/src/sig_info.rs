use crate::memory::{MemoryLocation, MemoryScanner, ScannableNotFoundError, Scannable};

pub struct SigInfo {
  pub name: String,
  scannable: Box<dyn Scannable>,
  offset: fn(MemoryLocation) -> MemoryLocation
}

impl SigInfo {
  pub fn new(name: String, scannable: Box<dyn Scannable>, offset: fn(MemoryLocation) -> MemoryLocation) -> Self {
    Self {
      name,
      scannable,
      offset
    }
  }

  pub fn run(&self, scanner: &dyn MemoryScanner) -> Result<MemoryLocation, ScannableNotFoundError> {
    scanner.scan(self.scannable.as_ref())
      .map(|l| (self.offset)(l))
  }
}
