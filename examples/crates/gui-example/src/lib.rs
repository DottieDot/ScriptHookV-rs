use std::{borrow::BorrowMut, cell::RefCell, sync::Arc, thread, time::Duration};

use async_trait::async_trait;
use log::{info, LevelFilter, Metadata, Record};
use scripthookv::{
  scripting::{yield_async, Script, ScriptCommands},
  shv_entrypoint, ModuleHandle, ScriptHookV, ScriptHookVBuilder, TimeoutExt
};
use scripthookv_gta::{
  gta::{
    entities::{Entity, Vehicle},
    game,
    input::Control,
    misc, Model
  },
  ScriptHookVGtaPlugin
};
use scripthookv_gui::gui::{
  options::MenuOptionBuilder, renderer::default::DefaultMenuRenderer, Menu, MenuControl,
  MenuControls, MenuKey, Submenu
};
use scripthookv_shv::ScriptHookVBackend;
use winapi::um::{
  consoleapi::AllocConsole,
  wincon::GetConsoleWindow,
  winuser::{ShowWindow, SW_SHOW}
};

struct MyScript {
  gui: Option<Menu>
}

#[async_trait(?Send)]
impl<'rt> Script<'rt> for MyScript {
  async fn start(&mut self, _commands: Arc<ScriptCommands<'rt>>) {
    let main_submenu = Submenu::new("Test", "Main Menu", |sub| {
      sub.add_multiple(vec![
        MenuOptionBuilder::default()
          .text("Hello There")
          .on_click(|o, _| println!("Clicked: {}", o.text()))
          .build(),
        MenuOptionBuilder::default().text("General Kenobi").build(),
        MenuOptionBuilder::default()
          .text("You're a bold one")
          .build(),
      ]);
      sub.add_multiple(
        (0..50)
          .map(|i| {
            MenuOptionBuilder::default()
              .text(format!("Option {i}"))
              .on_click(|o, _| println!("Clicked: {}", o.text()))
              .build()
          })
          .collect::<Vec<_>>()
      );
    });

    self.gui = Some(Menu::new(
      MenuControls {
        up:      MenuControl::new(
          MenuKey::GtaControl(Control::FrontendUp),
          MenuKey::GtaControl(Control::FrontendUp),
          true,
          Default::default()
        ),
        down:    MenuControl::new(
          MenuKey::GtaControl(Control::FrontendDown),
          MenuKey::GtaControl(Control::FrontendDown),
          true,
          Default::default()
        ),
        left:    MenuControl::new(
          MenuKey::GtaControl(Control::FrontendLeft),
          MenuKey::GtaControl(Control::FrontendLeft),
          true,
          Default::default()
        ),
        right:   MenuControl::new(
          MenuKey::GtaControl(Control::FrontendRight),
          MenuKey::GtaControl(Control::FrontendRight),
          true,
          Default::default()
        ),
        confirm: MenuControl::new(
          MenuKey::GtaControl(Control::FrontendAccept),
          MenuKey::GtaControl(Control::FrontendAccept),
          true,
          Default::default()
        ),
        back:    MenuControl::new(
          MenuKey::GtaControl(Control::FrontendCancel),
          MenuKey::GtaControl(Control::FrontendCancel),
          true,
          Default::default()
        ),
        delete:  MenuControl::new(
          MenuKey::GtaControl(Control::FrontendDelete),
          MenuKey::GtaControl(Control::FrontendDelete),
          true,
          Default::default()
        ),
        save:    MenuControl::new(
          MenuKey::GtaControl(Control::FrontendX),
          MenuKey::GtaControl(Control::FrontendX),
          true,
          Default::default()
        )
      },
      Arc::new(RefCell::new(main_submenu)),
      Box::new(DefaultMenuRenderer::default())
    ))
  }

  async fn update(&mut self, commands: Arc<ScriptCommands<'rt>>) {
    if let Some(gui) = self.gui.borrow_mut() {
      gui.process();
    }

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

          adder.load().timeout(Duration::from_secs(1)).await.unwrap();

          let vehicle = Vehicle::create(adder, coords, heading).unwrap();
          vehicle.set_explosion_proof(true);
        })
        .detach();
    }
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
fn entrypoint(module: ModuleHandle) -> ScriptHookV {
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

  ScriptHookVBuilder::new(module, ScriptHookVBackend)
    .plugin(ScriptHookVGtaPlugin)
    .startup_script_registrar(|mgr| {
      mgr.add_script(MyScript { gui: None });
    })
    .build()
}
