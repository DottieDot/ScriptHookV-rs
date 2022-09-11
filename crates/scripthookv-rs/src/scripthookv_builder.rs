use log::info;

use crate::{
  builder_plugin::BuilderPlugin,
  memory::{MemoryLocation, Scannable},
  scripting::ScriptManager,
  scripting_backend::{KeyboardHandler, PresentCallback, ScriptingBackend},
  sig_info::SigInfo,
  GameVersion, ModuleHandle, ScriptHookV
};

pub type ScriptFn = extern "C" fn();

pub struct ScriptHookVBuilder {
  backend:                   Box<dyn ScriptingBackend>,
  module:                    ModuleHandle,
  startup_script_registrars: Vec<fn(&mut ScriptManager)>,
  present_callbacks:         Vec<PresentCallback>,
  keyboard_handlers:         Vec<KeyboardHandler>,
  min_version:               Option<GameVersion>,
  max_version:               Option<GameVersion>,
  plugins:                   Vec<Box<dyn BuilderPlugin>>,
  sigs:                      Vec<SigInfo>
}

impl ScriptHookVBuilder {
  #[inline]
  #[must_use]
  pub fn new(module: ModuleHandle, backend: impl ScriptingBackend + 'static) -> Self {
    Self {
      backend: Box::new(backend),
      module,
      startup_script_registrars: Vec::default(),
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
  pub fn startup_script_registrar(mut self, registrar: fn(&mut ScriptManager)) -> Self {
    self.startup_script_registrars.push(registrar);
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
  pub fn plugin(mut self, mut plugin: impl BuilderPlugin + 'static) -> Self {
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
  pub fn build(mut self) -> ScriptHookV {
    let instance = ScriptHookV::new(
      self.backend,
      self.module,
      self.startup_script_registrars,
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
