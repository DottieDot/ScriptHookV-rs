use scripthookv::shv_bindings::{scriptRegister, scriptUnregister, scriptWait};
use scripthookv::call_native;
use scripthookv::types::{Void, Hash, Bool, Ped, Vector3, Vehicle};
use winapi::um::consoleapi::AllocConsole;
use std::ffi::CString;

extern "C" fn script_main() {
  unsafe {
    let test       = CString::new("test").unwrap();
    let adder      = CString::new("adder").unwrap();
    let test_hash  = call_native!(Hash, 0xD24D37CC275948CC, test.as_ptr());
    let adder_hash = call_native!(Hash, 0xD24D37CC275948CC, adder.as_ptr());

    loop {
      if call_native!(Bool, 0x557E43C447E700A8, test_hash) != 0 {
        println!("Cheat entered");

        let player_ped = call_native!(Ped, 0xD80958FC74E988A6);
        let coords     = call_native!(Vector3, 0x3FEF770D40960D5A, player_ped, true);
        let heading    = call_native!(f32, 0xE83D4F9BA2A38914, player_ped);

        call_native!(Void, 0x963D27A58DF860AC, adder_hash);
        while call_native!(Bool, 0x98A4EB5D89A0C952, adder_hash) == 0 {
          scriptWait(0);
        }

        println!("Spawning vehicle {} at {} with heading {}", adder_hash, coords, heading);

        call_native!(Vehicle, 0xAF35D0D2583051B0, adder_hash, coords, heading, false, false, false);
      }

      scriptWait(0);
    }
  }
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "stdcall" fn DllMain(
  instance: winapi::shared::minwindef::HINSTANCE,
  reason: winapi::shared::minwindef::DWORD,
  _reserved: winapi::shared::minwindef::LPVOID,
) -> winapi::shared::minwindef::BOOL {
  match reason {
    winapi::um::winnt::DLL_PROCESS_ATTACH => {
      unsafe {
        AllocConsole();
        scriptRegister(instance as *mut ::std::os::raw::c_void, script_main);
      }
      1
    }
    winapi::um::winnt::DLL_PROCESS_DETACH => {
      unsafe {
        scriptUnregister(instance as *mut ::std::os::raw::c_void);
      }
      1
    },
    _ => 1,
  }
}
