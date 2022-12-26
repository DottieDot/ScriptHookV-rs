use num_enum::IntoPrimitive;

#[derive(Clone, Copy, IntoPrimitive)]
#[repr(i32)]
pub enum Font {
  ChaletLondon = 0,
  HouseScript,
  Monospace,
  Icons,
  ChaletComprimeCologne,
  Numbers,
  ChaletComprimeCologneLight,
  Pricedown
}

impl Font {
  pub fn get_height(self, scale: f32) -> f32 {
    let base_size = match self {
      Font::ChaletLondon => 0.08f32,
      Font::HouseScript => 0.056f32,
      Font::Monospace => 0.072f32,
      Font::Icons => 0.1f32,
      Font::ChaletComprimeCologne => 0.072f32,
      Font::Numbers => 0.094f32,
      Font::ChaletComprimeCologneLight => 0.072f32,
      Font::Pricedown => 0.060f32
    };

    (base_size * scale) * 1080f32
  }
}
