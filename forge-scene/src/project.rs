use std::collections::HashMap;
use std::sync::Arc;
use serde_json::Value;

#[derive(Debug, Clone)]
pub struct ProjectManager {
    pub project_data: Arc<Value>,
    pub settings: HashMap<String, Value>,
}

impl ProjectManager {
    pub fn new() -> Self {
        Self { project_data: Arc::new(Value::Null), settings: HashMap::new() }
    }

    pub fn set_setting(&mut self, key: String, value: Value) {
        self.settings.insert(key, value);
    }

    pub fn get_setting(&self, key: &str) -> Option<&Value> {
        self.settings.get(key)
    }
}
