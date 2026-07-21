use std::collections::HashMap;
use std::sync::Arc;
use uuid::Uuid;
use serde_json::Value;

#[derive(Debug, Clone)]
pub struct ResourceLoader {
    pub loaded_resources: HashMap<Uuid, Arc<Value>>,
    pub loaded_by_path: HashMap<String, Uuid>,
}

impl ResourceLoader {
    pub fn new() -> Self {
        Self { loaded_resources: HashMap::new(), loaded_by_path: HashMap::new() }
    }

    pub fn load(&mut self, path: &str, resource: Value) -> Result<(), String> {
        let id = Uuid::new_v4();
        self.loaded_resources.insert(id, Arc::new(resource));
        self.loaded_by_path.insert(path.to_string(), id);
        Ok(())
    }

    pub fn get(&self, id: &Uuid) -> Option<Arc<Value>> {
        self.loaded_resources.get(id).cloned()
    }

    pub fn get_by_path(&self, path: &str) -> Option<Arc<Value>> {
        if let Some(&id) = self.loaded_by_path.get(path) {
            self.loaded_resources.get(&id).cloned()
        } else {
            None
        }
    }
}

impl Default for ResourceLoader {
    fn default() -> Self {
        Self::new()
    }
}