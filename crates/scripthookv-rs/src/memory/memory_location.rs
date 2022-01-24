use super::MemoryRestorer;
use std::mem::size_of;
use std::sync::{Arc, Mutex};
use std::{iter::Iterator, ptr};

#[derive(Clone, Debug)]
pub struct MemoryLocation {
  location: usize,
  restorer: Option<Arc<Mutex<MemoryRestorer>>>
}

impl MemoryLocation {
  pub fn new(location: usize) -> Self {
    Self {
      location,
      restorer: None
    }
  }

  pub fn new_with_restorer(location: usize, restorer: Arc<Mutex<MemoryRestorer>>) -> Self {
    Self {
      location,
      restorer: Some(restorer)
    }
  }

  pub fn with_restorer(&self, restorer: Arc<Mutex<MemoryRestorer>>) -> Self {
    Self {
      location: self.location,
      restorer: Some(restorer)
    }
  }

  pub fn add(&self, offset: usize) -> Self {
    Self {
      location: self.location + offset,
      restorer: self.restorer.clone()
    }
  }

  pub fn sub(&self, offset: usize) -> Self {
    Self {
      location: self.location - offset,
      restorer: self.restorer.clone()
    }
  }

  pub fn offset(&self, offset: isize) -> Self {
    if offset >= 0 {
      self.add(offset as usize)
    } else {
      self.sub(offset as usize)
    }
  }

  pub unsafe fn get<T>(&self) -> T {
    ptr::read(self.location as *mut T)
  }

  pub unsafe fn cast<T>(&self) -> T {
    ptr::read::<T>(&self.location as *const usize as *mut T)
  }

  pub unsafe fn rip(&self, length: i8) -> Self {
    let shift = (8 - length) * 8;

    self
      .offset(length as isize)
      .offset(self.get::<isize>() << shift >> shift)
  }

  pub unsafe fn get_call_address(&self) -> Self {
    self.add(1).rip(4)
  }

  pub unsafe fn get_jmp_address(&self) -> Self {
    self.add(1).rip(4)
  }

  pub unsafe fn get_lea_address(&self) -> Self {
    self.add(3).rip(4)
  }

  pub unsafe fn set(&self, value: u8, count: usize) -> &Self {
    if let Some(restorer) = &self.restorer {
      restorer.lock().unwrap().save_bytes(self.clone(), count);
    };
    ptr::write_bytes(self.cast::<*mut u8>(), value, count);
    self
  }

  pub unsafe fn write<T>(&self, value: T) -> &Self {
    if let Some(restorer) = &self.restorer {
      restorer
        .lock()
        .unwrap()
        .save_bytes(self.clone(), size_of::<T>());
    };
    ptr::write(self.cast::<*mut T>(), value);
    self
  }

  pub unsafe fn write_iterator<T>(&self, iterator: T) -> &Self
  where
    T: Iterator
  {
    for item in iterator {
      self.write(item);
    }
    self
  }

  pub unsafe fn nop(&self, count: usize) -> &Self {
    self.set(0x90, count)
  }

  pub unsafe fn ret(&self, stack_size: u16) -> &Self {
    if stack_size > 0 {
      self.write(0x2C).add(1).write(stack_size);
    } else {
      self.write(0x3C);
    }
    self
  }
}
