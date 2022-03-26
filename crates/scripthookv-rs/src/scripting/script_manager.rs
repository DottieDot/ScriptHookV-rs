use std::sync::{Arc, Mutex};

use super::{Script, ScriptRuntime, ScriptStatus};

#[derive(Default)]
pub struct ScriptManager<'mgr> {
  pending_scripts: Arc<Mutex<Vec<Box<dyn Script<'mgr>>>>>,
  scripts:         Vec<ScriptRuntime<'mgr>>
}

impl<'mgr> ScriptManager<'mgr> {
  pub fn add_script(&mut self, script: impl Script<'mgr> + 'static) {
    self.pending_scripts.lock().unwrap().push(Box::new(script));
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
pub struct ScriptManagerDelegate<'mgr> {
  pending_scripts: Arc<Mutex<Vec<Box<dyn Script<'mgr>>>>>
}

impl<'mgr> ScriptManagerDelegate<'mgr> {
  pub fn add_script(&self, script: Box<dyn Script<'mgr>>) {
    self.pending_scripts.lock().unwrap().push(script)
  }
}
