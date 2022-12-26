use scripthookv::types::Vector2;

use super::origin::{HorizontalOrigin, Origin, VerticalOrigin};

pub fn transform_origin(
  position: Vector2,
  size: Vector2,
  original_origin: Origin,
  new_origin: Origin
) -> Vector2 {
  let to_left = match original_origin.horizontal {
    HorizontalOrigin::Left => 0f32,
    HorizontalOrigin::Center => -0.5f32,
    HorizontalOrigin::Right => -1f32
  };

  let to_top = match original_origin.vertical {
    VerticalOrigin::Top => 0f32,
    VerticalOrigin::Center => -0.5f32,
    VerticalOrigin::Bottom => -1f32
  };

  let left_to_new = match new_origin.horizontal {
    HorizontalOrigin::Left => 0f32,
    HorizontalOrigin::Center => 0.5f32,
    HorizontalOrigin::Right => 1f32
  };

  let top_to_new = match new_origin.vertical {
    VerticalOrigin::Top => 0f32,
    VerticalOrigin::Center => 0.5f32,
    VerticalOrigin::Bottom => 1f32
  };

  Vector2::new(
    position.x + (size.x * (to_left + left_to_new)),
    position.y + (size.y * (to_top + top_to_new))
  )
}

pub fn relative_origin_position(size: Vector2, origin: Origin, relative_to: Origin) -> Vector2 {
  transform_origin(Vector2::zero(), size, relative_to, origin)
}

pub fn get_font_height() {}

pub fn ui_scale() -> f32 {
  // TODO: Actually calculate this
  1f32
}

pub fn pixel_space_to_proportional_space(vec: Vector2) -> Vector2 {
  // TODO: use actual game resolution
  let resolution = Vector2::new(2560f32, 1080f32);

  (vec / resolution) * ui_scale()
}
