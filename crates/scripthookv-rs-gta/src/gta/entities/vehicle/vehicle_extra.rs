use super::Vehicle;

use crate::natives::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VehicleExtra {
  vehicle:  Vehicle,
  extra_id: i32
}

impl VehicleExtra {
  #[inline]
  #[must_use]
  pub(super) fn new(vehicle: Vehicle, extra_id: i32) -> Self {
    Self { vehicle, extra_id }
  }

  /// Checks if the extra is enabled.
  #[inline]
  #[must_use]
  pub fn enabled(&self) -> bool {
    unsafe { vehicle::is_vehicle_extra_turned_on(self.vehicle.into(), self.extra_id) }
  }

  /// Enables the extra.
  #[inline]
  pub fn enable(&self, toggle: bool) {
    unsafe { vehicle::set_vehicle_extra(self.vehicle.into(), self.extra_id, !toggle) }
  }
}
