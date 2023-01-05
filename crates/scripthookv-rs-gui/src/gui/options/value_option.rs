use std::ops::{Add, Sub};

use num_traits::Bounded;

use crate::gui::{
  renderer::{MenuEntryRenderInfo, MenuEntryValue},
  MenuEntry, ValueSource
};

pub struct ValueOption<T> {
  text:  String,
  value: ValueSource<T>,
  step:  T,
  min:   Option<T>,
  max:   Option<T>
}

impl<T: Copy + Add<Output = T> + Sub<Output = T> + Bounded + PartialOrd> ValueOption<T> {
  fn increment(&mut self) {
    let mut value = self.value.get();

    let max_value = self.max.unwrap_or(T::max_value());
    if value == max_value {
      if let Some(min) = self.min {
        value = min
      }
    } else if value > max_value - self.step {
      value = max_value
    } else {
      value = value + self.step
    }

    self.value.set(value)
  }

  fn decrement(&mut self) {
    let mut value = self.value.get();

    let min_value = self.min.unwrap_or(T::min_value());
    if value == min_value {
      if let Some(max) = self.max {
        value = max
      }
    } else if value < min_value + self.step {
      value = min_value
    } else {
      value = value - self.step
    }

    self.value.set(value)
  }
}

impl<T: Copy + Add<Output = T> + Sub<Output = T> + ToString + Bounded + PartialOrd + 'static>
  MenuEntry for ValueOption<T>
{
  fn on_focus(&mut self) {}

  fn on_blur(&mut self) {}

  fn is_selectable(&self) -> bool {
    true
  }

  fn process(&mut self, controls: &crate::gui::MenuControls) {
    if controls.right.active() {
      self.increment()
    }

    if controls.left.active() {
      self.decrement()
    }
  }

  fn render_info(&self) -> crate::gui::renderer::MenuEntryRenderInfo {
    MenuEntryRenderInfo {
      text:   self.text.clone(),
      value:  Some(MenuEntryValue::Adjustable {
        value: self.value.get().to_string()
      }),
      toggle: None
    }
  }
}

#[derive(Default)]
pub struct ValueOptionBuilder<T> {
  text:  Option<String>,
  value: Option<ValueSource<T>>,
  step:  Option<ValueSource<T>>,
  min:   Option<ValueSource<T>>,
  max:   Option<ValueSource<T>>
}

impl<
    T: Default + Copy + Add<Output = T> + Sub<Output = T> + ToString + Bounded + PartialOrd + 'static
  > ValueOptionBuilder<T>
{
  pub fn text(mut self, text: impl Into<String>) -> Self {
    self.text = Some(text.into());
    self
  }

  pub fn value(mut self, source: impl Into<ValueSource<T>>) -> Self {
    self.value = Some(source.into());
    self
  }

  pub fn step(mut self, step_size: impl Into<ValueSource<T>>) -> Self {
    self.step = Some(step_size.into());
    self
  }

  pub fn min(mut self, min_value: impl Into<ValueSource<T>>) -> Self {
    self.min = Some(min_value.into());
    self
  }

  pub fn max(mut self, max_value: impl Into<ValueSource<T>>) -> Self {
    self.max = Some(max_value.into());
    self
  }
}

pub fn testing() {
  let builder = ValueOptionBuilder::default()
    .text("Hello")
    .value(15)
    .min(10)
    .max(20)
    .step(1);
}
