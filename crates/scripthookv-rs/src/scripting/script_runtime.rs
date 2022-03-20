use std::{
  sync::{Arc, Mutex, RwLock},
  task::Poll
};

use super::{Script, ScriptFuture, ScriptStatus};
use smol::LocalExecutor;

pub struct ScriptRuntime<'a> {
  executor: LocalExecutor<'a>,
  script:   Arc<Mutex<Box<dyn Script>>>,
  status:   Arc<RwLock<ScriptStatus>>
}

impl<'a> ScriptRuntime<'a> {
  pub fn new(script: Box<dyn Script>) -> Self {
    Self {
      executor: LocalExecutor::new(),
      script:   Arc::new(Mutex::new(script)),
      status:   Arc::new(RwLock::new(ScriptStatus::Pending))
    }
  }

  pub fn start(&self) {
    let script = self.script.clone();
    let status = self.status.clone();

    self
      .executor
      .spawn(async move {
        let mut locked_script = script.lock().unwrap();

        loop {
          if locked_script.should_stop() {
            *status.write().unwrap() = ScriptStatus::Stopping;
          }

          match *status.read().unwrap() {
            ScriptStatus::Pending => {
              *status.write().unwrap() = ScriptStatus::Starting;
              locked_script.start().await;
            }
            ScriptStatus::Starting => {
              *status.write().unwrap() = ScriptStatus::Running;
            }
            ScriptStatus::Running => {
              locked_script.update().await;
              yield_async().await;
            }
            ScriptStatus::Stopping => {
              locked_script.cleanup().await;
              *status.write().unwrap() = ScriptStatus::Terminated;
            }
            ScriptStatus::Terminated => {
              break;
            }
          }
        }
      })
      .detach();
  }

  pub fn tick(&mut self) {
    self.executor.try_tick();
  }

  pub fn stop(&mut self) {
    *self.status.write().unwrap() = ScriptStatus::Stopping;
  }

  pub fn status(&self) -> ScriptStatus {
    *self.status.read().unwrap()
  }
}

pub async fn yield_async() {
  let mut resume = false;
  ScriptFuture::new(move || {
    if !resume {
      resume = true;
      Poll::Pending
    } else {
      Poll::Ready(())
    }
  })
  .await;
}
