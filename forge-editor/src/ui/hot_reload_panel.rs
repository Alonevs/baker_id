//! # Hot Reload Panel Stub
//! 
//! Stub para HotReloadPanel.

/// Hot Reload Panel
#[derive(Debug, Clone, Default)]
pub struct HotReloadPanel {
    pub title: String,
    pub debounce_count: u64,
    pub preview_result: Option<String>,
    pub preview_error: Option<String>,
    pub selected_file: Option<String>,
    pub diff_view: crate::ui::DiffView,
    pub show_version_history: bool,
}

impl HotReloadPanel {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn ui(&self, _ctx: &eframe::egui::Context) {
        // stub
    }
}

