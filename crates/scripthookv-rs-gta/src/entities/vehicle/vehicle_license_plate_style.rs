use num_enum::{IntoPrimitive, TryFromPrimitive};

#[derive(Debug, Copy, Clone, PartialEq, Eq, IntoPrimitive, TryFromPrimitive)]
#[repr(i32)]
pub enum VehicleLicensePlateStyle {
  BlueOnWhite2  = 0,
  YellowOnBlack = 1,
  YellowOnBlue  = 2,
  BlueOnWhite1  = 3,
  BlueOnWhite3  = 4,
  NorthYankton  = 5
}
