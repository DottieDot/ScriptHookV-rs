use std::ffi::c_void;

use shv_bindings::getGlobalPtr;

pub extern crate shv_bindings;

pub mod call_native;
pub mod natives;
pub mod types;

mod callbacks;
mod game_version;
mod script;
mod texture;
mod world;
mod scripthookv_builder;
mod scripthookv;

pub(crate) use callbacks::*;
pub use game_version::*;
pub use script::*;
pub use texture::*;
pub use world::*;
pub use scripthookv_builder::*;
pub use scripthookv::*;

pub use scripthookv_rs_macros::*;

pub type ModuleHandle = *const c_void;

/// Gets a script global and returns it as a mutable reference.
///
/// ```
/// /* sets script global 1337 to 10 */
/// *get_global::<int>(1337) = 10
/// ```
#[inline]
pub unsafe fn get_global<T>(global_id: i32) -> &'static mut T {
  let ptr = getGlobalPtr(global_id);

  &mut *(ptr as *mut T)
}
