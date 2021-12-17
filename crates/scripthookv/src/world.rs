use std::vec::Vec;

use shv_bindings::{worldGetAllObjects, worldGetAllPeds, worldGetAllPickups, worldGetAllVehicles};

use crate::types::{Vehicle, Pickup, Ped, Object};

fn get_all_wrapper(function: unsafe extern "C" fn(arr: *mut i32, arrSize: i32) -> i32) -> Vec<i32> {
  unsafe {
    let mut buffer = [0; 1024 /* SHV has no way to query the required size */];
    let size = function(buffer.as_mut_ptr(), 1024) as usize;

    buffer.as_slice()[0..size].to_vec()
  }
}

pub fn get_all_world_objects() -> Vec<Object> {
  get_all_wrapper(worldGetAllObjects)
}

pub fn get_all_world_peds() -> Vec<Ped> {
  get_all_wrapper(worldGetAllPeds)
}

pub fn get_all_world_pickups() -> Vec<Pickup> {
  get_all_wrapper(worldGetAllPickups)
}

pub fn get_all_world_vehicles() -> Vec<Vehicle> {
  get_all_wrapper(worldGetAllVehicles)
}

