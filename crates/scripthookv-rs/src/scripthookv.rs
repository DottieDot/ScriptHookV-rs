use log::debug;
use shv_bindings::{KeyboardHandler, PresentCallback};

use crate::{
  get_game_version, register_additional_script_thread, register_keyboard_handler,
  register_present_callback, register_script, remove_keyboard_handler, remove_present_callback,
  remove_script, GameVersion, ModuleHandle, ScriptFn
};

#[derive(Debug)]
pub struct ScriptHookV {
  module:            ModuleHandle,
  scripts:           Vec<ScriptFn>,
  present_callbacks: Vec<PresentCallback>,
  keyboard_handlers: Vec<KeyboardHandler>,
  min_version:       Option<GameVersion>,
  max_version:       Option<GameVersion>
}

impl ScriptHookV {
  fn init(&mut self) {
    debug!(
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

    debug!("Registering {} scripts", self.scripts.len());
    for (i, script) in self.scripts.iter().enumerate() {
      if i == 0 {
        register_script(self.module, *script);
      } else {
        register_additional_script_thread(self.module, *script);
      }
    }

    debug!("Registering {} present callbacks", self.scripts.len());
    for callback in &self.present_callbacks {
      register_present_callback(*callback);
    }

    debug!("Registering {} keyboard handlers", self.scripts.len());
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
    min_version: Option<GameVersion>,
    max_version: Option<GameVersion>
  ) -> Self {
    let mut instance = Self {
      module,
      scripts,
      present_callbacks,
      keyboard_handlers,
      min_version,
      max_version
    };
    instance.init();
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
}

unsafe impl Send for ScriptHookV {}

unsafe impl Sync for ScriptHookV {}
