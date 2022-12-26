use super::MenuControls;

pub trait MenuEntry: 'static {
  fn on_focus(&mut self);

  fn on_blur(&mut self);

  fn is_selectable(&self) -> bool;

  fn process(&mut self, controls: &MenuControls);
}
