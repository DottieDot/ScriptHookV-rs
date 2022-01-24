use super::Scannable;
use thiserror::Error;

pub struct RawPattern {
  bytes: Vec<u8>,
  mask:  Vec<char>
}

impl Scannable for RawPattern {
  fn get_bytes(&self) -> &Vec<u8> {
    &self.bytes
  }

  fn get_mask(&self) -> &Vec<char> {
    &self.mask
  }

  fn len(&self) -> usize {
    self.bytes.len()
  }
}

fn check_invalid_char((_, char): &(usize, char)) -> bool {
  *char != 'x' && *char != '?'
}

impl RawPattern {
  pub fn new(bytes: &[u8], mask: &str) -> Result<Self, RawPatternError> {
    if bytes.len() != mask.len() {
      Err(RawPatternError::BytesAndMaskMismatch(
        bytes.len(),
        mask.len()
      ))
    } else if let Some((index, char)) = mask.char_indices().find(check_invalid_char) {
      Err(RawPatternError::InvalidMaskError(index, char))
    } else {
      Ok(Self {
        bytes: Vec::from(bytes),
        mask:  mask.chars().collect()
      })
    }
  }
}

#[derive(Error, Debug)]
pub enum RawPatternError {
  #[error("invalid mask, expected 'x' or '?' got {0} at {1}")]
  InvalidMaskError(usize, char),
  #[error("bytes length and mask length do not match {0} != {1}")]
  BytesAndMaskMismatch(usize, usize)
}
