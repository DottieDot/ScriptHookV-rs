use log::{info, warn};
use shv_bindings::{KeyboardHandler, PresentCallback};

use crate::{
  get_game_version,
  memory::ModuleMemoryScanner,
  memory_database::MemoryDatabase,
  register_keyboard_handler, register_present_callback, remove_keyboard_handler,
  remove_present_callback, remove_script,
  scripting::{Script, ScriptManager},
  sig_info::SigInfo,
  GameVersion, ModuleHandle
};

pub struct ScriptHookV<'a> {
  module:            ModuleHandle,
  present_callbacks: Vec<PresentCallback>,
  keyboard_handlers: Vec<KeyboardHandler>,
  min_version:       Option<GameVersion>,
  max_version:       Option<GameVersion>,
  memory:            MemoryDatabase,
  script_engine:     ScriptManager<'a>
}

impl<'a> ScriptHookV<'a> {
  fn init(&mut self, sigs: &Vec<SigInfo>, scripts: Vec<Box<dyn Script>>) {
    info!(
      "Checking game version {:?} (min: {:?}) (max: {:?})",
      get_game_version(),
      self.min_version,
      self.max_version
    );
    match (get_game_version(), self.min_version, self.max_version) {
      (Some(version), Some(min_version), _) if version < min_version => {
        panic!("Game version is too old")
      }
      (Some(version), _, Some(min_version)) if version < min_version => {
        panic!("Game version is not supported")
      }
      (None, _, Some(_)) | (None, Some(_), _) => panic!("Unknown game version"),
      _ => ()
    }

    info!("Searching for {} signatures", sigs.len());
    let scanner = ModuleMemoryScanner::default();
    for sig in sigs {
      if let Ok(location) = sig.run(&scanner) {
        info!("Signature for {} resolved to {location}", sig.name);
        self.memory.add(sig.name.clone(), location);
      } else {
        warn!("Failed to find signature for {}", sig.name);
      }
    }

    info!("Registering {} scripts", scripts.len());
    for script in scripts {
      self.script_engine.add_script(script);
    }

    info!(
      "Registering {} present callbacks",
      self.present_callbacks.len()
    );
    for callback in &self.present_callbacks {
      register_present_callback(*callback);
    }

    info!(
      "Registering {} keyboard handlers",
      self.keyboard_handlers.len()
    );
    for handler in &self.keyboard_handlers {
      register_keyboard_handler(*handler);
    }
  }

  #[must_use]
  pub(crate) fn new(
    module: ModuleHandle,
    scripts: Vec<Box<dyn Script>>,
    present_callbacks: Vec<PresentCallback>,
    keyboard_handlers: Vec<KeyboardHandler>,
    sigs: &Vec<SigInfo>,
    min_version: Option<GameVersion>,
    max_version: Option<GameVersion>
  ) -> Self {
    let mut instance = Self {
      module,
      present_callbacks,
      keyboard_handlers,
      min_version,
      max_version,
      memory: MemoryDatabase::default(),
      script_engine: Default::default()
    };
    instance.init(sigs, scripts);
    instance
  }

  pub fn cleanup(&self) {
    remove_script(self.module);

    for callback in &self.present_callbacks {
      remove_present_callback(*callback);
    }

    for handler in &self.keyboard_handlers {
      remove_keyboard_handler(*handler);
    }
  }

  pub fn get_memory(&self) -> &MemoryDatabase {
    &self.memory
  }

  pub fn update_scripts(&mut self) {
    self.script_engine.tick();
  }
}

unsafe impl<'a> Send for ScriptHookV<'a> {}

unsafe impl<'a> Sync for ScriptHookV<'a> {}
