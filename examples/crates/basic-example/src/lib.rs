use async_trait::async_trait;
use log::{LevelFilter, Metadata, Record};
use scripthookv::{
  scripting::{Script},
  shv_entrypoint, ModuleHandle, ScriptHookV, ScriptHookVBuilder,
};
use scripthookv_gta::{
  gta::{
    entities::{Entity, Vehicle},
    Model,
  },
  ScriptHookVGtaPlugin,
};
use std::ffi::CString;
use winapi::um::{
  consoleapi::AllocConsole,
  wincon::GetConsoleWindow,
  winuser::{ShowWindow, SW_SHOW},
};

mod natives;
use natives::*;

struct MyScript;

#[async_trait]
impl Script for MyScript {
  async fn start(&mut self) {}

  async fn update(&mut self) {
    unsafe {
      let test = CString::new("test").unwrap();
      let test_hash = misc::get_hash_key(test.as_ptr());

      if misc::_has_cheat_string_just_been_entered(test_hash) != 0 {
        let player_ped = player::player_ped_id();
        let coords = entity::get_entity_coords(player_ped, 1);
        let heading = entity::get_entity_heading(player_ped);
        let adder = Model::try_from("adder").unwrap();

        adder.load_async().await.unwrap();

        let vehicle = Vehicle::create(adder, coords, heading).unwrap();
        vehicle.set_explosion_proof(true);
      }
    }
  }

  async fn cleanup(&mut self) {}
}
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

static LOGGER: SimpleLogger = SimpleLogger;

#[shv_entrypoint]
fn entrypoint(module: ModuleHandle) -> ScriptHookV<'static> {
  unsafe {
    AllocConsole();
    ShowWindow(GetConsoleWindow(), SW_SHOW);
  }

  log::set_logger(&LOGGER)
    .map(|_| log::set_max_level(LevelFilter::Trace))
    .unwrap();

  ScriptHookVBuilder::new(module)
    .plugin(ScriptHookVGtaPlugin)
    .script(MyScript)
    .build()
}
