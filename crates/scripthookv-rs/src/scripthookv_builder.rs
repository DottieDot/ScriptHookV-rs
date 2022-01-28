use log::info;
use shv_bindings::{KeyboardHandler, PresentCallback};

use crate::{builder_plugin::BuilderPlugin, GameVersion, ModuleHandle, ScriptHookV, memory::{Scannable, MemoryLocation}, sig_info::SigInfo};

pub type ScriptFn = extern "C" fn();

pub struct ScriptHookVBuilder {
  module:            ModuleHandle,
  scripts:           Vec<ScriptFn>,
  present_callbacks: Vec<PresentCallback>,
  keyboard_handlers: Vec<KeyboardHandler>,
  min_version:       Option<GameVersion>,
  max_version:       Option<GameVersion>,
  plugins:           Vec<Box<dyn BuilderPlugin>>,
  sigs:              Vec<SigInfo>
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
      max_version: None,
      plugins: Vec::default(),
      sigs: Vec::default()
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
  pub fn plugin(&mut self, mut plugin: Box<dyn BuilderPlugin>) -> &mut Self {
    info!("registered builder plugin {}", plugin.name());
    plugin.build(self);
    self.plugins.push(plugin);
    self
  }

  #[inline]
  pub fn sig(&mut self, name: String, scannable: Box<dyn Scannable>) -> &mut Self {
    self.sig_with_offset(name, scannable, |l| l)
  }

  #[inline]
  pub fn sig_with_offset(&mut self, name: String, scannable: Box<dyn Scannable>, offset: fn(MemoryLocation) -> MemoryLocation) -> &mut Self {
    self.sigs.push(SigInfo::new(name, scannable, offset));
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
      &self.sigs,
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
