//! # Script Viewer API
//! 
//! Módulo para visualización de scripts.

use eframe::egui;

/// Script Viewer - visualización de scripts
#[derive(Default)]
pub struct ScriptViewer {
    pub current_script: Option<String>,
    pub line: usize,
    pub column: usize,
}

impl ScriptViewer {
    pub fn new() -> Self {
        Self::default()
    }
    
    pub fn render(&self, _ui: &mut egui::Ui, _app: &crate::ForgeEditorApp) {
        // Placeholder for script viewer UI
    }
}

