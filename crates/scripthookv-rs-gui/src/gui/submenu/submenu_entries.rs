use std::{
  borrow::Borrow,
  ops::{Range, RangeBounds},
  rc::Rc,
  sync::RwLock
};

use crate::gui::{MenuEntry, SubmenuSelection};

pub struct SubmenuEntries {
  entries:        Vec<Box<dyn MenuEntry>>,
  selection_info: Rc<RwLock<SubmenuSelection>>
}

impl SubmenuEntries {
  pub fn new(selection_info: Rc<RwLock<SubmenuSelection>>) -> Self {
    Self {
      entries: Default::default(),
      selection_info
    }
  }

  pub fn insert(&mut self, index: usize, entry: Box<dyn MenuEntry>) {
    self
      .selection_info
      .write()
      .unwrap()
      .update_for_inserted_entry(index, entry.borrow());

    self.entries.insert(index, entry)
  }

  pub fn add(&mut self, entry: Box<dyn MenuEntry>) {
    self.insert(self.entries.len(), entry)
  }

  pub fn insert_multiple(&mut self, index: usize, mut entries: Vec<Box<dyn MenuEntry>>) {
    let mut right = self.entries.split_off(index);
    for entry in entries.drain(0..) {
      self
        .selection_info
        .write()
        .unwrap()
        .update_for_inserted_entry(index, entry.borrow());
      self.entries.push(entry)
    }
    self.entries.append(&mut right)
  }

  pub fn add_multiple(&mut self, entries: Vec<Box<dyn MenuEntry>>) {
    self.insert_multiple(self.entries.len(), entries)
  }

  pub fn remove(&mut self, range: impl RangeBounds<usize>) {
    for (index, entry) in self.entries.drain(range).enumerate() {
      self
        .selection_info
        .write()
        .unwrap()
        .update_for_removed_entry(index, entry.borrow())
    }
  }

  pub fn filter_range(
    &mut self,
    range: Range<usize>,
    predicate: impl Fn(usize, &Box<dyn MenuEntry>) -> bool
  ) {
    let mut deleted = 0;
    for i in range {
      let index = i - deleted;
      if predicate(index, &self.entries[index]) {
        self
          .selection_info
          .write()
          .unwrap()
          .update_for_removed_entry(index, self.entries[index].borrow());

        self.entries.remove(index);
        deleted += 1;
      }
    }
  }

  pub fn filter(&mut self, predicate: impl Fn(usize, &Box<dyn MenuEntry>) -> bool) {
    self.filter_range(0..self.entries.len(), predicate)
  }

  pub fn raw(&self) -> &Vec<Box<dyn MenuEntry>> {
    &self.entries
  }
}
