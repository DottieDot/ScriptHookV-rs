use super::Vehicle;

use crate::{
  color::{Color, RGB},
  natives::*
};

#[derive(Debug, Copy, Clone)]
pub struct VehicleColorManager {
  vehicle: Vehicle
}

impl VehicleColorManager {
  #[inline]
  #[must_use]
  pub fn new(vehicle: Vehicle) -> Self {
    Self { vehicle }
  }

  #[inline]
  #[must_use]
  pub fn primary_color(&self) -> i32 {
    let mut primary_color = 0;
    let mut secondary_color = 0;
    unsafe {
      vehicle::get_vehicle_colours(
        self.vehicle.into(),
        &mut primary_color,
        &mut secondary_color
      );
    }
    primary_color
  }

  #[inline]
  #[must_use]
  pub fn secondary_color(&self) -> i32 {
    let mut primary_color = 0;
    let mut secondary_color = 0;
    unsafe {
      vehicle::get_vehicle_colours(
        self.vehicle.into(),
        &mut primary_color,
        &mut secondary_color
      );
    }
    secondary_color
  }

  #[inline]
  pub fn set_primary_color(&self, color: i32) {
    unsafe { vehicle::set_vehicle_colours(self.vehicle.into(), color, self.secondary_color()) }
  }

  #[inline]
  pub fn set_secondary_color(&self, color: i32) {
    unsafe { vehicle::set_vehicle_colours(self.vehicle.into(), self.secondary_color(), color) }
  }

  #[inline]
  #[must_use]
  pub fn number_of_color_combinations(&self) -> i32 {
    unsafe { vehicle::get_number_of_vehicle_colours(self.vehicle.into()) }
  }

  #[inline]
  #[must_use]
  pub fn color_combination(&self) -> i32 {
    unsafe { vehicle::get_vehicle_colour_combination(self.vehicle.into()) }
  }

  #[inline]
  pub fn set_color_combination(&self, color_combination: i32) {
    unsafe { vehicle::set_vehicle_colour_combination(self.vehicle.into(), color_combination) }
  }

  #[inline]
  #[must_use]
  pub fn neon_lights_color(&self) -> Color {
    let mut rgb_buffer = (0, 0, 0);
    unsafe {
      vehicle::_get_vehicle_neon_lights_colour(
        self.vehicle.into(),
        &mut rgb_buffer.0,
        &mut rgb_buffer.1,
        &mut rgb_buffer.2
      );
    }
    RGB::new(rgb_buffer.0 as u8, rgb_buffer.1 as u8, rgb_buffer.2 as u8).into()
  }

  #[inline]
  pub fn set_neon_lights_color(&self, color: Color) {
    let rgb: RGB = color.into();
    unsafe {
      vehicle::_set_vehicle_neon_lights_colour(
        self.vehicle.into(),
        rgb.r.into(),
        rgb.g.into(),
        rgb.b.into()
      )
    }
  }

  #[inline]
  #[must_use]
  pub fn xenon_lights_color(&self) -> i32 {
    unsafe { vehicle::_get_vehicle_xenon_lights_color(self.vehicle.into()) }
  }

  #[inline]
  pub fn set_xenon_lights_color(&self, color: i32) {
    unsafe { vehicle::_set_vehicle_xenon_lights_color(self.vehicle.into(), color) }
  }

  #[inline]
  #[must_use]
  pub fn pearlescent_color(&self) -> i32 {
    let mut pearlescent = 0;
    let mut wheel = 0;
    unsafe { vehicle::get_vehicle_extra_colours(self.vehicle.into(), &mut pearlescent, &mut wheel) }
    pearlescent
  }

  #[inline]
  #[must_use]
  pub fn wheel_color(&self) -> i32 {
    let mut pearlescent = 0;
    let mut wheel = 0;
    unsafe { vehicle::get_vehicle_extra_colours(self.vehicle.into(), &mut pearlescent, &mut wheel) }
    wheel
  }

  #[inline]
  pub fn set_pearlescent_color(&self, color: i32) {
    unsafe { vehicle::set_vehicle_extra_colours(self.vehicle.into(), color, self.wheel_color()) }
  }

  #[inline]
  pub fn set_wheel_color(&self, color: i32) {
    unsafe {
      vehicle::set_vehicle_extra_colours(self.vehicle.into(), self.pearlescent_color(), color)
    }
  }

  #[inline]
  #[must_use]
  pub fn interior_color(&self) -> i32 {
    let mut color = 0;
    unsafe { vehicle::_get_vehicle_interior_color(self.vehicle.into(), &mut color) }
    color
  }

  #[inline]
  pub fn set_interior_color(&self, color: i32) {
    unsafe { vehicle::_set_vehicle_interior_color(self.vehicle.into(), color) }
  }

  #[inline]
  #[must_use]
  pub fn dashboard_color(&self) -> i32 {
    let mut color = 0;
    unsafe { vehicle::_get_vehicle_dashboard_color(self.vehicle.into(), &mut color) }
    color
  }

  #[inline]
  pub fn set_dashboard_color(&self, color: i32) {
    unsafe { vehicle::_set_vehicle_dashboard_color(self.vehicle.into(), color) }
  }

  #[inline]
  #[must_use]
  pub fn custom_primary_color(&self) -> Color {
    let mut rgb = (0, 0, 0);
    unsafe {
      vehicle::get_vehicle_custom_primary_colour(
        self.vehicle.into(),
        &mut rgb.0,
        &mut rgb.1,
        &mut rgb.2
      );
    }
    RGB::new(rgb.0 as u8, rgb.0 as u8, rgb.0 as u8).into()
  }

  #[inline]
  pub fn clear_custom_primary_color(&self) {
    unsafe { vehicle::clear_vehicle_custom_primary_colour(self.vehicle.into()) }
  }

  #[inline]
  pub fn set_custom_primary_color(&self, color: Color) {
    let rgb: RGB = color.into();
    self.clear_custom_primary_color();
    unsafe {
      vehicle::set_vehicle_custom_primary_colour(
        self.vehicle.into(),
        rgb.r.into(),
        rgb.g.into(),
        rgb.b.into()
      )
    }
  }

  #[inline]
  #[must_use]
  pub fn custom_secondary_color(&self) -> Color {
    let mut rgb = (0, 0, 0);
    unsafe {
      vehicle::get_vehicle_custom_secondary_colour(
        self.vehicle.into(),
        &mut rgb.0,
        &mut rgb.1,
        &mut rgb.2
      );
    }
    RGB::new(rgb.0 as u8, rgb.0 as u8, rgb.0 as u8).into()
  }

  #[inline]
  pub fn clear_custom_secondary_color(&self) {
    unsafe { vehicle::clear_vehicle_custom_secondary_colour(self.vehicle.into()) }
  }

  #[inline]
  pub fn set_custom_secondary_color(&self, color: Color) {
    let rgb: RGB = color.into();
    self.clear_custom_secondary_color();
    unsafe {
      vehicle::set_vehicle_custom_secondary_colour(
        self.vehicle.into(),
        rgb.r.into(),
        rgb.g.into(),
        rgb.b.into()
      )
    }
  }

  #[inline]
  #[must_use]
  pub fn has_custom_primary_color(&self) -> bool {
    unsafe { vehicle::get_is_vehicle_primary_colour_custom(self.vehicle.into()) }
  }

  #[inline]
  #[must_use]
  pub fn has_custom_secondary_color(&self) -> bool {
    unsafe { vehicle::get_is_vehicle_secondary_colour_custom(self.vehicle.into()) }
  }
}
