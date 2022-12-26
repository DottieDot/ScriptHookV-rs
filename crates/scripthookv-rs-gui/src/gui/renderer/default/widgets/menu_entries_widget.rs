use scripthookv::types::Vector2;

use crate::rendering::{
  widgets::VerticalStack, HorizontalOrigin::Left, Origin, RenderedWidget, Widget
};

pub struct MenuEntriesWidget {
  stack: VerticalStack
}

impl MenuEntriesWidget {
  pub fn new(menu_entries: Vec<Box<dyn Widget>>) -> Self {
    Self {
      stack: VerticalStack::new(menu_entries, 0f32, None, Left)
    }
  }
}

impl Widget for MenuEntriesWidget {
  fn draw(&self, position: Vector2, origin: Origin) -> RenderedWidget {
    self.stack.draw(position, origin)
  }

  fn size_hint(&mut self, width: Option<f32>, height: Option<f32>) {
    self.stack.size_hint(width, height)
  }

  fn size(&self) -> Vector2 {
    self.stack.size()
  }
}
