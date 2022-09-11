use std::ffi::CString;

use scripthookv::ScriptingBackend;
use shv_bindings::{
  createTexture, drawTexture, getGameVersion, getGlobalPtr, getScriptHandleBaseAddress,
  keyboardHandlerRegister, keyboardHandlerUnregister, nativeCall, nativeInit, nativePush64,
  presentCallbackRegister, presentCallbackUnregister, scriptRegister, scriptUnregister, scriptWait,
  worldGetAllObjects, worldGetAllPeds, worldGetAllPickups, worldGetAllVehicles
};

pub struct ScriptHookVBackend;

impl ScriptingBackend for ScriptHookVBackend {
  unsafe fn create_texture(&self, path: &std::path::Path) -> i32 {
    let cstring = CString::new(
      path
        .as_os_str()
        .to_str()
        .expect("Path includes invalid unicode")
    )
    .expect("CString::new failed");

    createTexture(cstring.as_ptr())
  }

  unsafe fn draw_texture(
    &self,
    id: i32,
    index: i32,
    level: i32,
    time: i32,
    size_x: f32,
    size_y: f32,
    center_x: f32,
    center_y: f32,
    pos_x: f32,
    pos_y: f32,
    rotation: f32,
    screen_height_scale_factor: f32,
    r: f32,
    g: f32,
    b: f32,
    a: f32
  ) {
    drawTexture(
      id,
      index,
      level,
      time,
      size_x,
      size_y,
      center_x,
      center_y,
      pos_x,
      pos_y,
      rotation,
      screen_height_scale_factor,
      r,
      g,
      b,
      a
    )
  }

  unsafe fn present_callback_register(&self, cb: scripthookv::PresentCallback) {
    presentCallbackRegister(cb)
  }

  unsafe fn present_callback_unregister(&self, cb: scripthookv::PresentCallback) {
    presentCallbackUnregister(cb)
  }

  unsafe fn keyboard_handler_register(&self, cb: scripthookv::KeyboardHandler) {
    keyboardHandlerRegister(cb)
  }

  unsafe fn keyboard_handler_unregister(&self, cb: scripthookv::KeyboardHandler) {
    keyboardHandlerUnregister(cb)
  }

  unsafe fn script_wait(&self, time: u32) {
    scriptWait(time)
  }

  unsafe fn script_register(&self, module: *const std::ffi::c_void, script_main: extern "C" fn()) {
    scriptRegister(module, script_main)
  }

  unsafe fn script_unregister(&self, module: *const std::ffi::c_void) {
    scriptUnregister(module)
  }

  unsafe fn native_init(&self, hash: u64) {
    nativeInit(hash)
  }

  unsafe fn native_push(&self, val: u64) {
    nativePush64(val)
  }

  unsafe fn native_call(&self) -> *mut u64 {
    nativeCall()
  }

  unsafe fn get_global(&self, global_id: i32) -> *mut u64 {
    getGlobalPtr(global_id)
  }

  unsafe fn get_all_vehicles(&self, arr: *mut i32, arr_size: i32) -> i32 {
    worldGetAllVehicles(arr, arr_size)
  }

  unsafe fn get_all_objects(&self, arr: *mut i32, arr_size: i32) -> i32 {
    worldGetAllObjects(arr, arr_size)
  }

  unsafe fn get_all_peds(&self, arr: *mut i32, arr_size: i32) -> i32 {
    worldGetAllPeds(arr, arr_size)
  }

  unsafe fn get_all_pickups(&self, arr: *mut i32, arr_size: i32) -> i32 {
    worldGetAllPickups(arr, arr_size)
  }

  unsafe fn get_script_handle_base_address(&self, handle: i32) -> *mut std::ffi::c_void {
    getScriptHandleBaseAddress(handle) as *mut std::ffi::c_void
  }

  unsafe fn get_game_version(&self) -> i32 {
    getGameVersion() as i32
  }
}
