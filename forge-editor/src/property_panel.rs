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
pub struct PropertyPanel;

impl PropertyPanel {
    pub fn new() -> Self {
        Self
    }
    
    pub fn show(&self, ctx: &egui::Context, ui: &mut egui::Ui, entity_id: &EntityId) {
        ui.label("Property Panel");
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
    pub properties: HashMap<String, String>,
}

impl Default for ComponentProperties {
    fn default() -> Self {
        Self {
            component_type: String::new(),
            properties: HashMap::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ScriptProperties {
    pub script_path: String,
    pub parameters: HashMap<String, String>,
}

impl Default for ScriptProperties {
    fn default() -> Self {
        Self {
            script_path: String::new(),
            parameters: HashMap::new(),
        }
    }
}
