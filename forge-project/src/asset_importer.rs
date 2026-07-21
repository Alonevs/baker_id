use std::path::PathBuf;
use serde_json::Value;

#[derive(Debug, Clone)]
pub struct AssetImporter {
    pub current_path: PathBuf,
}

impl AssetImporter {
    pub fn new() -> Self {
        Self { current_path: PathBuf::from(".") }
    }

    pub fn import(&mut self, _path: &str, _data: Value) -> Result<(), String> {
        Ok(())
    }
}

impl Default for AssetImporter {
    fn default() -> Self {
        Self::new()
    }
}