use scripthookv::types::Vector2;

use crate::rendering::{Origin, Padding, RenderedWidget, Widget};

#[derive(Clone)]
pub struct Layout<W: Widget> {
  padding:       Padding,
  widget:        W,
  widget_origin: Origin,
  width_sizing:  LayoutSizing,
  height_sizing: LayoutSizing,
  width:         f32,
  height:        f32
}

impl<W: Widget> Layout<W> {
  pub fn new_dynamic(
    widget: W,
    widget_origin: Origin,
    padding: Padding,
    width_sizing: DynamicLayoutSizing,
    height_sizing: DynamicLayoutSizing
  ) -> Self {
    let mut instance = Self {
      padding,
      widget,
      widget_origin,
      width_sizing: width_sizing.into(),
      height_sizing: height_sizing.into(),
      width: 0f32,
      height: 0f32
    };

    instance.update_size();
    instance.hint_inner();

    instance
  }

  pub fn new_fixed(
    widget: W,
    widget_origin: Origin,
    padding: Padding,
    width: f32,
    height: f32
  ) -> Self {
    let mut instance = Self {
      padding,
      widget,
      widget_origin,
      width_sizing: LayoutSizing::Fixed,
      height_sizing: LayoutSizing::Fixed,
      width,
      height
    };

    instance.update_size();
    instance.hint_inner();

    instance
  }

  fn update_size(&mut self) {
    if self.width_sizing.is_inside_out() {
      self.width = self.widget.size().x + self.padding.left + self.padding.right
    }

    if self.height_sizing.is_inside_out() {
      self.height = self.widget.size().y + self.padding.top + self.padding.bottom
    }
  }

  fn hint_inner(&mut self) {
    let size = self.inner_size();
    self.widget.size_hint(Some(size.x), Some(size.y))
  }

  fn inner_size(&self) -> Vector2 {
    let padding = Vector2::new(
      self.padding.left + self.padding.right,
      self.padding.top + self.padding.bottom
    );
    self.size() - padding
  }
}

impl<W: Widget> Widget for Layout<W> {
  fn draw(&self, position: Vector2, origin: Origin) -> RenderedWidget {
    let inner_box = RenderedWidget {
      position,
      origin,
      size: self.inner_size()
    };
    self.widget.draw_offsetted(
      inner_box,
      Vector2::new(self.padding.left, self.padding.top),
      self.widget_origin,
      self.widget_origin
    );

    RenderedWidget {
      position,
      origin,
      size: self.size()
    }
  }

  fn size_hint(&mut self, width: Option<f32>, height: Option<f32>) {
    let inner_width_hint = match self.width_sizing {
      LayoutSizing::InsideOut => width,
      LayoutSizing::Fixed => Some(self.inner_size().x),
      LayoutSizing::OutsideIn => {
        self.width = width.unwrap_or(self.width);
        Some(self.inner_size().x)
      }
    };

    let inner_height_hint = match self.height_sizing {
      LayoutSizing::InsideOut => height,
      LayoutSizing::Fixed => Some(self.inner_size().y),
      LayoutSizing::OutsideIn => {
        self.height = height.unwrap_or(self.height);
        Some(self.inner_size().y)
      }
    };

    self.widget.size_hint(inner_width_hint, inner_height_hint);

    self.update_size()
  }

  fn size(&self) -> Vector2 {
    Vector2::new(self.width, self.height)
  }
}

#[derive(Clone, Copy)]
pub enum DynamicLayoutSizing {
  InsideOut,
  OutsideIn
}

#[derive(Clone, Copy)]
enum LayoutSizing {
  InsideOut,
  Fixed,
  OutsideIn
}

impl LayoutSizing {
  fn is_inside_out(self) -> bool {
    matches!(self, LayoutSizing::InsideOut)
  }
}

impl From<DynamicLayoutSizing> for LayoutSizing {
  fn from(value: DynamicLayoutSizing) -> Self {
    match value {
      DynamicLayoutSizing::InsideOut => Self::InsideOut,
      DynamicLayoutSizing::OutsideIn => Self::OutsideIn
    }
  }
}
