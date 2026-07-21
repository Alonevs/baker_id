use std::path::PathBuf;
use serde_json::Value;

#[derive(Debug, Clone)]
pub struct AssetExporter {
    pub current_path: PathBuf,
}

impl AssetExporter {
    pub fn new() -> Self {
        Self { current_path: PathBuf::from(".") }
    }

    pub fn export(&mut self, _path: &str, _data: &Value) -> Result<(), String> {
        Ok(())
    }
}

impl Default for AssetExporter {
    fn default() -> Self {
        Self::new()
    }
}