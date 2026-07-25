use crate::script_executor::ScriptExecutor;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ChangeType {
    Added,
    Modified,
    Removed,
}

pub struct HotReloadManager {
    pub script_executor: Option<ScriptExecutor>,
    pub hot_reload_panel: crate::hot_reload_panel::HotReloadPanel,
    pub pending_changes: Vec<PendingChange>,
    pub is_hot_reload_enabled: bool,
}

#[derive(Debug, Clone)]
pub struct PendingChange {
    pub file_path: String,
    pub change_type: ChangeType,
    pub old_content: Option<String>,
    pub new_content: Option<String>,
}

impl Default for HotReloadManager {
    fn default() -> Self {
        Self::new()
    }
}

impl HotReloadManager {
    pub fn new() -> Self {
        Self {
            script_executor: Some(ScriptExecutor::new()),
            hot_reload_panel: crate::hot_reload_panel::HotReloadPanel::new(),
            pending_changes: Vec::new(),
            is_hot_reload_enabled: true,
        }
    }
    
    pub fn enable(&mut self) {
        self.is_hot_reload_enabled = true;
    }
    
    pub fn disable(&mut self) {
        self.is_hot_reload_enabled = false;
    }
    
    pub fn is_enabled(&self) -> bool {
        self.is_hot_reload_enabled
    }
    
    pub fn register_change(&mut self, file_path: String, change_type: ChangeType, 
                          old_content: Option<String>, new_content: Option<String>) {
        self.pending_changes.push(PendingChange {
            file_path,
            change_type,
            old_content,
            new_content,
        });
    }
    
    pub fn process_pending_changes(&mut self) {
        for change in self.pending_changes.drain(..) {
            match change.change_type {
                ChangeType::Added => {
                    println!("[HOT RELOAD] Script added: {}", change.file_path);
                    if let Some(content) = change.new_content {
                        self.hot_reload_panel.diff_view.new_lines = 
                            content.lines().map(|l| l.to_string()).collect();
                    }
                }
                ChangeType::Modified => {
                    println!("[HOT RELOAD] Script modified: {}", change.file_path);
                    if let Some(old) = change.old_content {
                        self.hot_reload_panel.diff_view.old_lines = 
                            old.lines().map(|l| l.to_string()).collect();
                    }
                    if let Some(new) = change.new_content {
                        self.hot_reload_panel.diff_view.new_lines = 
                            new.lines().map(|l| l.to_string()).collect();
                    }
                }
                ChangeType::Removed => {
                    println!("[HOT RELOAD] Script removed: {}", change.file_path);
                    if let Some(content) = change.old_content {
                        self.hot_reload_panel.diff_view.old_lines = 
                            content.lines().map(|l| l.to_string()).collect();
                    }
                }
            }
        }
    }
    
    pub fn get_panel(&self) -> &crate::hot_reload_panel::HotReloadPanel {
        &self.hot_reload_panel
    }
    
    pub fn get_panel_mut(&mut self) -> &mut crate::hot_reload_panel::HotReloadPanel {
        &mut self.hot_reload_panel
    }
}
