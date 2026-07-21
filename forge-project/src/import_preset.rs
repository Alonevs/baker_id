use serde_json::Value;

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct ImportPreset {
    pub name: String,
    pub extensions: Vec<String>,
    pub default_settings: Value,
}

#[allow(dead_code)]
impl ImportPreset {
    pub fn new(name: String, extensions: Vec<String>, default_settings: Value) -> Self {
        Self { name, extensions, default_settings }
    }
}

impl Default for ImportPreset {
    fn default() -> Self {
        Self {
            name: "Default".to_string(),
            extensions: Vec::new(),
            default_settings: Value::Null,
        }
    }
}