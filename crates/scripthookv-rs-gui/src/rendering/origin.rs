#[derive(Clone, Copy)]
pub enum HorizontalOrigin {
  Left,
  Center,
  Right
}

#[derive(Clone, Copy)]
pub enum VerticalOrigin {
  Top,
  Center,
  Bottom
}

#[derive(Clone, Copy)]
pub struct Origin {
  pub horizontal: HorizontalOrigin,
  pub vertical:   VerticalOrigin
}
