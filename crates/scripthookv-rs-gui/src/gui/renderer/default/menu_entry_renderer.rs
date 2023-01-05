use crate::{
  gui::{renderer::MenuEntryRenderer, MenuEntry},
  rendering::Widget
};

use super::widgets::MenuEntryWidget;

pub struct DefaultMenuEntryRenderer;

impl MenuEntryRenderer for DefaultMenuEntryRenderer {
  fn widget(&self, entry: &dyn MenuEntry, selected: bool) -> Box<dyn Widget> {
    Box::new(MenuEntryWidget::new(entry.render_info(), selected))
  }
}
