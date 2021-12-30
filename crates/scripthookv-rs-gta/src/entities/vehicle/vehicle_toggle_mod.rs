use num_enum::{IntoPrimitive, TryFromPrimitive};
use strum_macros::EnumIter;

use super::Vehicle;

use crate::natives::*;

#[derive(Debug, Clone, Copy, EnumIter, PartialEq, Eq, TryFromPrimitive, IntoPrimitive)]
#[repr(i32)]
pub enum VehicleToggleMods {
  Turbo = 18,
  Xenon = 22
}

pub struct VehicleToggleMod {
  vehicle: Vehicle,
  mod_id:  VehicleToggleMods
}

impl VehicleToggleMod {
  #[inline]
  #[must_use]
  pub(super) fn new(vehicle: Vehicle, mod_id: VehicleToggleMods) -> Self {
    Self { vehicle, mod_id }
  }

  #[inline]
  #[must_use]
  pub fn enabled(&self) -> bool {
    unsafe { vehicle::is_toggle_mod_on(self.vehicle.into(), self.mod_id as i32) }
  }

  #[inline]
  pub fn enable(&self, toggle: bool) {
    unsafe { vehicle::toggle_vehicle_mod(self.vehicle.into(), self.mod_id as i32, toggle) }
  }
}
