//! # Property Editor API
//! 
//! Módulo para el editor de propiedades de entidades.

use eframe::egui;

/// Property Editor UI para edición de propiedades de entidades
/// 
/// Conecta con: transform_properties
#[derive(Debug, Clone)]
pub struct PropertyEditor {
    pub property_type: String,
    pub values: Vec<String>,
    pub show_values: bool,
    pub show_visibility: bool,
    pub show_scale: bool,
    pub current_entity: Option<usize>,
    pub active_tab: usize,
}

impl Default for PropertyEditor {
    fn default() -> Self {
        Self {
            property_type: "Transform".to_string(),
            values: Vec::new(),
            show_values: true,
            show_visibility: true,
            show_scale: true,
            current_entity: None,
            active_tab: 0,
        }
    }
}

impl PropertyEditor {
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Renderiza el editor de propiedades en la UI
    /// 
    /// Conecta con: transform_properties
    pub fn render(ui: &mut egui::Ui, app: &mut crate::ForgeEditorApp) {
        ui.heading("Entity Properties");
        ui.add_space(10.0);
        ui.separator();
        ui.add_space(5.0);
        
        ui.group(|ui| {
            ui.label("Entity Properties");
            ui.add_space(5.0);
            ui.label(format!("Transform Properties - Position: ({}, {})", app.transform_props.position[0], app.transform_props.position[1]));
            ui.add_space(5.0);
            
            // Mostrar sprite_path asignado
            if let Some(sprite_path) = &app.selected_entity_sprite_path {
                ui.label(format!("📄 Sprite: {}", sprite_path));
                ui.add_space(3.0);
            } else {
                ui.label("No sprite assigned");
            }
        });
    }
    
    /// Renderiza el editor de propiedades
    pub fn ui(&self, ui: &mut egui::Ui) {
        ui.heading("Entity Properties");
        ui.add_space(10.0);
        ui.separator();
        ui.add_space(5.0);
        
        ui.group(|ui| {
            ui.label("Transform");
            ui.add_space(5.0);
            ui.label("Position:");
            ui.horizontal(|ui| {
                ui.label("X:");
                ui.label("0.00");
            });
            ui.horizontal(|ui| {
                ui.label("Y:");
                ui.label("0.00");
            });
            ui.horizontal(|ui| {
                ui.label("Z:");
                ui.label("0.00");
            });
        });
    }

    pub fn update_properties(&mut self, entity_idx: usize, props: Vec<String>) {
        self.current_entity = Some(entity_idx);
        self.values = props;
    }

    pub fn change_property_type(&mut self, ty: &str) {
        self.property_type = ty.to_string();
    }
}


