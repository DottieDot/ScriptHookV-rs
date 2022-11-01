use std::{
  sync::{Arc, RwLock},
  task::Poll,
  time::{Duration, Instant}
};

use super::{Script, ScriptCommands, ScriptFuture, ScriptManagerDelegate, ScriptStatus};
use smol::{lock::Mutex, LocalExecutor};

pub struct ScriptRuntime<'rt> {
  executor:       Arc<LocalExecutor<'rt>>,
  script:         Arc<Mutex<Box<dyn Script<'rt>>>>,
  status:         Arc<RwLock<ScriptStatus>>,
  script_manager: ScriptManagerDelegate<'rt>
}

impl<'rt> ScriptRuntime<'rt> {
  pub fn new(script: Box<dyn Script<'rt>>, script_manager: ScriptManagerDelegate<'rt>) -> Self {
    Self {
      executor: Arc::new(LocalExecutor::new()),
      script: Arc::new(Mutex::new(script)),
      status: Arc::new(RwLock::new(ScriptStatus::Pending)),
      script_manager
    }
  }

  pub fn start(&self) {
    let script = self.script.clone();
    let status = self.status.clone();
    let commands = Arc::new(ScriptCommands::new(
      self.script_manager.clone(),
      self.executor.clone()
    ));

    self
      .executor
      .spawn(async move {
        let mut locked_script = script.lock().await;

        loop {
          if locked_script.should_stop() {
            *status.write().unwrap() = ScriptStatus::Stopping;
          }

          let st = *status.read().unwrap();
          match st {
            ScriptStatus::Pending => {
              *status.write().unwrap() = ScriptStatus::Starting;
              locked_script.start(commands.clone()).await;
            }
            ScriptStatus::Starting => {
              *status.write().unwrap() = ScriptStatus::Running;
            }
            ScriptStatus::Running => {
              locked_script.update(commands.clone()).await;
              yield_async().await;
            }
            ScriptStatus::Stopping => {
              locked_script.cleanup(commands.clone()).await;
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

pub async fn wait_async(duration: Duration) {
  let wake_at = Instant::now() + duration;
  ScriptFuture::new(move || {
    if Instant::now() >= wake_at {
      Poll::Ready(())
    } else {
      Poll::Pending
    }
  })
  .await;
}
