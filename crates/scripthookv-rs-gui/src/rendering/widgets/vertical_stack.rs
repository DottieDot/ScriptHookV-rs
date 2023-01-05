use scripthookv::types::Vector2;

use crate::rendering::{
  util::transform_origin, HorizontalOrigin, Origin, RenderedWidget, VerticalOrigin, Widget
};

pub struct VerticalStack {
  children:          Vec<Box<dyn Widget>>,
  gap:               f32,
  width:             Option<f32>,
  horizontal_origin: HorizontalOrigin,
  height:            f32
}

impl VerticalStack {
  pub fn new(
    children: Vec<Box<dyn Widget>>,
    gap: f32,
    width: Option<f32>,
    horizontal_origin: HorizontalOrigin
  ) -> Self {
    let mut instance = Self {
      children,
      gap,
      width,
      horizontal_origin,
      height: 0f32
    };
    instance.update_size();
    instance.hint_children();
    instance
  }

  fn hint_children(&mut self) {
    for child in self.children.iter_mut() {
      child.size_hint(self.width, None)
    }
  }

  fn update_size(&mut self) {
    if self.children.is_empty() {
      self.height = 0f32;
      return;
    }

    let mut height = self.gap * (self.children.len() - 1) as f32;
    for child in self.children.iter() {
      height += child.size().y;
    }
    self.height = height;

    if self.width.is_none() {
      let max_width = self
        .children
        .iter()
        .map(|c| c.size().x)
        .fold(f32::NEG_INFINITY, |a, b| a.max(b));

      if max_width != 0f32 {
        self.width = Some(max_width);
      }
    }
  }
}

impl Widget for VerticalStack {
  fn draw(&self, position: Vector2, origin: Origin) -> RenderedWidget {
    let size = self.size();
    let draw_origin = Origin {
      horizontal: self.horizontal_origin,
      vertical:   VerticalOrigin::Top
    };
    let offset_origin = Origin {
      horizontal: draw_origin.horizontal,
      vertical:   VerticalOrigin::Bottom
    };
    let gap_offset = Vector2::new(0f32, self.gap);
    let draw_position = transform_origin(position, size, origin, draw_origin);

    let mut last = RenderedWidget {
      position: draw_position - gap_offset,
      origin:   draw_origin,
      size:     Vector2::zero()
    };
    for child in self.children.iter() {
      last = child.draw_offsetted(last, gap_offset, offset_origin, draw_origin);
    }

    RenderedWidget {
      position,
      origin,
      size
    }
  }

  fn size_hint(&mut self, width: Option<f32>, _: Option<f32>) {
    self.width = self.width.or(width);
    self.hint_children();
  }

  fn size(&self) -> Vector2 {
    Vector2::new(self.width.unwrap_or_default(), self.height)
  }
}
