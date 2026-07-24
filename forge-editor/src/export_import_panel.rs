//! Export Import Panel - Panel para exportar e importar assets

use eframe::egui;

pub struct ExportImportPanel;

impl ExportImportPanel {
    pub fn new() -> Self {
        Self
    }
    
    pub fn show(&self, ctx: &egui::Context, ui: &mut egui::Ui) {
        ui.label("Export Import Panel");
    }
}
