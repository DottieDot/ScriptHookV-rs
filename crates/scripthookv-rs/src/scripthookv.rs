use log::{info, warn};
use shv_bindings::{KeyboardHandler, PresentCallback};

use crate::{
  get_game_version, memory::ModuleMemoryScanner, memory_database::MemoryDatabase,
  register_keyboard_handler, register_present_callback, remove_keyboard_handler,
  remove_present_callback, remove_script, scripting::ScriptManager, sig_info::SigInfo, GameVersion,
  ModuleHandle
};

pub struct ScriptHookV {
  module:                     ModuleHandle,
  present_callbacks:          Vec<PresentCallback>,
  keyboard_handlers:          Vec<KeyboardHandler>,
  min_version:                Option<GameVersion>,
  max_version:                Option<GameVersion>,
  memory:                     MemoryDatabase,
  startup_scripts_registrars: Vec<fn(&mut ScriptManager)>
}

impl ScriptHookV {
  fn init(&mut self, sigs: &[SigInfo]) {
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
    startup_scripts_registrars: Vec<fn(&mut ScriptManager)>,
    present_callbacks: Vec<PresentCallback>,
    keyboard_handlers: Vec<KeyboardHandler>,
    sigs: &[SigInfo],
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
      startup_scripts_registrars
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

  pub fn new_script_manager_for_thread<'mgr>(&self) -> ScriptManager<'mgr> {
    let mut manager = ScriptManager::default();
    for registrar in &self.startup_scripts_registrars {
      registrar(&mut manager)
    }
    manager
  }
}

unsafe impl Send for ScriptHookV {}
