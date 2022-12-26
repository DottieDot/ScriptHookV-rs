#[derive(Clone, Copy)]
pub enum Sizing {
  Defined(f32),
  OutsideIn()
}

#[derive(Clone, Copy)]
pub enum SizingMode {
  Defined,
  OutsideIn
}

impl SizingMode {
  pub fn is_defined(self) -> bool {
    matches!(self, Self::Defined)
  }

  pub fn is_outside_in(self) -> bool {
    matches!(self, Self::OutsideIn)
  }
}

#[derive(Clone, Copy)]
pub struct Size {
  pub size: f32,
  pub mode: SizingMode
}

impl From<Sizing> for Size {
  fn from(value: Sizing) -> Self {
    match value {
      Sizing::Defined(size) => {
        Self {
          size,
          mode: SizingMode::Defined
        }
      }
      Sizing::OutsideIn() => {
        Self {
          size: 0f32,
          mode: SizingMode::OutsideIn
        }
      }
    }
  }
}
