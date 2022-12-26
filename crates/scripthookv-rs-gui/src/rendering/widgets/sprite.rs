use std::ffi::CString;

use scripthookv::types::Vector2;
use scripthookv_gta::{
  color::{Color, RGBA},
  natives::graphics
};

use crate::rendering::{
  origin::{HorizontalOrigin, Origin, VerticalOrigin},
  util::{pixel_space_to_proportional_space, transform_origin},
  widget::Widget,
  RenderedWidget, SpriteDefinition
};

#[derive(Clone)]
pub struct Sprite {
  color:  Color,
  sprite: SpriteDefinition,
  width:  Option<f32>,
  height: Option<f32>
}

impl Sprite {
  pub fn new(
    sprite: SpriteDefinition,
    color: Color,
    width: Option<f32>,
    height: Option<f32>
  ) -> Self {
    Self {
      sprite,
      color,
      width,
      height
    }
  }

  pub fn set_size(&mut self, x: Option<f32>, y: Option<f32>) {
    self.width = x.or(self.width);
    self.height = y.or(self.height);
  }
}

impl Widget for Sprite {
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

    if !self.sprite.texture_dict.loaded() {
      self.sprite.texture_dict.request();
    }

    unsafe {
      let texture_dict = CString::new(self.sprite.texture_dict.name.as_str()).unwrap();
      let texture_name = CString::new(self.sprite.texture_name.as_str()).unwrap();
      graphics::draw_sprite(
        texture_dict.as_ptr(),
        texture_name.as_ptr(),
        draw_position,
        draw_size.x,
        draw_size.y,
        0f32,
        r as i32,
        g as i32,
        b as i32,
        a as i32,
        false,
        0
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
