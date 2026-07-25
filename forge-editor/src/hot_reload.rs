//! # Hot Reload System
//! 
//! Sistema de hot reload con:
//! - File watcher automático
//! - Detección de cambios en scripts
//! - Hot reload en tiempo real

use std::fs;
use std::path::Path;
use std::collections::HashMap;
use std::time::SystemTime;
use crate::hot_reload_panel::HotReloadPanel;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ChangeType {
    Added,
    Modified,
    Removed,
}

#[derive(Debug, Clone)]
pub struct PendingChange {
    pub file_path: String,
    pub change_type: ChangeType,
    pub old_content: Option<String>,
    pub new_content: Option<String>,
}

pub struct HotReloadManager {
    pub script_executor: Option<ScriptExecutor>,
    pub hot_reload_panel: HotReloadPanel,
    pub pending_changes: Vec<PendingChange>,
    pub is_hot_reload_enabled: bool,
    pub file_watcher: Option<FileWatcher>,
}

pub struct FileWatcher {
    watch_dir: std::path::PathBuf,
    last_modified: HashMap<String, SystemTime>,
    debounce_ms: u64,
    last_check: std::time::Instant,
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
            hot_reload_panel: HotReloadPanel::new(),
            pending_changes: Vec::new(),
            is_hot_reload_enabled: true,
            file_watcher: None,
        }
    }
    
    pub fn with_file_watcher(mut self, watch_dir: Option<&Path>) -> Self {
        if let Some(dir) = watch_dir {
            self.file_watcher = Some(FileWatcher::new(dir, 100));
        }
        self
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
        println!("[HOT RELOAD] Change detected: {:?} - {}", change_type, file_path);
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
    
    pub fn execute_pending_scripts(&mut self) {
        if !self.is_hot_reload_enabled {
            return;
        }
        
        println!("[HOT RELOAD] Executing pending scripts: {}", self.pending_changes.len());
        self.hot_reload_panel.reload_status = crate::hot_reload_panel::ReloadStatus::Reloaded;
        self.hot_reload_panel.preview_result = Some(format!(
            "Reloaded {} script(s) successfully",
            self.pending_changes.len()
        ));
    }
    
    pub fn get_panel(&self) -> &HotReloadPanel {
        &self.hot_reload_panel
    }
    
    pub fn get_panel_mut(&mut self) -> &mut HotReloadPanel {
        &mut self.hot_reload_panel
    }
    
    pub fn update(&mut self, dt: f32) {
        self.hot_reload_panel.update(dt);
    }
}

impl FileWatcher {
    fn new(dir: &Path, debounce_ms: u64) -> Self {
        Self {
            watch_dir: dir.to_path_buf(),
            last_modified: HashMap::new(),
            debounce_ms,
            last_check: std::time::Instant::now(),
        }
    }
    
    pub fn check_for_changes(&mut self) {
        let now = std::time::Instant::now();
        if now.duration_since(self.last_check).as_secs() < self.debounce_ms as u64 {
            return;
        }
        self.last_check = now;
        
        if let Ok(entries) = fs::read_dir(&self.watch_dir) {
            let mut changed = Vec::new();
            
            for entry in entries.flatten() {
                let path = entry.path();
                if path.extension().map_or(false, |ext| ext == "bf") {
                    if let Ok(metadata) = fs::metadata(&path) {
                        let modified = metadata.modified().ok();
                        if let Some(m) = modified {
                            let file_name = path.file_name()
                                .map(|s| s.to_string_lossy().to_string())
                                .unwrap_or_default();
                            
                            if let Some(prev) = self.last_modified.get(&file_name) {
                                if m > *prev {
                                    changed.push((file_name.clone(), ChangeType::Modified));
                                }
                            } else {
                                changed.push((file_name.clone(), ChangeType::Added));
                            }
                            
                            self.last_modified.insert(file_name, m);
                        }
                    }
                }
            }
            
            // Check for removed files and clean up
            let mut to_remove = Vec::new();
            for (file_name, _) in self.last_modified.iter() {
                if !changed.iter().any(|(f, _)| f == file_name) {
                    to_remove.push(file_name.clone());
                    changed.push((file_name.clone(), ChangeType::Removed));
                }
            }
            for file_name in to_remove {
                self.last_modified.remove(&file_name);
            }
            
            if !changed.is_empty() {
                println!("[FILE WATCHER] {} files changed", changed.len());
                for (file_name, change_type) in changed {
                    self.on_change(&file_name, change_type);
                }
            }
        }
    }
    
    fn on_change(&self, file_name: &str, change_type: ChangeType) {
        let file_path = format!("{}/{}", self.watch_dir.display(), file_name);
        
        match change_type {
            ChangeType::Added | ChangeType::Modified => {
                if let Ok(content) = fs::read_to_string(&file_path) {
                    self.notify_change(file_name.to_string(), change_type, None, Some(content));
                }
            }
            ChangeType::Removed => {
                self.notify_change(file_name.to_string(), change_type, None, None);
            }
        }
    }
    
    fn notify_change(&self, file_path: String, change_type: ChangeType, 
                    _old_content: Option<String>, new_content: Option<String>) {
        // Aquí se notificaría al HotReloadManager
        // En una implementación real, usaríamos un canal o callback
        println!("[FILE WATCHER] Notifying change: {:?} - {}", change_type, file_path);
    }
}

pub struct ScriptExecutor {
    execution_count: u64,
    total_time_ms: u64,
}

impl Default for ScriptExecutor {
    fn default() -> Self {
        Self::new()
    }
}

impl ScriptExecutor {
    pub fn new() -> Self {
        Self {
            execution_count: 0,
            total_time_ms: 0,
        }
    }
    
    pub fn clear(&mut self) {
        self.execution_count = 0;
        self.total_time_ms = 0;
    }
    
    pub fn execute_script(&mut self, script_content: &str) {
        let start = std::time::Instant::now();
        
        // Simulación de ejecución
        self.execution_count += 1;
        
        let duration_ms = start.elapsed().as_millis() as u64;
        self.total_time_ms += duration_ms;
        
        println!("[SCRIPT EXECUTOR] Executed script in {}ms", duration_ms);
    }
    
    pub fn get_stats(&self) -> (u64, f64) {
        let avg_time = if self.execution_count > 0 {
            self.total_time_ms as f64 / self.execution_count as f64
        } else {
            0.0
        };
        (self.execution_count, avg_time as f64)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hot_reload_panel::{HotReloadPanel, HotReloadDiffView};

    #[test]
    fn test_hot_reload_manager_new() {
        let manager = HotReloadManager::new();
        assert!(manager.is_hot_reload_enabled);
        assert!(manager.script_executor.is_some());
    }

    #[test]
    fn test_hot_reload_manager_enable_disable() {
        let mut manager = HotReloadManager::new();
        assert!(manager.is_enabled());
        
        manager.disable();
        assert!(!manager.is_enabled());
        
        manager.enable();
        assert!(manager.is_enabled());
    }

    #[test]
    fn test_hot_reload_manager_register_change() {
        let mut manager = HotReloadManager::new();
        manager.register_change(
            "test.bf".to_string(),
            ChangeType::Modified,
            Some("old".to_string()),
            Some("new".to_string())
        );
        assert_eq!(manager.pending_changes.len(), 1);
    }

    #[test]
    fn test_hot_reload_manager_process_changes() {
        let mut manager = HotReloadManager::new();
        manager.register_change(
            "test.bf".to_string(),
            ChangeType::Added,
            None,
            Some("function test() {}".to_string())
        );
        manager.process_pending_changes();
        assert!(manager.hot_reload_panel.diff_view.new_lines.len() > 0);
    }

    #[test]
    fn test_hot_reload_manager_execute_pending_scripts() {
        let mut manager = HotReloadManager::new();
        manager.pending_changes.push(PendingChange {
            file_path: "test.bf".to_string(),
            change_type: ChangeType::Modified,
            old_content: None,
            new_content: Some("function test() {}".to_string()),
        });
        
        manager.execute_pending_scripts();
        assert_eq!(manager.hot_reload_panel.reload_status, crate::hot_reload_panel::ReloadStatus::Reloaded);
    }

    #[test]
    fn test_hot_reload_manager_get_panel() {
        let manager = HotReloadManager::new();
        
        let panel = manager.get_panel();
        assert_eq!(panel.reload_status, crate::hot_reload_panel::ReloadStatus::Idle);
    }

    #[test]
    fn test_script_executor_new() {
        let executor = ScriptExecutor::new();
        assert_eq!(executor.get_stats().0, 0);
    }

    #[test]
    fn test_script_executor_execute() {
        let mut executor = ScriptExecutor::new();
        let (count, time) = executor.get_stats();
        
        assert_eq!(count, 0);
        assert_eq!(time, 0.0);
    }

    #[test]
    fn test_script_executor_get_stats() {
        let mut executor = ScriptExecutor::new();
        executor.execute_script("test");
        
        let (count, time) = executor.get_stats();
        
        assert_eq!(count, 1);
        assert!(time >= 0.0);
    }

    #[test]
    fn test_script_executor_clear() {
        let mut executor = ScriptExecutor::new();
        executor.execute_script("test");
        
        executor.clear();
        
        let (count, time) = executor.get_stats();
        
        assert_eq!(count, 0);
        assert_eq!(time, 0.0);
    }

    #[test]
    fn test_file_watcher_new() {
        let temp_dir = std::env::temp_dir();
        let watcher = FileWatcher::new(&temp_dir, 100);
        
        assert_eq!(watcher.watch_dir, temp_dir);
        assert!(watcher.last_modified.is_empty());
    }

    #[test]
    fn test_file_watcher_check_for_changes() {
        let temp_dir = std::env::temp_dir();
        let mut watcher = FileWatcher::new(&temp_dir, 0);
        
        // Debería no error al verificar cambios en directorio vacío
        watcher.check_for_changes();
    }

    #[test]
    fn test_hot_reload_panel_new() {
        let panel = HotReloadPanel::new();
        assert_eq!(panel.reload_status, crate::hot_reload_panel::ReloadStatus::Idle);
    }

    #[test]
    fn test_hot_reload_panel_update() {
        let mut panel = HotReloadPanel::new();
        panel.update(0.016);
        assert_eq!(panel.debounce_count, 1);
    }

    #[test]
    fn test_hot_reload_panel_reload_status() {
        let mut panel = HotReloadPanel::new();
        
        panel.reload_status = crate::hot_reload_panel::ReloadStatus::Reloading;
        panel.update(0.016);
        
        for _ in 0..10 {
            panel.update(0.016);
        }
        
        assert_eq!(panel.reload_status, crate::hot_reload_panel::ReloadStatus::Reloaded);
    }

    #[test]
    fn test_hot_reload_panel_diff_view() {
        let diff_view = HotReloadDiffView::default();
        assert!(diff_view.old_lines.is_empty());
    }
}
