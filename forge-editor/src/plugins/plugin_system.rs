//! # Plugin System Module
//! 
//! Sistema completo de plugins con:
//! - Plugin manager
//! - Plugin lifecycle
//! - Plugin hooks
//! - Extension loader
//! - Plugin discovery
//! - Plugin configuration
//! - Plugin events
//! - Plugin dependencies

use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

/// Plugin identifier
#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct PluginId {
    pub name: String,
    pub version: String,
    pub author: String,
}

impl PluginId {
    pub fn new(name: String, version: String, author: String) -> Self {
        Self { name, version, author }
    }

    pub fn default_() -> Self {
        Self {
            name: String::new(),
            version: "0.0.0".to_string(),
            author: String::new(),
        }
    }
}

/// Plugin metadata
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PluginMetadata {
    pub id: PluginId,
    pub name: String,
    pub description: String,
    pub version: String,
    pub author: String,
    pub enabled: bool,
    pub loaded: bool,
    pub error: Option<String>,
    pub dependencies: Vec<String>,
    pub config: HashMap<String, String>,
}

impl PluginMetadata {
    pub fn new(
        name: String,
        description: String,
        version: String,
        author: String,
        dependencies: Vec<String>,
        config: HashMap<String, String>,
    ) -> Self {
        Self {
            id: PluginId::default_(),
            name,
            description,
            version,
            author,
            enabled: false,
            loaded: false,
            error: None,
            dependencies,
            config,
        }
    }

    pub fn with_id(mut self, id: PluginId) -> Self {
        self.id = id;
        self
    }

    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    pub fn set_loaded(&mut self, loaded: bool) {
        self.loaded = loaded;
    }

    pub fn set_error(&mut self, error: Option<String>) {
        self.error = error;
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    pub fn is_loaded(&self) -> bool {
        self.loaded
    }

    pub fn has_error(&self) -> bool {
        self.error.is_some()
    }
}

/// Plugin hook
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PluginHook {
    Init,
    Start,
    Shutdown,
    Destroy,
    BeforeCompile,
    AfterCompile,
    BeforeSave,
    AfterSave,
    BeforeLoad,
    AfterLoad,
    OnEvent,
    OnUpdate,
    OnRender,
    OnInput,
    OnFocus,
    OnBlur,
    ConfigChanged,
}

/// Plugin event
#[derive(Debug, Clone, PartialEq)]
pub struct PluginEvent {
    pub hook: PluginHook,
    pub plugin_id: PluginId,
    pub data: Option<serde_json::Value>,
    pub timestamp: u64,
}

impl PluginEvent {
    pub fn new(hook: PluginHook, plugin_id: PluginId, data: Option<serde_json::Value>) -> Self {
        Self {
            hook,
            plugin_id,
            data,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        }
    }
}

/// Plugin interface
pub trait Plugin {
    fn id(&self) -> PluginId;
    fn name(&self) -> String;
    fn description(&self) -> String;
    fn version(&self) -> String;
    fn author(&self) -> String;
    fn dependencies(&self) -> Vec<String>;
    fn config(&self) -> HashMap<String, String>;
    fn init(&mut self) -> Result<(), String>;
    fn start(&mut self) -> Result<(), String>;
    fn shutdown(&mut self);
    fn destroy(&mut self);
    fn on_event(&mut self, event: &PluginEvent) -> Option<serde_json::Value>;
    fn on_update(&mut self, dt: f64);
    fn on_render(&mut self, ctx: &mut egui::Context);
    fn on_input(&mut self, event: &egui::InputState) -> bool;
    fn on_focus(&mut self) {
        // Default: do nothing
    }
    fn on_blur(&mut self) {
        // Default: do nothing
    }
    fn config_changed(&mut self, key: &str, old_value: &str, new_value: &str);
}

/// Plugin manager
pub struct PluginManager {
    pub plugins: HashMap<PluginId, PluginMetadata>,
    pub loaded_plugins: Vec<PluginId>,
    pub enabled_plugins: Vec<PluginId>,
    pub plugin_path: PathBuf,
    pub cache_path: PathBuf,
    pub events: Vec<PluginEvent>,
    pub event_handlers: HashMap<PluginHook, Vec<Box<dyn Fn(PluginEvent)>>>,
}

impl PluginManager {
    pub fn new(plugin_path: Option<PathBuf>, cache_path: Option<PathBuf>) -> Self {
        let plugin_path = plugin_path.unwrap_or_else(|| {
            dirs::data_local_dir()
                .map(|p| p.join("forge-editor/plugins"))
                .unwrap_or_else(|| PathBuf::from("plugins"))
        });

        let cache_path = cache_path.unwrap_or_else(|| {
            dirs::data_local_dir()
                .map(|p| p.join("forge-editor/plugins"))
                .unwrap_or_else(|| PathBuf::from("cache/plugins"))
        });

        Self {
            plugins: HashMap::new(),
            loaded_plugins: Vec::new(),
            enabled_plugins: Vec::new(),
            plugin_path,
            cache_path,
            events: Vec::new(),
            event_handlers: HashMap::new(),
        }
    }

    /// Carga todos los plugins desde el directorio
    pub fn load_all(&mut self) -> Result<(), String> {
        if !self.plugin_path.exists() {
            fs::create_dir_all(&self.plugin_path)
                .map_err(|e| format!("Error creando directorio de plugins: {}", e))?;
            return Ok(());
        }

        let mut error_count = 0;

        for entry in fs::read_dir(&self.plugin_path)
            .map_err(|e| format!("Error leyendo directorio de plugins: {}", e))?
        {
            let entry = entry.map_err(|e| format!("Error leyendo entrada: {}", e))?;
            let path = entry.path();

            if path.extension().and_then(|s| s.to_str()) == Some("json") {
                if let Ok(metadata) = self.load_plugin_json(&path) {
                    self.plugins.insert(metadata.id.clone(), metadata);
                } else {
                    error_count += 1;
                }
            }
        }

        if error_count > 0 {
            eprintln!("{} plugins no pudieron cargarse", error_count);
        }

        Ok(())
    }

    /// Carga un plugin desde un archivo JSON
    fn load_plugin_json(&self, path: &Path) -> Result<PluginMetadata, String> {
        let content = fs::read_to_string(path)
            .map_err(|e| format!("Error leyendo plugin: {}", e))?;

        let metadata: PluginMetadata = serde_json::from_str(&content)
            .map_err(|e| format!("Error parseando plugin: {}", e))?;

        // Set plugin ID from path
        let plugin_name = path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("unknown")
            .to_string();

        let mut id = PluginId::new(plugin_name, metadata.version.clone(), metadata.author.clone());
        id.name = metadata.name.clone();

        Ok(PluginMetadata {
            id,
            name: metadata.name,
            description: metadata.description,
            version: metadata.version,
            author: metadata.author,
            enabled: metadata.enabled,
            loaded: metadata.loaded,
            error: metadata.error,
            dependencies: metadata.dependencies,
            config: metadata.config,
        })
    }

    /// Habilita un plugin
    pub fn enable_plugin(&mut self, plugin_id: &PluginId) -> Result<(), String> {
        if let Some(plugin) = self.plugins.get_mut(plugin_id) {
            if plugin.is_loaded() {
                return Err(format!("Plugin '{}' ya está cargado", plugin.name));
            }
            plugin.set_enabled(true);
            self.enabled_plugins.push(plugin_id.clone());
            Ok(())
        } else {
            Err(format!("Plugin '{}' no existe", plugin_id.name))
        }
    }

    /// Deshabilita un plugin
    pub fn disable_plugin(&mut self, plugin_id: &PluginId) -> Result<(), String> {
        if let Some(plugin) = self.plugins.get_mut(plugin_id) {
            plugin.set_enabled(false);
            self.enabled_plugins.retain(|id| id != plugin_id);
            Ok(())
        } else {
            Err(format!("Plugin '{}' no existe", plugin_id.name))
        }
    }

    /// Carga un plugin
    pub fn load_plugin(&mut self, plugin_id: &PluginId) -> Result<(), String> {
        // Check if plugin exists first
        if !self.plugins.contains_key(plugin_id) {
            return Err(format!("Plugin '{}' no existe", plugin_id.name));
        }

        // Get plugin metadata first (immutable borrow)
        let plugin = self.plugins.get(plugin_id).ok_or_else(|| {
            format!("Plugin '{}' no existe", plugin_id.name)
        })?;

        if plugin.is_enabled() {
            return Err(format!("Plugin '{}' está habilitado pero no cargado", plugin.name));
        }

        // Check dependencies (immutable borrow)
        for dep in &plugin.dependencies {
            if !self.plugins.contains_key(&PluginId::default_()) {
                return Err(format!("Plugin '{}' depende de '{}' que no está cargado", plugin.name, dep));
            }
        }

        // Now mutate the plugin (mutable borrow)
        if let Some(plugin_mut) = self.plugins.get_mut(plugin_id) {
            plugin_mut.set_loaded(true);
            self.loaded_plugins.push(plugin_id.clone());
            Ok(())
        } else {
            Err(format!("Plugin '{}' no existe", plugin_id.name))
        }
    }

    /// Descarga un plugin
    pub fn unload_plugin(&mut self, plugin_id: &PluginId) -> Result<(), String> {
        if let Some(plugin) = self.plugins.get_mut(plugin_id) {
            plugin.set_loaded(false);
            self.loaded_plugins.retain(|id| id != plugin_id);
            Ok(())
        } else {
            Err(format!("Plugin '{}' no existe", plugin_id.name))
        }
    }

    /// Obtiene todos los plugins
    pub fn get_all_plugins(&self) -> Vec<PluginMetadata> {
        self.plugins.values().cloned().collect()
    }

    /// Obtiene los plugins habilitados
    pub fn get_enabled_plugins(&self) -> Vec<&PluginMetadata> {
        self.enabled_plugins.iter()
            .filter_map(|id| self.plugins.get(id))
            .collect()
    }

    /// Obtiene los plugins cargados
    pub fn get_loaded_plugins(&self) -> Vec<&PluginMetadata> {
        self.loaded_plugins.iter()
            .filter_map(|id| self.plugins.get(id))
            .collect()
    }

    /// Obtiene un plugin por ID
    pub fn get_plugin(&self, plugin_id: &PluginId) -> Option<&PluginMetadata> {
        self.plugins.get(plugin_id)
    }

    /// Obtiene un plugin por ID mutable
    pub fn get_plugin_mut(&mut self, plugin_id: &PluginId) -> Option<&mut PluginMetadata> {
        self.plugins.get_mut(plugin_id)
    }

    /// Obtiene el número de plugins
    pub fn plugin_count(&self) -> usize {
        self.plugins.len()
    }

    /// Obtiene el número de plugins habilitados
    pub fn enabled_plugin_count(&self) -> usize {
        self.enabled_plugins.len()
    }

    /// Obtiene el número de plugins cargados
    pub fn loaded_plugin_count(&self) -> usize {
        self.loaded_plugins.len()
    }

    /// Obtiene los eventos
    pub fn get_events(&self) -> &[PluginEvent] {
        &self.events
    }

    /// Obtiene el número de eventos
    pub fn event_count(&self) -> usize {
        self.events.len()
    }

    /// Registra un handler de evento
    pub fn register_handler<F>(&mut self, hook: PluginHook, handler: F)
    where
        F: Fn(PluginEvent) + 'static,
    {
        self.event_handlers
            .entry(hook)
            .or_insert_with(Vec::new)
            .push(Box::new(handler));
    }

    /// Dispara un evento
    pub fn emit_event(&mut self, event: PluginEvent) {
        self.events.push(event.clone());

        if let Some(handlers) = self.event_handlers.get(&event.hook) {
            for handler in handlers {
                handler(event.clone());
            }
        }
    }

    /// Limpia eventos
    pub fn clear_events(&mut self) {
        self.events.clear();
    }

    /// Obtiene el path del plugin
    pub fn plugin_path(&self) -> &PathBuf {
        &self.plugin_path
    }

    /// Obtiene el path del cache
    pub fn cache_path(&self) -> &PathBuf {
        &self.cache_path
    }

    /// Obtiene plugins por autor
    pub fn plugins_by_author(&self, author: &str) -> Vec<&PluginMetadata> {
        self.plugins.values()
            .filter(|p| p.author == author)
            .collect()
    }

    /// Obtiene plugins por nombre
    pub fn plugins_by_name(&self, name: &str) -> Vec<&PluginMetadata> {
        self.plugins.values()
            .filter(|p| p.name == name)
            .collect()
    }

    /// Obtiene plugins habilitados por autor
    pub fn enabled_plugins_by_author(&self, author: &str) -> Vec<&PluginMetadata> {
        self.enabled_plugins.iter()
            .filter_map(|id| self.plugins.get(id))
            .filter(|p| p.author == author)
            .collect()
    }

    /// Obtiene plugins habilitados por nombre
    pub fn enabled_plugins_by_name(&self, name: &str) -> Vec<&PluginMetadata> {
        self.enabled_plugins.iter()
            .filter_map(|id| self.plugins.get(id))
            .filter(|p| p.name == name)
            .collect()
    }

    /// Obtiene plugins con error
    pub fn plugins_with_error(&self) -> Vec<&PluginMetadata> {
        self.plugins.values()
            .filter(|p| p.has_error())
            .collect()
    }

    /// Obtiene plugins sin error
    pub fn plugins_without_error(&self) -> Vec<&PluginMetadata> {
        self.plugins.values()
            .filter(|p| !p.has_error())
            .collect()
    }

    /// Obtiene plugins con dependencia no satisfecha
    pub fn plugins_with_missing_deps(&self) -> Vec<&PluginMetadata> {
        self.plugins.values()
            .filter(|p| {
                p.dependencies.iter().any(|dep| {
                    !self.plugins.iter().any(|(id, _)| {
                        id.name == *dep || id.version == *dep || id.author == *dep
                    })
                })
            })
            .collect()
    }

    /// Obtiene plugins actualizados
    pub fn get_updated_plugins(&self) -> Vec<&PluginMetadata> {
        // Placeholder for update check logic
        Vec::new()
    }

    /// Actualiza plugins
    pub fn update_plugins(&mut self) -> Result<(), String> {
        // Placeholder for update logic
        Ok(())
    }
}

impl Default for PluginManager {
    fn default() -> Self {
        Self::new(None, None)
    }
}

/// Plugin configuration
#[derive(Debug, Clone)]
pub struct PluginConfig {
    pub name: String,
    pub version: String,
    pub enabled: bool,
    pub settings: HashMap<String, serde_json::Value>,
}

impl PluginConfig {
    pub fn new(name: String, version: String) -> Self {
        Self {
            name,
            version,
            enabled: true,
            settings: HashMap::new(),
        }
    }

    pub fn set_setting(&mut self, key: String, value: serde_json::Value) {
        self.settings.insert(key, value);
    }

    pub fn get_setting(&self, key: &str) -> Option<&serde_json::Value> {
        self.settings.get(key)
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }
}

/// Plugin manifest
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PluginManifest {
    pub id: PluginId,
    pub name: String,
    pub description: String,
    pub version: String,
    pub author: String,
    pub enabled: bool,
    pub loaded: bool,
    pub error: Option<String>,
    pub dependencies: Vec<String>,
    pub config: HashMap<String, String>,
}

impl PluginManifest {
    pub fn new(
        name: String,
        description: String,
        version: String,
        author: String,
        dependencies: Vec<String>,
        config: HashMap<String, String>,
    ) -> Self {
        Self {
            id: PluginId::default_(),
            name,
            description,
            version,
            author,
            enabled: false,
            loaded: false,
            error: None,
            dependencies,
            config,
        }
    }
}

