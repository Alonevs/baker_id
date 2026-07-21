//! # Hot Reload Panel API
//! 
//! Módulo para panel de hot reload.

use eframe::egui;

/// Hot Reload Panel - panel para hot reload
#[derive(Default)]
pub struct HotReloadPanel {
    pub selected_file: Option<String>,
    pub preview_result: Option<String>,
    pub preview_error: Option<String>,
    pub diff_view: HotReloadDiffView,
    pub show_version_history: bool,
    pub debounce_count: u64,
}

#[derive(Default, Clone)]
pub struct HotReloadDiffView {
    pub old_lines: Vec<String>,
    pub new_lines: Vec<String>,
}

impl HotReloadPanel {
    pub fn new() -> Self {
        Self::default()
    }
    
    pub fn ui(&self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("Hot Reload Panel");
        });
    }
}

