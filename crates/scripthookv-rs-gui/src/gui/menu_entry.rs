pub trait MenuEntry: 'static {
  fn is_selectable(&self) -> bool;
}
