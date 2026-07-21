use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct PluginManager {
    pub plugins: HashMap<String, serde_json::Value>,
}

impl PluginManager {
    pub fn new() -> Self {
        Self { plugins: HashMap::new() }
    }

    pub fn register(&mut self, name: String, plugin: serde_json::Value) {
        self.plugins.insert(name, plugin);
    }

    pub fn get(&self, name: &str) -> Option<&serde_json::Value> {
        self.plugins.get(name)
    }
}

impl Default for PluginManager {
    fn default() -> Self {
        Self::new()
    }
}