//! # Compile Panel API
//! 
//! Módulo para panel de compilación.

use eframe::egui;

/// Compile Panel - panel para compilación de scripts
#[derive(Default)]
pub struct CompilePanel {
    pub source: String,
    pub last_result: Option<String>,
    pub error: Option<String>,
}

impl CompilePanel {
    pub fn new() -> Self {
        Self::default()
    }
    
    pub fn ui(&self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("Compile Panel");
        });
    }
}

