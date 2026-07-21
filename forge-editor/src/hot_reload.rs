//! # Hot Reload Module Stub
//! 
//! Módulo temporal para hot reload.

use std::collections::HashMap;
use crate::{ScriptExecutor, ui};

/// Hot Reload Manager stub
#[derive(Debug, Clone)]
pub struct HotReloadManager {
    pub manager: String,
    pub diff_view: ui::DiffView,
    pub state: HotReloadState,
    pub config: HotReloadConfig,
}

/// Estado de hot reload
#[derive(Debug, Clone, Default)]
pub struct HotReloadState {
    pub preview: LivePreview,
    pub pending_changes: HashMap<String, FileChange>,
    pub reload_count: u64,
    pub execution_time: f64,
    pub version_map: HashMap<String, FileVersion>,
    pub script_executor: ScriptExecutor,
    pub watch_extensions: Vec<String>,
    pub debounce_ms: u64,
    pub preview_mode: PreviewMode,
}

impl Default for HotReloadManager {
    fn default() -> Self {
        Self {
            manager: String::new(),
            diff_view: ui::DiffView::default(),
            state: HotReloadState::default(),
            config: HotReloadConfig::default(),
        }
    }
}

impl HotReloadManager {
    pub fn new(config: Option<HotReloadConfig>) -> Self {
        Self {
            manager: String::new(),
            diff_view: ui::DiffView::default(),
            state: HotReloadState::default(),
            config: config.unwrap_or_default(),
        }
    }
    
    pub fn config(&self) -> &HotReloadConfig {
        &self.config
    }
    
    pub fn state(&self) -> &HotReloadState {
        &self.state
    }
    
    pub fn get_live_preview(&self) -> &LivePreview {
        &self.state.preview
    }
    
    pub fn get_pending_changes(&self) -> &HashMap<String, FileChange> {
        &self.state.pending_changes
    }
    
    pub fn reload_count(&self) -> u64 {
        self.state.reload_count
    }
    
    pub fn execution_time(&self) -> f64 {
        self.state.execution_time
    }
    
    pub fn version_map(&self) -> &HashMap<String, FileVersion> {
        &self.state.version_map
    }
    
    pub fn script_executor(&self) -> &ScriptExecutor {
        &self.state.script_executor
    }
    
    pub fn get_current_script(&self) -> Option<&str> {
        None
    }
    
    pub fn get_last_error(&self) -> Option<&crate::compile_system::CompileError> {
        None
    }
    
    pub fn watch_extensions(&self) -> &[String] {
        &self.state.watch_extensions
    }
    
    pub fn debounce_ms(&self) -> u64 {
        self.state.debounce_ms
    }
    
    pub fn preview_mode(&self) -> &PreviewMode {
        &self.state.preview_mode
    }
    
    pub fn set_preview_mode(&mut self, mode: PreviewMode) {
        self.state.preview_mode = mode;
    }
    
    pub fn register_file(&mut self, _file_path: &str, _content: &str) {
        // stub
    }
    
    pub fn update_file(&mut self, _file_path: &str, _content: &str) {
        // stub
    }
    
    pub fn revert_file(&mut self, _file_path: &str) -> Option<String> {
        None
    }
}



/// Hot Reload Config
#[derive(Debug, Clone, Default)]
pub struct HotReloadConfig {
    pub debounce_ms: u64,
    pub watch_extensions: Vec<String>,
    pub preview_mode: PreviewMode,
}

/// Live Preview
#[derive(Debug, Clone, Default)]
pub struct LivePreview {
    pub preview_mode: PreviewMode,
    pub content: String,
    pub path: String,
}

/// Preview Mode
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PreviewMode {
    #[default]
    SideBySide,
    Inline,
    Tab,
}

/// File Change
#[derive(Debug, Clone)]
pub struct FileChange {
    pub file: String,
    pub change_type: ChangeType,
    pub version: FileVersion,
}

impl Default for FileChange {
    fn default() -> Self {
        Self {
            file: String::new(),
            change_type: ChangeType::Add,
            version: FileVersion::default(),
        }
    }
}

/// Change Type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ChangeType {
    #[default]
    Add,
    Modify,
    Delete,
}

/// File Version
#[derive(Debug, Clone, Default)]
pub struct FileVersion {
    pub timestamp: u64,
    pub hash: String,
}

impl FileVersion {
    pub fn new(timestamp: u64, hash: String) -> Self {
        Self { timestamp, hash }
    }
}

