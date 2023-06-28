use std::{any::TypeId, cell::RefCell, ffi::c_void, rc::Rc};

use crate::{
  gui::{renderer::MenuEntryRenderInfo, EmitForMenuEntry, MenuControls, MenuEntry},
  EventEmitter
};

use super::events::ClickEvent;

pub struct MenuOption {
  text:    String,
  emitter: Rc<EventEmitter<MenuOption>>
}

impl MenuOption {
  pub fn text(&self) -> &str {
    &self.text
  }
}

impl MenuEntry for MenuOption {
  fn on_focus(&mut self) {}

  fn on_blur(&mut self) {}

  fn is_selectable(&self) -> bool {
    true
  }

  fn process(&mut self, controls: &MenuControls, selected: bool) {
    if !selected {
      return;
    }

    if controls.confirm.active() {
      self.emit(&ClickEvent)
    }
  }

  fn render_info(&self) -> MenuEntryRenderInfo {
    MenuEntryRenderInfo {
      text:   self.text.clone(),
      value:  None,
      toggle: None
    }
  }

  unsafe fn emit_raw(&mut self, data: *const c_void, type_id: &TypeId) {
    let emitter = self.emitter.clone();

    emitter.emit_raw(self, data, type_id)
  }
}

#[derive(Default)]
pub struct MenuOptionBuilder {
  text:    Option<String>,
  emitter: EventEmitter<MenuOption>
}

impl MenuOptionBuilder {
  pub fn text(mut self, text: impl Into<String>) -> Self {
    self.text = Some(text.into());
    self
  }

  pub fn on_click(mut self, on_click: impl Fn(&mut MenuOption, &ClickEvent) + 'static) -> Self {
    self.emitter.on(on_click);
    self
  }

  pub fn build(self) -> Box<RefCell<(dyn MenuEntry + 'static)>> {
    Box::new(RefCell::new(MenuOption {
      text:    self.text.unwrap_or_default(),
      emitter: Rc::new(self.emitter)
    }))
  }
}
