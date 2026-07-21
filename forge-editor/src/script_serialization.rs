//! # Script Serialization API
//! 
//! Módulo para serialización de scripts.

use serde::{Deserialize, Serialize};

/// Serialized script
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SerializedScript {
    pub name: String,
    pub code: String,
    pub variables: Vec<SerializedVariable>,
}

/// Serialized variable
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SerializedVariable {
    pub name: String,
    pub value: serde_json::Value,
    pub ty: String,
}

/// Serialized script error
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SerializedScriptError {
    pub line: usize,
    pub message: String,
}

/// Serialized event graph
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SerializedEventGraph {
    pub nodes: Vec<SerializedEventNode>,
    pub edges: Vec<SerializedEdge>,
}

/// Serialized event node
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SerializedEventNode {
    pub id: String,
    pub ty: String,
    pub props: serde_json::Value,
}

/// Serialized edge
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SerializedEdge {
    pub from: String,
    pub to: String,
}

mod vec2_serde {
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    pub fn serialize<S>(vec: &eframe::egui::Vec2, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        [vec.x, vec.y].serialize(serializer)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<eframe::egui::Vec2, D::Error>
    where
        D: Deserializer<'de>,
    {
        let arr = <[f32; 2]>::deserialize(deserializer)?;
        Ok(eframe::egui::Vec2::new(arr[0], arr[1]))
    }
}

/// Editor config
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EditorConfig {
    pub theme: String,
    pub font_size: f32,
    #[serde(with = "vec2_serde")]
    pub window_size: egui::Vec2,
}

/// Complete project export
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompleteProjectExport {
    pub scene: serde_json::Value,
    pub scripts: Vec<SerializedScript>,
    pub events: SerializedEventGraph,
    pub config: EditorConfig,
}

