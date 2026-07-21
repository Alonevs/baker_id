//! forge-map-cart - Formato RON para archivos .map

use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MapFile {
    pub name: String,
    pub version: String,
    pub width: u32,
    pub height: u32,
    pub layers: Vec<Layer>,
    pub entities: Vec<Entity>,
    pub metadata: serde_json::Map<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Layer {
    pub id: String,
    pub name: String,
    pub tiles: Vec<Tile>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tile {
    pub x: i32,
    pub y: i32,
    pub tile_id: String,
    pub properties: serde_json::Map<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Entity {
    pub id: String,
    pub name: String,
    pub position: (f32, f32),
    pub rotation: f32,
    pub scale: (f32, f32),
    pub layer: String,
    pub components: Vec<Component>,
    pub parent: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Component {
    pub name: String,
    pub properties: serde_json::Map<String, serde_json::Value>,
}

pub struct MapParser;

impl MapParser {
    pub fn parse(path: &Path) -> Result<MapFile, String> {
        let content = fs::read_to_string(path)
            .map_err(|e| format!("Failed to read file: {}", e))?;
        
        serde_json::from_str(&content)
            .map_err(|e| format!("Failed to parse JSON: {}", e))
    }

    pub fn save(&self, map: &MapFile, path: &Path) -> Result<(), String> {
        let content = serde_json::to_string_pretty(map)
            .map_err(|e| format!("Failed to serialize: {}", e))?;
        
        fs::write(path, content)
            .map_err(|e| format!("Failed to write file: {}", e))
    }
}
