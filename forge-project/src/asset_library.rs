use std::collections::HashMap;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct AssetLibrary {
    pub assets: HashMap<String, Arc<serde_json::Value>>,
}

impl AssetLibrary {
    pub fn new() -> Self {
        Self { assets: HashMap::new() }
    }

    pub fn add(&mut self, key: String, asset: serde_json::Value) {
        self.assets.insert(key, Arc::new(asset));
    }

    pub fn get(&self, key: &str) -> Option<Arc<serde_json::Value>> {
        self.assets.get(key).cloned()
    }
}

impl Default for AssetLibrary {
    fn default() -> Self {
        Self::new()
    }
}