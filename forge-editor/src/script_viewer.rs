//! # Script Viewer API
//! 
//! Módulo para visualizador de scripts.

use eframe::egui;

/// Script Viewer - visualizador de scripts
#[derive(Default)]
pub struct ScriptViewer {
    pub code: String,
    pub syntax_highlighting: bool,
}

impl ScriptViewer {
    pub fn new() -> Self {
        Self::default()
    }
}

