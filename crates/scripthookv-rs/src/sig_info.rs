use crate::memory::{MemoryLocation, MemoryScanner, Scannable, ScannableNotFoundError};
use log::debug;

pub struct SigInfo {
  pub name:  String,
  scannable: Box<dyn Scannable>,
  offset:    fn(MemoryLocation) -> MemoryLocation
}

impl SigInfo {
  pub fn new(
    name: String,
    scannable: Box<dyn Scannable>,
    offset: fn(MemoryLocation) -> MemoryLocation
  ) -> Self {
    Self {
      name,
      scannable,
      offset
    }
  }

  pub fn run(&self, scanner: &dyn MemoryScanner) -> Result<MemoryLocation, ScannableNotFoundError> {
    scanner.scan(self.scannable.as_ref()).map(|l| {
      debug!("Offsetting {l} for {}", self.name);
      (self.offset)(l)
    })
  }
}
