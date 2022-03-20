use std::sync::{Arc, Mutex};

use super::{Script, ScriptRuntime, ScriptStatus};

#[derive(Default)]
pub struct ScriptManager<'a> {
  pending_scripts: Arc<Mutex<Vec<Box<dyn Script>>>>,
  scripts:         Vec<ScriptRuntime<'a>>
}

impl<'a> ScriptManager<'a> {
  pub fn add_script(&mut self, script: Box<dyn Script>) {
    self.pending_scripts.lock().unwrap().push(script);
  }

  pub fn tick(&mut self) {
    for pending_script in &mut self.pending_scripts.lock().unwrap().drain(0..) {
      let script_runtime = ScriptRuntime::new(
        pending_script,
        ScriptManagerDelegate {
          pending_scripts: self.pending_scripts.clone()
        }
      );
      script_runtime.start();
      self.scripts.push(script_runtime);
    }

    for script in &mut self.scripts {
      script.tick();
    }
    self
      .scripts
      .retain(|s| s.status() != ScriptStatus::Terminated);
  }
}

#[derive(Clone)]
pub struct ScriptManagerDelegate {
  pending_scripts: Arc<Mutex<Vec<Box<dyn Script>>>>
}

impl ScriptManagerDelegate {
  pub fn add_script(&mut self, script: Box<dyn Script>) {
    self.pending_scripts.lock().unwrap().push(script)
  }
}
