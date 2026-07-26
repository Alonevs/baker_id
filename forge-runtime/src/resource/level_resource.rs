//! Level Resource - Gestión de niveles y escenas

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;

// Importar trait Resource del módulo padre
use super::resource_manager::{Resource, ResourceType};

/// Level Resource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LevelResource {
    pub name: String,
    pub width: u32,
    pub height: u32,
    pub tiles: Vec<TileData>,
    pub entities: Vec<EntityData>,
    pub assets: Vec<String>,
    pub metadata: Option<serde_json::Value>,
}

impl LevelResource {
    /// Crea nuevo nivel
    pub fn new(name: &str, width: u32, height: u32) -> Self {
        Self {
            name: name.to_string(),
            width,
            height,
            tiles: Vec::new(),
            entities: Vec::new(),
            assets: Vec::new(),
            metadata: None,
        }
    }

    /// Crea nivel desde JSON
    pub fn from_json(name: &str, data: &serde_json::Value) -> Result<Self, String> {
        let width = data["width"]
            .as_u64()
            .map(|w| w as u32)
            .ok_or_else(|| format!("Missing width in level: {}", name))?;

        let height = data["height"]
            .as_u64()
            .map(|h| h as u32)
            .ok_or_else(|| format!("Missing height in level: {}", name))?;

        let tiles = data["tiles"]
            .as_array()
            .map(|t| t.iter().map(|tile| TileData::from_json(tile)).collect())
            .unwrap_or_default();

        let entities = data["entities"]
            .as_array()
            .map(|e| e.iter().map(|entity| EntityData::from_json(entity)).collect())
            .unwrap_or_default();

        let assets = data["assets"]
            .as_array()
            .map(|a| a.iter().map(|s| s.as_str().unwrap_or("").to_string()).collect())
            .unwrap_or_default();

        Ok(Self {
            name: name.to_string(),
            width,
            height,
            tiles,
            entities,
            assets,
            metadata: data.get("metadata").cloned(),
        })
    }

    /// Obtiene tamaño del nivel
    pub fn size(&self) -> (u32, u32) {
        (self.width, self.height)
    }

    /// Obtiene número de tiles
    pub fn tile_count(&self) -> usize {
        self.tiles.len()
    }

    /// Obtiene número de entidades
    pub fn entity_count(&self) -> usize {
        self.entities.len()
    }

    /// Obtiene lista de assets
    pub fn assets(&self) -> &[String] {
        &self.assets
    }
}

impl Default for LevelResource {
    fn default() -> Self {
        Self::new("default", 100, 100)
    }
}

/// Data de un tile
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TileData {
    pub x: u32,
    pub y: u32,
    pub tile_id: String,
    pub properties: HashMap<String, serde_json::Value>,
}

impl TileData {
    /// Crea tile desde JSON
    pub fn from_json(data: &serde_json::Value) -> Self {
        Self {
            x: data["x"]
                .as_u64()
                .map(|x| x as u32)
                .unwrap_or(0),
            y: data["y"]
                .as_u64()
                .map(|y| y as u32)
                .unwrap_or(0),
            tile_id: data["tile_id"]
                .as_str()
                .unwrap_or("")
                .to_string(),
            properties: data.get("properties")
                .and_then(|p| p.as_object())
                .map(|o| {
                    let mut map = HashMap::new();
                    for (k, v) in o {
                        map.insert(k.clone(), v.clone());
                    }
                    map
                })
                .unwrap_or_default(),
        }
    }
}

/// Data de una entidad
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntityData {
    pub id: String,
    pub name: String,
    pub position: (f32, f32),
    pub components: Vec<String>,
    pub metadata: Option<serde_json::Value>,
}

impl EntityData {
    /// Crea entidad desde JSON
    pub fn from_json(data: &serde_json::Value) -> Self {
        Self {
            id: data["id"]
                .as_str()
                .unwrap_or("")
                .to_string(),
            name: data["name"]
                .as_str()
                .unwrap_or("")
                .to_string(),
            position: (
                data["position"]["x"]
                    .as_f64()
                    .map(|x| x as f32)
                    .unwrap_or(0.0),
                data["position"]["y"]
                    .as_f64()
                    .map(|y| y as f32)
                    .unwrap_or(0.0),
            ),
            components: data["components"]
                .as_array()
                .map(|c| c.iter().map(|s| s.as_str().unwrap_or("").to_string()).collect())
                .unwrap_or_default(),
            metadata: data.get("metadata").cloned(),
        }
    }
}

/// Scene Graph para niveles
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SceneGraph {
    pub root: Option<String>,
    pub children: Vec<String>,
    pub metadata: Option<serde_json::Value>,
}

impl Default for SceneGraph {
    fn default() -> Self {
        Self {
            root: None,
            children: Vec::new(),
            metadata: None,
        }
    }
}

/// Transition de escena
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum TransitionType {
    Fade,
    Slide,
    Dissolve,
    Wipe,
}

impl TransitionType {
    /// Crea transition desde JSON
    pub fn from_json(data: &serde_json::Value) -> Self {
        match data.as_str() {
            Some("fade") | None => TransitionType::Fade,
            Some("slide") => TransitionType::Slide,
            Some("dissolve") => TransitionType::Dissolve,
            Some("wipe") => TransitionType::Wipe,
            _ => TransitionType::Fade,
        }
    }
}

// Implementación del trait Resource
impl Resource for LevelResource {
    fn name(&self) -> &str { &self.name }
    fn resource_type(&self) -> &ResourceType { &ResourceType::Level }
    fn load(&mut self) -> Result<(), String> { Ok(()) }
    fn unload(self) where Self: Sized {}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_level_resource_new() {
        let level = LevelResource::new("test", 100, 100);
        assert_eq!(level.width, 100);
        assert_eq!(level.height, 100);
        assert!(level.tiles.is_empty());
        assert!(level.entities.is_empty());
    }

    #[test]
    fn test_level_resource_default() {
        let level = LevelResource::default();
        assert_eq!(level.width, 100);
        assert_eq!(level.height, 100);
    }

    #[test]
    fn test_tile_data_from_json() {
        let json = r#"{"x": 10, "y": 20, "tile_id": "grass", "properties": {}}"#;
        let data: serde_json::Value = serde_json::from_str(json).unwrap();
        let tile = TileData::from_json(&data);
        assert_eq!(tile.x, 10);
        assert_eq!(tile.y, 20);
        assert_eq!(tile.tile_id, "grass");
    }

    #[test]
    fn test_entity_data_from_json() {
        let json = r#"{"id": "player", "name": "Player", "position": {"x": 0.0, "y": 0.0}, "components": ["transform", "renderer"]}"#;
        let data: serde_json::Value = serde_json::from_str(json).unwrap();
        let entity = EntityData::from_json(&data);
        assert_eq!(entity.id, "player");
        assert_eq!(entity.name, "Player");
        assert_eq!(entity.position, (0.0, 0.0));
    }

    #[test]
    fn test_transition_type_from_json() {
        let fade = TransitionType::from_json(&serde_json::Value::String("fade".to_string()));
        assert_eq!(fade, TransitionType::Fade);

        let slide = TransitionType::from_json(&serde_json::Value::String("slide".to_string()));
        assert_eq!(slide, TransitionType::Slide);
    }

    #[test]
    fn test_scene_graph_default() {
        let graph = SceneGraph::default();
        assert!(graph.root.is_none());
        assert!(graph.children.is_empty());
    }
}
