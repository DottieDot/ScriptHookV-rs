use strum::IntoEnumIterator;

use super::{
  Vehicle, VehicleExtra, VehicleModType, VehicleModTypes, VehicleModTypesIter, VehicleToggleMod,
  VehicleToggleMods, VehicleToggleModsIter
};

use crate::natives::*;

pub enum VehicleNeonLights {
  Left  = 0,
  Right = 1,
  Front = 2,
  Back  = 3
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VehicleModManager {
  vehicle: Vehicle
}

impl VehicleModManager {
  #[inline]
  #[must_use]
  pub(super) fn new(vehicle: Vehicle) -> Self {
    Self { vehicle }
  }

  #[inline]
  #[must_use]
  pub fn mod_kit(&self) -> i32 {
    unsafe { vehicle::get_vehicle_mod_kit(self.vehicle.into()) }
  }

  #[inline]
  pub fn set_mod_kit(&self, mod_kit: i32) {
    unsafe { vehicle::set_vehicle_mod_kit(self.vehicle.into(), mod_kit) }
  }

  #[inline]
  #[must_use]
  pub fn num_mod_kits(&self) -> i32 {
    unsafe { vehicle::get_num_mod_kits(self.vehicle.into()) }
  }

  #[inline]
  #[must_use]
  pub fn has_mod_type(&self, mod_type: VehicleModTypes) -> bool {
    unsafe { vehicle::get_num_vehicle_mods(self.vehicle.into(), mod_type as i32) != 0 }
  }

  #[inline]
  #[must_use]
  pub fn get_mod_type(&self, mod_type: VehicleModTypes) -> Option<VehicleModType> {
    self
      .has_mod_type(mod_type)
      .then(|| VehicleModType::new(self.vehicle, mod_type))
  }

  #[inline]
  #[must_use]
  pub fn has_mod_of_type(&self, mod_type: VehicleModTypes) -> bool {
    self
      .get_mod_type(mod_type)
      .and_then(|m| Some(m.has_mod()))
      .unwrap_or(false)
  }

  #[inline]
  pub fn iter_mod_types(&self) -> VehicleModTypeIterator {
    VehicleModTypeIterator::new(*self)
  }

  #[inline]
  #[must_use]
  pub fn has_extra(&self, extra_id: i32) -> bool {
    unsafe { vehicle::does_extra_exist(self.vehicle.into(), extra_id) }
  }

  #[inline]
  pub fn get_extra(&self, extra_id: i32) -> Option<VehicleExtra> {
    self
      .has_extra(extra_id)
      .then(|| VehicleExtra::new(self.vehicle, extra_id))
  }

  #[inline]
  #[must_use]
  pub fn iter_extras(&self) -> VehicleExtraIterator {
    VehicleExtraIterator::new(*self)
  }

  #[inline]
  pub fn get_toggle_mod(&self, mod_id: VehicleToggleMods) -> VehicleToggleMod {
    VehicleToggleMod::new(self.vehicle, mod_id)
  }

  #[inline]
  pub fn iter_toggle_mods(&self) -> VehicleToggleModIterator {
    VehicleToggleModIterator::new(*self)
  }

  #[inline]
  #[must_use]
  pub fn neon_light_enabled(&self, neon: VehicleNeonLights) -> bool {
    unsafe { vehicle::_is_vehicle_neon_light_enabled(self.vehicle.into(), neon as i32) }
  }

  #[inline]
  pub fn enable_neon_light(&self, neon: VehicleNeonLights, toggle: bool) {
    unsafe { vehicle::_set_vehicle_neon_light_enabled(self.vehicle.into(), neon as i32, toggle) }
  }
}

pub struct VehicleModTypeIterator {
  mod_manager: VehicleModManager,
  current:     VehicleModTypesIter
}

impl VehicleModTypeIterator {
  #[inline]
  #[must_use]
  pub fn new(mod_manager: VehicleModManager) -> Self {
    Self {
      mod_manager,
      current: VehicleModTypes::iter()
    }
  }
}

impl Iterator for VehicleModTypeIterator {
  type Item = VehicleModType;

  #[must_use]
  fn next(&mut self) -> Option<Self::Item> {
    self
      .current
      .find(|&value| self.mod_manager.has_mod_type(value))
      .and_then(|value| self.mod_manager.get_mod_type(value))
  }
}

pub struct VehicleExtraIterator {
  mod_manager: VehicleModManager,
  current:     i32
}

impl VehicleExtraIterator {
  #[inline]
  #[must_use]
  pub fn new(mod_manager: VehicleModManager) -> Self {
    Self {
      mod_manager,
      current: 0
    }
  }
}

impl Iterator for VehicleExtraIterator {
  type Item = VehicleExtra;

  #[must_use]
  fn next(&mut self) -> Option<Self::Item> {
    let extra = self.mod_manager.get_extra(self.current);
    if extra.is_some() {
      self.current += 1;
    }
    extra
  }
}

pub struct VehicleToggleModIterator {
  mod_manager: VehicleModManager,
  current:     VehicleToggleModsIter
}

impl VehicleToggleModIterator {
  #[inline]
  #[must_use]
  pub fn new(mod_manager: VehicleModManager) -> Self {
    Self {
      mod_manager,
      current: VehicleToggleMods::iter()
    }
  }
}

impl Iterator for VehicleToggleModIterator {
  type Item = VehicleToggleMod;

  #[inline]
  #[must_use]
  fn next(&mut self) -> Option<Self::Item> {
    self
      .current
      .next()
      .and_then(|m| Some(self.mod_manager.get_toggle_mod(m)))
  }
}
