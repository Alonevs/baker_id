use std::collections::HashMap;
use uuid::Uuid;
use serde_json::Value;

#[derive(Debug, Clone)]
pub struct ResourceSaver {
    pub last_saved: Option<Value>,
    pub saved_resources: HashMap<Uuid, String>,
}

impl ResourceSaver {
    pub fn new() -> Self {
        Self { last_saved: None, saved_resources: HashMap::new() }
    }

    pub fn save(&mut self, resource: &Value) -> Result<(), String> {
        self.last_saved = Some(resource.clone());
        Ok(())
    }
}

impl Default for ResourceSaver {
    fn default() -> Self {
        Self::new()
    }
}