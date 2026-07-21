use std::collections::HashMap;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct ResourceManager {
    pub resources: HashMap<String, Arc<serde_json::Value>>,
}

impl ResourceManager {
    pub fn new() -> Self {
        Self { resources: HashMap::new() }
    }

    pub fn add(&mut self, name: String, resource: serde_json::Value) {
        self.resources.insert(name, Arc::new(resource));
    }

    pub fn get(&self, name: &str) -> Option<Arc<serde_json::Value>> {
        self.resources.get(name).cloned()
    }
}
