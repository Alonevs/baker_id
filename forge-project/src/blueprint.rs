use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct BlueprintSystem {
    pub blueprints: HashMap<String, serde_json::Value>,
}

impl BlueprintSystem {
    pub fn new() -> Self {
        Self { blueprints: HashMap::new() }
    }

    pub fn add(&mut self, name: String, blueprint: serde_json::Value) {
        self.blueprints.insert(name, blueprint);
    }

    pub fn get(&self, name: &str) -> Option<&serde_json::Value> {
        self.blueprints.get(name)
    }
}

impl Default for BlueprintSystem {
    fn default() -> Self {
        Self::new()
    }
}