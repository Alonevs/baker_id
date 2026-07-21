//! # Debugger Panel Stub
//! 
//! Stub para DebuggerPanel.

/// Debugger Panel
#[derive(Debug, Clone)]
pub struct DebuggerPanel {
    pub title: String,
    pub stack: Vec<String>,
    pub variables: Vec<String>,
}

impl Default for DebuggerPanel {
    fn default() -> Self {
        Self {
            title: String::new(),
            stack: Vec::new(),
            variables: Vec::new(),
        }
    }
}

impl DebuggerPanel {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn ui(&self, _ctx: &eframe::egui::Context) {
        // stub
    }
}

