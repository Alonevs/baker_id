//! Property Panel - Panel de propiedades para entidades

use eframe::egui;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct EntityId(pub u64);

impl Default for EntityId {
    fn default() -> Self {
        Self(0)
    }
}

#[derive(Debug, Clone)]
pub struct PropertyPanel {
    pub selected_entity_id: Option<EntityId>,
    pub transform_properties: TransformProperties,
    pub component_properties: ComponentProperties,
    pub script_properties: ScriptProperties,
}

impl Default for PropertyPanel {
    fn default() -> Self {
        Self::new()
    }
}

impl PropertyPanel {
    pub fn new() -> Self {
        Self {
            selected_entity_id: None,
            transform_properties: TransformProperties::default(),
            component_properties: ComponentProperties::default(),
            script_properties: ScriptProperties::default(),
        }
    }
    
    /// Configura panel con entidad seleccionada
    pub fn set_entity(&mut self, entity_id: EntityId) {
        self.selected_entity_id = Some(entity_id);
    }
    
    /// Obtiene entidad seleccionada
    pub fn get_selected_entity(&self) -> Option<EntityId> {
        self.selected_entity_id.clone()
    }
    
    /// Renderiza el Property Panel en egui
    pub fn render(&mut self, ctx: &egui::Context) {
        egui::SidePanel::right("property_panel").show(ctx, |ui| {
            ui.heading("📋 Property Panel");
            ui.separator();
            
            // Seleccionar entidad
            ui.label("Entity Selection:");
            ui.horizontal(|ui| {
                let id_text = match &self.selected_entity_id {
                    Some(id) => format!("Entity #{}", id.0),
                    None => "None Selected".to_string(),
                };
                ui.label(id_text);
                if ui.button("Select Entity 1").clicked() {
                    self.set_entity(EntityId(1));
                }
            });
            
            ui.separator();
            
            // Transform Properties
            ui.heading("📐 Transform");
            ui.horizontal(|ui| {
                ui.label("X:");
                ui.add(egui::DragValue::new(&mut self.transform_properties.position.0).speed(0.5));
                ui.label("Y:");
                ui.add(egui::DragValue::new(&mut self.transform_properties.position.1).speed(0.5));
            });
            ui.horizontal(|ui| {
                ui.label("Rotation:");
                ui.add(egui::DragValue::new(&mut self.transform_properties.rotation).speed(1.0).suffix("°"));
            });
            ui.horizontal(|ui| {
                ui.label("Scale:");
                ui.add(egui::DragValue::new(&mut self.transform_properties.scale).speed(0.05));
            });
            
            ui.separator();
            
            // Component Properties
            ui.heading("🧱 Components");
            ui.horizontal(|ui| {
                ui.label("Type:");
                ui.add(egui::TextEdit::singleline(&mut self.component_properties.component_type));
            });
            
            ui.separator();
            
            // Script Properties
            ui.heading("💻 Script");
            ui.horizontal(|ui| {
                ui.label("Path:");
                ui.add(egui::TextEdit::singleline(&mut self.script_properties.script_path));
            });
            
            ui.separator();
            
            // Botones de acción
            ui.horizontal(|ui| {
                if ui.button("🔄 Reset").clicked() {
                    self.transform_properties = TransformProperties::default();
                }
                if ui.button("🗑️ Clear").clicked() {
                    self.selected_entity_id = None;
                }
            });
        });
    }
}

#[derive(Debug, Clone)]
pub struct TransformProperties {
    pub position: (f32, f32),
    pub rotation: f32,
    pub scale: f32,
}

impl Default for TransformProperties {
    fn default() -> Self {
        Self {
            position: (0.0, 0.0),
            rotation: 0.0,
            scale: 1.0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ComponentProperties {
    pub component_type: String,
    pub properties: crate::VecStringDisplay,
}

impl std::fmt::Display for ComponentProperties {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Component Properties:")?;
        for item in &self.properties {
            writeln!(f, "  {}", item)?;
        }
        Ok(())
    }
}

impl Default for ComponentProperties {
    fn default() -> Self {
        Self {
            component_type: String::new(),
            properties: crate::VecStringDisplay::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ScriptProperties {
    pub script_path: String,
    pub parameters: crate::VecStringDisplay,
}

impl std::fmt::Display for ScriptProperties {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Script Properties:")?;
        writeln!(f, "  Path: {}", self.script_path)?;
        for item in &self.parameters {
            writeln!(f, "  {}", item)?;
        }
        Ok(())
    }
}



impl Default for ScriptProperties {
    fn default() -> Self {
        Self {
            script_path: String::new(),
            parameters: crate::VecStringDisplay::new(),
        }
    }
}
