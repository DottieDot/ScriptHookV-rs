use num_enum::{IntoPrimitive, TryFromPrimitive};
use strum_macros::EnumIter;

#[derive(Debug, Copy, Clone, Eq, PartialEq, TryFromPrimitive, IntoPrimitive, EnumIter)]
#[repr(i32)]
pub enum VehicleWheelBoneId {
  WheelLeftFront    = 11,
  WheelRightFront   = 12,
  WheelLeftRear     = 13,
  WheelRightRear    = 14,
  WheelLeftMiddle1  = 15,
  WheelRightMiddle1 = 16,
  WheelLeftMiddle2  = 17,
  WheelRightMiddle2 = 18,
  WheelLeftMiddle3  = 19,
  WheelRightMiddle3 = 20
}
