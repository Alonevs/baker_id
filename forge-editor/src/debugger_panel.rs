//! # Debugger Panel API
//! 
//! Módulo para panel de debugger.

use eframe::egui;

/// Debugger Panel - panel para debugger
#[derive(Default)]
pub struct DebuggerPanel {
    pub breakpoints: Vec<usize>,
    pub watch_vars: Vec<String>,
    pub show_stack: bool,
    pub show_locals: bool,
}

impl DebuggerPanel {
    pub fn new() -> Self {
        Self::default()
    }
    
    pub fn ui(&self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("Debugger Panel");
        });
    }
}

