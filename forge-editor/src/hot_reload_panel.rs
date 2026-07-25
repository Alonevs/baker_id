//! # Hot Reload Panel API
//! 
//! Módulo para panel de hot reload.

use eframe::egui;

/// Hot Reload Panel - panel para hot reload
pub struct HotReloadPanel {
    pub selected_file: Option<String>,
    pub preview_result: Option<String>,
    pub preview_error: Option<String>,
    pub diff_view: HotReloadDiffView,
    pub show_version_history: bool,
    pub debounce_count: u64,
    pub reload_status: ReloadStatus,
}

#[derive(Default, Clone, Debug)]
pub struct HotReloadDiffView {
    pub old_lines: Vec<String>,
    pub new_lines: Vec<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ReloadStatus {
    Idle,
    Reloading,
    Reloaded,
    Error(String),
}

impl Default for ReloadStatus {
    fn default() -> Self {
        ReloadStatus::Idle
    }
}

impl HotReloadPanel {
    pub fn new() -> Self {
        Self {
            selected_file: None,
            preview_result: None,
            preview_error: None,
            diff_view: HotReloadDiffView::default(),
            show_version_history: false,
            debounce_count: 0,
            reload_status: ReloadStatus::Idle,
        }
    }
    
    pub fn ui(&mut self, ctx: &egui::Context) {
        egui::SidePanel::left("hot_reload_panel")
            .resizable(true)
            .min_width(300.0)
            .show(ctx, |ui| {
                ui.heading("🔥 Hot Reload");
                ui.separator();
                self.render_status(ui);
                ui.separator();
                self.render_file_selector(ui);
                ui.separator();
                self.render_diff_view(ui);
                ui.separator();
                self.render_preview(ui);
                ui.separator();
                self.render_actions(ui);
            });
    }
    
    fn render_status(&self, ui: &mut egui::Ui) {
        let status_text = match &self.reload_status {
            ReloadStatus::Idle => "⏸️ Ready".to_string(),
            ReloadStatus::Reloading => "🔄 Reloading...".to_string(),
            ReloadStatus::Reloaded => "✅ Reloaded".to_string(),
            ReloadStatus::Error(e) => format!("❌ Error: {}", e),
        };
        ui.label(status_text);
    }
    
    fn render_file_selector(&mut self, ui: &mut egui::Ui) {
        ui.label("Select Script:");
        let files = vec![
            "scripts/enemy_ai.bf".to_string(),
            "scripts/player_controller.bf".to_string(),
            "scripts/game_loop.bf".to_string(),
            "scripts/input_handler.bf".to_string(),
        ];
        for file in &files {
            ui.horizontal(|ui| {
                ui.label(file);
                if ui.button("▶").clicked() {
                    self.selected_file = Some(file.clone());
                }
            });
        }
    }
    
    fn render_diff_view(&self, ui: &mut egui::Ui) {
        ui.label("Diff View:");
        ui.label("OLD:");
        for line in &self.diff_view.old_lines {
            ui.label(format!("  {}", line));
        }
        ui.label("NEW:");
        for line in &self.diff_view.new_lines {
            ui.label(format!("    {}", line));
        }
    }
    
    fn render_preview(&self, ui: &mut egui::Ui) {
        ui.label("Preview:");
        if let Some(result) = &self.preview_result {
            ui.label(result.clone());
        } else {
            ui.label("No preview available");
        }
    }
    
    fn render_actions(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            if ui.button("🔄 Reload").clicked() {
                self.reload_status = ReloadStatus::Reloading;
                self.debounce_count = 0;
            }
            if ui.button("⏹ Stop").clicked() {
                self.reload_status = ReloadStatus::Idle;
                self.preview_error = None;
            }
            if ui.button("📋 Clear").clicked() {
                self.preview_result = None;
                self.preview_error = None;
                self.diff_view = HotReloadDiffView::default();
            }
        });
    }
    
    pub fn update(&mut self, dt: f32) {
        self.debounce_count += 1;
        if self.reload_status == ReloadStatus::Reloading {
            if self.debounce_count > 10 {
                self.reload_status = ReloadStatus::Reloaded;
                self.debounce_count = 0;
            }
        }
    }
}

impl Default for HotReloadPanel {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hot_reload_panel_new() {
        let panel = HotReloadPanel::new();
        assert_eq!(panel.reload_status, ReloadStatus::Idle);
    }

    #[test]
    fn test_hot_reload_panel_default() {
        let panel = HotReloadPanel::default();
        assert_eq!(panel.reload_status, ReloadStatus::Idle);
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
        panel.reload_status = ReloadStatus::Reloading;
        panel.update(0.016);
        for _ in 0..10 {
            panel.update(0.016);
        }
        assert_eq!(panel.reload_status, ReloadStatus::Reloaded);
    }

    #[test]
    fn test_hot_reload_panel_diff_view() {
        let diff_view = HotReloadDiffView::default();
        assert!(diff_view.old_lines.is_empty());
    }
}
