use crate::{ScriptHookV, ScriptHookVBuilder};

pub trait BuilderPlugin<'b> {
  fn build(&mut self, builder: ScriptHookVBuilder<'b>) -> ScriptHookVBuilder<'b>;

  fn init(&mut self, shv: &ScriptHookV);

  fn name(&self) -> &'static str;
}
