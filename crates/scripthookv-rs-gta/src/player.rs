use std::fmt;

use crate::{entities::Ped, natives::*};

pub struct Player {
  handle: i32
}

impl Player {
  #[inline]
  #[must_use]
  pub fn character(&self) -> Option<Ped> {
    let ped = unsafe { player::get_player_ped(self.handle) };
    Ped::try_from(ped).ok()
  }
}

#[derive(Debug)]
pub struct InvalidPlayerHandleError {
  handle: i32
}

impl fmt::Display for InvalidPlayerHandleError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{} is not a valid player handle", self.handle)
  }
}

impl TryFrom<i32> for Player {
  type Error = InvalidPlayerHandleError;

  #[inline]
  fn try_from(value: i32) -> Result<Self, Self::Error> {
    // TODO: Further validation
    if (0..30).contains(&value) {
      Ok(Self { handle: value })
    } else {
      Err(InvalidPlayerHandleError { handle: value })
    }
  }
}
