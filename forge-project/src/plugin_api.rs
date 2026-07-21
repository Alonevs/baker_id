use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct PluginApi {
    pub functions: HashMap<String, serde_json::Value>,
}

impl PluginApi {
    pub fn new() -> Self {
        Self { functions: HashMap::new() }
    }

    pub fn register_function(&mut self, name: String, func: serde_json::Value) {
        self.functions.insert(name, func);
    }

    pub fn get_function(&self, name: &str) -> Option<&serde_json::Value> {
        self.functions.get(name)
    }
}

impl Default for PluginApi {
    fn default() -> Self {
        Self::new()
    }
}