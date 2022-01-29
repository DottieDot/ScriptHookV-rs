use crate::global_memory::GET_SCRIPT_ENTITY_SAFE;
use scripthookv::{memory::IdaPattern, BuilderPlugin, ScriptHookV, ScriptHookVBuilder};

pub struct ScriptHookVGtaPlugin;

impl BuilderPlugin for ScriptHookVGtaPlugin {
  fn build(&mut self, builder: &mut ScriptHookVBuilder) {
    builder.sig_with_offset(
      "CGameScriptHandler::GetScriptEntitySafe".into(),
      IdaPattern::new("E8 ? ? ? ? 48 89 44 24 ? 45 33 C9").unwrap(),
      |l| unsafe { l.get_call_address() }
    );
  }

  fn init(&mut self, shv: &ScriptHookV) {
    let memory = shv.get_memory();

    GET_SCRIPT_ENTITY_SAFE.get_or_init(|| {
      memory
        .get("CGameScriptHandler::GetScriptEntitySafe")
        .unwrap()
        .clone()
    });
  }

  fn name(&self) -> &'static str {
    "ScriptHookVGtaPlugin"
  }
}
