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

/// Gets all objects present in the world.
/// 
/// Gets all object handles stored in `atDblablaNode` using ScriptHookV's worldGetAllObjects. The result is limited to a maximum of 1024 objects.
/// 
/// ```
/// for object in get_all_world_objects().iter() {
///   /* do something with the object handle */
/// }
/// ```
pub fn get_all_world_objects() -> Vec<Object> {
  get_all_wrapper(worldGetAllObjects)
}

/// Gets all peds present in the world.
/// 
/// Gets all ped handles stored in `atDblablaNode` using ScriptHookV's worldGetAllPeds. The result is limited to a maximum of 1024 peds.
/// 
/// ```
/// for ped in get_all_world_peds().iter() {
///   /* do something with the ped handle */
/// }
/// ```
pub fn get_all_world_peds() -> Vec<Ped> {
  get_all_wrapper(worldGetAllPeds)
}

/// Gets all pickups present in the world.
/// 
/// Gets all pickup handles in `atDblablaNode` using ScriptHookV's worldGetAllPickups. The result is limited to a maximum of 1024 pickups.
/// 
/// ```
/// for pickup in get_all_world_pickups().iter() {
///   /* do something with the pickup handle */
/// }
/// ```
pub fn get_all_world_pickups() -> Vec<Pickup> {
  get_all_wrapper(worldGetAllPickups)
}

/// Gets all vehicles present in the world.
/// 
/// Gets all vehicle handles in `atDblablaNode` using ScriptHookV's worldGetAllVehicles. The result is limited to a maximum of 1024 vehicles.
/// 
/// ```
/// for vehicle in get_all_world_vehicles().iter() {
///   /* do something with the vehicle handle */
/// }
/// ```
pub fn get_all_world_vehicles() -> Vec<Vehicle> {
  get_all_wrapper(worldGetAllVehicles)
}

