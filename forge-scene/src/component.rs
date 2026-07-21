use uuid::Uuid;
use serde::{Deserialize, Serialize};

/// Component types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ComponentType {
    Transform,
    Collider,
    Renderer,
    Audio,
    Physics,
    Behavior,
}

/// Component represents a component in the ECS system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Component {
    pub id: Uuid,
    pub name: String,
    pub component_type: ComponentType,
    pub data: serde_json::Value,
}

impl Component {
    pub fn new(id: Uuid, name: &str, component_type: ComponentType) -> Self {
        Component {
            id,
            name: name.to_string(),
            component_type,
            data: serde_json::Value::Null,
        }
    }

    pub fn get_data(&self) -> &serde_json::Value {
        &self.data
    }

    pub fn set_data(&mut self, data: serde_json::Value) {
        self.data = data;
    }
}
