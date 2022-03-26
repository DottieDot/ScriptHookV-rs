use log::info;
use shv_bindings::{KeyboardHandler, PresentCallback};

use crate::{
  builder_plugin::BuilderPlugin,
  memory::{MemoryLocation, Scannable},
  scripting::Script,
  sig_info::SigInfo,
  GameVersion, ModuleHandle, ScriptHookV
};

pub type ScriptFn = extern "C" fn();

pub struct ScriptHookVBuilder<'b> {
  module:            ModuleHandle,
  scripts:           Vec<Box<dyn Script<'b>>>,
  present_callbacks: Vec<PresentCallback>,
  keyboard_handlers: Vec<KeyboardHandler>,
  min_version:       Option<GameVersion>,
  max_version:       Option<GameVersion>,
  plugins:           Vec<Box<dyn BuilderPlugin<'b>>>,
  sigs:              Vec<SigInfo>
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
      plugins: Vec::default(),
      sigs: Vec::default()
    }
  }

  #[inline]
  pub fn min_version(mut self, version: GameVersion) -> Self {
    self.min_version = Some(version);
    self
  }

  #[inline]
  pub fn max_version(mut self, version: GameVersion) -> Self {
    self.max_version = Some(version);
    self
  }

  #[inline]
  pub fn script(mut self, script: impl Script<'b> + 'static) -> Self {
    self.scripts.push(Box::new(script));
    self
  }

  #[inline]
  pub fn present_callback(mut self, callback: PresentCallback) -> Self {
    self.present_callbacks.push(callback);
    self
  }

  #[inline]
  pub fn keyboard_handler(mut self, handler: KeyboardHandler) -> Self {
    self.keyboard_handlers.push(handler);
    self
  }

  #[inline]
  pub fn plugin(mut self, mut plugin: impl BuilderPlugin<'b> + 'static) -> Self {
    info!("registered builder plugin {}", plugin.name());
    self = plugin.build(self);
    self.plugins.push(Box::new(plugin));
    self
  }

  #[inline]
  pub fn sig(self, name: String, scannable: impl Scannable + 'static) -> Self {
    self.sig_with_offset(name, scannable, |l| l)
  }

  #[inline]
  pub fn sig_with_offset(
    mut self,
    name: String,
    scannable: impl Scannable + 'static,
    offset: fn(MemoryLocation) -> MemoryLocation
  ) -> Self {
    self
      .sigs
      .push(SigInfo::new(name, Box::new(scannable), offset));
    self
  }

  #[inline]
  #[must_use]
  pub fn build(mut self) -> ScriptHookV<'b> {
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
