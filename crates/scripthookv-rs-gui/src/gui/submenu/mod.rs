use std::{
  cell::{Ref, RefCell},
  ops::Range
};

use crate::{gui::MenuEntry, EventEmitter};

use self::{submenu_entries::SubmenuEntries, submenu_selection::SubmenuSelection};

mod submenu_entries;
mod submenu_selection;

pub struct Submenu {
  title:         String,
  subtitle:      String,
  event_emitter: EventEmitter<Self>,
  entries:       SubmenuEntries,
  selection:     SubmenuSelection
}

impl Submenu {
  pub fn new(
    title: impl Into<String>,
    subtitle: impl Into<String>,
    build_fn: impl Fn(&mut Self)
  ) -> Self {
    let mut result = Self {
      title:         title.into(),
      subtitle:      subtitle.into(),
      event_emitter: Default::default(),
      entries:       Default::default(),
      selection:     Default::default()
    };
    build_fn(&mut result);

    result
  }

  pub fn on<Event: 'static>(
    &mut self,
    callback: impl Fn(&mut Self, &Event) + 'static
  ) -> &mut Self {
    self.event_emitter.on(callback);
    self
  }

  pub(crate) fn navigated_from(&mut self) {
    todo!()
  }

  pub(crate) fn navigated_to(&mut self) {
    todo!()
  }

  pub(crate) fn menu_closed(&mut self)  {
    todo!()
  }

  pub(crate) fn menu_opened(&mut self) {
    todo!()
  }

  pub fn set_title(&mut self, title: String) {
    self.title = title
  }

  pub fn set_subtitle(&mut self, subtitle: String) {
    self.subtitle = subtitle
  }

  pub fn entries_mut(&mut self) -> &mut SubmenuEntries {
    &mut self.entries
  }

  pub fn scroll_up(&mut self) {
    self.selection.scroll_up(&self.entries);
  }

  pub fn scroll_down(&mut self) {
    self.selection.scroll_down(&self.entries);
  }

  pub fn insert(&mut self, index: usize, entry: Box<RefCell<dyn MenuEntry>>) {
    let (inserted_index, inserted_entry) = self.entries.insert(index, entry);
    self
      .selection
      .update_for_inserted_entry(inserted_index, inserted_entry)
  }

  pub fn add(&mut self, entry: Box<RefCell<dyn MenuEntry>>) {
    let (inserted_index, inserted_entry) = self.entries.add(entry);
    self
      .selection
      .update_for_inserted_entry(inserted_index, inserted_entry)
  }

  pub fn insert_multiple(&mut self, index: usize, entries: Vec<Box<RefCell<dyn MenuEntry>>>) {
    let inserted = self.entries.insert_multiple(index, entries);
    self.selection.update_for_inserted_entries(inserted)
  }

  pub fn add_multiple(&mut self, entries: Vec<Box<RefCell<dyn MenuEntry>>>) {
    let inserted = self.entries.add_multiple(entries);
    self.selection.update_for_inserted_entries(inserted)
  }

  pub fn remove(&mut self, index: usize) {
    let (removed_index, removed_entry) = self.entries.remove(index);
    self.selection.update_for_removed_entry(
      removed_index,
      removed_entry.borrow_mut(),
      &self.entries
    );
  }

  pub fn remove_range(&mut self, range: Range<usize>) {
    let removed_entries = self.entries.remove_range(range);
    self
      .selection
      .update_for_removed_entries(&removed_entries, &self.entries)
  }

  pub fn filter_range(
    &mut self,
    range: Range<usize>,
    predicate: impl Fn(usize, Ref<dyn MenuEntry>) -> bool
  ) {
    let removed_entries = self.entries.filter_range(range, predicate);
    self
      .selection
      .update_for_removed_entries(&removed_entries, &self.entries)
  }

  pub fn filter(&mut self, predicate: impl Fn(usize, Ref<dyn MenuEntry>) -> bool) {
    let removed_entries = self.entries.filter(predicate);
    self
      .selection
      .update_for_removed_entries(&removed_entries, &self.entries)
  }
}
