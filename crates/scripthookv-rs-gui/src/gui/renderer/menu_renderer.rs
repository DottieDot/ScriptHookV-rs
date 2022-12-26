use crate::gui::Submenu;

use super::{MenuEntryRenderer, SubmenuRenderer};

pub trait MenuRenderer {
  fn submenu_renderer(&self) -> &dyn SubmenuRenderer;

  fn entry_renderer(&self) -> &dyn MenuEntryRenderer;

  fn render(&self, submenu: &Submenu);
}
