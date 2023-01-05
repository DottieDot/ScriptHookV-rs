use std::borrow::Borrow;

use scripthookv::types::Vector2;

use crate::{
  gui::{
    renderer::{MenuEntryRenderer, MenuRenderer, SubmenuRenderer},
    Submenu
  },
  rendering::{HorizontalOrigin, Origin, VerticalOrigin}
};

use super::{menu_entry_renderer::DefaultMenuEntryRenderer, DefaultSubmenuRenderer};

pub struct DefaultMenuRenderer {
  submenu_renderer: Box<dyn SubmenuRenderer>,
  entry_renderer:   Box<dyn MenuEntryRenderer>
}

impl Default for DefaultMenuRenderer {
  fn default() -> Self {
    Self {
      submenu_renderer: Box::new(DefaultSubmenuRenderer::new(15)),
      entry_renderer:   Box::new(DefaultMenuEntryRenderer)
    }
  }
}

impl MenuRenderer for DefaultMenuRenderer {
  fn submenu_renderer(&self) -> &dyn SubmenuRenderer {
    self.submenu_renderer.borrow()
  }

  fn entry_renderer(&self) -> &dyn MenuEntryRenderer {
    self.entry_renderer.borrow()
  }

  fn render(&self, submenu: &Submenu) {
    self.submenu_renderer.widget(self, submenu).draw(
      Vector2::new(50f32, 50f32),
      Origin {
        horizontal: HorizontalOrigin::Left,
        vertical:   VerticalOrigin::Top
      }
    );
  }
}
