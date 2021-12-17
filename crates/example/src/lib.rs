use scripthookv::{register_script, remove_script, script_yield};
use natives::{misc, vehicle, player, entity, streaming};
use std::ffi::CString;
use winapi::shared::minwindef::{HINSTANCE, DWORD, LPVOID, BOOL};

mod natives;

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

#[no_mangle]
#[allow(non_snake_case)]
pub extern "stdcall" fn DllMain(
  instance: HINSTANCE,
  reason: DWORD,
  _reserved: LPVOID,
) ->BOOL {
  match reason {
    winapi::um::winnt::DLL_PROCESS_ATTACH => {
      register_script(instance, script_main);
      1
    }
    winapi::um::winnt::DLL_PROCESS_DETACH => {
      remove_script(instance);
      1
    },
    _ => 1,
  }
}
