use crate::gui::MenuEntry;

#[derive(Default)]
pub struct SubmenuSelection {
  selected_index:      usize,
  selectable_count:    usize,
  selectable_selected: usize
}

impl SubmenuSelection {
  pub fn update_for_removed_entry(&mut self, index: usize, entry: &dyn MenuEntry) {
    if entry.is_selectable() {
      self.selectable_count -= 1
    }
    if self.selected_index <= index {
      self.selected_index -= 1
    }
    if self.selected_index <= index && entry.is_selectable() {
      self.selectable_selected -= 1
    }
  }

  pub fn update_for_inserted_entry(&mut self, index: usize, entry: &dyn MenuEntry) {
    if entry.is_selectable() {
      self.selectable_count += 1;
    }
    if index <= self.selected_index {
      self.selected_index += 1
    }
    if index <= self.selected_index && entry.is_selectable() {
      self.selectable_selected += 1
    }
  }

  pub fn scroll_up(&mut self, entries: &Vec<Box<dyn MenuEntry>>) -> usize {
    let mut new_selectable_selected = self.selectable_selected;
    for n in 1..entries.len() {
      let index = (self.selected_index - n) % entries.len();

      if index == (entries.len() - 1) {
        new_selectable_selected = self.selectable_count - 1;
      }

      if entries[index].is_selectable() {
        self.selectable_selected = new_selectable_selected;
        return index
      } else {
        new_selectable_selected -= 1;
      }
    }
    
    self.selected_index
  }

  pub fn scroll_down(&mut self, entries: &Vec<Box<dyn MenuEntry>>) -> usize {
    let mut new_selectable_selected = self.selectable_selected;
    for n in 1..entries.len() {
      let index = (self.selected_index + n) % entries.len();

      if index == 0 {
        new_selectable_selected = 0;
      }

      if entries[index].is_selectable() {
        self.selectable_selected = new_selectable_selected;
        return index
      }
    }
    
    self.selected_index
  }

  pub fn get_selection(&self) -> usize {
    self.selected_index
  }
}
