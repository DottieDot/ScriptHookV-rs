use num_enum::{FromPrimitive, IntoPrimitive};

#[derive(Debug, Copy, Clone, PartialEq, Eq, IntoPrimitive, FromPrimitive)]
#[repr(i32)]
pub enum VehicleLicensePlateStyle {
  BlueOnWhite2  = 0,
  YellowOnBlack = 1,
  YellowOnBlue  = 2,
  BlueOnWhite1  = 3,
  BlueOnWhite3  = 4,
  NorthYankton  = 5,

  #[num_enum(catch_all)]
  Unknown(i32)
}
