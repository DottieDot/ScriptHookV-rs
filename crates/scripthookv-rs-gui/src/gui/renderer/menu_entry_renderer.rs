use crate::{gui::MenuEntry, rendering::Widget};

pub trait MenuEntryRenderer {
  fn widget(&self, entry: &dyn MenuEntry, selected: bool) -> Box<dyn Widget>;
}
