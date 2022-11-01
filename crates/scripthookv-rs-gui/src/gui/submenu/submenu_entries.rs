use std::{
  cell::{Ref, RefCell, RefMut},
  cmp,
  ops::Range
};

use crate::gui::MenuEntry;

#[derive(Default)]
pub struct SubmenuEntries {
  entries: Vec<Box<RefCell<dyn MenuEntry>>>
}

impl SubmenuEntries {
  pub fn insert(
    &mut self,
    index: usize,
    entry: Box<RefCell<dyn MenuEntry>>
  ) -> (usize, RefMut<dyn MenuEntry>) {
    self.entries.insert(index, entry);
    (index, self.entries[index].borrow_mut())
  }

  pub fn add(&mut self, entry: Box<RefCell<dyn MenuEntry>>) -> (usize, RefMut<dyn MenuEntry>) {
    self.insert(self.entries.len(), entry)
  }

  pub fn insert_multiple(
    &mut self,
    index: usize,
    mut entries: Vec<Box<RefCell<dyn MenuEntry>>>
  ) -> Vec<(usize, RefMut<dyn MenuEntry>)> {
    let mut right = self.entries.split_off(index);
    self.entries.append(&mut entries);
    self.entries.append(&mut right);

    (index..entries.len())
      .map(|index| (index, self.entries[index].borrow_mut()))
      .collect::<Vec<_>>()
  }

  pub fn add_multiple(
    &mut self,
    entries: Vec<Box<RefCell<dyn MenuEntry>>>
  ) -> Vec<(usize, RefMut<dyn MenuEntry>)> {
    self.insert_multiple(self.entries.len(), entries)
  }

  pub fn remove(&mut self, index: usize) -> (usize, Box<RefCell<dyn MenuEntry>>) {
    (index, self.entries.remove(index))
  }

  pub fn remove_range(&mut self, range: Range<usize>) -> Vec<(usize, Box<RefCell<dyn MenuEntry>>)> {
    self.entries.drain(range).enumerate().collect::<Vec<_>>()
  }

  pub fn filter_range(
    &mut self,
    range: Range<usize>,
    predicate: impl Fn(usize, Ref<dyn MenuEntry>) -> bool
  ) -> Vec<(usize, Box<RefCell<dyn MenuEntry>>)> {
    let mut deleted: Vec<(usize, Box<RefCell<dyn MenuEntry>>)> = Vec::new();

    for i in range {
      let index = i - deleted.len();
      if predicate(index, self.entries[index].borrow()) {
        let removed = self.entries.remove(index);
        deleted.push((index, removed));
      }
    }

    deleted
  }

  pub fn filter(
    &mut self,
    predicate: impl Fn(usize, Ref<dyn MenuEntry>) -> bool
  ) -> Vec<(usize, Box<RefCell<dyn MenuEntry>>)> {
    self.filter_range(0..self.entries.len(), predicate)
  }

  pub fn find_nearest_entry_index(
    &self,
    index: usize,
    predicate: impl Fn(usize, Ref<dyn MenuEntry>) -> bool
  ) -> Option<usize> {
    if self.entries.is_empty() {
      return None;
    }

    let center = if index < self.entries.len() {
      index
    } else {
      self.entries.len() - 1
    };
    let range = cmp::max(center, self.entries.len() - 1 - center);

    if predicate(center, self.entries[center].borrow()) {
      return Some(center);
    }

    for n in 1..=range {
      if let Some(before) = center.checked_sub(n) {
        if predicate(before, self.entries[before].borrow()) {
          return Some(before);
        }
      }

      if let Some(after) = center.checked_add(n) {
        if predicate(after, self.entries[after].borrow()) {
          return Some(after);
        }
      }
    }

    None
  }

  pub fn is_selectable(&self, index: usize) -> bool {
    self
      .entries
      .get(index)
      .map(|r| r.borrow().is_selectable())
      .unwrap_or(false)
  }

  pub fn get(&self, index: usize) -> Option<Ref<dyn MenuEntry>> {
    self.entries.get(index).map(|b| b.borrow())
  }

  pub fn get_mut(&self, index: usize) -> Option<RefMut<dyn MenuEntry>> {
    self.entries.get(index).map(|b| b.borrow_mut())
  }

  pub fn len(&self) -> usize {
    self.entries.len()
  }

  pub fn is_empty(&self) -> bool {
    self.entries.is_empty()
  }
}
