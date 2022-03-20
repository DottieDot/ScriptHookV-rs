#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ScriptStatus {
  Pending,
  Starting,
  Running,
  Stopping,
  Terminated
}
