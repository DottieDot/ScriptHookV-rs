use std::{
  sync::{Arc, Mutex},
  task::Poll,
};

use super::{Script, ScriptFuture, ScriptStatus, ScriptInfo};
use smol::LocalExecutor;

pub struct ScriptRuntime<'a> {
  script_info: Arc<Mutex<ScriptInfo>>,
  executor: LocalExecutor<'a>,
}

impl<'a> ScriptRuntime<'a> {
  pub fn new(script: impl Script + 'static) -> Self {
    Self {
      executor: LocalExecutor::new(),
      script_info: Arc::new(Mutex::new(ScriptInfo {
        script: Box::new(script),
        status: ScriptStatus::Pending,
      })),
    }
  }

  pub fn start(&self) {
    let script_info = self.script_info.clone();
    self
      .executor
      .spawn(async move {
        script_info.lock().unwrap().start().await;
      })
      .detach();
  }

  pub fn tick(&mut self) {
    self.executor.try_tick();
  }

  pub fn stop(&mut self) {
    self.script_info.lock().unwrap().stop();
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
