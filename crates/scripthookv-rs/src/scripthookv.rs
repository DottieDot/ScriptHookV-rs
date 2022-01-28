use log::{info, warn};
use shv_bindings::{KeyboardHandler, PresentCallback};

use crate::{
  get_game_version, register_additional_script_thread, register_keyboard_handler,
  register_present_callback, register_script, remove_keyboard_handler, remove_present_callback,
  remove_script, sig_info::SigInfo, GameVersion, ModuleHandle, ScriptFn, memory::ModuleMemoryScanner, memory_database::MemoryDatabase
};

pub struct ScriptHookV {
  module:            ModuleHandle,
  scripts:           Vec<ScriptFn>,
  present_callbacks: Vec<PresentCallback>,
  keyboard_handlers: Vec<KeyboardHandler>,
  min_version:       Option<GameVersion>,
  max_version:       Option<GameVersion>,
  memory: MemoryDatabase
}

impl ScriptHookV {
  fn init(&mut self, sigs: &Vec<SigInfo>) {
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

    info!("Registering {} scripts", self.scripts.len());
    for (i, script) in self.scripts.iter().enumerate() {
      if i == 0 {
        register_script(self.module, *script);
      } else {
        register_additional_script_thread(self.module, *script);
      }
    }

    info!("Registering {} present callbacks", self.scripts.len());
    for callback in &self.present_callbacks {
      register_present_callback(*callback);
    }

    info!("Registering {} keyboard handlers", self.scripts.len());
    for handler in &self.keyboard_handlers {
      register_keyboard_handler(*handler);
    }
  }

  #[must_use]
  pub(crate) fn new(
    module: ModuleHandle,
    scripts: Vec<ScriptFn>,
    present_callbacks: Vec<PresentCallback>,
    keyboard_handlers: Vec<KeyboardHandler>,
    sigs: &Vec<SigInfo>,
    min_version: Option<GameVersion>,
    max_version: Option<GameVersion>
  ) -> Self {
    let mut instance = Self {
      module,
      scripts,
      present_callbacks,
      keyboard_handlers,
      min_version,
      max_version,
      memory: MemoryDatabase::default()
    };
    instance.init(sigs);
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
}

unsafe impl Send for ScriptHookV {}

unsafe impl Sync for ScriptHookV {}
