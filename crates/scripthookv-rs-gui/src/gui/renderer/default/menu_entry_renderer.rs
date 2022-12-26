use crate::{
  gui::{renderer::MenuEntryRenderer, MenuEntry},
  rendering::Widget
};

use super::widgets::MenuEntryWidget;

pub struct DefaultMenuEntryRenderer {}

impl MenuEntryRenderer for DefaultMenuEntryRenderer {
  fn widget(&self, _entry: &dyn MenuEntry) -> Box<dyn Widget> {
    Box::new(MenuEntryWidget::new("Entry"))
  }
}
