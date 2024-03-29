use num_enum::{FromPrimitive, IntoPrimitive};
use strum_macros::EnumIter;

use super::Vehicle;
use crate::natives::*;

#[derive(Debug, Clone, Copy, EnumIter, PartialEq, Eq, FromPrimitive, IntoPrimitive)]
#[repr(i32)]
pub enum VehicleModTypes {
  Spoiler         = 0,
  FrontBumper     = 1,
  RearBumper      = 2,
  SideSkirt       = 3,
  Exhaust         = 4,
  ChassisRollCage = 5,
  Grille          = 6,
  Hood            = 7,
  Fender          = 8,
  RightFender     = 9,
  Roof            = 10,
  Engine          = 11,
  Brakes          = 12,
  Transmission    = 13,
  Horns           = 14,
  Suspension      = 15,
  Armor           = 16,
  TireSmoke       = 20,
  FrontWheels     = 23,
  RearWheels      = 24,
  PlateHolder     = 25,
  VanityPlates    = 26,
  Trim            = 27,
  Ornaments       = 28,
  Dashboard       = 29,
  Dial            = 30,
  DoorSpeaker     = 31,
  Seats           = 32,
  SteeringWheel   = 33,
  ShifterLeavers  = 34,
  Plaques         = 35,
  Speakers        = 36,
  Trunk           = 37,
  Hydraulics      = 38,
  EngineBlock     = 39,
  AirFilter       = 40,
  Struts          = 41,
  ArchCover       = 42,
  Aerials         = 43,
  Trim2           = 44,
  FuelTank        = 45,
  Windows         = 46,
  Livery          = 48,

  #[num_enum(catch_all)]
  Unknown(i32)
}

#[must_use]
#[derive(Debug)]
pub struct VehicleModType {
  vehicle:  Vehicle,
  mod_type: VehicleModTypes
}

impl VehicleModType {
  #[inline]
  pub(super) fn new(vehicle: Vehicle, mod_type: VehicleModTypes) -> Self {
    Self { vehicle, mod_type }
  }

  /// Gets the currently installed mod.
  #[inline]
  #[must_use]
  pub fn get(&self) -> i32 {
    unsafe { vehicle::get_vehicle_mod(self.vehicle.into(), self.mod_type.into()) }
  }

  /// Sets the vehicle mod.
  #[inline]
  pub fn set(&self, mod_id: i32) {
    unsafe {
      vehicle::set_vehicle_mod(
        self.vehicle.into(),
        self.mod_type.into(),
        mod_id,
        self.get_variation() == 1
      )
    }
  }

  /// Removes the currently installed mod.
  #[inline]
  pub fn clear(&self) {
    unsafe { vehicle::remove_vehicle_mod(self.vehicle.into(), self.mod_type.into()) }
  }

  /// Checks if there is a mod currently installed
  #[inline]
  pub fn has_mod(&self) -> bool {
    self.get() != -1
  }

  /// Sets the variation for the mod.
  ///
  /// Only used for wheels, set to `true` for custom tires.
  #[inline]
  pub fn set_variation(&self, variation: bool) {
    unsafe {
      vehicle::set_vehicle_mod(
        self.vehicle.into(),
        self.mod_type.into(),
        self.get(),
        variation
      )
    }
  }

  /// Gets the variation for the mod.
  ///
  /// Only used for wheels and returns `true` for custom tires.
  #[inline]
  #[must_use]
  pub fn get_variation(&self) -> i32 {
    unsafe { vehicle::get_vehicle_mod_variation(self.vehicle.into(), self.mod_type.into()) }
  }
}
