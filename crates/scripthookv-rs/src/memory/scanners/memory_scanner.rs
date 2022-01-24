use crate::memory::{MemoryLocation, Scannable};
use thiserror::Error;

pub trait MemoryScanner {
  fn scan(&self, scannable: &dyn Scannable) -> Result<MemoryLocation, ScannableNotFoundError>;
}

#[derive(Error, Debug)]
#[error("Failed to find scannable")]
pub struct ScannableNotFoundError;
