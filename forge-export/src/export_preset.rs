use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Compression {
    None,
    Gzip,
    Zlib,
    Brotli,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Platform {
    Windows,
    Linux,
    Macos,
    Web,
    Android,
    Ios,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BuildSettings {
    pub optimize: bool,
    pub strip_symbols: bool,
    pub debug: bool,
    pub enable_gpu: bool,
    pub multithreaded: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Metadata {
    pub title: String,
    pub description: String,
    pub version: String,
    pub author: String,
    pub copyright: String,
    pub keywords: Vec<String>,
    pub engine_version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub struct ExportPreset {
    pub id: String,
    pub name: String,
    pub platform: Platform,
    pub compression: Compression,
    pub build_settings: BuildSettings,
    pub include_assets: Vec<String>,
    pub exclude_assets: Vec<String>,
    pub metadata: Metadata,
}

impl ExportPreset {
    pub fn new(id: String, name: String, platform: Platform) -> Self {
        Self {
            id,
            name,
            platform,
            compression: Compression::None,
            build_settings: BuildSettings::default(),
            include_assets: Vec::new(),
            exclude_assets: Vec::new(),
            metadata: Metadata::default(),
        }
    }

    pub fn with_metadata(mut self, metadata: Metadata) -> Self {
        self.metadata = metadata;
        self
    }

    pub fn get_output_path(&self) -> String {
        let platform_str = match self.platform {
            Platform::Windows => "windows",
            Platform::Linux => "linux",
            Platform::Macos => "macos",
            Platform::Web => "web",
            Platform::Android => "android",
            Platform::Ios => "ios",
        };
        format!("exports/{}/{}", platform_str, self.name)
    }
}

impl Default for ExportPreset {
    fn default() -> Self {
        Self::new("00000000-0000-0000-0000-000000000000".to_string(), String::new(), Platform::Windows)
    }
}
