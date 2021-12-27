use num_enum::{TryFromPrimitive, IntoPrimitive};
use strum_macros::EnumIter;

#[derive(Debug, Clone, Copy, EnumIter, TryFromPrimitive, IntoPrimitive)]
#[repr(i32)]
pub enum VehicleDoorIndex {
  FrontLeftDoor  = 0,
  FrontRightDoor = 1,
  BackLeftDoor   = 2,
  BackRightDoor  = 3,
  Hood           = 4,
  Trunk          = 5
}
