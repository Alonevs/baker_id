//! Gestor de Variables Globales

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Variable global del juego
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalVariable {
    pub id: Uuid,
    pub name: String,
    pub variable_type: VariableType,
    pub default_value: serde_json::Value,
    pub current_value: serde_json::Value,
    pub is_persistent: bool,
    pub metadata: VariableMetadata,
}

impl GlobalVariable {
    pub fn new(id: Uuid, name: impl Into<String>, variable_type: VariableType) -> Self {
        Self {
            id,
            name: name.into(),
            variable_type,
            default_value: serde_json::Value::Null,
            current_value: serde_json::Value::Null,
            is_persistent: false,
            metadata: VariableMetadata::default(),
        }
    }

    pub fn set_value(&mut self, value: impl Into<serde_json::Value>) {
        self.current_value = value.into();
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum VariableType {
    Boolean,
    Integer,
    Float,
    String,
    Vector2,
    Color,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct VariableMetadata {
    pub category: String,
    pub description: String,
    pub flags: Vec<String>,
}

/// Set de variables (plantilla reutilizable)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VariableSet {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub variables: Vec<Uuid>,
    pub is_template: bool,
}

/// Gestor de variables globales
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalVariableManager {
    pub variables: Vec<GlobalVariable>,
    pub variable_sets: Vec<VariableSet>,
    pub last_modified: String,
}

impl GlobalVariableManager {
    pub fn new() -> Self {
        Self {
            variables: Vec::new(),
            variable_sets: Vec::new(),
            last_modified: String::new(),
        }
    }

    pub fn add_variable(&mut self, name: impl Into<String>, variable_type: VariableType) -> Uuid {
        let id = Uuid::new_v4();
        let mut var = GlobalVariable::new(id, name, variable_type);
        var.set_value(serde_json::Value::Null);
        self.variables.push(var);
        id
    }

    pub fn get_variable(&self, id: &Uuid) -> Option<&GlobalVariable> {
        self.variables.iter().find(|v| v.id == *id)
    }

    pub fn get_variable_mut(&mut self, id: &Uuid) -> Option<&mut GlobalVariable> {
        self.variables.iter_mut().find(|v| v.id == *id)
    }

    pub fn set_variable_value(&mut self, id: &Uuid, value: impl Into<serde_json::Value>) {
        if let Some(var) = self.get_variable_mut(id) {
            var.set_value(value);
            self.last_modified = chrono::Utc::now().to_rfc3339();
        }
    }

    pub fn import_set(&mut self, set: VariableSet) {
        self.variable_sets.push(set);
    }

    pub fn export_set(&self, set_name: &str) -> Option<VariableSet> {
        self.variable_sets.iter().find(|s| s.name == set_name).cloned()
    }
}

/// Plantillas de variables predefinidas
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FlagSetTemplate {
    pub id: Uuid,
    pub name: String,
    pub mission_flags: Vec<MissionFlag>,
    pub inventory_flags: Vec<InventoryFlag>,
    pub world_flags: Vec<WorldFlag>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MissionFlag {
    pub name: String,
    pub default_value: bool,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InventoryFlag {
    pub name: String,
    pub item_id: String,
    pub quantity: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorldFlag {
    pub name: String,
    pub default_value: bool,
    pub description: String,
}

/// Autocompletado para variables en nodos
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VariableAutocomplete {
    pub variable_name: String,
    pub possible_values: Vec<serde_json::Value>,
    pub variable_type: VariableType,
}
