pub mod command_center;
pub mod event_graph_tool;
pub mod sync_system;

pub use command_center::CommandCenterTool;
pub use event_graph_tool::EventGraphTool;
pub use sync_system::{SharedState, SyncSystem};

pub struct ToolManager {
    pub command_center: CommandCenterTool,
    pub event_graph: EventGraphTool,
    pub sync_system: SharedState,
}

impl ToolManager {
    pub fn new() -> Self {
        Self {
            command_center: CommandCenterTool::new(),
            event_graph: EventGraphTool::new(),
            sync_system: SharedState::new(),
        }
    }

    pub fn init_sync(&mut self) {
        self.sync_system.init();
    }
}
