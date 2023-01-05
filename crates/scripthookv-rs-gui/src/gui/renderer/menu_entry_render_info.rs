use crate::rendering::SpriteDefinition;

pub struct MenuEntryRenderInfo {
  pub text:   String,
  pub value:  Option<MenuEntryValue>,
  pub toggle: Option<MenuEntryToggle>
}

pub enum MenuEntryValue {
  List {
    value: String,
    index: usize,
    count: usize
  },
  Adjustable {
    value: String
  },
  ReadOnly {
    value: String
  }
}

pub struct MenuEntryToggle {
  pub toggle: bool,
  pub style:  Option<MenuEntryToggleStyle>
}

pub struct MenuEntryToggleStyle {
  pub light_on:  SpriteDefinition,
  pub light_off: SpriteDefinition,
  pub dark_on:   SpriteDefinition,
  pub dark_off:  SpriteDefinition
}
