use std::cell::RefCell;

use super::{renderer::MenuEntryRenderInfo, MenuControls};

pub trait MenuEntry: 'static {
  fn on_focus(&mut self);

  fn on_blur(&mut self);

  fn is_selectable(&self) -> bool;

  fn process(&mut self, controls: &MenuControls);

  fn render_info(&self) -> MenuEntryRenderInfo;
}

pub trait MenuEntryBuilder {
  fn build(self) -> Box<RefCell<dyn MenuEntry>>;
}
