use std::ffi::CString;

use once_cell::sync::Lazy;
use scripthookv::types::Vector2;
use scripthookv_gta::{
  color::{Color, RGBA},
  natives::hud
};

use crate::rendering::{
  font::Font,
  util::{pixel_space_to_proportional_space, transform_origin},
  HorizontalOrigin, Origin, RenderedWidget, VerticalOrigin, Widget
};

static CELL_EMAIL_BCON: Lazy<CString> = Lazy::new(|| CString::new("CELL_EMAIL_BCON").unwrap());

pub struct Text {
  text:      String,
  font:      Font,
  font_size: f32,
  color:     Color,
  outline:   bool,
  width:     f32,
  height:    f32
}

impl Text {
  pub fn new(
    text: impl Into<String>,
    font: Font,
    font_size: f32,
    color: Color,
    outline: bool
  ) -> Self {
    let mut instance = Self {
      text: text.into(),
      font,
      font_size,
      color,
      outline,
      width: 0f32,
      height: 0f32
    };

    instance.width = instance.calculate_width();
    instance.height = instance.calculate_height();

    instance
  }

  fn set_properties(&self, horizontal_origin: HorizontalOrigin) {
    let RGBA { r, g, b, a } = self.color.into();

    unsafe {
      hud::set_text_font(self.font.into());
      hud::set_text_scale(0f32, self.font_size);
      hud::set_text_colour(r as i32, g as i32, b as i32, a as i32);
      if self.outline {
        hud::set_text_outline();
      }
      hud::set_text_justification(match horizontal_origin {
        HorizontalOrigin::Left => 1,
        HorizontalOrigin::Center => 0,
        HorizontalOrigin::Right => 2
      });
    }
  }

  fn push_text(&self) {
    let mut chars = self.text.chars();

    for substring in (0..)
      .map(|_| chars.by_ref().take(90).collect::<String>())
      .take_while(|s| !s.is_empty())
    {
      let cstring = CString::new(substring).unwrap();
      unsafe {
        hud::add_text_component_substring_player_name(cstring.as_ptr());
      }
    }
  }

  fn calculate_width(&self) -> f32 {
    unsafe {
      hud::begin_text_command_get_screen_width_of_display_text(CELL_EMAIL_BCON.as_ptr());
    }

    self.push_text();
    self.set_properties(HorizontalOrigin::Center);

    unsafe { hud::end_text_command_get_screen_width_of_display_text(true) * 2560f32 }
  }

  fn calculate_height(&self) -> f32 {
    unsafe {
      hud::begin_text_command_get_number_of_lines_for_string(CELL_EMAIL_BCON.as_ptr());
    }

    self.push_text();
    self.set_properties(HorizontalOrigin::Left);

    let lines =
      unsafe { hud::end_text_command_get_number_of_lines_for_string(Vector2::new(0f32, 0f32)) };

    lines as f32 * self.font.get_height(self.font_size)
  }
}

impl Widget for Text {
  fn draw(&self, position: Vector2, origin: Origin) -> RenderedWidget {
    let size = self.size();

    let draw_origin = transform_origin(
      position,
      size,
      origin,
      Origin {
        horizontal: origin.horizontal,
        vertical:   VerticalOrigin::Top
      }
    );

    let draw_position = pixel_space_to_proportional_space(draw_origin);

    unsafe {
      hud::begin_text_command_display_text(CELL_EMAIL_BCON.as_ptr());
    }

    self.set_properties(origin.horizontal);
    self.push_text();

    unsafe {
      hud::end_text_command_display_text(draw_position, 0);
    }

    RenderedWidget {
      position,
      origin,
      size
    }
  }

  fn size_hint(&mut self, _: Option<f32>, _: Option<f32>) {}

  fn size(&self) -> Vector2 {
    Vector2::new(self.width, self.height)
  }
}
