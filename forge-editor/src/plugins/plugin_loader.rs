//! # Plugin Loader Module
//! 
//! Sistema de carga de plugins con:
//! - Plugin loader
//! - Plugin discovery
//! - Plugin validation
//! - Plugin cache
//! - Plugin hot reload
//! - Plugin sandbox

use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

/// Plugin loader
#[derive(Debug, Clone)]
pub struct PluginLoader {
    pub plugins_dir: PathBuf,
    pub cache_dir: PathBuf,
    pub loaded_plugins: HashMap<String, PathBuf>,
    pub plugin_cache: HashMap<String, String>,
    pub discovery_enabled: bool,
    pub auto_load: bool,
    pub max_plugins: usize,
}

impl PluginLoader {
    pub fn new(plugins_dir: Option<PathBuf>, cache_dir: Option<PathBuf>) -> Self {
        let plugins_dir = plugins_dir.unwrap_or_else(|| {
            dirs::data_local_dir()
                .map(|p| p.join("forge-editor/plugins"))
                .unwrap_or_else(|| PathBuf::from("plugins"))
        });

        let cache_dir = cache_dir.unwrap_or_else(|| {
            dirs::data_local_dir()
                .map(|p| p.join("forge-editor/plugins"))
                .unwrap_or_else(|| PathBuf::from("cache/plugins"))
        });

        Self {
            plugins_dir,
            cache_dir,
            loaded_plugins: HashMap::new(),
            plugin_cache: HashMap::new(),
            discovery_enabled: true,
            auto_load: false,
            max_plugins: 100,
        }
    }

    /// Discover plugins in directory
    pub fn discover_plugins(&self) -> Vec<PathBuf> {
        if !self.plugins_dir.exists() {
            return Vec::new();
        }

        let mut plugins = Vec::new();

        let entries = match fs::read_dir(&self.plugins_dir) {
            Ok(e) => e,
            Err(_) => return Vec::new(),
        };
        for entry in entries {
            let entry = match entry {
                Ok(e) => e,
                Err(_) => continue,
            };

            let path = entry.path();
            if path.is_dir() {
                let manifest_path = path.join("plugin.json");
                if manifest_path.exists() {
                    plugins.push(path);
                }
            }
        }

        plugins.truncate(self.max_plugins);
        plugins
    }

    /// Load plugin from path
    pub fn load_plugin(&mut self, path: &Path) -> Result<PathBuf, String> {
        if !path.exists() {
            return Err(format!("Plugin path does not exist: {:?}", path));
        }

        let plugin_name = path
            .file_name()
            .and_then(|s| s.to_str())
            .unwrap_or("unknown")
            .to_string();

        if self.loaded_plugins.contains_key(&plugin_name) {
            return Err(format!("Plugin '{}' is already loaded", plugin_name));
        }

        // Load plugin manifest
        let manifest_path = path.join("plugin.json");
        if !manifest_path.exists() {
            return Err(format!("Plugin manifest not found: {:?}", manifest_path));
        }

        let content = fs::read_to_string(&manifest_path)
            .map_err(|e| format!("Failed to read manifest: {}", e))?;

        let plugin_hash = hash_string(&content);

        if self.plugin_cache.get(&plugin_hash).map_or(false, |h| *h == plugin_hash) {
            return Err(format!("Plugin '{}' is already loaded", plugin_name));
        }

        self.loaded_plugins.insert(plugin_name.clone(), path.to_path_buf());
        self.plugin_cache.insert(plugin_hash.clone(), plugin_hash.clone());

        Ok(path.to_path_buf())
    }

    /// Unload plugin
    pub fn unload_plugin(&mut self, plugin_name: &str) -> Result<(), String> {
        if !self.loaded_plugins.contains_key(plugin_name) {
            return Err(format!("Plugin '{}' is not loaded", plugin_name));
        }

        self.loaded_plugins.remove(plugin_name);
        self.plugin_cache.remove(&hash_string(&format!("plugin_{}", plugin_name)));

        Ok(())
    }

    /// Check if plugin is loaded
    pub fn is_loaded(&self, plugin_name: &str) -> bool {
        self.loaded_plugins.contains_key(plugin_name)
    }

    /// Get loaded plugin path
    pub fn get_plugin_path(&self, plugin_name: &str) -> Option<&PathBuf> {
        self.loaded_plugins.get(plugin_name)
    }

    /// Get all loaded plugins
    pub fn get_loaded_plugins(&self) -> Vec<&PathBuf> {
        self.loaded_plugins.values().collect()
    }

    /// Get plugin count
    pub fn plugin_count(&self) -> usize {
        self.loaded_plugins.len()
    }

    /// Enable auto-discovery
    pub fn enable_discovery(&mut self) {
        self.discovery_enabled = true;
    }

    /// Disable auto-discovery
    pub fn disable_discovery(&mut self) {
        self.discovery_enabled = false;
    }

    /// Is discovery enabled?
    pub fn is_discovery_enabled(&self) -> bool {
        self.discovery_enabled
    }

    /// Enable auto-load
    pub fn enable_auto_load(&mut self) {
        self.auto_load = true;
    }

    /// Disable auto-load
    pub fn disable_auto_load(&mut self) {
        self.auto_load = false;
    }

    /// Is auto-load enabled?
    pub fn is_auto_load_enabled(&self) -> bool {
        self.auto_load
    }

    /// Set max plugins
    pub fn set_max_plugins(&mut self, max: usize) {
        self.max_plugins = max;
    }

    /// Get cache directory
    pub fn cache_dir(&self) -> &PathBuf {
        &self.cache_dir
    }

    /// Clear cache
    pub fn clear_cache(&mut self) {
        self.plugin_cache.clear();
    }

    /// Invalidate plugin cache
    pub fn invalidate_cache(&mut self, plugin_hash: &str) {
        self.plugin_cache.remove(plugin_hash);
    }
}

/// Plugin discovery result
#[derive(Debug, Clone)]
pub struct PluginDiscoveryResult {
    pub plugins: Vec<PathBuf>,
    pub count: usize,
    pub error: Option<String>,
}

impl PluginDiscoveryResult {
    pub fn new(plugins: Vec<PathBuf>, count: usize, error: Option<String>) -> Self {
        Self {
            plugins,
            count,
            error,
        }
    }
}

/// Plugin validation result
#[derive(Debug, Clone)]
pub struct PluginValidationResult {
    pub valid: bool,
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
}

impl PluginValidationResult {
    pub fn new(valid: bool, errors: Vec<String>, warnings: Vec<String>) -> Self {
        Self {
            valid,
            errors,
            warnings,
        }
    }

    pub fn is_valid(&self) -> bool {
        self.valid
    }

    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }

    pub fn has_warnings(&self) -> bool {
        !self.warnings.is_empty()
    }
}

/// Plugin validator
#[derive(Debug, Clone)]
pub struct PluginValidator {
    pub allowed_extensions: Vec<String>,
    pub required_files: Vec<String>,
    pub max_size_mb: f64,
    pub min_version: String,
    pub max_version: String,
}

impl PluginValidator {
    pub fn new(
        allowed_extensions: Vec<String>,
        required_files: Vec<String>,
        max_size_mb: f64,
        min_version: String,
        max_version: String,
    ) -> Self {
        Self {
            allowed_extensions,
            required_files,
            max_size_mb,
            min_version,
            max_version,
        }
    }

    /// Validate plugin
    pub fn validate(&self, plugin_path: &Path) -> PluginValidationResult {
        let mut errors = Vec::new();
        let mut warnings = Vec::new();

        // Check if path exists
        if !plugin_path.exists() {
            return PluginValidationResult::new(false, vec!["Plugin path does not exist".to_string()], Vec::new());
        }

        // Check manifest
        let manifest_path = plugin_path.join("plugin.json");
        if !manifest_path.exists() {
            errors.push("Missing plugin.json manifest".to_string());
        }

        // Check required files
        for required_file in &self.required_files {
            let file_path = plugin_path.join(required_file);
            if !file_path.exists() {
                errors.push(format!("Missing required file: {}", required_file));
            }
        }

        // Check file size
        if let Ok(metadata) = fs::metadata(plugin_path) {
            let size_mb = metadata.len() as f64 / (1024.0 * 1024.0);
            if size_mb > self.max_size_mb {
                warnings.push(format!("Plugin size ({:.2}MB) exceeds recommended limit ({:.2}MB)", size_mb, self.max_size_mb));
            }
        }

        let valid = errors.is_empty();
        PluginValidationResult::new(valid, errors, warnings)
    }

    /// Validate plugin manifest
    pub fn validate_manifest(&self, manifest_content: &str) -> PluginValidationResult {
        let mut errors = Vec::new();
        let mut warnings = Vec::new();

        // Check JSON validity
        if serde_json::from_str::<serde_json::Value>(manifest_content).is_err() {
            errors.push("Invalid JSON in plugin.json".to_string());
            return PluginValidationResult::new(false, errors, warnings);
        }

        // Check required fields
        let manifest: serde_json::Value = serde_json::from_str(manifest_content).unwrap();

        if manifest.get("name").is_none() {
            errors.push("Missing 'name' field".to_string());
        }

        if manifest.get("version").is_none() {
            errors.push("Missing 'version' field".to_string());
        }

        if manifest.get("author").is_none() {
            warnings.push("Missing 'author' field (recommended)".to_string());
        }

        let valid = errors.is_empty();
        PluginValidationResult::new(valid, errors, warnings)
    }
}

/// Plugin hot reload handler
pub struct PluginHotReload {
    pub plugin_path: PathBuf,
    pub reload_callback: Option<Box<dyn Fn()>>,
    pub debounce_ms: u64,
}

impl PluginHotReload {
    pub fn new(plugin_path: PathBuf, reload_callback: Option<Box<dyn Fn()>>, debounce_ms: u64) -> Self {
        Self {
            plugin_path,
            reload_callback,
            debounce_ms,
        }
    }

    /// Check if plugin needs reload
    pub fn needs_reload(&self, current_hash: &str) -> bool {
        // Placeholder for hash comparison
        false
    }

    /// Trigger reload
    pub fn reload(&self) {
        if let Some(callback) = &self.reload_callback {
            callback();
        }
    }

    /// Get plugin path
    pub fn plugin_path(&self) -> &PathBuf {
        &self.plugin_path
    }
}

/// Plugin sandbox
#[derive(Debug, Clone)]
pub struct PluginSandbox {
    pub isolated_runtime: bool,
    pub memory_limit_mb: u64,
    pub execution_time_ms: u64,
    pub allowed_imports: Vec<String>,
    pub blocked_imports: Vec<String>,
    pub network_access: bool,
    pub file_access: Vec<String>,
}

impl PluginSandbox {
    pub fn new(
        isolated_runtime: bool,
        memory_limit_mb: u64,
        execution_time_ms: u64,
        allowed_imports: Vec<String>,
        blocked_imports: Vec<String>,
        network_access: bool,
        file_access: Vec<String>,
    ) -> Self {
        Self {
            isolated_runtime,
            memory_limit_mb,
            execution_time_ms,
            allowed_imports,
            blocked_imports,
            network_access,
            file_access,
        }
    }

    /// Check if import is allowed
    pub fn is_import_allowed(&self, import: &str) -> bool {
        if self.blocked_imports.contains(&import.to_string()) {
            false
        } else {
            self.allowed_imports.iter().any(|allowed| {
                allowed == import || import.starts_with(allowed)
            })
        }
    }

    /// Check if file access is allowed
    pub fn is_file_access_allowed(&self, path: &Path) -> bool {
        if !self.file_access.is_empty() {
            self.file_access.iter().any(|allowed| {
                path.to_string_lossy().starts_with(allowed)
            })
        } else {
            true
        }
    }

    /// Check if network access is allowed
    pub fn is_network_allowed(&self) -> bool {
        self.network_access
    }

    /// Get memory limit
    pub fn memory_limit_mb(&self) -> u64 {
        self.memory_limit_mb
    }

    /// Get execution time limit
    pub fn execution_time_ms(&self) -> u64 {
        self.execution_time_ms
    }
}

/// Plugin sandbox executor
pub struct PluginSandboxExecutor {
    pub sandbox: PluginSandbox,
    pub timeout: Option<std::time::Duration>,
    pub memory_monitor: Option<Box<dyn Fn(u64) + Send + Sync>>,
}

impl PluginSandboxExecutor {
    pub fn new(sandbox: PluginSandbox, timeout: Option<std::time::Duration>) -> Self {
        Self {
            sandbox,
            timeout,
            memory_monitor: None,
        }
    }

    /// Set memory monitor
    pub fn set_memory_monitor(&mut self, monitor: Box<dyn Fn(u64) + Send + Sync>) {
        self.memory_monitor = Some(monitor);
    }

    /// Execute code in sandbox
    pub fn execute(&self, code: &str) -> Result<String, String> {
        // Placeholder for sandbox execution
        Ok(code.to_string())
    }

    /// Check if code is allowed to execute
    pub fn is_code_allowed(&self, code: &str) -> bool {
        // Placeholder for code analysis
        true
    }
}

/// Hash string for plugin caching
fn hash_string(s: &str) -> String {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    let mut hasher = DefaultHasher::new();
    s.hash(&mut hasher);
    let hash = hasher.finish();
    format!("{:x}", hash)
}

