use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct PluginRegistry {
    pub registered: HashMap<String, serde_json::Value>,
}

impl PluginRegistry {
    pub fn new() -> Self {
        Self { registered: HashMap::new() }
    }

    pub fn register(&mut self, name: String, plugin: serde_json::Value) {
        self.registered.insert(name, plugin);
    }

    pub fn is_registered(&self, name: &str) -> bool {
        self.registered.contains_key(name)
    }
}

impl Default for PluginRegistry {
    fn default() -> Self {
        Self::new()
    }
}