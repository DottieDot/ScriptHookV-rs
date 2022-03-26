use std::{sync::Arc, future::Future};

use smol::{LocalExecutor, Task};

use super::{Script, ScriptManagerDelegate};

pub struct ScriptCommands<'rt> {
  script_manager: ScriptManagerDelegate<'rt>,
  executor: Arc<LocalExecutor<'rt>>
}

impl<'rt> ScriptCommands<'rt> {
  pub fn new(script_manager: ScriptManagerDelegate<'rt>, executor: Arc<LocalExecutor<'rt>>) -> Self {
    Self { script_manager, executor }
  }

  pub fn spawn_script(&self, script: impl Script<'rt> + 'static) {
    self.script_manager.add_script(Box::new(script))
  }

  pub fn spawn_task<T: 'rt>(&self, future: impl Future<Output = T> + 'rt) -> Task<T>
  {
    self.executor.spawn(future)
  }
}

unsafe impl<'rt> Sync for ScriptCommands<'rt> {}
unsafe impl<'rt> Send for ScriptCommands<'rt> {}
