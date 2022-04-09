use std::{rc::Rc, sync::RwLock};

use crate::EventEmitter;

use super::{SubmenuEntries, SubmenuSelection};

pub struct Submenu {
  title:         String,
  subtitle:      String,
  event_emitter: EventEmitter<Self>,
  entries:       SubmenuEntries,
  selection:     Rc<RwLock<SubmenuSelection>>
}

impl Submenu {
  pub fn new(title: String, subtitle: String, build_fn: impl Fn(&mut SubmenuEntries)) -> Self {
    let selection: Rc<RwLock<SubmenuSelection>> = Rc::new(RwLock::new(SubmenuSelection::default()));

    let mut entries = SubmenuEntries::new(selection.clone());
    build_fn(&mut entries);

    Self {
      title,
      subtitle,
      event_emitter: Default::default(),
      entries,
      selection
    }
  }

  pub fn on<Event: 'static>(
    &mut self,
    callback: impl Fn(&mut Self, &Event) + 'static
  ) -> &mut Self {
    self.event_emitter.on(callback);
    self
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
    self
      .selection
      .write()
      .unwrap()
      .scroll_up(self.entries.list());
  }

  pub fn scroll_down(&mut self) {
    self
      .selection
      .write()
      .unwrap()
      .scroll_down(self.entries.list());
  }
}