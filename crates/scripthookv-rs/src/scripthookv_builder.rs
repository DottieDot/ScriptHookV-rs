use shv_bindings::{KeyboardHandler, PresentCallback};

use crate::{GameVersion, ScriptHookV, ModuleHandle};

pub type ScriptFn = extern "C" fn();

#[derive(Debug)]
pub struct ScriptHookVBuilder {
  pub(crate) module:            ModuleHandle,
  pub(crate) scripts:           Vec<ScriptFn>,
  pub(crate) present_callbacks: Vec<PresentCallback>,
  pub(crate) keyboard_handlers: Vec<KeyboardHandler>,
  pub(crate) min_version:       Option<GameVersion>,
  pub(crate) max_version:       Option<GameVersion>
}

impl ScriptHookVBuilder {
  #[inline]
  #[must_use]
  pub fn new(module: ModuleHandle) -> Self {
    Self {
      module,
      scripts: Vec::default(),
      present_callbacks: Vec::default(),
      keyboard_handlers: Vec::default(),
      min_version: None,
      max_version: None
    }
  }

  #[inline]
  pub fn min_version(&mut self, version: GameVersion) -> &mut Self {
    self.min_version = Some(version);
    self
  }

  #[inline]
  pub fn max_version(&mut self, version: GameVersion) -> &mut Self {
    self.max_version = Some(version);
    self
  }

  #[inline]
  pub fn script(&mut self, entrypoint: ScriptFn) -> &mut Self {
    self.scripts.push(entrypoint);
    self
  }

  #[inline]
  pub fn present_callback(&mut self, callback: PresentCallback) -> &mut Self {
    self.present_callbacks.push(callback);
    self
  }

  #[inline]
  pub fn keyboard_handler(&mut self, handler: KeyboardHandler) -> &mut Self {
    self.keyboard_handlers.push(handler);
    self
  }

  #[inline]
  #[must_use]
  pub fn build(&self) -> ScriptHookV {
    ScriptHookV::init_from_builder(self)
  }
}
