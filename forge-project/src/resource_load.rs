use std::sync::Arc;
use serde_json::Value;

#[derive(Debug, Clone)]
pub struct ResourceLoader {
    pub loaded_resources: HashMap<String, Arc<Value>>,
}

impl ResourceLoader {
    pub fn new() -> Self {
        Self { loaded_resources: HashMap::new() }
    }

    pub fn load(&mut self, path: &str, resource: Value) -> Result<(), String> {
        self.loaded_resources.insert(path.to_string(), Arc::new(resource));
        Ok(())
    }

    pub fn get(&self, path: &str) -> Option<Arc<Value>> {
        self.loaded_resources.get(path).cloned()
    }
}

impl Default for ResourceLoader {
    fn default() -> Self {
        Self::new()
    }
}