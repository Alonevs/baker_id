//! # Compile Panel Stub
//! 
//! Stub para CompilePanel.

/// Compile Panel
#[derive(Debug, Clone)]
pub struct CompilePanel {
    pub title: String,
    pub output: String,
    pub errors: Vec<String>,
}

impl Default for CompilePanel {
    fn default() -> Self {
        Self {
            title: String::new(),
            output: String::new(),
            errors: Vec::new(),
        }
    }
}

impl CompilePanel {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn ui(&self, _ctx: &eframe::egui::Context) {
        // stub
    }
}

