pub trait Scannable {
  fn get_bytes(&self) -> &Vec<u8>;
  fn get_mask(&self) -> &Vec<char>;
  fn len(&self) -> usize;
  fn is_empty(&self) -> bool {
    self.len() == 0
  }
}
