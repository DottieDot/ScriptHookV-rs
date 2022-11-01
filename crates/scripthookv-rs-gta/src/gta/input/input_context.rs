use num_enum::IntoPrimitive;


#[derive(IntoPrimitive, Copy, Clone)]
#[repr(i32)]
pub enum InputContext {
  PlayerControl,
  CameraControl,
  FrontendControl
}
