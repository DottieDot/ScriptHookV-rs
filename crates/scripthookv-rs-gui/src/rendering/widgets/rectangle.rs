use scripthookv::types::Vector2;
use scripthookv_gta::{
  color::{Color, RGBA},
  natives::graphics
};

use crate::rendering::{
  origin::{HorizontalOrigin, Origin, VerticalOrigin},
  util::{pixel_space_to_proportional_space, transform_origin},
  widget::Widget,
  RenderedWidget, Resizable
};

#[derive(Clone, Copy)]
pub struct Rectangle {
  color:  Color,
  width:  Option<f32>,
  height: Option<f32>
}

impl Rectangle {
  pub fn new(color: Color, width: Option<f32>, height: Option<f32>) -> Self {
    Self {
      color,
      width,
      height
    }
  }
}

impl Widget for Rectangle {
  fn draw(&self, position: Vector2, origin: Origin) -> RenderedWidget {
    let RGBA { r, g, b, a } = self.color.into();
    let size = self.size();

    let center_position = transform_origin(
      position,
      size,
      origin,
      Origin {
        horizontal: HorizontalOrigin::Center,
        vertical:   VerticalOrigin::Center
      }
    );

    let draw_position = pixel_space_to_proportional_space(center_position);
    let draw_size = pixel_space_to_proportional_space(size);

    unsafe {
      graphics::draw_rect(
        draw_position,
        draw_size.x,
        draw_size.y,
        r as i32,
        g as i32,
        b as i32,
        a as i32,
        true
      );
    }

    RenderedWidget {
      position,
      origin,
      size
    }
  }

  fn size_hint(&mut self, width: Option<f32>, height: Option<f32>) {
    self.width = self.width.or(width);
    self.height = self.height.or(height);
  }

  fn size(&self) -> Vector2 {
    Vector2::new(
      self.width.unwrap_or_default(),
      self.height.unwrap_or_default()
    )
  }
}

impl Resizable for Rectangle {
  fn set_size(&mut self, width: Option<f32>, height: Option<f32>) {
    self.width = width.or(self.width);
    self.height = height.or(self.height);
  }
}
