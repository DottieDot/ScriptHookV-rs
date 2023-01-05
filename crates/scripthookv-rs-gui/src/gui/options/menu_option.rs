use std::cell::RefCell;

use crate::gui::{renderer::MenuEntryRenderInfo, MenuControls, MenuEntry, MenuEntryBuilder};

pub struct MenuOption {
  text: String
}

impl MenuEntry for MenuOption {
  fn on_focus(&mut self) {}

  fn on_blur(&mut self) {}

  fn is_selectable(&self) -> bool {
    true
  }

  fn process(&mut self, _controls: &MenuControls) {}

  fn render_info(&self) -> MenuEntryRenderInfo {
    MenuEntryRenderInfo {
      text:   self.text.clone(),
      value:  None,
      toggle: None
    }
  }
}

#[derive(Default)]
pub struct OptionBuilder {
  text: Option<String>
}

impl OptionBuilder {
  pub fn text(mut self, text: impl Into<String>) -> Self {
    self.text = Some(text.into());
    self
  }
}

impl MenuEntryBuilder for OptionBuilder {
  fn build(self) -> Box<RefCell<dyn MenuEntry>> {
    Box::new(RefCell::new(MenuOption {
      text: self.text.unwrap_or_default()
    }))
  }
}
