use crate::gui::MenuEntry;

use super::SubmenuEntries;

#[derive(Default)]
pub struct SubmenuSelection {
  selected_index:      Option<usize>,
  selectable_count:    usize,
  selectable_selected: usize
}

impl SubmenuSelection {
  pub fn update_for_removed_entry(&mut self, index: usize, entry: &dyn MenuEntry) {
    if entry.is_selectable() {
      self.selectable_count -= 1
    }
    if self.selected_index.is_none() {
      return;
    }
    let selected = self.selected_index.as_mut().unwrap();
    if *selected < index {
      *selected -= 1
    }
    if *selected < index && entry.is_selectable() {
      self.selectable_selected -= 1
    }
  }

  pub fn update_for_inserted_entry(&mut self, index: usize, entry: &dyn MenuEntry) {
    if entry.is_selectable() {
      self.selectable_count += 1;
    }
    if self.selected_index.is_none() {
      return;
    }
    let selected = self.selected_index.as_mut().unwrap();
    if index <= *selected {
      *selected += 1
    }
    if index <= *selected && entry.is_selectable() {
      self.selectable_selected += 1
    }
  }

  pub fn post_drain(&mut self, entries: &SubmenuEntries) {
    // No entries so nothing can be selected
    if entries.is_empty() {
      self.selected_index = None
    } 
    // Nothing was selected so nothing can be selected now
    else if self.selected_index.is_none() {
      // no-op
    } 
    // If the selection is now out of range, then get the last selectable entry
    else if self.selected_index.unwrap() >= entries.len() {
      self.selected_index =
        entries.find_nearest_entry_index(entries.len(), |_, e| e.is_selectable());
      if self.selected_index.is_some() {
        self.selectable_selected = self.selectable_count - 1
      }
    } 
    // If the current selection is not selectable, then get the nearest selectable
    else if unsafe {
      !entries
        .list()
        .get_unchecked(self.selected_index.unwrap())
        .is_selectable()
    } {
      if let Some(new) =
        entries.find_nearest_entry_index(self.selected_index.unwrap(), |_, e| e.is_selectable())
      {
        if new < self.selected_index.unwrap() {
          self.scroll_up(entries.list());
        } else {
          self.scroll_down(entries.list());
        }
      }
    }
  }

  pub fn post_insert(&mut self, entries: &SubmenuEntries) {
    if self.selected_index.is_none() {
      if let Some(new_index) = entries.find_nearest_entry_index(0, |_, e| e.is_selectable()) {
        self.selected_index = Some(new_index);
        self.selectable_selected = 1
      }
    }
  }

  pub fn scroll_up(&mut self, entries: &Vec<Box<dyn MenuEntry>>) {
    if self.selected_index.is_none() {
      return;
    }
    let selected = self.selected_index.as_mut().unwrap();
    let mut new_selectable_selected = self.selectable_selected;
    for n in 1..entries.len() {
      let index = (*selected - n) % entries.len();

      if index == (entries.len() - 1) {
        new_selectable_selected = self.selectable_count - 1;
      }

      if entries[index].is_selectable() {
        self.selectable_selected = new_selectable_selected;
        *selected = index;
        break;
      } else {
        new_selectable_selected -= 1;
      }
    }
  }

  pub fn scroll_down(&mut self, entries: &Vec<Box<dyn MenuEntry>>) {
    if self.selected_index.is_none() {
      return;
    }
    let selected = self.selected_index.as_mut().unwrap();
    let mut new_selectable_selected = self.selectable_selected;
    for n in 1..entries.len() {
      let index = (*selected + n) % entries.len();

      if index == 0 {
        new_selectable_selected = 0;
      }

      if entries[index].is_selectable() {
        self.selectable_selected = new_selectable_selected;
        *selected = index;
        break;
      }
    }
  }

  pub fn get_selection(&self) -> Option<usize> {
    self.selected_index
  }
}
