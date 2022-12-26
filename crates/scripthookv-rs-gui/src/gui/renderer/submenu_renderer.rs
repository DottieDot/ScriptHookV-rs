use std::cell::RefMut;

use crate::gui::{MenuEntry, Submenu};

use crate::gui::submenu::{SubmenuEntries, SubmenuSelection};
use crate::rendering::Widget;

use super::MenuRenderer;

pub trait SubmenuRenderer {
  fn get_visible_entries<'se>(
    &self,
    selection: &SubmenuSelection,
    entries: &'se SubmenuEntries
  ) -> Vec<std::cell::Ref<'se, dyn MenuEntry>>;

  fn get_visible_entries_mut<'se>(
    &self,
    selection: &SubmenuSelection,
    entries: &'se SubmenuEntries
  ) -> Vec<RefMut<'se, dyn MenuEntry>>;

  fn widget(&self, menu_renderer: &dyn MenuRenderer, submenu: &Submenu) -> Box<dyn Widget>;
}
