use std::{
  cell::{Ref, RefCell, RefMut},
  sync::Arc
};

use crate::gui::Submenu;

pub struct MenuHistory {
  history: Vec<Arc<RefCell<Submenu>>>
}

impl MenuHistory {
  #[must_use]
  pub fn new(main_submenu: Arc<RefCell<Submenu>>) -> Self {
    main_submenu.borrow_mut().navigated_to();

    Self {
      history: vec![main_submenu]
    }
  }

  #[must_use]
  pub fn current(&self) -> Ref<Submenu> {
    self
      .history
      .last()
      .expect("MenuHistory should always contain at least one submenu")
      .as_ref()
      .borrow()
  }

  #[must_use]
  pub fn current_mut(&self) -> RefMut<Submenu> {
    self
      .history
      .last()
      .expect("MenuHistory should always contain at least one submenu")
      .as_ref()
      .borrow_mut()
  }

  pub fn push(&mut self, submenu: Arc<RefCell<Submenu>>) {
    self.current_mut().navigated_from();
    self.history.push(submenu);
    self.current_mut().navigated_to();
  }

  pub fn pop(&mut self) -> bool {
    if self.history.len() > 1 {
      let removed = self
        .history
        .pop()
        .expect("MenuHistory should always contain at least one submenu");

      removed.borrow_mut().navigated_from();
      self.current_mut().navigated_to();

      true
    } else {
      false
    }
  }
}
