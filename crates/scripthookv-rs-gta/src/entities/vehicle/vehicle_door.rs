use crate::{
  entities::{Vehicle, VehicleDoorIndex},
  natives::*
};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct VehicleDoor {
  vehicle: Vehicle,
  door:    VehicleDoorIndex
}

impl VehicleDoor {
  #[inline]
  #[must_use]
  pub fn new(vehicle: Vehicle, door: VehicleDoorIndex) -> Self {
    Self { vehicle, door }
  }

  #[inline]
  #[must_use]
  pub fn angle_ratio(&self) -> f32 {
    unsafe { vehicle::get_vehicle_door_angle_ratio(self.vehicle.into(), self.door.into()) }
  }

  #[inline]
  #[must_use]
  pub fn is_open(&self) -> bool {
    self.angle_ratio() >= 0.05f32
  }

  #[inline]
  #[must_use]
  pub fn is_fully_open(&self) -> bool {
    unsafe { vehicle::is_vehicle_door_fully_open(self.vehicle.into(), self.door.into()) }
  }

  #[inline]
  pub fn open(&self, loose: bool, instantly: bool) {
    unsafe {
      vehicle::set_vehicle_door_open(self.vehicle.into(), self.door.into(), loose, instantly)
    }
  }

  #[inline]
  pub fn close(&self, instantly: bool) {
    unsafe { vehicle::set_vehicle_door_shut(self.vehicle.into(), self.door.into(), instantly) }
  }

  #[inline]
  #[must_use]
  pub fn is_broken(&self) -> bool {
    unsafe { vehicle::is_vehicle_door_damaged(self.vehicle.into(), self.door.into()) }
  }

  #[inline]
  pub fn set_can_break(&self, toggle: bool) {
    unsafe { vehicle::_set_vehicle_door_can_break(self.vehicle.into(), self.door.into(), toggle) }
  }

  #[inline]
  pub fn destroy(&self, delete_door: bool) {
    unsafe { vehicle::set_vehicle_door_broken(self.vehicle.into(), self.door.into(), delete_door) }
  }

  #[inline]
  pub fn set_angle_ratio(&self, ratio: f32) {
    unsafe {
      vehicle::set_vehicle_door_control(
        self.vehicle.into(),
        self.door.into(),
        1,
        ratio
      )
    }
  }
}
