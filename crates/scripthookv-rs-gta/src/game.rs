use joaat;
use scripthookv::types::Hash;

use crate::{entities::Ped, natives::*, Player};

pub fn generate_hash(string: &str) -> Hash {
  joaat::hash_ascii_lowercase(string.as_bytes())
}

pub fn get_character() -> Option<Ped> {
  get_player().character()
}

pub fn get_player() -> Player {
  let player = unsafe { player::player_id() };
  Player::try_from(player).expect("invalid player handle")
}
