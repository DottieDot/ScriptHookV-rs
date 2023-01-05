use std::cell::{RefCell, RefMut};

use crate::gui::MenuEntry;

use super::SubmenuEntries;

macro_rules! some_or_return {
  ($expr:expr) => {
    if let Some(v) = $expr {
      v
    } else {
      return;
    }
  };
}

#[derive(Default)]
pub struct SubmenuSelection {
  selected_index:      Option<usize>,
  selectable_count:    usize,
  selectable_selected: usize
}

impl SubmenuSelection {
  fn update_for_drained_entry(&mut self, index: usize, entry: &RefMut<dyn MenuEntry>) {
    if entry.is_selectable() {
      self.selectable_count -= 1
    }
    let selected = some_or_return!(self.selected_index.as_mut());

    if *selected < index && *selected != 0 {
      *selected -= 1
    }
    if *selected >= index && *selected != 0 && entry.is_selectable() {
      self.selectable_selected -= 1
    }
  }

  pub fn update_for_removed_entry(
    &mut self,
    index: usize,
    mut entry: RefMut<dyn MenuEntry>,
    entries: &SubmenuEntries
  ) {
    let removed_selected = matches!(self.selected_index, Some(i) if i == index);

    if removed_selected {
      entry.on_blur();
    }

    self.update_for_drained_entry(index, &entry);

    self.post_drain(entries, removed_selected);
  }

  pub fn update_for_removed_entries(
    &mut self,
    removed_entries: &[(usize, Box<RefCell<dyn MenuEntry>>)],
    entries: &SubmenuEntries
  ) {
    let mut removed_selected = false;
    if let Some(selected) = self.selected_index {
      if let Ok(index) = removed_entries.binary_search_by(|(i, _)| i.cmp(&selected)) {
        removed_entries[index].1.borrow_mut().on_blur();
        removed_selected = true;
      }
    }

    for (index, entry) in removed_entries {
      self.update_for_drained_entry(*index, &entry.borrow_mut())
    }

    self.post_drain(entries, removed_selected);
  }

  fn post_drain(&mut self, entries: &SubmenuEntries, removed_selected: bool) {
    if let Some(selected) = self.selected_index {
      self.selected_index = entries.find_nearest_entry_index(selected, |_, e| e.is_selectable())
    }

    if removed_selected {
      if let Some(selected) = self.selected_index {
        if let Some(mut e) = entries.get_mut(selected) {
          e.on_focus()
        }
      }
    }
  }

  pub fn update_for_inserted_entry(&mut self, index: usize, mut entry: RefMut<dyn MenuEntry>) {
    if entry.is_selectable() {
      self.selectable_count += 1;
      if self.selected_index.is_none() {
        self.selected_index = Some(index);
        self.selectable_selected = 0;
        entry.on_focus()
      }
      return;
    }

    let selected = some_or_return!(self.selected_index.as_mut());
    if index <= *selected {
      *selected += 1
    }
    if index <= *selected && entry.is_selectable() {
      self.selectable_selected += 1
    }
  }

  pub fn update_for_inserted_entries(&mut self, entries: Vec<(usize, RefMut<dyn MenuEntry>)>) {
    for (index, entry) in entries {
      self.update_for_inserted_entry(index, entry)
    }
  }

  pub fn scroll_up(&mut self, entries: &SubmenuEntries) {
    let selected = some_or_return!(self.selected_index);
    let mut new_selectable_selected = self.selectable_selected;
    for n in 1..entries.len() {
      let index = selected
        .checked_sub(n)
        .unwrap_or_else(|| entries.len() - n - selected);

      if index == (entries.len() - 1) {
        new_selectable_selected = self.selectable_count - 1;
      }

      if entries.is_selectable(index) {
        self.selectable_selected = new_selectable_selected;
        self.set_selected(index, entries);
        break;
      } else {
        new_selectable_selected -= 1;
      }
    }
  }

  pub fn scroll_down(&mut self, entries: &SubmenuEntries) {
    let selected = some_or_return!(self.selected_index);
    let mut new_selectable_selected = self.selectable_selected;
    for n in 1..entries.len() {
      let index = (selected + n) % entries.len();

      if index == 0 {
        new_selectable_selected = 0;
      }

      if entries.is_selectable(index) {
        self.selectable_selected = new_selectable_selected;
        self.set_selected(index, entries);
        break;
      }
    }
  }

  pub fn selected_index(&self) -> Option<usize> {
    self.selected_index
  }

  fn set_selected(&mut self, index: usize, entries: &SubmenuEntries) {
    if let Some(selected) = self.selected_index {
      entries.get_mut(selected).unwrap().on_blur();
    }
    entries.get_mut(index).unwrap().on_focus();
    self.selected_index = Some(index);
  }
}
