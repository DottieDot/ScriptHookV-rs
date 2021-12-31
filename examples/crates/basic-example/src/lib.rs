use scripthookv::{script_yield, ScriptHookVBuilder, ScriptHookV, ModuleHandle};
use scripthookv_rs_macros::shv_entrypoint;
use std::ffi::CString;

mod natives;
use natives::*;

extern "C" fn script_main() {
  unsafe {
    let test       = CString::new("test").unwrap();
    let adder      = CString::new("adder").unwrap();
    let test_hash  = misc::get_hash_key(test.as_ptr());
    let adder_hash = misc::get_hash_key(adder.as_ptr());

    loop {
      if misc::_has_cheat_string_just_been_entered(test_hash) != 0 {
        let player_ped = player::player_ped_id();
        let coords     = entity::get_entity_coords(player_ped, 1);
        let heading    = entity::get_entity_heading(player_ped);

        streaming::request_model(adder_hash);
        while streaming::has_model_loaded(adder_hash) == 0 {
          script_yield();
        }

        vehicle::create_vehicle(adder_hash, coords, heading, 0, 0, 0);
      }

      script_yield();
    }
  }
}

#[shv_entrypoint]
fn entrypoint(module: ModuleHandle) -> ScriptHookV {
  ScriptHookVBuilder::new(module)
    .script(script_main)
    .build()
}
