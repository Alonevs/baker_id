use std::collections::HashMap;
use std::sync::Arc;
use uuid::Uuid;
use serde_json::Value;

#[derive(Debug, Clone)]
pub struct ResourceManager {
    pub resources: HashMap<Uuid, Arc<Value>>,
    pub resource_by_name: HashMap<String, Uuid>,
}

impl ResourceManager {
    pub fn new() -> Self {
        Self { resources: HashMap::new(), resource_by_name: HashMap::new() }
    }

    pub fn add(&mut self, resource: Value) {
        let id = Uuid::new_v4();
        let resource_arc = Arc::new(resource.clone());
        self.resources.insert(id, resource_arc.clone());
        if let Some(name) = resource.get("name").and_then(|v| v.as_str()) {
            self.resource_by_name.insert(name.to_string(), id);
        }
    }

    pub fn get(&self, id: &Uuid) -> Option<Arc<Value>> {
        self.resources.get(id).cloned()
    }

    pub fn get_by_name(&self, name: &str) -> Option<Arc<Value>> {
        if let Some(&id) = self.resource_by_name.get(name) {
            self.resources.get(&id).cloned()
        } else {
            None
        }
    }
}

impl Default for ResourceManager {
    fn default() -> Self {
        Self::new()
    }
}