use std::task::Poll;

use scripthookv::{scripting::ScriptFuture, types::Hash};

use super::game::generate_hash;
use crate::natives::*;

#[must_use]
#[derive(Debug, Clone, Copy)]
pub struct Model {
  hash: Hash
}

impl Model {
  /// Returns the hash of the model
  #[inline]
  #[must_use]
  pub fn hash(&self) -> Hash {
    self.hash
  }

  /// Requests the model to be loaded.
  #[inline]
  pub fn request(&self) {
    unsafe { streaming::request_model(self.hash) }
  }

  /// Checks if the model has loaded.
  #[inline]
  #[must_use]
  pub fn loaded(&self) -> bool {
    unsafe { streaming::has_model_loaded(self.hash) }
  }

  /// Loads the model asynchronously
  pub async fn load(&self) {
    self.request();
    ScriptFuture::new(move || {
      if !self.loaded() {
        Poll::Pending
      } else {
        Poll::Ready(())
      }
    })
    .await
  }

  /// Checks if the model is a vehicle.
  #[inline]
  #[must_use]
  pub fn is_vehicle(&self) -> bool {
    unsafe { streaming::is_model_a_vehicle(self.hash) }
  }

  /// Checks if the model is a ped.
  #[inline]
  #[must_use]
  pub fn is_ped(&self) -> bool {
    unsafe { streaming::is_model_a_ped(self.hash) }
  }

  /// Checks if the model is valid
  #[inline]
  #[must_use]
  pub fn is_valid(&self) -> bool {
    unsafe { streaming::is_model_valid(self.hash) }
  }
}

impl std::fmt::Display for Model {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{:#010x}", self.hash)
  }
}

#[derive(Debug)]
pub struct InvalidModelError {
  hash: Hash
}

impl std::fmt::Display for InvalidModelError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "hash {:#010x} is not a valid model", self.hash)
  }
}

impl TryFrom<Hash> for Model {
  type Error = InvalidModelError;

  fn try_from(value: Hash) -> Result<Self, Self::Error> {
    unsafe {
      if streaming::is_model_valid(value) && streaming::is_model_in_cdimage(value) {
        Ok(Model { hash: value })
      } else {
        Err(InvalidModelError { hash: value })
      }
    }
  }
}

impl TryFrom<&str> for Model {
  type Error = InvalidModelError;

  #[inline]
  fn try_from(value: &str) -> Result<Self, Self::Error> {
    Model::try_from(generate_hash(value))
  }
}
