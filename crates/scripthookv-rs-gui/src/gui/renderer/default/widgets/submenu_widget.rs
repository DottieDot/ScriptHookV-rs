use scripthookv::types::Vector2;

use crate::{
  gui::Submenu,
  rendering::{widgets::VerticalStack, HorizontalOrigin, Origin, RenderedWidget, Widget}
};

use super::{MenuEntriesWidget, MenuHeaderWidget};

pub struct SubmenuWidget {
  content: VerticalStack
}

impl SubmenuWidget {
  pub fn new(submenu: &Submenu, entries: Vec<Box<dyn Widget>>) -> Self {
    Self {
      content: VerticalStack::new(
        vec![
          Box::new(MenuHeaderWidget::new(submenu.title().clone())),
          Box::new(MenuEntriesWidget::new(entries)),
        ],
        0f32,
        Some(200f32),
        HorizontalOrigin::Left
      )
    }
  }
}

impl Widget for SubmenuWidget {
  fn draw(&self, position: Vector2, origin: Origin) -> RenderedWidget {
    self.content.draw(position, origin)
  }

  fn size_hint(&mut self, width: Option<f32>, height: Option<f32>) {
    self.content.size_hint(width, height)
  }

  fn size(&self) -> Vector2 {
    self.content.size()
  }
}
