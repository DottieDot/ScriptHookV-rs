use once_cell::sync::OnceCell;
use scripthookv::memory::MemoryLocation;

pub static GET_SCRIPT_ENTITY_SAFE: OnceCell<MemoryLocation> = OnceCell::new();
