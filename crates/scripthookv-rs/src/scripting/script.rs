use std::sync::Arc;

use async_trait::async_trait;

use super::ScriptCommands;

#[async_trait(?Send)]
pub trait Script<'rt> {
  async fn start(&mut self, commands: Arc<ScriptCommands<'rt>>);
  async fn update(&mut self, commands: Arc<ScriptCommands<'rt>>);
  async fn cleanup(&mut self, commands: Arc<ScriptCommands<'rt>>);

  fn should_stop(&self) -> bool {
    false
  }
}
