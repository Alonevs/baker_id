use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct TileMapSystem {
    pub tilesets: HashMap<String, serde_json::Value>,
    pub layers: HashMap<String, serde_json::Value>,
}

impl TileMapSystem {
    pub fn new() -> Self {
        Self { tilesets: HashMap::new(), layers: HashMap::new() }
    }

    pub fn add_tileset(&mut self, name: String, tileset: serde_json::Value) {
        self.tilesets.insert(name, tileset);
    }

    pub fn add_layer(&mut self, name: String, layer: serde_json::Value) {
        self.layers.insert(name, layer);
    }
}

impl Default for TileMapSystem {
    fn default() -> Self {
        Self::new()
    }
}