use std::path::PathBuf;
use std::collections::HashMap;
use uuid::Uuid;
use serde::{Serialize, Deserialize};
use thiserror::Error;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Resource {
    pub id: Uuid,
    pub name: String,
    pub path: Option<PathBuf>,
    pub data: serde_json::Value,
    pub metadata: HashMap<String, String>,
    pub is_loading: bool,
    pub is_dirty: bool,
}

impl Resource {
    pub fn new(name: &str) -> Self {
        Self {
            id: Uuid::new_v4(),
            name: name.to_string(),
            path: None,
            data: serde_json::Value::Null,
            metadata: HashMap::new(),
            is_loading: false,
            is_dirty: false,
        }
    }

    pub fn set_data(&mut self, data: serde_json::Value) {
        self.data = data;
        self.is_dirty = true;
    }

    pub fn get_data(&self) -> Option<&serde_json::Value> {
        if self.is_loading {
            None
        } else {
            Some(&self.data)
        }
    }

    pub fn save(&self, path: &PathBuf) -> Result<(), SaveError> {
        let content = serde_json::to_string_pretty(&self.data)?;
        std::fs::write(path, content)?;
        Ok(())
    }

    pub fn load(path: &PathBuf) -> Result<Self, LoadError> {
        let content = std::fs::read_to_string(path)?;
        let resource: Resource = serde_json::from_str(&content)?;
        Ok(resource)
    }
}

#[derive(Debug, Error)]
pub enum SaveError {
    #[error("Error saving resource: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),
}

#[derive(Debug, Error)]
pub enum LoadError {
    #[error("Error loading resource: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Deserialization error: {0}")]
    DeserializationError(#[from] serde_json::Error),
    #[error("Invalid resource format")]
    InvalidFormat,
}
