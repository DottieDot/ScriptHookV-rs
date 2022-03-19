use super::{Script, ScriptRuntime, ScriptStatus};

#[derive(Default)]
pub struct ScriptManager<'a> {
  pending_scripts: Vec<Box<dyn Script>>,
  scripts: Vec<ScriptRuntime<'a>>,
}

impl<'a> ScriptManager<'a> {
  pub fn add_script(&mut self, script: Box<dyn Script>) {
    self.pending_scripts.push(script);
  }

  pub fn tick(&mut self) {
    for pending_script in &mut self.pending_scripts.drain(0..) {
      let script_runtime = ScriptRuntime::new(pending_script);
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
