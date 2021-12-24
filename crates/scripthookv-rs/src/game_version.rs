use shv_bindings::getGameVersion;

// Taken from https://github.com/E66666666/GTAVManualTransmission/blob/1e3e73070ad293536ea8a2d0c8fea58477830b9d/Gears/Memory/Versions.h

/// Used for identifying game versions.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy)]
pub enum GameVersion {
  Build_335_2_Steam        = 00,
  Build_335_2_NotSteam     = 01,
  
  Build_350_1_Steam        = 02,
  Build_350_2_NotSteam     = 03,
  
  Build_372_2_Steam        = 04,
  Build_372_2_NotSteam     = 05,
  
  Build_393_2_Steam        = 06,
  Build_393_2_NotSteam     = 07,
  
  Build_393_4_Steam        = 08,
  Build_393_4_NotSteam     = 09,
  
  Build_463_1_Steam        = 10,
  Build_463_1_NotSteam     = 11,
  
  Build_505_2_Steam        = 12,
  Build_505_2_NotSteam     = 13,
  
  Build_573_1_Steam        = 14,
  Build_573_1_NotSteam     = 15,
  
  Build_617_1_Steam        = 16,
  Build_617_1_NotSteam     = 17,
  
  Build_678_1_Steam        = 18,
  Build_678_1_NotSteam     = 19,
  
  Build_757_2_Steam        = 20,
  Build_757_2_NotSteam     = 21,
  
  Build_757_4_Steam        = 22,
  Build_757_4_NotSteam     = 23,
  
  Build_791_2_Steam        = 24,
  Build_791_2_NotSteam     = 25,
  
  Build_877_1_Steam        = 26,
  Build_877_1_NotSteam     = 27,
  
  Build_944_2_Steam        = 28,
  Build_944_2_NotSteam     = 29,
  
  Build_1011_1_Steam       = 30,
  Build_1011_1_NotSteam    = 31,
  
  Build_1032_1_Steam       = 32,
  Build_1032_1_NotSteam    = 33,
  
  Build_1103_2_Steam       = 34,
  Build_1103_2_NotSteam    = 35,
  
  Build_1180_2_Steam       = 36,
  Build_1180_2_NotSteam    = 37,
  
  Build_1290_1_Steam       = 38,
  Build_1290_1_NotSteam    = 39,
  
  Build_1365_1_Steam       = 40,
  Build_1365_1_NotSteam    = 41,
  
  Build_1493_0_Steam       = 42,
  Build_1493_0_NotSteam    = 43,
  
  Build_1493_1_Steam       = 44,
  Build_1493_1_NotSteam    = 45,
  
  Build_1604_0_Steam       = 46,
  Build_1604_0_NotSteam    = 47,
  
  Build_1604_1_Steam       = 48,
  Build_1604_1_NotSteam    = 49,

  Build_1737_0_Steam       = 50,
  Build_1737_0_NotSteam    = 51,
  
  Build_1737_6_Steam       = 52,
  Build_1737_6_NotSteam    = 53,
  
  Build_1868_0_Steam       = 54,
  Build_1868_0_NotSteam    = 55,
  
  Build_1868_1_Steam       = 56,
  Build_1868_1_NotSteam    = 57,
  
  Build_1868_4_EGS         = 58,
  
  Build_2060_0_Steam       = 59,
  Build_2060_0_NotSteam    = 60,
  
  Build_2060_1_Steam       = 61,
  Build_2060_1_NotSteam    = 62,
  
  Build_2189_0_Steam       = 63,
  Build_2189_0_NotSteam    = 64,
  
  Build_2215_0_Steam       = 65,
  Build_2215_0_NotSteam    = 66,
  
  Build_2245_0_Steam       = 67,
  Build_2245_0_NotSteam    = 68,
  
  Build_2372_0_Steam       = 69,
  Build_2372_0_NotSteam    = 70,

  Build_2545_0_Steam       = 71,
  Build_2545_0_NotSteam    = 72
}

#[derive(Debug, Clone)]
pub struct TryFromIntError {
  value: i32
}

impl std::fmt::Display for TryFromIntError {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(f, "{} is not a known game version", self.value)
  }
}


impl TryFrom<i32> for GameVersion {
  type Error = TryFromIntError;

  fn try_from(value: i32) -> Result<Self, Self::Error> {
    match value {
      x if x == GameVersion::Build_335_2_Steam     as i32 => Ok(GameVersion::Build_335_2_Steam),
      x if x == GameVersion::Build_335_2_NotSteam  as i32 => Ok(GameVersion::Build_335_2_NotSteam),
   
      x if x == GameVersion::Build_350_1_Steam     as i32 => Ok(GameVersion::Build_350_1_Steam),
      x if x == GameVersion::Build_350_2_NotSteam  as i32 => Ok(GameVersion::Build_350_2_NotSteam),
       
      x if x == GameVersion::Build_372_2_Steam     as i32 => Ok(GameVersion::Build_372_2_Steam),
      x if x == GameVersion::Build_372_2_NotSteam  as i32 => Ok(GameVersion::Build_372_2_NotSteam),
       
      x if x == GameVersion::Build_393_2_Steam     as i32 => Ok(GameVersion::Build_393_2_Steam),
      x if x == GameVersion::Build_393_2_NotSteam  as i32 => Ok(GameVersion::Build_393_2_NotSteam),
       
      x if x == GameVersion::Build_393_4_Steam     as i32 => Ok(GameVersion::Build_393_4_Steam),
      x if x == GameVersion::Build_393_4_NotSteam  as i32 => Ok(GameVersion::Build_393_4_NotSteam),
       
      x if x == GameVersion::Build_463_1_Steam     as i32 => Ok(GameVersion::Build_463_1_Steam),
      x if x == GameVersion::Build_463_1_NotSteam  as i32 => Ok(GameVersion::Build_463_1_NotSteam),
       
      x if x == GameVersion::Build_505_2_Steam     as i32 => Ok(GameVersion::Build_505_2_Steam),
      x if x == GameVersion::Build_505_2_NotSteam  as i32 => Ok(GameVersion::Build_505_2_NotSteam),
       
      x if x == GameVersion::Build_573_1_Steam     as i32 => Ok(GameVersion::Build_573_1_Steam),
      x if x == GameVersion::Build_573_1_NotSteam  as i32 => Ok(GameVersion::Build_573_1_NotSteam),
       
      x if x == GameVersion::Build_617_1_Steam     as i32 => Ok(GameVersion::Build_617_1_Steam),
      x if x == GameVersion::Build_617_1_NotSteam  as i32 => Ok(GameVersion::Build_617_1_NotSteam),
       
      x if x == GameVersion::Build_678_1_Steam     as i32 => Ok(GameVersion::Build_678_1_Steam),
      x if x == GameVersion::Build_678_1_NotSteam  as i32 => Ok(GameVersion::Build_678_1_NotSteam),
       
      x if x == GameVersion::Build_757_2_Steam     as i32 => Ok(GameVersion::Build_757_2_Steam),
      x if x == GameVersion::Build_757_2_NotSteam  as i32 => Ok(GameVersion::Build_757_2_NotSteam),
       
      x if x == GameVersion::Build_757_4_Steam     as i32 => Ok(GameVersion::Build_757_4_Steam),
      x if x == GameVersion::Build_757_4_NotSteam  as i32 => Ok(GameVersion::Build_757_4_NotSteam),
       
      x if x == GameVersion::Build_791_2_Steam     as i32 => Ok(GameVersion::Build_791_2_Steam),
      x if x == GameVersion::Build_791_2_NotSteam  as i32 => Ok(GameVersion::Build_791_2_NotSteam),
       
      x if x == GameVersion::Build_877_1_Steam     as i32 => Ok(GameVersion::Build_877_1_Steam),
      x if x == GameVersion::Build_877_1_NotSteam  as i32 => Ok(GameVersion::Build_877_1_NotSteam),
       
      x if x == GameVersion::Build_944_2_Steam     as i32 => Ok(GameVersion::Build_944_2_Steam),
      x if x == GameVersion::Build_944_2_NotSteam  as i32 => Ok(GameVersion::Build_944_2_NotSteam),
      
      x if x == GameVersion::Build_1011_1_Steam    as i32 => Ok(GameVersion::Build_1011_1_Steam),
      x if x == GameVersion::Build_1011_1_NotSteam as i32 => Ok(GameVersion::Build_1011_1_NotSteam),
      
      x if x == GameVersion::Build_1032_1_Steam    as i32 => Ok(GameVersion::Build_1032_1_Steam),
      x if x == GameVersion::Build_1032_1_NotSteam as i32 => Ok(GameVersion::Build_1032_1_NotSteam),
      
      x if x == GameVersion::Build_1103_2_Steam    as i32 => Ok(GameVersion::Build_1103_2_Steam),
      x if x == GameVersion::Build_1103_2_NotSteam as i32 => Ok(GameVersion::Build_1103_2_NotSteam),
      
      x if x == GameVersion::Build_1180_2_Steam    as i32 => Ok(GameVersion::Build_1180_2_Steam),
      x if x == GameVersion::Build_1180_2_NotSteam as i32 => Ok(GameVersion::Build_1180_2_NotSteam),
      
      x if x == GameVersion::Build_1290_1_Steam    as i32 => Ok(GameVersion::Build_1290_1_Steam),
      x if x == GameVersion::Build_1290_1_NotSteam as i32 => Ok(GameVersion::Build_1290_1_NotSteam),
      
      x if x == GameVersion::Build_1365_1_Steam    as i32 => Ok(GameVersion::Build_1365_1_Steam),
      x if x == GameVersion::Build_1365_1_NotSteam as i32 => Ok(GameVersion::Build_1365_1_NotSteam),
      
      x if x == GameVersion::Build_1493_0_Steam    as i32 => Ok(GameVersion::Build_1493_0_Steam),
      x if x == GameVersion::Build_1493_0_NotSteam as i32 => Ok(GameVersion::Build_1493_0_NotSteam),
      
      x if x == GameVersion::Build_1493_1_Steam    as i32 => Ok(GameVersion::Build_1493_1_Steam),
      x if x == GameVersion::Build_1493_1_NotSteam as i32 => Ok(GameVersion::Build_1493_1_NotSteam),
      
      x if x == GameVersion::Build_1604_0_Steam    as i32 => Ok(GameVersion::Build_1604_0_Steam),
      x if x == GameVersion::Build_1604_0_NotSteam as i32 => Ok(GameVersion::Build_1604_0_NotSteam),
      
      x if x == GameVersion::Build_1604_1_Steam    as i32 => Ok(GameVersion::Build_1604_1_Steam),
      x if x == GameVersion::Build_1604_1_NotSteam as i32 => Ok(GameVersion::Build_1604_1_NotSteam),
    
      x if x == GameVersion::Build_1737_0_Steam    as i32 => Ok(GameVersion::Build_1737_0_Steam),
      x if x == GameVersion::Build_1737_0_NotSteam as i32 => Ok(GameVersion::Build_1737_0_NotSteam),
      
      x if x == GameVersion::Build_1737_6_Steam    as i32 => Ok(GameVersion::Build_1737_6_Steam),
      x if x == GameVersion::Build_1737_6_NotSteam as i32 => Ok(GameVersion::Build_1737_6_NotSteam),
      
      x if x == GameVersion::Build_1868_0_Steam    as i32 => Ok(GameVersion::Build_1868_0_Steam),
      x if x == GameVersion::Build_1868_0_NotSteam as i32 => Ok(GameVersion::Build_1868_0_NotSteam),
      
      x if x == GameVersion::Build_1868_1_Steam    as i32 => Ok(GameVersion::Build_1868_1_Steam),
      x if x == GameVersion::Build_1868_1_NotSteam as i32 => Ok(GameVersion::Build_1868_1_NotSteam),
      
      x if x == GameVersion::Build_1868_4_EGS      as i32 => Ok(GameVersion::Build_1868_4_EGS),
      
      x if x == GameVersion::Build_2060_0_Steam    as i32 => Ok(GameVersion::Build_2060_0_Steam),
      x if x == GameVersion::Build_2060_0_NotSteam as i32 => Ok(GameVersion::Build_2060_0_NotSteam),
      
      x if x == GameVersion::Build_2060_1_Steam    as i32 => Ok(GameVersion::Build_2060_1_Steam),
      x if x == GameVersion::Build_2060_1_NotSteam as i32 => Ok(GameVersion::Build_2060_1_NotSteam),
      
      x if x == GameVersion::Build_2189_0_Steam    as i32 => Ok(GameVersion::Build_2189_0_Steam),
      x if x == GameVersion::Build_2189_0_NotSteam as i32 => Ok(GameVersion::Build_2189_0_NotSteam),
      
      x if x == GameVersion::Build_2215_0_Steam    as i32 => Ok(GameVersion::Build_2215_0_Steam),
      x if x == GameVersion::Build_2215_0_NotSteam as i32 => Ok(GameVersion::Build_2215_0_NotSteam),
      
      x if x == GameVersion::Build_2245_0_Steam    as i32 => Ok(GameVersion::Build_2245_0_Steam),
      x if x == GameVersion::Build_2245_0_NotSteam as i32 => Ok(GameVersion::Build_2245_0_NotSteam),
      
      x if x == GameVersion::Build_2372_0_Steam    as i32 => Ok(GameVersion::Build_2372_0_Steam),
      x if x == GameVersion::Build_2372_0_NotSteam as i32 => Ok(GameVersion::Build_2372_0_NotSteam),

      x if x == GameVersion::Build_2545_0_Steam    as i32 => Ok(GameVersion::Build_2545_0_Steam),
      x if x == GameVersion::Build_2545_0_NotSteam as i32 => Ok(GameVersion::Build_2545_0_NotSteam),

      _ => Err(TryFromIntError { value })
    }
  }
}

impl PartialEq for GameVersion {
  fn eq(&self, other: &Self) -> bool {
    core::mem::discriminant(self) == core::mem::discriminant(other)
  }
}

impl PartialOrd for GameVersion {
  #[inline]
  #[must_use]
  fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
    (*self as i32).partial_cmp(&(*other as i32))
  }
}

/// Returns the current game version.
/// 
/// Returns an error if the version returned by ScriptHookV isn't recognized.
/// 
/// ```
/// if get_game_version().unwrap() == GameVersion::Build_335_2_Steam {
///   /* do something specific for Build_335_2_Steam */
/// }
/// ```
#[inline]
pub fn get_game_version() -> Option<GameVersion> {
  unsafe {
    GameVersion::try_from(getGameVersion() as i32)
      .ok()
  }
}
