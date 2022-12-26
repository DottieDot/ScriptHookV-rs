use scripthookv::types::Vector2;
use scripthookv_gta::color::RGB;

use crate::rendering::{
  widgets::{DynamicLayoutSizing, Layout, Rectangle, Text, WithBackground},
  Font, HorizontalOrigin, Origin, Padding, RenderedWidget, VerticalOrigin, Widget
};

pub struct MenuEntryWidget {
  content: WithBackground<Layout<Text>, Rectangle>
}

impl MenuEntryWidget {
  pub fn new(text: impl Into<String>) -> Self {
    Self {
      content: WithBackground::new(
        Layout::new_dynamic(
          Text::new(
            text,
            Font::ChaletComprimeCologne,
            0.2f32,
            RGB::new(255, 255, 255).into(),
            false
          ),
          Origin {
            horizontal: HorizontalOrigin::Left,
            vertical:   VerticalOrigin::Center
          },
          Padding {
            top:    4f32,
            bottom: 4f32,
            left:   8f32,
            right:  8f32
          },
          DynamicLayoutSizing::OutsideIn,
          DynamicLayoutSizing::InsideOut
        ),
        Rectangle::new(RGB::new(40, 40, 40).into(), None, None)
      )
    }
  }
}

impl Widget for MenuEntryWidget {
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
