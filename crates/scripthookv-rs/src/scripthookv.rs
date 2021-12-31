use shv_bindings::{KeyboardHandler, PresentCallback};

use crate::{
  get_game_version, register_additional_script_thread, register_keyboard_handler,
  register_present_callback, register_script, remove_keyboard_handler, remove_present_callback,
  remove_script, GameVersion, ScriptFn, ScriptHookVBuilder, ModuleHandle
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
  fn init(&self) {
    match (get_game_version(), self.min_version, self.max_version) {
      (Some(version), Some(min_version), _) if version < min_version => {
        panic!("Game version is too old")
      }
      (Some(version), _, Some(min_version)) if version < min_version => {
        panic!("Game version is not supported")
      }
      (None, Some(_), Some(_)) => panic!("Unknown game version"),
      _ => ()
    }

    for (i, script) in self.scripts.iter().enumerate() {
      if i == 0 {
        register_script(self.module, *script);
      } else {
        register_additional_script_thread(self.module, *script);
      }
    };

    for callback in &self.present_callbacks {
      register_present_callback(*callback);
    }

    for handler in &self.keyboard_handlers {
      register_keyboard_handler(*handler);
    }
  }

  #[must_use]
  pub(crate) fn init_from_builder(
    ScriptHookVBuilder {
      module,
      scripts,
      present_callbacks,
      keyboard_handlers,
      min_version,
      max_version
    }: &ScriptHookVBuilder
  ) -> Self {
    let instance = Self {
      module:            *module,
      scripts:           scripts.clone(),
      present_callbacks: present_callbacks.clone(),
      keyboard_handlers: keyboard_handlers.clone(),
      min_version:       *min_version,
      max_version:       *max_version
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
