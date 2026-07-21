use std::collections::HashMap;
use std::sync::Arc;
use serde_json::Value;

#[derive(Debug, Clone)]
pub struct RenderSystem {
    pub renderers: HashMap<String, Arc<Value>>,
}

impl RenderSystem {
    pub fn new() -> Self {
        Self { renderers: HashMap::new() }
    }

    pub fn add(&mut self, name: String, renderer: Value) {
        self.renderers.insert(name, Arc::new(renderer));
    }

    pub fn get(&self, name: &str) -> Option<Arc<Value>> {
        self.renderers.get(name).cloned()
    }
}

impl Default for RenderSystem {
    fn default() -> Self {
        Self::new()
    }
}