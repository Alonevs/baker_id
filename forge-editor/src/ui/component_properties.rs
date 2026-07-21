//! # Component Properties API
//! 
//! Módulo para propiedades de componentes.

use eframe::egui;
use crate::debugger::LogLevel;

/// Component Properties - propiedades de componentes
#[derive(Debug, Clone)]
pub struct ComponentProperties {
    pub component_type: String,
    pub keys: Vec<String>,
    pub values: Vec<String>,
    pub enabled: bool,
}

impl Default for ComponentProperties {
    fn default() -> Self {
        Self {
            component_type: "None".to_string(),
            keys: Vec::new(),
            values: Vec::new(),
            enabled: true,
        }
    }
}

impl ComponentProperties {
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Renderiza el editor de componentes en la UI
    /// 
    /// Conecta con: component_editor
    pub fn render(ui: &mut egui::Ui, app: &mut crate::ForgeEditorApp) {
        ui.heading("Component Properties");
        ui.add_space(10.0);
        ui.separator();
        ui.add_space(5.0);
        
        ui.group(|ui| {
            ui.label("Component Properties");
            ui.add_space(5.0);
            ui.label(format!("Component type: {}", app.component_props.component_type));
            ui.label(format!("Enabled: {}", app.component_props.enabled));
        });
    }
    
    /// Renderiza el panel de componentes
    pub fn ui(&mut self, ui: &mut egui::Ui) {
        ui.heading("Component Properties");
        ui.add_space(10.0);
        ui.separator();
        ui.add_space(5.0);
        
        ui.group(|ui| {
            ui.label("Type:");
            ui.text_edit_singleline(&mut self.component_type);
            ui.add_space(5.0);
            
            if !self.keys.is_empty() {
                ui.label("Properties:");
                for (key, value) in self.keys.iter().zip(self.values.iter()) {
                    ui.horizontal(|ui| {
                        ui.label(key);
                        ui.label(":");
                        ui.label(value);
                    });
                }
            }
        });
    }
    
    /// Renderiza propiedades de sprite para entidad seleccionada
    pub fn render_sprite_properties(ui: &mut egui::Ui, app: &mut crate::ForgeEditorApp) {
        ui.group(|ui| {
            ui.label("Sprite Asset");
            ui.add_space(5.0);
            
            if let Some(sprite_path) = &app.selected_entity_sprite_path {
                ui.label(format!("📄 Selected: {}", sprite_path));
                ui.add_space(3.0);
                
                if ui.button("❌ Clear Sprite").clicked() {
                    app.selected_entity_sprite_path = None;
                    app.console.add_message(LogLevel::Info, "Sprite cleared");
                }
            } else {
                ui.label("No sprite assigned. Select an image from Asset Browser and click 'Assign to Selected Entity'");
            }
        });
    }

    pub fn set_component_type(&mut self, ty: &str) {
        self.component_type = ty.to_string();
    }

    pub fn add_property(&mut self, key: &str, value: &str) {
        self.keys.push(key.to_string());
        self.values.push(value.to_string());
    }

    pub fn remove_property(&mut self, index: usize) {
        if index < self.keys.len() {
            self.keys.remove(index);
            self.values.remove(index);
        }
    }
}


