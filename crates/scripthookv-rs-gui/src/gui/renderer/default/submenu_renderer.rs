use std::{
  cell::{Ref, RefMut},
  ops::Range
};

use crate::{
  gui::{
    renderer::{MenuRenderer, SubmenuRenderer},
    submenu::SubmenuEntries,
    submenu::SubmenuSelection,
    MenuEntry, Submenu
  },
  rendering::Widget
};

use super::widgets::SubmenuWidget;

pub struct DefaultSubmenuRenderer {
  number_to_render: usize
}

impl DefaultSubmenuRenderer {
  pub fn new(number_to_render: usize) -> Self {
    Self { number_to_render }
  }

  fn get_visible_entry_range(
    &self,
    selection: &SubmenuSelection,
    entries: &SubmenuEntries
  ) -> Option<Range<usize>> {
    self
      .get_last_visible_entry_index(selection, entries)
      .map(|last_index| {
        let first_index = last_index.saturating_sub(self.number_to_render);
        first_index..last_index
      })
  }

  fn get_last_visible_entry_index(
    &self,
    selection: &SubmenuSelection,
    entries: &SubmenuEntries
  ) -> Option<usize> {
    let half_to_render = (self.number_to_render / 2) + (self.number_to_render % 2);

    selection.selected_index().map(|index| {
      if index < half_to_render || entries.len() <= self.number_to_render {
        std::cmp::min(entries.len(), self.number_to_render)
      } else {
        std::cmp::min(index + half_to_render, entries.len())
      }
    })
  }
}

impl SubmenuRenderer for DefaultSubmenuRenderer {
  fn get_visible_entries<'se>(
    &self,
    selection: &SubmenuSelection,
    entries: &'se SubmenuEntries
  ) -> Vec<Ref<'se, dyn MenuEntry>> {
    let range = self.get_visible_entry_range(selection, entries);
    range
      .map(|range| entries.get_range(range))
      .unwrap_or_default()
  }

  fn get_visible_entries_mut<'se>(
    &self,
    selection: &SubmenuSelection,
    entries: &'se SubmenuEntries
  ) -> Vec<RefMut<'se, dyn MenuEntry>> {
    let range = self.get_visible_entry_range(selection, entries);
    range
      .map(|range| entries.get_range_mut(range))
      .unwrap_or_default()
  }

  fn widget(&self, menu_renderer: &dyn MenuRenderer, submenu: &Submenu) -> Box<dyn Widget> {
    let entry_widgets = self
      .get_visible_entries(submenu.selection(), submenu.entries())
      .into_iter()
      .map(|entry| menu_renderer.entry_renderer().widget(&*entry))
      .collect::<Vec<_>>();

    Box::new(SubmenuWidget::new(submenu, entry_widgets))
  }
}
