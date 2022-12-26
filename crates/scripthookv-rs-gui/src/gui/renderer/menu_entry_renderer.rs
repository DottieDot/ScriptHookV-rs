use crate::{gui::MenuEntry, rendering::Widget};

pub trait MenuEntryRenderer {
  fn widget(&self, entry: &dyn MenuEntry) -> Box<dyn Widget>;
}
