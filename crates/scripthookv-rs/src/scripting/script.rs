use async_trait::async_trait;

use super::ScriptCommands;

#[async_trait]
pub trait Script {
  async fn start(&mut self, commands: &mut ScriptCommands);
  async fn update(&mut self, commands: &mut ScriptCommands);
  async fn cleanup(&mut self, commands: &mut ScriptCommands);

  fn should_stop(&self) -> bool {
    false
  }
}
