use super::{Vehicle, VehicleWheel, VehicleWheelBoneId};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct VehicleWheels {
  vehicle: Vehicle
}

impl VehicleWheels {
  #[inline]
  #[must_use]
  pub(super) fn new(vehicle: Vehicle) -> Self {
    Self { vehicle }
  }

  pub fn get_wheel(&self, bone_id: VehicleWheelBoneId) -> VehicleWheel {
    VehicleWheel::new(self.vehicle, bone_id)
  }
}
