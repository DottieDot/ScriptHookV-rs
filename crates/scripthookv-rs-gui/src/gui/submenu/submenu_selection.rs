use crate::gui::MenuEntry;

use super::SubmenuEntries;

#[derive(Default)]
pub struct SubmenuSelection {
  selected_index:      Option<usize>,
  selectable_count:    usize,
  selectable_selected: usize
}

impl SubmenuSelection {
  pub fn update_for_drained_entry(&mut self, index: usize, entry: &dyn MenuEntry) {
    if entry.is_selectable() {
      self.selectable_count -= 1
    }
    if self.selected_index.is_none() {
      return;
    }
    let selected = self.selected_index.as_mut().unwrap();
    if *selected < index && *selected != 0 {
      *selected -= 1
    }
    if *selected < index && *selected != 0 && entry.is_selectable() {
      self.selectable_selected -= 1
    }
  }

  pub fn update_for_removed_entry(
    &mut self,
    index: usize,
    entry: &dyn MenuEntry,
    entries: &SubmenuEntries
  ) {
    self.update_for_drained_entry(index, entry);
    if let Some(selected) = self.selected_index {
      match entries.list().get(selected) {
        Some(entry) if !entry.is_selectable() => {
          self.selected_index = entries.find_nearest_entry_index(selected, |_, e| e.is_selectable())
        }
        None => self.selected_index = None,
        _ => {}
      }
    }
  }

  pub fn post_drain(&mut self, entries: &SubmenuEntries) {
    match self.selected_index {
      Some(selected) => {
        self.selected_index = entries.find_nearest_entry_index(selected, |_, e| e.is_selectable())
      }
      None => {}
    }
  }

  pub fn update_for_inserted_entry(&mut self, index: usize, entry: &dyn MenuEntry) {
    if entry.is_selectable() {
      self.selectable_count += 1;
      if self.selected_index.is_none() {
        self.selected_index = Some(index);
        self.selectable_selected = 0
      }
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
