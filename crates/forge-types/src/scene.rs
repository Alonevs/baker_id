//! Estructuras de datos para proyecto.toml

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Entity {
    pub id: Uuid,
    pub name: String,
    pub position: (f32, f32),
    pub rotation: f32,
    pub scale: (f32, f32),
    pub layer: String,
    pub components: Vec<String>,
    pub parent: Option<Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Scene {
    pub version: String,
    pub entities: Vec<Entity>,
    pub layers: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MapFile {
    pub name: String,
    pub entities: Vec<Entity>,
    pub layers: Vec<String>,
}
