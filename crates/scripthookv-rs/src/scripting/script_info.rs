use super::{Script, ScriptStatus, yield_async};

pub struct ScriptInfo {
  pub(super) script: Box<dyn Script>,
  pub(super) status: ScriptStatus,
}

impl ScriptInfo {
  pub async fn start(&mut self) {
    self.status = ScriptStatus::Starting;
    self.script.start().await;

    if self.status == ScriptStatus::Starting {
      self.status = ScriptStatus::Running;
      while self.status == ScriptStatus::Running {
        self.script.update().await;
        yield_async().await;
      }
    }

    if self.status == ScriptStatus::Stopping {
      self.script.cleanup().await;
    }

    self.status = ScriptStatus::Terminated;
  }

  pub fn stop(&mut self) {
    self.status = ScriptStatus::Stopping;
  }
}
