mod menu_control;
mod menu_history;

pub use menu_control::*;

use std::{borrow::Borrow, cell::RefCell, sync::Arc, time::Duration};

use scripthookv_gta::gta::{
  hud,
  input::{Control, InputContext},
  mobile, Cam
};

use self::menu_history::MenuHistory;

use super::{renderer::MenuRenderer, Submenu};

pub struct Menu {
  open:     bool,
  controls: MenuControls,
  history:  MenuHistory,
  renderer: Box<dyn MenuRenderer>
}

impl Menu {
  pub fn new(
    controls: MenuControls,
    main_submenu: Arc<RefCell<Submenu>>,
    renderer: impl Into<Box<dyn MenuRenderer>>
  ) -> Self {
    Self {
      open: true,
      controls,
      history: MenuHistory::new(main_submenu),
      renderer: renderer.into()
    }
  }

  pub fn open(&mut self) {
    if !self.open {
      self.open = true;
      self.history.current_mut().menu_opened()
    }
  }

  pub fn close(&mut self) {
    if self.open {
      self.open = false;
      self.history.current_mut().menu_closed()
    }
  }

  pub fn navigate(&mut self, submenu: Arc<RefCell<Submenu>>) {
    self.history.push(submenu);
  }

  pub fn back(&mut self) {
    if !self.history.pop() {
      self.close()
    }
  }

  #[must_use]
  pub fn is_open(&self) -> bool {
    self.open
  }
}

impl Menu {
  pub fn process(&mut self) {
    if self.is_open() {
      self.disable_game_behavior();
      self.update_controls();
      self.update()
    }
  }

  fn disable_game_behavior(&self) {
    use InputContext::{CameraControl, FrontendControl, PlayerControl};

    hud::hide_help_text_this_frame();
    mobile::cell_cam_activate_selfie_mode(false);
    Cam::set_cinematic_button_active(false);

    Control::NextCamera.disable(FrontendControl);
    Control::Phone.disable(FrontendControl);

    Control::VehCinCam.disable(CameraControl);
    Control::VehNextRadio.disable(CameraControl);
    Control::VehPrevRadio.disable(CameraControl);

    Control::PrevWeapon.disable(PlayerControl);
    Control::MeleeAttackLight.disable(PlayerControl);
    Control::MeleeAttackHeavy.disable(PlayerControl);
    Control::MeleeAttackAlternate.disable(PlayerControl);

    Control::SelectCharacterFranklin.disable(FrontendControl);
    Control::SelectCharacterMichael.disable(FrontendControl);
    Control::SelectCharacterTrevor.disable(FrontendControl);
    Control::SelectCharacterMultiplayer.disable(FrontendControl);
    Control::CharacterWheel.disable(FrontendControl);

    Control::ReplayRecord.disable(FrontendControl);

    Control::MultiplayerInfo.disable(FrontendControl);
    Control::MapPoi.disable(FrontendControl);

    self.controls.up.disable_game_behavior();
    self.controls.down.disable_game_behavior();
    self.controls.left.disable_game_behavior();
    self.controls.right.disable_game_behavior();
    self.controls.confirm.disable_game_behavior();
    self.controls.back.disable_game_behavior();
    self.controls.delete.disable_game_behavior();
    self.controls.save.disable_game_behavior();
  }

  fn update_controls(&mut self) {
    let interval = Duration::from_millis(150);

    self.controls.up.update(&interval);
    self.controls.down.update(&interval);
    self.controls.left.update(&interval);
    self.controls.right.update(&interval);
    self.controls.confirm.update(&interval);
    self.controls.back.update(&interval);
    self.controls.delete.update(&interval);
    self.controls.save.update(&interval);
  }

  fn update(&mut self) {
    if self.controls.up.active() {
      self.history.current_mut().scroll_up()
    }

    if self.controls.down.active() {
      self.history.current_mut().scroll_down()
    }

    if self.controls.back.active() {
      self.back()
    }

    self
      .history
      .current_mut()
      .process(&self.controls, self.renderer.borrow())
  }
}
