use async_trait::async_trait;
use log::{LevelFilter, Metadata, Record};
use scripthookv::{
  scripting::Script,
  shv_entrypoint, ModuleHandle, ScriptHookV, ScriptHookVBuilder,
};
use scripthookv_gta::{
  gta::{
    entities::{Entity, Vehicle},
    Model, game, misc
  },
  ScriptHookVGtaPlugin,
};
use winapi::um::{
  consoleapi::AllocConsole,
  wincon::GetConsoleWindow,
  winuser::{ShowWindow, SW_SHOW},
};

struct MyScript;

#[async_trait]
impl Script for MyScript {
  async fn start(&mut self) {}

  async fn update(&mut self) {
    if misc::has_cheat_code_just_been_entered("test") {
      let player_ped = game::get_character().unwrap();
      let coords = player_ped.position();
      let heading = player_ped.heading();
      let adder = Model::try_from("adder").unwrap();

      adder.load_async().await.unwrap();

      let vehicle = Vehicle::create(adder, coords, heading).unwrap();
      vehicle.set_explosion_proof(true);
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
