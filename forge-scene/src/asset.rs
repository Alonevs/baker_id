use std::collections::HashMap;
use std::sync::Arc;
use serde::{Deserialize, Serialize};

/// Asset - tipo de asset
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Asset {
    pub id: String,
    pub name: String,
    pub path: String,
    pub asset_type: AssetType,
    pub size: u64,
    pub is_loaded: bool,
}

impl Asset {
    pub fn new(id: String, name: String, path: String, asset_type: AssetType) -> Self {
        Self {
            id,
            name,
            path,
            asset_type,
            size: 0,
            is_loaded: false,
        }
    }
}

/// Tipos de assets
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AssetType {
    Sprite,
    Audio,
    Script,
    Texture,
    Dialogue,
    Other,
}

#[derive(Debug, Clone)]
pub struct AssetManager {
    pub assets: HashMap<String, Arc<serde_json::Value>>,
    pub loaded_paths: HashMap<String, String>,
}

impl AssetManager {
    pub fn new() -> Self {
        Self { assets: HashMap::new(), loaded_paths: HashMap::new() }
    }

    pub fn load(&mut self, path: &str, data: serde_json::Value) {
        self.assets.insert(path.to_string(), Arc::new(data));
        self.loaded_paths.insert(path.to_string(), path.to_string());
    }

    pub fn get(&self, path: &str) -> Option<Arc<serde_json::Value>> {
        self.assets.get(path).cloned()
    }
}
