use log::{Metadata, Record, LevelFilter};
use scripthookv::{script_yield, shv_entrypoint, ModuleHandle, ScriptHookV, ScriptHookVBuilder};
use scripthookv_gta::{
  entities::{Entity, Vehicle},
  ScriptHookVGtaPlugin
};
use std::ffi::CString;
use winapi::um::{consoleapi::AllocConsole, winuser::{ShowWindow, SW_SHOW}, wincon::GetConsoleWindow};

mod natives;
use natives::*;

struct SimpleLogger;

impl log::Log for SimpleLogger {
  fn enabled(&self, _: &Metadata) -> bool {
    true
  }

  fn log(&self, record: &Record) {
    if self.enabled(record.metadata()) {
      println!("{} - {}", record.level(), record.args());
    }
  }

  fn flush(&self) {}
}

extern "C" fn script_main() {
  unsafe {
    let test = CString::new("test").unwrap();
    let adder = CString::new("adder").unwrap();
    let test_hash = misc::get_hash_key(test.as_ptr());
    let adder_hash = misc::get_hash_key(adder.as_ptr());

    loop {
      if misc::_has_cheat_string_just_been_entered(test_hash) != 0 {
        let player_ped = player::player_ped_id();
        let coords = entity::get_entity_coords(player_ped, 1);
        let heading = entity::get_entity_heading(player_ped);

        streaming::request_model(adder_hash);
        while streaming::has_model_loaded(adder_hash) == 0 {
          script_yield();
        }

        let vehicle = Vehicle::create(adder_hash.try_into().unwrap(), coords, heading).unwrap();
        vehicle.set_explosion_proof(true);
      }

      script_yield();
    }
  }
}

static LOGGER: SimpleLogger = SimpleLogger;

#[shv_entrypoint]
fn entrypoint(module: ModuleHandle) -> ScriptHookV {
  unsafe {
    AllocConsole();
    ShowWindow(GetConsoleWindow(), SW_SHOW);
  }

  log::set_logger(&LOGGER)
    .map(|_| log::set_max_level(LevelFilter::Trace))
    .unwrap();

  ScriptHookVBuilder::new(module)
    .plugin(ScriptHookVGtaPlugin)
    .script(script_main)
    .build()
}
