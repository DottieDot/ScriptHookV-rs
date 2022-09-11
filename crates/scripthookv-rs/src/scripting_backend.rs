use std::{ffi::c_void, path::Path};

use once_cell::sync::OnceCell;

pub type PresentCallback = unsafe extern "C" fn(arg1: *mut c_void);

pub type KeyboardHandler =
  unsafe extern "C" fn(arg1: u32, arg2: u16, arg3: u8, arg4: i8, arg5: i8, arg6: i8, arg7: i8);

pub trait ScriptingBackend: Send + Sync {
  unsafe fn create_texture(&self, path: &Path) -> i32;

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
  );

  unsafe fn present_callback_register(&self, cb: PresentCallback);

  unsafe fn present_callback_unregister(&self, cb: PresentCallback);

  unsafe fn keyboard_handler_register(&self, cb: KeyboardHandler);

  unsafe fn keyboard_handler_unregister(&self, cb: KeyboardHandler);

  unsafe fn script_wait(&self, time: u32);

  unsafe fn script_register(&self, module: *const c_void, script_main: extern "C" fn());

  unsafe fn script_unregister(&self, module: *const c_void);

  unsafe fn native_init(&self, hash: u64);

  unsafe fn native_push(&self, val: u64);

  unsafe fn native_call(&self) -> *mut u64;

  unsafe fn get_global(&self, global_id: i32) -> *mut u64;

  unsafe fn get_all_vehicles(&self, arr: *mut i32, arr_size: i32) -> i32;

  unsafe fn get_all_objects(&self, arr: *mut i32, arr_size: i32) -> i32;

  unsafe fn get_all_peds(&self, arr: *mut i32, arr_size: i32) -> i32;

  unsafe fn get_all_pickups(&self, arr: *mut i32, arr_size: i32) -> i32;

  unsafe fn get_script_handle_base_address(&self, handle: i32) -> *mut c_void;

  unsafe fn get_game_version(&self) -> i32;
}

pub static BACKEND: OnceCell<Box<dyn ScriptingBackend>> = OnceCell::new();
