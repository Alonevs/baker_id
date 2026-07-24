use crate::script_executor::ScriptExecutor;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ChangeType {
    Added,
    Modified,
    Removed,
}

pub struct HotReloadManager {
    pub script_executor: Option<ScriptExecutor>,
}

impl Default for HotReloadManager {
    fn default() -> Self {
        Self {
            script_executor: None,
        }
    }
}
