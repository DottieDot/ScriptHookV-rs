#![feature(arbitrary_enum_discriminant)]

mod natives;

pub mod color;
pub mod five;
pub mod gta;

mod global_memory;
mod plugin;

pub use plugin::*;
