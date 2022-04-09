use std::{
  borrow::Borrow,
  ops::{Range, RangeBounds},
  rc::Rc,
  sync::RwLock, cmp
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
    let mut selection_info = self.selection_info.write().unwrap();
    let mut right = self.entries.split_off(index);
    for entry in entries.drain(0..) {
      selection_info.update_for_inserted_entry(index, entry.borrow());
      self.entries.push(entry)
    }
    self.entries.append(&mut right)
  }

  pub fn add_multiple(&mut self, entries: Vec<Box<dyn MenuEntry>>) {
    self.insert_multiple(self.entries.len(), entries)
  }

  pub fn remove(&mut self, range: impl RangeBounds<usize>) {
    let mut selection_info = self.selection_info.write().unwrap();
    for (index, entry) in self.entries.drain(range).enumerate() {
      selection_info.update_for_removed_entry(index, entry.borrow())
    }
    selection_info.post_drain(self);
  }

  pub fn filter_range(
    &mut self,
    range: Range<usize>,
    predicate: impl Fn(usize, &Box<dyn MenuEntry>) -> bool
  ) {
    let mut selection_info = self.selection_info.write().unwrap();
    let mut deleted = 0;
    for i in range {
      let index = i - deleted;
      if predicate(index, &self.entries[index]) {
        selection_info.update_for_removed_entry(index, self.entries[index].borrow());

        self.entries.remove(index);
        deleted += 1;
      }
    }
    selection_info.post_drain(self);
  }

  pub fn filter(&mut self, predicate: impl Fn(usize, &Box<dyn MenuEntry>) -> bool) {
    self.filter_range(0..self.entries.len(), predicate)
  }

  pub fn list(&self) -> &Vec<Box<dyn MenuEntry>> {
    &self.entries
  }

  pub fn find_nearest_entry_index(&self, index: usize, predicate: impl Fn(usize, &Box<dyn MenuEntry>) -> bool) -> Option<usize> {
    if self.entries.is_empty() {
      return None
    }

    let center = if index < self.entries.len() {
      index
    } else {
      self.entries.len() - 1
    };
    let range = cmp::max(center, self.entries.len() - 1 - center);

    if predicate(center, &self.entries[center]) {
      return Some(center)
    }

    for n in 1..=range {
      if let Some(before) = center.checked_sub(n) {
        if predicate(before, &self.entries[before]) {
          return Some(before)
        }
      }

      if let Some(after) = center.checked_add(n) {
        if predicate(after, &self.entries[after]) {
          return Some(after)
        }
      }
    }

    None
  }

  pub fn len(&self) -> usize {
    self.entries.len()
  }

  pub fn is_empty(&self) -> bool {
    self.entries.is_empty()
  }
}
