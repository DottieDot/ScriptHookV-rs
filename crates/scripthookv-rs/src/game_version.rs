use num_enum::FromPrimitive;

use crate::scripting_backend::BACKEND;

// Taken from https://github.com/E66666666/GTAVManualTransmission/blob/1e3e73070ad293536ea8a2d0c8fea58477830b9d/Gears/Memory/Versions.h

/// Used for identifying game versions.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, FromPrimitive)]
#[repr(i32)]
pub enum GameVersion {
  Build_335_2_Steam     = 0,
  Build_335_2_NotSteam  = 1,

  Build_350_1_Steam     = 2,
  Build_350_2_NotSteam  = 3,

  Build_372_2_Steam     = 4,
  Build_372_2_NotSteam  = 5,

  Build_393_2_Steam     = 6,
  Build_393_2_NotSteam  = 7,

  Build_393_4_Steam     = 8,
  Build_393_4_NotSteam  = 9,

  Build_463_1_Steam     = 10,
  Build_463_1_NotSteam  = 11,

  Build_505_2_Steam     = 12,
  Build_505_2_NotSteam  = 13,

  Build_573_1_Steam     = 14,
  Build_573_1_NotSteam  = 15,

  Build_617_1_Steam     = 16,
  Build_617_1_NotSteam  = 17,

  Build_678_1_Steam     = 18,
  Build_678_1_NotSteam  = 19,

  Build_757_2_Steam     = 20,
  Build_757_2_NotSteam  = 21,

  Build_757_4_Steam     = 22,
  Build_757_4_NotSteam  = 23,

  Build_791_2_Steam     = 24,
  Build_791_2_NotSteam  = 25,

  Build_877_1_Steam     = 26,
  Build_877_1_NotSteam  = 27,

  Build_944_2_Steam     = 28,
  Build_944_2_NotSteam  = 29,

  Build_1011_1_Steam    = 30,
  Build_1011_1_NotSteam = 31,

  Build_1032_1_Steam    = 32,
  Build_1032_1_NotSteam = 33,

  Build_1103_2_Steam    = 34,
  Build_1103_2_NotSteam = 35,

  Build_1180_2_Steam    = 36,
  Build_1180_2_NotSteam = 37,

  Build_1290_1_Steam    = 38,
  Build_1290_1_NotSteam = 39,

  Build_1365_1_Steam    = 40,
  Build_1365_1_NotSteam = 41,

  Build_1493_0_Steam    = 42,
  Build_1493_0_NotSteam = 43,

  Build_1493_1_Steam    = 44,
  Build_1493_1_NotSteam = 45,

  Build_1604_0_Steam    = 46,
  Build_1604_0_NotSteam = 47,

  Build_1604_1_Steam    = 48,
  Build_1604_1_NotSteam = 49,

  Build_1737_0_Steam    = 50,
  Build_1737_0_NotSteam = 51,

  Build_1737_6_Steam    = 52,
  Build_1737_6_NotSteam = 53,

  Build_1868_0_Steam    = 54,
  Build_1868_0_NotSteam = 55,

  Build_1868_1_Steam    = 56,
  Build_1868_1_NotSteam = 57,

  Build_1868_4_EGS      = 58,

  Build_2060_0_Steam    = 59,
  Build_2060_0_NotSteam = 60,

  Build_2060_1_Steam    = 61,
  Build_2060_1_NotSteam = 62,

  Build_2189_0_Steam    = 63,
  Build_2189_0_NotSteam = 64,

  Build_2215_0_Steam    = 65,
  Build_2215_0_NotSteam = 66,

  Build_2245_0_Steam    = 67,
  Build_2245_0_NotSteam = 68,

  Build_2372_0_Steam    = 69,
  Build_2372_0_NotSteam = 70,

  Build_2545_0_Steam    = 71,
  Build_2545_0_NotSteam = 72,

  #[num_enum(catch_all)]
  Unknown(i32)
}

impl GameVersion {
  /// Checks if the game version isn't unknown
  #[inline]
  #[must_use]
  pub fn known(&self) -> bool {
    !matches!(self, Self::Unknown(_))
  }
}

/// Returns the current game version.
///
/// ```
/// if get_game_version() == GameVersion::Build_335_2_Steam {
///   /* do something specific for Build_335_2_Steam */
/// }
/// ```
#[inline]
#[must_use]
pub fn get_game_version() -> GameVersion {
  unsafe { GameVersion::from(BACKEND.get().expect("runtime not set").get_game_version()) }
}
