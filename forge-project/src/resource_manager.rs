use std::collections::HashMap;
use std::sync::Arc;
use serde_json::Value;

#[derive(Debug, Clone)]
pub struct ResourceManager {
    pub resources: HashMap<String, Arc<Value>>,
}

impl ResourceManager {
    pub fn new() -> Self {
        Self { resources: HashMap::new() }
    }

    pub fn add(&mut self, name: String, resource: Value) {
        self.resources.insert(name, Arc::new(resource));
    }

    pub fn get(&self, name: &str) -> Option<Arc<Value>> {
        self.resources.get(name).cloned()
    }
}

impl Default for ResourceManager {
    fn default() -> Self {
        Self::new()
    }
}