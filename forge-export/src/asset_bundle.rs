use std::fs;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetBundle {
    pub id: String,
    pub name: String,
    pub assets: Vec<AssetReference>,
    pub compression: String,
    pub metadata: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetReference {
    pub path: String,
    pub kind: String,
    pub size: u64,
    pub hash: String,
}

impl AssetBundle {
    pub fn new(id: String, name: String) -> Self {
        Self {
            id,
            name,
            assets: Vec::new(),
            compression: String::new(),
            metadata: String::new(),
        }
    }

    pub fn add_asset(&mut self, path: String, kind: String) {
        let size = fs::metadata(path.as_str()).map(|m| m.len()).unwrap_or(0);
        let hash = format!("{:x}", rand::random::<u128>());
        
        self.assets.push(AssetReference {
            path,
            kind,
            size,
            hash,
        });
    }

    pub fn get_asset_count(&self) -> usize {
        self.assets.len()
    }

    pub fn get_total_size(&self) -> u64 {
        self.assets.iter().map(|a| a.size).sum()
    }

    pub fn write(&self, output_path: &PathBuf) -> Result<(), String> {
        let json_path = output_path.join("bundle.json");
        let json = serde_json::to_string_pretty(self).map_err(|e| e.to_string())?;
        let write_result = fs::write(&json_path, json);
        if let Err(e) = write_result {
            return Err(e.to_string());
        }
        
        Ok(())
    }
}

impl Default for AssetBundle {
    fn default() -> Self {
        Self::new("00000000-0000-0000-0000-000000000000".to_string(), String::new())
    }
}
