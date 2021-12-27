use std::ffi::CString;
use std::time::Duration;

use crate::{natives::*, Model};
use scripthookv::types::{Bool, Vector3, Vehicle as NativeVehicle};
use scripthookv::{get_game_version, GameVersion};

use crate::entities::{Entity, VehicleDoors};

use super::{VehicleClass, VehicleModManager};

#[must_use]
#[derive(Debug, Clone, Copy)]
pub struct Vehicle {
  handle: NativeVehicle
}

impl Vehicle {
  #[inline]
  #[must_use]
  pub fn create(model: Model, coords: Vector3, heading: f32) -> Option<Self> {
    unsafe {
      vehicle::create_vehicle(model.hash(), coords, heading, false, false, false)
        .try_into()
        .ok()
    }
  }

  #[inline]
  pub fn set_on_ground_properly(&self) -> bool {
    unsafe { vehicle::set_vehicle_on_ground_properly(self.handle, 5f32) }
  }

  #[inline]
  #[must_use]
  pub fn is_stuck_on_roof(&self) -> bool {
    unsafe { vehicle::is_vehicle_stuck_on_roof(self.handle) }
  }

  #[inline]
  #[must_use]
  pub fn is_stopped(&self) -> bool {
    unsafe { vehicle::is_vehicle_stopped(self.handle) }
  }

  #[inline]
  pub fn explode(&self, audible: bool, invisible: bool) {
    unsafe { vehicle::explode_vehicle(self.handle, audible, invisible) }
  }

  #[inline]
  #[must_use]
  pub fn enveff_scale(&self) -> f32 {
    unsafe { vehicle::get_vehicle_enveff_scale(self.handle) }
  }

  #[inline]
  pub fn set_enveff_scale(&self, fade: f32) {
    unsafe { vehicle::set_vehicle_enveff_scale(self.handle, fade) }
  }

  #[inline]
  #[must_use]
  pub fn is_siren_on(&self) -> bool {
    unsafe { vehicle::is_vehicle_siren_on(self.handle) }
  }

  #[inline]
  pub fn set_siren_on(&self, toggle: bool) {
    unsafe { vehicle::set_vehicle_siren(self.handle, toggle) }
  }

  #[inline]
  #[must_use]
  pub fn is_siren_audio_on(&self) -> bool {
    unsafe { vehicle::is_vehicle_siren_audio_on(self.handle) }
  }

  #[inline]
  pub fn set_strong(&self, toggle: bool) {
    unsafe { vehicle::set_vehicle_strong(self.handle, toggle) }
  }

  #[inline]
  pub fn set_out_of_control(&self, kill_driver: bool, explode_on_impact: bool) {
    unsafe { vehicle::set_vehicle_out_of_control(self.handle, kill_driver, explode_on_impact) }
  }

  #[inline]
  pub fn set_forward_speed(&self, speed: f32) {
    unsafe { vehicle::set_vehicle_forward_speed(self.handle, speed) }
  }

  #[inline]
  pub fn bring_to_halt(&self, distance: f32, duration: Duration) {
    unsafe {
      vehicle::bring_vehicle_to_halt(self.handle, distance, duration.as_millis() as i32, true)
    }
  }

  #[inline]
  pub fn stop_bring_to_halt(&self) {
    unsafe { vehicle::_stop_bring_vehicle_to_halt(self.handle) }
  }

  #[inline]
  #[must_use]
  pub fn is_being_halted(&self) -> bool {
    unsafe { vehicle::_is_vehicle_being_halted(self.handle) }
  }

  #[inline]
  pub fn repair(&self) {
    unsafe { vehicle::set_vehicle_fixed(self.handle) }
  }

  #[inline]
  #[must_use]
  pub fn dirt_level(&self) -> f32 {
    unsafe { vehicle::get_vehicle_dirt_level(self.handle) }
  }

  #[inline]
  pub fn set_dirt_level(&self, dirt_level: f32) {
    unsafe { vehicle::set_vehicle_dirt_level(self.handle, dirt_level) }
  }

  #[inline]
  pub fn wash(&self) {
    self.set_dirt_level(0f32)
  }

  #[inline]
  #[must_use]
  pub fn is_stolen(&self) -> bool {
    unsafe { vehicle::is_vehicle_stolen(self.handle) }
  }

  #[inline]
  pub fn set_is_stolen(&self, stolen: bool) {
    unsafe { vehicle::set_vehicle_is_stolen(self.handle, stolen) }
  }

  #[inline]
  pub fn set_is_wanted(&self, toggle: bool) {
    unsafe { vehicle::set_vehicle_is_wanted(self.handle, toggle) }
  }

  #[inline]
  pub fn set_needs_to_be_hotwired(&self, toggle: bool) {
    unsafe { vehicle::set_vehicle_needs_to_be_hotwired(self.handle, toggle) }
  }

  #[inline]
  #[must_use]
  pub fn can_jump(&self) -> bool {
    match get_game_version() {
      Some(version) if version >= GameVersion::Build_944_2_Steam => unsafe {
        vehicle::_get_can_vehicle_jump(self.handle)
      },
      _ => false
    }
  }

  #[inline]
  #[must_use]
  pub fn vehicle_class(&self) -> Option<VehicleClass> {
    unsafe { vehicle::get_vehicle_class(self.handle).try_into().ok() }
  }

  #[inline]
  #[must_use]
  pub fn body_health(&self) -> f32 {
    unsafe { vehicle::get_vehicle_body_health(self.handle) }
  }

  #[inline]
  pub fn set_body_health(&self, health: f32) {
    unsafe { vehicle::set_vehicle_body_health(self.handle, health) }
  }

  #[inline]
  #[must_use]
  pub fn engine_health(&self) -> f32 {
    unsafe { vehicle::get_vehicle_engine_health(self.handle) }
  }

  #[inline]
  pub fn set_engine_health(&self, health: f32) {
    unsafe { vehicle::set_vehicle_engine_health(self.handle, health) }
  }

  #[inline]
  #[must_use]
  pub fn petrol_tank_health(&self) -> f32 {
    unsafe { vehicle::get_vehicle_petrol_tank_health(self.handle) }
  }

  #[inline]
  pub fn set_petrol_tank_health(&self, health: f32) {
    unsafe { vehicle::set_vehicle_petrol_tank_health(self.handle, health) }
  }

  #[inline]
  pub fn enable_radio(&self, toggle: bool) {
    unsafe { audio::set_vehicle_radio_enabled(self.handle, toggle) }
  }

  #[inline]
  pub fn set_radio_station(&self, radio_station: &str) {
    let cstring = CString::new(radio_station).expect("CString::new failed");
    unsafe { audio::set_veh_radio_station(self.handle, cstring.as_ptr()) }
  }

  #[inline]
  pub fn set_engine_can_degrade(&self, toggle: bool) {
    unsafe { vehicle::set_vehicle_engine_can_degrade(self.handle, toggle) }
  }

  #[inline]
  pub fn engine_power_multiplier(&self, multiplier: f32) {
    unsafe { vehicle::modify_vehicle_top_speed(self.handle, multiplier) }
  }

  #[inline]
  pub fn engine_torque_multiplier(&self, multiplier: f32) {
    unsafe { vehicle::set_vehicle_cheat_power_increase(self.handle, multiplier) }
  }

  #[inline]
  #[must_use]
  pub fn is_alarm_active(&self) -> bool {
    unsafe { vehicle::is_vehicle_alarm_activated(self.handle) }
  }

  #[inline]
  pub fn start_alarm(&self) {
    unsafe { vehicle::start_vehicle_alarm(self.handle) }
  }

  #[inline]
  #[must_use]
  pub fn is_siren_active(&self) -> bool {
    unsafe { vehicle::is_vehicle_siren_on(self.handle) }
  }

  #[inline]
  pub fn set_siren_active(&self, toggle: bool) {
    unsafe { vehicle::set_vehicle_siren(self.handle, toggle) }
  }

  #[inline]
  pub fn sound_horn(&self, duration: Duration) {
    unsafe { vehicle::start_vehicle_horn(self.handle, duration.as_millis() as i32, 0, false) }
  }

  #[inline]
  #[must_use]
  pub fn mod_manager(&self) -> VehicleModManager {
    VehicleModManager::new(*self)
  }

  #[inline]
  #[must_use]
  pub fn doors(&self) -> VehicleDoors {
    VehicleDoors::new(*self)
  }

  #[inline]
  pub fn turn_off_neon_lights(&self, toggle: bool) {
    unsafe { vehicle::_disable_vehicle_neon_lights(self.handle(), toggle) }
  }

  #[inline]
  #[must_use]
  pub fn lights_on(&self) -> bool {
    let mut lights_on: Bool = 0;
    let mut high_beams_on: Bool = 0;
    unsafe {
      vehicle::get_vehicle_lights_state(self.handle, &mut lights_on, &mut high_beams_on);
    }
    lights_on != 0
  }

  #[inline]
  #[must_use]
  pub fn are_high_beams_on(&self) -> bool {
    let mut lights_on: Bool = 0;
    let mut high_beams_on: Bool = 0;
    unsafe {
      vehicle::get_vehicle_lights_state(self.handle, &mut lights_on, &mut high_beams_on);
    }
    high_beams_on != 0
  }

  #[inline]
  pub fn set_high_beams_on(&self, toggle: bool) {
    unsafe { vehicle::set_vehicle_fullbeam(self.handle, toggle) }
  }

  #[inline]
  #[must_use]
  pub fn is_search_light_on(&self) -> bool {
    unsafe { vehicle::is_vehicle_searchlight_on(self.handle) }
  }

  #[inline]
  pub fn set_search_light_on(&self, toggle: bool) {
    unsafe { vehicle::set_vehicle_searchlight(self.handle, toggle, false) }
  }

  #[inline]
  #[must_use]
  pub fn is_taxi_light_on(&self) -> bool {
    unsafe { vehicle::is_taxi_light_on(self.handle) }
  }

  #[inline]
  pub fn set_taxi_light_on(&self, toggle: bool) {
    unsafe { vehicle::set_taxi_lights(self.handle, toggle) }
  }

  #[inline]
  pub fn set_right_indicator_light(&self, toggle: bool) {
    unsafe { vehicle::set_vehicle_indicator_lights(self.handle, 0, toggle) }
  }

  #[inline]
  pub fn set_left_indicator_light(&self, toggle: bool) {
    unsafe { vehicle::set_vehicle_indicator_lights(self.handle, 1, toggle) }
  }

  #[inline]
  pub fn set_brake_lights_on(&self, toggle: bool) {
    unsafe { vehicle::set_vehicle_brake_lights(self.handle, toggle) }
  }
}

impl Entity for Vehicle {
  /// Gets the underlying entity handle.
  #[inline]
  #[must_use]
  fn handle(&self) -> scripthookv::types::Entity {
    self.handle
  }
}

/// The given handle is not a vehicle handle.
#[derive(Debug)]
pub struct NotAVehicleHandleError {
  handle: i32
}

impl std::fmt::Display for NotAVehicleHandleError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "handle {} is not a valid vehicle handle", self.handle)
  }
}

impl TryFrom<i32> for Vehicle {
  type Error = NotAVehicleHandleError;

  fn try_from(handle: i32) -> Result<Self, Self::Error> {
    unsafe {
      if entity::does_entity_exist(handle) && !entity::is_entity_a_vehicle(handle) {
        Ok(Self { handle })
      } else {
        Err(Self::Error { handle })
      }
    }
  }
}

impl Into<i32> for Vehicle {
  #[inline]
  #[must_use]
  fn into(self) -> i32 {
    self.handle()
  }
}
