use thiserror::Error;

use super::scannable::Scannable;

pub struct IdaPattern {
  bytes: Vec<u8>,
  mask:  Vec<char>
}

impl Scannable for IdaPattern {
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

impl IdaPattern {
  pub fn new(pattern: &str) -> Result<Self, InvalidIdaPatternError> {
    let mut bytes: Vec<u8> = Vec::new();
    let mut mask: Vec<char> = Vec::new();

    let split = pattern.split(' ');
    for byte_string in split {
      if byte_string.starts_with('?') {
        mask.push('?');
        bytes.push(0x00);
      } else {
        mask.push('x');
        let byte = u8::from_str_radix(byte_string, 16).map_err(|_| {
          InvalidIdaPatternError {
            pattern: pattern.into()
          }
        })?;
        bytes.push(byte);
      }
    }

    Ok(Self { bytes, mask })
  }
}

#[derive(Debug, Error)]
#[error("The IDA pattern \"{pattern}\" is malformed")]
pub struct InvalidIdaPatternError {
  pattern: String
}
