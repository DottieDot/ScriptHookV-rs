use crate::natives::hud;

#[inline]
pub fn hide_help_text_this_frame() {
  unsafe { hud::hide_help_text_this_frame() }
}
