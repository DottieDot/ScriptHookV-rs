use crate::natives::mobile;

pub fn cell_cam_activate_selfie_mode(toggle: bool) {
  unsafe { mobile::cell_cam_activate_selfie_mode(toggle) }
}
