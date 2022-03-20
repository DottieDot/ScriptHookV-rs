use super::{Script, ScriptManagerDelegate};

pub struct ScriptCommands {
  script_manager: ScriptManagerDelegate
}

impl ScriptCommands {
  pub fn new(script_manager: ScriptManagerDelegate) -> Self {
    Self { script_manager }
  }

  pub fn spawn_script(&mut self, script: impl Script + 'static) {
    self.script_manager.add_script(Box::new(script))
  }
}

unsafe impl Sync for ScriptCommands {}
unsafe impl Send for ScriptCommands {}
