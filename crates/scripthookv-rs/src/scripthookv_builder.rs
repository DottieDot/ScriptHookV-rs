use log::info;
use shv_bindings::{KeyboardHandler, PresentCallback};

use crate::{builder_plugin::BuilderPlugin, GameVersion, ModuleHandle, ScriptHookV, memory::{Scannable, MemoryLocation}};

pub type ScriptFn = extern "C" fn();

#[derive(Debug)]
pub struct ScriptHookVBuilder<'b> {
  pub(crate) module:            ModuleHandle,
  pub(crate) scripts:           Vec<ScriptFn>,
  pub(crate) present_callbacks: Vec<PresentCallback>,
  pub(crate) keyboard_handlers: Vec<KeyboardHandler>,
  pub(crate) min_version:       Option<GameVersion>,
  pub(crate) max_version:       Option<GameVersion>,
  pub(crate) plugins:           Vec<&'b mut dyn BuilderPlugin>
}

impl<'b> ScriptHookVBuilder<'b> {
  #[inline]
  #[must_use]
  pub fn new(module: ModuleHandle) -> Self {
    Self {
      module,
      scripts: Vec::default(),
      present_callbacks: Vec::default(),
      keyboard_handlers: Vec::default(),
      min_version: None,
      max_version: None,
      plugins: Vec::default()
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
  pub fn plugin(&mut self, plugin: &'b mut dyn BuilderPlugin) -> &mut Self {
    info!("registered builder plugin {}", plugin.name());
    plugin.build(self);
    self.plugins.push(plugin);
    self
  }

  #[inline]
  pub fn sig(&mut self, name: &str, scannable: &dyn Scannable) -> &mut Self {
    self.sig_with_offset(name, scannable, |l| l)
  }

  #[inline]
  pub fn sig_with_offset(&mut self, name: &str, scannable: &dyn Scannable, offset_fn: fn(MemoryLocation) -> MemoryLocation) -> &mut Self {
    self.sig_with_offset_and_scanner("gta", name, scannable, offset_fn)
  }

  #[inline]
  pub fn sig_with_scanner(&mut self, scanner: &str, name: &str, scannable: &dyn Scannable) -> &mut Self {
    self.sig_with_offset_and_scanner(scanner, name, scannable, |l| l)
  }

  #[inline]
  pub fn sig_with_offset_and_scanner(&mut self, scanner: &str, name: &str, scannable: &dyn Scannable, offset_fn: fn(MemoryLocation) -> MemoryLocation) -> &mut Self {
    self
  }

  #[inline]
  #[must_use]
  pub fn build(mut self) -> ScriptHookV {
    let instance = ScriptHookV::new(
      self.module,
      self.scripts,
      self.present_callbacks,
      self.keyboard_handlers,
      self.min_version,
      self.max_version
    );
    for plugin in &mut self.plugins {
      info!("initializing builder plugin {}", plugin.name());
      plugin.init(&instance)
    }
    instance
  }
}
