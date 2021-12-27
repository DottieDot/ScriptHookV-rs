use std::vec::Vec;

use shv_bindings::{worldGetAllObjects, worldGetAllPeds, worldGetAllPickups, worldGetAllVehicles};

use crate::types::{Object, Ped, Pickup, Vehicle};

fn get_all_wrapper(function: unsafe extern "C" fn(arr: *mut i32, arrSize: i32) -> i32) -> Vec<i32> {
  unsafe {
    // ScriptHookV doesn't allow for querying the required buffer size.
    // In the official examples a buffer of 1024 but ScriptHooKV extends multiple pools to a size of 3072.
    // 4096 is chosen because it's a neat number and leaves some headroom.
    let mut buffer = [0; 4096];
    let size = function(buffer.as_mut_ptr(), 4096) as usize;

    buffer.as_slice()[0..size].to_vec()
  }
}

/// Gets all objects present in the world.
///
/// Gets all object handles stored in `atDScriptObjectNode` using ScriptHookV's worldGetAllObjects. The result is limited to a maximum of 4096 objects.
///
/// ```
/// for object in get_all_world_objects() {
///   /* do something with the object handle */
/// }
/// ```
pub fn get_all_world_objects() -> Vec<Object> {
  get_all_wrapper(worldGetAllObjects)
}

/// Gets all peds present in the world.
///
/// Gets all ped handles stored in `Peds` using ScriptHookV's worldGetAllPeds. The result is limited to a maximum of 4096 peds.
///
/// ```
/// for ped in get_all_world_peds() {
///   /* do something with the ped handle */
/// }
/// ```
pub fn get_all_world_peds() -> Vec<Ped> {
  get_all_wrapper(worldGetAllPeds)
}

/// Gets all pickups present in the world.
///
/// Gets all pickup handles in `CPickup` using ScriptHookV's worldGetAllPickups. The result is limited to a maximum of 4096 pickups.
///
/// ```
/// for pickup in get_all_world_pickups() {
///   /* do something with the pickup handle */
/// }
/// ```
pub fn get_all_world_pickups() -> Vec<Pickup> {
  get_all_wrapper(worldGetAllPickups)
}

/// Gets all vehicles present in the world.
///
/// Gets all vehicle handles in `Vehicles` using ScriptHookV's worldGetAllVehicles. The result is limited to a maximum of 4096 vehicles.
///
/// ```
/// for vehicle in get_all_world_vehicles() {
///   /* do something with the vehicle handle */
/// }
/// ```
pub fn get_all_world_vehicles() -> Vec<Vehicle> {
  get_all_wrapper(worldGetAllVehicles)
}
