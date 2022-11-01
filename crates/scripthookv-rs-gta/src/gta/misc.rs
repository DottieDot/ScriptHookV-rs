use super::game;
use crate::natives::*;

pub fn has_cheat_code_hash_just_been_entered(cheat_code_hash: u32) -> bool {
  unsafe { misc::has_pc_cheat_with_hash_been_activated(cheat_code_hash) }
}

pub fn has_cheat_code_just_been_entered(cheat_code: &str) -> bool {
  let hash = game::generate_hash(cheat_code);
  has_cheat_code_hash_just_been_entered(hash)
}
