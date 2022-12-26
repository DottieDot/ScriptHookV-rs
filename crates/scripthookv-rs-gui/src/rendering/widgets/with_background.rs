use scripthookv::types::Vector2;

use crate::rendering::{Origin, RenderedWidget, Resizable, Widget};

pub struct WithBackground<Inner: Widget, Background: Widget + Resizable> {
  inner:      Inner,
  background: Background
}

impl<Inner: Widget, Background: Widget + Resizable> WithBackground<Inner, Background> {
  pub fn new(inner: Inner, background: Background) -> Self {
    Self { inner, background }
  }

  fn update_background_size(&mut self) {
    let size = self.size();
    self.background.set_size(Some(size.x), Some(size.y))
  }
}

impl<Inner: Widget, Background: Widget + Resizable> Widget for WithBackground<Inner, Background> {
  fn draw(&self, position: Vector2, origin: Origin) -> RenderedWidget {
    self.background.draw(position, origin);

    self.inner.draw(position, origin)
  }

  fn size_hint(&mut self, width: Option<f32>, height: Option<f32>) {
    self.inner.size_hint(width, height);
    self.update_background_size()
  }

  fn size(&self) -> Vector2 {
    self.inner.size()
  }
}
