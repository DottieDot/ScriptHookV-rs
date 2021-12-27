use crate::entities::{VehicleDoor, VehicleDoorIndex, VehicleDoorIndexIter};
use crate::{entities::Vehicle, natives::*};

use strum::IntoEnumIterator;

#[derive(Debug, Copy, Clone)]
pub struct VehicleDoors {
  vehicle: Vehicle
}

impl VehicleDoors {
  #[inline]
  #[must_use]
  pub fn new(vehicle: Vehicle) -> Self {
    Self { vehicle }
  }

  #[inline]
  #[must_use]
  pub fn has_door(&self, _door: VehicleDoorIndex) -> bool {
    true // TODO: Implement this
  }

  #[inline]
  #[must_use]
  pub fn get_door(&self, door: VehicleDoorIndex) -> Option<VehicleDoor> {
    self
      .has_door(door)
      .then(|| VehicleDoor::new(self.vehicle, door))
  }

  #[inline]
  pub fn close_all(&self, instantly: bool) {
    unsafe { vehicle::set_vehicle_doors_shut(self.vehicle.into(), instantly) }
  }

  #[inline]
  #[must_use]
  pub fn iter(&self) -> VehicleDoorIterator {
    VehicleDoorIterator::new(*self)
  }
}

pub struct VehicleDoorIterator {
  doors:   VehicleDoors,
  current: VehicleDoorIndexIter
}

impl VehicleDoorIterator {
  #[inline]
  #[must_use]
  pub fn new(doors: VehicleDoors) -> Self {
    Self {
      doors,
      current: VehicleDoorIndex::iter()
    }
  }
}

impl Iterator for VehicleDoorIterator {
  type Item = VehicleDoor;

  #[inline]
  #[must_use]
  fn next(&mut self) -> Option<Self::Item> {
    self
      .current
      .find(|door| self.doors.has_door(*door))
      .and_then(|door| self.doors.get_door(door))
  }
}
