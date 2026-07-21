use serde::{Deserialize, Serialize};
use crate::{ExportPreset, Platform, Compression, BuildSettings, Metadata};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportPresetSaveData {
    pub id: String,
    pub name: String,
    pub platform: Platform,
    pub compression: Compression,
    pub build_settings: BuildSettings,
    pub include_assets: Vec<String>,
    pub exclude_assets: Vec<String>,
    pub metadata: Metadata,
}

impl ExportPresetSaveData {
    pub fn to_preset(&self) -> ExportPreset {
        ExportPreset {
            id: self.id.clone(),
            name: self.name.clone(),
            platform: self.platform,
            compression: self.compression,
            build_settings: self.build_settings.clone(),
            include_assets: self.include_assets.clone(),
            exclude_assets: self.exclude_assets.clone(),
            metadata: self.metadata.clone(),
        }
    }
}
