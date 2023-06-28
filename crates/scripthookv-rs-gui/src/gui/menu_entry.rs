use std::{any::TypeId, cell::RefCell, ffi::c_void};

use super::{renderer::MenuEntryRenderInfo, MenuControls};

pub trait MenuEntry: 'static {
  fn on_focus(&mut self);

  fn on_blur(&mut self);

  fn is_selectable(&self) -> bool;

  fn process(&mut self, controls: &MenuControls, selected: bool);

  fn render_info(&self) -> MenuEntryRenderInfo;

  unsafe fn emit_raw(&mut self, data: *const c_void, type_id: &TypeId);
}

pub trait MenuEntryBuilder {
  fn build(self) -> Box<RefCell<dyn MenuEntry>>;
}

pub trait EmitForMenuEntry {
  fn emit<E: 'static>(&mut self, data: &E);
}

impl<T: MenuEntry + 'static> EmitForMenuEntry for T {
  fn emit<E: 'static>(&mut self, data: &E) {
    unsafe { self.emit_raw(data as *const E as *const c_void, &TypeId::of::<E>()) }
  }
}
