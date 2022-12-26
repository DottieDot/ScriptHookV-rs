pub trait Resizable {
  fn set_size(&mut self, width: Option<f32>, height: Option<f32>);
}
