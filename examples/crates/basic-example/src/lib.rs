use std::{sync::Arc, thread, time::Duration};

use async_trait::async_trait;
use log::{info, LevelFilter, Metadata, Record};
use scripthookv::{
  scripting::{yield_async, Script, ScriptCommands},
  shv_entrypoint, ModuleHandle, ScriptHookV, ScriptHookVBuilder
};
use scripthookv_gta::{
  gta::{
    entities::{Entity, Vehicle},
    game, misc, Model
  },
  ScriptHookVGtaPlugin
};
use winapi::um::{
  consoleapi::AllocConsole,
  wincon::GetConsoleWindow,
  winuser::{ShowWindow, SW_SHOW}
};

struct MyScript;

#[async_trait]
impl<'rt> Script<'rt> for MyScript {
  async fn start(&mut self, _commands: Arc<ScriptCommands<'rt>>) {}

  async fn update(&mut self, commands: Arc<ScriptCommands<'rt>>) {
    if misc::has_cheat_code_just_been_entered("test") {
      info!("Start spawn task");
      commands
        .spawn_task(async move {
          let player_ped = game::get_character().unwrap();
          let coords = player_ped.position();
          let heading = player_ped.heading();
          let adder = Model::try_from("adder").unwrap();

          for _ in 0..10 {
            info!("Waiting...");
            yield_async().await;
          }

          adder.load_async().await.unwrap();

          let vehicle = Vehicle::create(adder, coords, heading).unwrap();
          vehicle.set_explosion_proof(true);
        })
        .detach();
    }
    info!("Tick");
  }

  async fn cleanup(&mut self, _commands: Arc<ScriptCommands<'rt>>) {}
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
  // Dumb hack, should improve this
  thread::spawn(|| {
    thread::sleep(Duration::from_secs(5));
    unsafe {
      AllocConsole();
      ShowWindow(GetConsoleWindow(), SW_SHOW);
    }
  });

  log::set_logger(&LOGGER)
    .map(|_| log::set_max_level(LevelFilter::Trace))
    .unwrap();

  ScriptHookVBuilder::new(module)
    .plugin(ScriptHookVGtaPlugin)
    .script(MyScript)
    .build()
}
