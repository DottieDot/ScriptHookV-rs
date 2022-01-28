use std::collections::HashMap;

use crate::memory::MemoryLocation;

#[derive(Default)]
pub struct MemoryDatabase {
  locations: HashMap<String, MemoryLocation>
}

impl MemoryDatabase {
  pub fn add(&mut self, name: String, location: MemoryLocation) {
    self.locations.insert(name, location);
  }

  pub fn get(&self, name: &str) -> Option<&MemoryLocation> {
    self.locations.get(name)
  }
}
