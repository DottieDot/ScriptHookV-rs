pub enum ValueSource<T> {
  Owned(T)
}

impl<T: Copy> ValueSource<T> {
  pub fn get(&self) -> T {
    match self {
      ValueSource::Owned(v) => *v
    }
  }

  pub fn set(&mut self, value: T) {
    match self {
      ValueSource::Owned(v) => *v = value
    }
  }
}

impl<T: Copy> From<T> for ValueSource<T> {
  fn from(value: T) -> Self {
    Self::Owned(value)
  }
}
