//! # Serialization Panel API
//! 
//! Módulo para panel de serialización.

use eframe::egui;

/// Serialization Panel - panel para serialización de datos
#[derive(Default)]
pub struct SerializationPanel {
    pub data: Option<serde_json::Value>,
    pub show_pretty: bool,
    pub selected_type: String,
}

impl SerializationPanel {
    pub fn new() -> Self {
        Self::default()
    }
    
    pub fn set_data(&mut self, data: serde_json::Value) {
        self.data = Some(data);
    }
    
    pub fn set_show_pretty(&mut self, show: bool) {
        self.show_pretty = show;
    }
    
    pub fn set_selected_type(&mut self, ty: String) {
        self.selected_type = ty;
    }
    
    pub fn ui(&self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("Serialization Panel");
        });
    }
}

