use super::{Vehicle, VehicleWheelBoneId};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct VehicleWheel {
  vehicle: Vehicle,
  bone_id: VehicleWheelBoneId
}

impl VehicleWheel {
  pub(super) fn new(vehicle: Vehicle, bone_id: VehicleWheelBoneId) -> Self {
    Self {
      vehicle,
      bone_id
    }
  }


}
