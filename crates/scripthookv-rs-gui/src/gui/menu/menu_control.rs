use std::time::{Duration, Instant};

use scripthookv_gta::gta::input::{Control, InputContext};

#[derive(Clone, Copy)]
pub enum MenuKey {
  GtaControl(Control)
}

impl MenuKey {
  pub fn is_down(self) -> bool {
    match self {
      Self::GtaControl(control) => control.is_pressed(InputContext::FrontendControl, true)
    }
  }

  pub fn just_pressed(self) -> bool {
    match self {
      Self::GtaControl(control) => control.is_pressed(InputContext::FrontendControl, true)
    }
  }

  pub fn just_released(self) -> bool {
    match self {
      Self::GtaControl(control) => control.is_just_released(InputContext::FrontendControl, true)
    }
  }
}

pub struct MenuControl {
  keyboard_key:              MenuKey,
  controller_key:            MenuKey,
  repeating:                 bool,
  extra_controls_to_disable: Vec<Control>,
  next_repeat_check:         Instant,
  active:                    bool
}

impl MenuControl {
  pub fn new(
    keyboard_key: MenuKey,
    controller_key: MenuKey,
    repeating: bool,
    extra_controls_to_disable: Vec<Control>
  ) -> Self {
    Self {
      keyboard_key,
      controller_key,
      repeating,
      extra_controls_to_disable,
      next_repeat_check: Instant::now(),
      active: false
    }
  }

  pub fn active(&self) -> bool {
    self.active
  }

  pub fn update(&mut self, repeating_interval: &Duration) {
    if self.repeating {
      self.active = false;

      if self.get_active_key().just_released() {
        self.next_repeat_check = Instant::now()
      } else if self.get_active_key().is_down() && Instant::now() >= self.next_repeat_check {
        self.next_repeat_check = Instant::now() + *repeating_interval;
        self.active = true;
      }
    } else {
      self.active = self.get_active_key().just_pressed();
    }
  }

  pub fn disable_game_behavior(&self) {
    use InputContext::{CameraControl, FrontendControl, PlayerControl};

    match self.get_active_key() {
      MenuKey::GtaControl(control) => {
        control.disable(FrontendControl);
        control.disable(PlayerControl);
        control.disable(CameraControl);
      }
    }

    for control in &self.extra_controls_to_disable {
      control.disable(FrontendControl);
      control.disable(PlayerControl);
      control.disable(CameraControl);
    }
  }

  #[must_use]
  fn get_active_key(&self) -> MenuKey {
    if Control::is_using_keyboard(InputContext::FrontendControl) {
      self.keyboard_key
    } else {
      self.controller_key
    }
  }
}

pub struct MenuControls {
  pub up:      MenuControl,
  pub down:    MenuControl,
  pub left:    MenuControl,
  pub right:   MenuControl,
  pub confirm: MenuControl,
  pub back:    MenuControl,
  pub delete:  MenuControl,
  pub save:    MenuControl
}
