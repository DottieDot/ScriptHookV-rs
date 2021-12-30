use num_enum::{IntoPrimitive, TryFromPrimitive};
use strum_macros::EnumIter;

#[derive(Debug, Clone, Copy, EnumIter, TryFromPrimitive, IntoPrimitive, PartialEq, Eq)]
#[repr(i32)]
pub enum VehicleDoorIndex {
  FrontLeftDoor  = 0,
  FrontRightDoor = 1,
  BackLeftDoor   = 2,
  BackRightDoor  = 3,
  Hood           = 4,
  Trunk          = 5
}
