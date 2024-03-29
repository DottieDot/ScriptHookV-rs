use std::ffi::c_void;

pub mod call_native;
pub mod memory;
pub mod natives;
pub mod types;

mod builder_plugin;
mod callbacks;
mod game_version;
mod memory_database;
mod script;
mod scripthookv;
mod scripthookv_builder;
pub mod scripting;
mod scripting_backend;
mod sig_info;
mod texture;
mod timeout;
mod winapi;
mod world;

pub use builder_plugin::*;
pub(crate) use callbacks::*;
pub use game_version::*;
pub use memory_database::*;
pub(crate) use script::*;
pub use scripthookv::*;
pub use scripthookv_builder::*;
pub use scripting_backend::*;
pub use texture::*;
pub use timeout::*;
pub use world::*;

pub use scripthookv_rs_macros::*;

pub type ModuleHandle = *const c_void;

/// Gets a script global and returns it as a mutable reference.
///
/// ```
/// /* sets script global 1337 to 10 */
/// *get_global::<int>(1337) = 10
/// ```
///
/// # Safety
/// This function will not do any checks in regard to the type and validity of the global. It will interpret whatever is in memory as `T`.
#[inline]
pub unsafe fn get_global<T>(global_id: i32) -> &'static mut T {
  let ptr = scripting_backend::BACKEND
    .get()
    .expect("runtime not set")
    .get_global(global_id);

  &mut *(ptr as *mut T)
}
