use crate::gui::MenuEntry;

pub struct MenuOption {
  i: i32
}

impl MenuOption {
  pub fn new() -> Self {
    Self { i: 10 }
  }
}

impl MenuEntry for MenuOption {
  fn on_focus(&mut self) {}

  fn on_blur(&mut self) {}

  fn is_selectable(&self) -> bool {
    true
  }

  fn process(&mut self, _controls: &crate::gui::MenuControls) {}
}
