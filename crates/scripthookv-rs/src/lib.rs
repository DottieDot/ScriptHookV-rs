use shv_bindings::{getGlobalPtr};

pub extern crate shv_bindings;

pub mod natives;
pub mod call_native;
pub mod types;

mod world;
mod script;
mod texture;
mod callbacks;
mod game_version;

pub use world::*;
pub use script::*;
pub use texture::*;
pub use callbacks::*;
pub use game_version::*;

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
