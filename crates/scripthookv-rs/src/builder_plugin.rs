use std::fmt::Debug;

use crate::{ScriptHookV, ScriptHookVBuilder};

pub trait BuilderPlugin: Debug {
  fn build(&mut self, builder: &mut ScriptHookVBuilder);

  fn init(&mut self, shv: &ScriptHookV);

  fn name(&self) -> &'static str;
}
