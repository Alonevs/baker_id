//! # Extension Loader Module
//! 
//! Sistema de carga de extensiones con:
//! - Extension loader
//! - Extension manager
//! - Extension registry
//! - Extension activation
//! - Extension deactivation
//! - Extension state
//! - Extension updates

use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

/// Extension type
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExtensionType {
    Editor,
    Renderer,
    Tool,
    Script,
    Asset,
    Debug,
    Plugin,
    Custom,
}

impl Default for ExtensionType {
    fn default() -> Self {
        Self::Editor
    }
}

/// Extension metadata
#[derive(Debug, Clone)]
pub struct ExtensionMetadata {
    pub id: String,
    pub name: String,
    pub description: String,
    pub version: String,
    pub author: String,
    pub publisher: Option<String>,
    pub enabled: bool,
    pub installed: bool,
    pub activated: bool,
    pub error: Option<String>,
    pub extension_type: ExtensionType,
    pub dependencies: Vec<String>,
    pub contributes: HashMap<String, Vec<String>>,
    pub activation_events: Vec<String>,
    pub deactivation_events: Vec<String>,
    pub extension_uri: String,
    pub config_schema: Option<String>,
    pub manifest_version: u8,
}

impl ExtensionMetadata {
    pub fn new(
        id: String,
        name: String,
        description: String,
        version: String,
        author: String,
        dependencies: Vec<String>,
        config_schema: Option<String>,
        manifest_version: u8,
        extension_type: ExtensionType,
    ) -> Self {
        Self {
            id: id.clone(),
            name,
            description,
            version: version.clone(),
            author,
            publisher: None,
            enabled: false,
            installed: false,
            activated: false,
            error: None,
            extension_type,
            dependencies,
            contributes: HashMap::new(),
            activation_events: Vec::new(),
            deactivation_events: Vec::new(),
            extension_uri: format!("ext://{}/{}", id, version),
            config_schema,
            manifest_version,
        }
    }

    pub fn with_publisher(mut self, publisher: String) -> Self {
        self.publisher = Some(publisher);
        self
    }

    pub fn with_extension_uri(mut self, uri: String) -> Self {
        self.extension_uri = uri;
        self
    }

    pub fn with_contributions(mut self, contributes: HashMap<String, Vec<String>>) -> Self {
        self.contributes = contributes;
        self
    }

    pub fn with_activation_events(mut self, events: Vec<String>) -> Self {
        self.activation_events = events;
        self
    }

    pub fn with_deactivation_events(mut self, events: Vec<String>) -> Self {
        self.deactivation_events = events;
        self
    }

    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    pub fn set_installed(&mut self, installed: bool) {
        self.installed = installed;
    }

    pub fn set_activated(&mut self, activated: bool) {
        self.activated = activated;
    }

    pub fn set_error(&mut self, error: Option<String>) {
        self.error = error;
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    pub fn is_installed(&self) -> bool {
        self.installed
    }

    pub fn is_activated(&self) -> bool {
        self.activated
    }

    pub fn has_error(&self) -> bool {
        self.error.is_some()
    }

    pub fn get_extension_type(&self) -> &ExtensionType {
        &self.extension_type
    }

    pub fn get_contributions(&self, key: &str) -> Option<&Vec<String>> {
        self.contributes.get(key)
    }

    pub fn get_activation_events(&self) -> &Vec<String> {
        &self.activation_events
    }

    pub fn get_deactivation_events(&self) -> &Vec<String> {
        &self.deactivation_events
    }
}

/// Extension loader
#[derive(Debug, Clone)]
pub struct ExtensionLoader {
    pub extensions_dir: PathBuf,
    pub cache_dir: PathBuf,
    pub extensions: HashMap<String, ExtensionMetadata>,
    pub loaded_extensions: Vec<String>,
    pub activated_extensions: Vec<String>,
    pub extension_registry: HashMap<String, String>,
    pub auto_update: bool,
    pub update_interval_ms: u64,
    pub download_progress: HashMap<String, f64>,
}

impl ExtensionLoader {
    pub fn new(
        extensions_dir: Option<PathBuf>,
        cache_dir: Option<PathBuf>,
        auto_update: bool,
        update_interval_ms: u64,
    ) -> Self {
        let extensions_dir = extensions_dir.unwrap_or_else(|| {
            dirs::data_local_dir()
                .map(|p| p.join("forge-editor/extensions"))
                .unwrap_or_else(|| PathBuf::from("extensions"))
        });

        let cache_dir = cache_dir.unwrap_or_else(|| {
            dirs::data_local_dir()
                .map(|p| p.join("forge-editor/extensions"))
                .unwrap_or_else(|| PathBuf::from("cache/extensions"))
        });

        Self {
            extensions_dir,
            cache_dir,
            extensions: HashMap::new(),
            loaded_extensions: Vec::new(),
            activated_extensions: Vec::new(),
            extension_registry: HashMap::new(),
            auto_update,
            update_interval_ms,
            download_progress: HashMap::new(),
        }
    }

    /// Load all extensions
    pub fn load_all(&mut self) -> Result<(), String> {
        if !self.extensions_dir.exists() {
            fs::create_dir_all(&self.extensions_dir)
                .map_err(|e| format!("Error creating extensions directory: {}", e))?;
            return Ok(());
        }

        let entries = match fs::read_dir(&self.extensions_dir) {
            Ok(e) => e,
            Err(_) => return Ok(()),
        };
        for entry in entries {
            let entry = match entry {
                Ok(e) => e,
                Err(_) => continue,
            };

            let path = entry.path();
            if path.is_dir() {
                let manifest_path = path.join("extension.json");
                if manifest_path.exists() {
                    if let Ok(metadata) = self.load_extension_json(&path) {
                        self.extensions.insert(metadata.id.clone(), metadata);
                    }
                }
            }
        }

        Ok(())
    }

    /// Load extension from JSON
    fn load_extension_json(&self, path: &Path) -> Result<ExtensionMetadata, String> {
        let content = fs::read_to_string(path)
            .map_err(|e| format!("Error reading extension: {}", e))?;

        let manifest: serde_json::Value = serde_json::from_str(&content)
            .map_err(|e| format!("Error parsing extension: {}", e))?;

        let id = manifest.get("id")
            .and_then(|v| v.as_str())
            .unwrap_or("unknown")
            .to_string();

        let name = manifest.get("name")
            .and_then(|v| v.as_str())
            .unwrap_or("Unknown")
            .to_string();

        let description = manifest.get("description")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();

        let version = manifest.get("version")
            .and_then(|v| v.as_str())
            .unwrap_or("0.0.0")
            .to_string();

        let author = manifest.get("author")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();

        let publisher = manifest.get("publisher")
            .and_then(|v| v.as_str())
            .map(String::from);

        let enabled = manifest.get("enabled")
            .and_then(|v| v.as_bool())
            .unwrap_or(false);

        let installed = manifest.get("installed")
            .and_then(|v| v.as_bool())
            .unwrap_or(false);

        let activated = manifest.get("activated")
            .and_then(|v| v.as_bool())
            .unwrap_or(false);

        let error = manifest.get("error")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());

        let extension_type: ExtensionType = manifest.get("type")
            .and_then(|v| v.as_str())
            .map(|s| {
                match s.to_lowercase().as_str() {
                    "editor" => ExtensionType::Editor,
                    "renderer" => ExtensionType::Renderer,
                    "tool" => ExtensionType::Tool,
                    "script" => ExtensionType::Script,
                    "asset" => ExtensionType::Asset,
                    "debug" => ExtensionType::Debug,
                    "plugin" => ExtensionType::Plugin,
                    "custom" => ExtensionType::Custom,
                    _ => ExtensionType::Editor,
                }
            })
            .unwrap_or(ExtensionType::Editor);

        let dependencies: Vec<String> = manifest.get("dependencies")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str())
                    .map(String::from)
                    .collect()
            })
            .unwrap_or_default();

        let contributes: HashMap<String, Vec<String>> = manifest.get("contributes")
            .and_then(|v| v.as_object())
            .map(|obj| {
                obj.iter()
                    .map(|(k, v)| {
                        let values: Vec<String> = v.as_array()
                            .map(|arr| {
                                arr.iter()
                                    .filter_map(|item| item.as_str())
                                    .map(String::from)
                                    .collect()
                            })
                            .unwrap_or_default();
                        (k.clone(), values)
                    })
                    .collect()
            })
            .unwrap_or_default();

        let activation_events: Vec<String> = manifest.get("activationEvents")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str())
                    .map(String::from)
                    .collect()
            })
            .unwrap_or_default();

        let deactivation_events: Vec<String> = manifest.get("deactivationEvents")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str())
                    .map(String::from)
                    .collect()
            })
            .unwrap_or_default();

        let manifest_version = manifest.get("manifestVersion")
            .and_then(|v| v.as_u64())
            .unwrap_or(1) as u8;

        let config_schema = manifest.get("configuration")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());

        let mut metadata = ExtensionMetadata::new(
            id.clone(),
            name,
            description,
            version.clone(),
            author,
            dependencies,
            config_schema,
            manifest_version,
            extension_type,
        )
        .with_publisher(publisher.unwrap_or_default())
        .with_extension_uri(format!("ext://{}/{}", id, version))
        .with_contributions(contributes)
        .with_activation_events(activation_events)
        .with_deactivation_events(deactivation_events);

        metadata.set_enabled(enabled);
        metadata.set_installed(installed);
        metadata.set_activated(activated);
        metadata.set_error(error);

        Ok(metadata)
    }

    /// Enable extension
    pub fn enable_extension(&mut self, extension_id: &str) -> Result<(), String> {
        if let Some(ext) = self.extensions.get_mut(extension_id) {
            if ext.is_activated() {
                return Err(format!("Extension '{}' is already activated", extension_id));
            }
            ext.set_enabled(true);
            Ok(())
        } else {
            Err(format!("Extension '{}' not found", extension_id))
        }
    }

    /// Disable extension
    pub fn disable_extension(&mut self, extension_id: &str) -> Result<(), String> {
        if let Some(ext) = self.extensions.get_mut(extension_id) {
            ext.set_enabled(false);
            Ok(())
        } else {
            Err(format!("Extension '{}' not found", extension_id))
        }
    }

    /// Load extension
    pub fn load_extension(&mut self, extension_id: &str) -> Result<(), String> {
        // Check dependencies first
        let deps: Vec<String> = self.extensions.get(extension_id)
            .map(|ext| ext.dependencies.clone())
            .unwrap_or_default();
        for dep in &deps {
            if !self.extensions.contains_key(dep) {
                return Err(format!("Extension '{}' depends on '{}' which is not installed", extension_id, dep));
            }
        }

        if let Some(ext) = self.extensions.get_mut(extension_id) {
            if ext.is_enabled() {
                return Err(format!("Extension '{}' is enabled but not loaded", extension_id));
            }
            ext.set_activated(true);
            self.activated_extensions.push(extension_id.to_string());
            self.loaded_extensions.push(extension_id.to_string());

            // Register extension
            self.extension_registry.insert(extension_id.to_string(), ext.extension_uri.clone());

            Ok(())
        } else {
            Err(format!("Extension '{}' not found", extension_id))
        }
    }

    /// Unload extension
    pub fn unload_extension(&mut self, extension_id: &str) -> Result<(), String> {
        if let Some(ext) = self.extensions.get_mut(extension_id) {
            ext.set_activated(false);
            self.activated_extensions.retain(|id| id != extension_id);
            self.loaded_extensions.retain(|id| id != extension_id);
            self.extension_registry.remove(extension_id);

            Ok(())
        } else {
            Err(format!("Extension '{}' not found", extension_id))
        }
    }

    /// Get all extensions
    pub fn get_all_extensions(&self) -> Vec<&ExtensionMetadata> {
        self.extensions.values().collect()
    }

    /// Get enabled extensions
    pub fn get_enabled_extensions(&self) -> Vec<&ExtensionMetadata> {
        self.extensions.values()
            .filter(|e| e.is_enabled())
            .collect()
    }

    /// Get activated extensions
    pub fn get_activated_extensions(&self) -> Vec<&ExtensionMetadata> {
        self.extensions.values()
            .filter(|e| e.is_activated())
            .collect()
    }

    /// Get extension by ID
    pub fn get_extension(&self, extension_id: &str) -> Option<&ExtensionMetadata> {
        self.extensions.get(extension_id)
    }

    /// Get extension by ID mutable
    pub fn get_extension_mut(&mut self, extension_id: &str) -> Option<&mut ExtensionMetadata> {
        self.extensions.get_mut(extension_id)
    }

    /// Get extension count
    pub fn extension_count(&self) -> usize {
        self.extensions.len()
    }

    /// Get enabled extension count
    pub fn enabled_extension_count(&self) -> usize {
        self.extensions.values().filter(|e| e.is_enabled()).count()
    }

    /// Get activated extension count
    pub fn activated_extension_count(&self) -> usize {
        self.extensions.values().filter(|e| e.is_activated()).count()
    }

    /// Get extensions by type
    pub fn get_extensions_by_type(&self, extension_type: ExtensionType) -> Vec<&ExtensionMetadata> {
        self.extensions.values()
            .filter(|e| e.extension_type == extension_type)
            .collect()
    }

    /// Get extensions by author
    pub fn get_extensions_by_author(&self, author: &str) -> Vec<&ExtensionMetadata> {
        self.extensions.values()
            .filter(|e| e.author == author)
            .collect()
    }

    /// Get extensions by publisher
    pub fn get_extensions_by_publisher(&self, publisher: &str) -> Vec<&ExtensionMetadata> {
        self.extensions.values()
            .filter(|e| e.publisher.as_deref() == Some(publisher))
            .collect()
    }

    /// Get extensions with errors
    pub fn get_extensions_with_errors(&self) -> Vec<&ExtensionMetadata> {
        self.extensions.values()
            .filter(|e| e.has_error())
            .collect()
    }

    /// Get download progress
    pub fn get_download_progress(&self, extension_id: &str) -> Option<f64> {
        self.download_progress.get(extension_id).copied()
    }

    /// Set download progress
    pub fn set_download_progress(&mut self, extension_id: &str, progress: f64) {
        self.download_progress.insert(extension_id.to_string(), progress);
    }

    /// Clear download progress
    pub fn clear_download_progress(&mut self) {
        self.download_progress.clear();
    }

    /// Get extensions directory
    pub fn extensions_dir(&self) -> &PathBuf {
        &self.extensions_dir
    }

    /// Get cache directory
    pub fn cache_dir(&self) -> &PathBuf {
        &self.cache_dir
    }

    /// Is auto-update enabled?
    pub fn is_auto_update_enabled(&self) -> bool {
        self.auto_update
    }

    /// Get update interval
    pub fn update_interval_ms(&self) -> u64 {
        self.update_interval_ms
    }

    /// Set auto-update
    pub fn set_auto_update(&mut self, enabled: bool) {
        self.auto_update = enabled;
    }

    /// Set update interval
    pub fn set_update_interval(&mut self, interval_ms: u64) {
        self.update_interval_ms = interval_ms;
    }
}

/// Extension registry
#[derive(Debug, Clone)]
pub struct ExtensionRegistry {
    pub extensions: HashMap<String, ExtensionMetadata>,
    pub registry_url: String,
    pub last_sync: Option<u64>,
    pub sync_interval_ms: u64,
    pub available_extensions: Vec<ExtensionMetadata>,
    pub installed_extensions: Vec<String>,
}

impl ExtensionRegistry {
    pub fn new(registry_url: Option<String>, sync_interval_ms: u64) -> Self {
        let registry_url = registry_url.unwrap_or_else(|| {
            "https://extensions.forge-editor.com".to_string()
        });

        Self {
            extensions: HashMap::new(),
            registry_url,
            last_sync: None,
            sync_interval_ms,
            available_extensions: Vec::new(),
            installed_extensions: Vec::new(),
        }
    }

    /// Sync with registry
    pub fn sync(&mut self) -> Result<(), String> {
        // Placeholder for registry sync
        Ok(())
    }

    /// Register extension
    pub fn register_extension(&mut self, extension: ExtensionMetadata) {
        self.extensions.insert(extension.id.clone(), extension);
    }

    /// Get available extensions
    pub fn get_available_extensions(&self) -> Vec<&ExtensionMetadata> {
        self.available_extensions.iter().collect()
    }

    /// Get installed extensions
    pub fn get_installed_extensions(&self) -> Vec<&str> {
        self.installed_extensions.iter().map(|s| s.as_str()).collect()
    }

    /// Get last sync time
    pub fn last_sync(&self) -> Option<u64> {
        self.last_sync
    }

    /// Get sync interval
    pub fn sync_interval_ms(&self) -> u64 {
        self.sync_interval_ms
    }

    /// Get registry URL
    pub fn registry_url(&self) -> &str {
        &self.registry_url
    }
}

/// Extension update check
#[derive(Debug, Clone)]
pub struct ExtensionUpdateCheck {
    pub extension_id: String,
    pub current_version: String,
    pub latest_version: String,
    pub is_available: bool,
    pub update_url: Option<String>,
    pub release_notes: Option<String>,
}

impl ExtensionUpdateCheck {
    pub fn new(
        extension_id: String,
        current_version: String,
        latest_version: String,
        is_available: bool,
        update_url: Option<String>,
        release_notes: Option<String>,
    ) -> Self {
        Self {
            extension_id,
            current_version,
            latest_version,
            is_available,
            update_url,
            release_notes,
        }
    }

    /// Check if update is available
    pub fn is_update_available(&self) -> bool {
        self.is_available && self.current_version != self.latest_version
    }

    /// Get update URL
    pub fn update_url(&self) -> Option<&str> {
        self.update_url.as_deref()
    }

    /// Get release notes
    pub fn release_notes(&self) -> Option<&str> {
        self.release_notes.as_deref()
    }
}

