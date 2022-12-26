use scripthookv::types::Vector2;

use super::{origin::Origin, util::relative_origin_position};

pub trait Widget {
  fn draw(&self, position: Vector2, origin: Origin) -> RenderedWidget;

  fn draw_offsetted(
    &self,
    widget: RenderedWidget,
    offset: Vector2,
    offset_origin: Origin,
    draw_origin: Origin
  ) -> RenderedWidget {
    let offset_from = widget.position
      + relative_origin_position(widget.size, offset_origin, widget.origin)
      + offset;

    self.draw(offset_from, draw_origin)
  }

  fn size_hint(&mut self, width: Option<f32>, height: Option<f32>);

  fn size(&self) -> Vector2;
}

#[derive(Clone, Copy)]
pub struct RenderedWidget {
  pub position: Vector2,
  pub origin:   Origin,
  pub size:     Vector2
}
