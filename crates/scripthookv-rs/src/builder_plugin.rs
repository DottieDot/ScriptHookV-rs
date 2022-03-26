use crate::{ScriptHookV, ScriptHookVBuilder};

pub trait BuilderPlugin {
  fn build(&mut self, builder: ScriptHookVBuilder) -> ScriptHookVBuilder;

  fn init(&mut self, shv: &ScriptHookV);

  fn name(&self) -> &'static str;
}
