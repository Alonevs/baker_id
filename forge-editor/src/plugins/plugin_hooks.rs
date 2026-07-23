//! # Plugin Hooks Module
//! 
//! Sistema de hooks para plugins con:
//! - Plugin hook manager
//! - Hook registration
//! - Hook execution
//! - Hook callbacks
//! - Hook events
//! - Hook lifecycle

use std::collections::HashMap;
use std::sync::Arc;

use crate::plugins::PluginEvent;

/// Hook callback
pub type HookCallback = Box<dyn Fn(&PluginEvent) + Send + Sync>;

/// Hook manager
pub struct HookManager {
    pub hooks: HashMap<String, Vec<HookCallback>>,
    pub hook_count: usize,
    pub callbacks_count: usize,
}

impl HookManager {
    pub fn new() -> Self {
        Self {
            hooks: HashMap::new(),
            hook_count: 0,
            callbacks_count: 0,
        }
    }

    /// Register hook callback
    pub fn register<F>(&mut self, hook_name: &str, callback: F)
    where
        F: Fn(&PluginEvent) + Send + Sync + 'static,
    {
        self.callbacks_count += 1;
        self.hooks
            .entry(hook_name.to_string())
            .or_insert_with(Vec::new)
            .push(Box::new(callback));

        if self.hooks.contains_key(hook_name) {
            self.hook_count += 1;
        }
    }

    /// Unregister hook callback
    pub fn unregister(&mut self, hook_name: &str) -> usize {
        let count = self.hooks.remove(hook_name).map_or(0, |callbacks| callbacks.len());
        if count > 0 {
            self.hook_count -= 1;
        }
        count
    }

    /// Execute hook
    pub fn execute(&self, hook_name: &str, event: &PluginEvent) -> Vec<serde_json::Value> {
        let results = if let Some(callbacks) = self.hooks.get(hook_name) {
            callbacks
                .iter()
                .map(|callback| {
                    callback(event);
                    serde_json::Value::Null
                })
                .collect()
        } else {
            Vec::new()
        };

        results
    }

    /// Get hook count
    pub fn hook_count(&self) -> usize {
        self.hook_count
    }

    /// Get callbacks count
    pub fn callbacks_count(&self) -> usize {
        self.callbacks_count
    }

    /// Get hooks
    pub fn get_hooks(&self) -> &HashMap<String, Vec<HookCallback>> {
        &self.hooks
    }

    /// Get hooks mutable
    pub fn get_hooks_mut(&mut self) -> &mut HashMap<String, Vec<HookCallback>> {
        &mut self.hooks
    }

    /// Clear all hooks
    pub fn clear(&mut self) {
        self.hooks.clear();
        self.hook_count = 0;
        self.callbacks_count = 0;
    }

    /// Check if hook exists
    pub fn has_hook(&self, hook_name: &str) -> bool {
        self.hooks.contains_key(hook_name)
    }

    /// Get hook callbacks
    pub fn get_callbacks(&self, hook_name: &str) -> Option<&Vec<HookCallback>> {
        self.hooks.get(hook_name)
    }

    /// Get hook callbacks mutable
    pub fn get_callbacks_mut(&mut self, hook_name: &str) -> Option<&mut Vec<HookCallback>> {
        self.hooks.get_mut(hook_name)
    }

    /// Remove specific callback
    pub fn remove_callback(&mut self, hook_name: &str, callback: &HookCallback) -> bool {
        if let Some(callbacks) = self.hooks.get_mut(hook_name) {
            if let Some(pos) = callbacks.iter().position(|c| {
                let p1 = c.as_ref() as *const (dyn Fn(&PluginEvent) + Send + Sync) as *const ();
                let p2 = callback.as_ref() as *const (dyn Fn(&PluginEvent) + Send + Sync) as *const ();
                p1 == p2
            }) {
                let _ = callbacks.remove(pos);
                self.callbacks_count -= 1;
                if callbacks.is_empty() {
                    self.hooks.remove(hook_name);
                    self.hook_count -= 1;
                }
                true
            } else {
                false
            }
        } else {
            false
        }
    }
}

/// Hook lifecycle manager
pub struct HookLifecycleManager {
    pub init_hooks: Vec<HookCallback>,
    pub start_hooks: Vec<HookCallback>,
    pub shutdown_hooks: Vec<HookCallback>,
    pub destroy_hooks: Vec<HookCallback>,
    pub before_compile_hooks: Vec<HookCallback>,
    pub after_compile_hooks: Vec<HookCallback>,
    pub before_save_hooks: Vec<HookCallback>,
    pub after_save_hooks: Vec<HookCallback>,
    pub before_load_hooks: Vec<HookCallback>,
    pub after_load_hooks: Vec<HookCallback>,
    pub on_event_hooks: Vec<HookCallback>,
    pub on_update_hooks: Vec<HookCallback>,
    pub on_render_hooks: Vec<HookCallback>,
    pub on_input_hooks: Vec<HookCallback>,
    pub on_focus_hooks: Vec<HookCallback>,
    pub on_blur_hooks: Vec<HookCallback>,
    pub config_changed_hooks: Vec<HookCallback>,
}

impl HookLifecycleManager {
    pub fn new() -> Self {
        Self {
            init_hooks: Vec::new(),
            start_hooks: Vec::new(),
            shutdown_hooks: Vec::new(),
            destroy_hooks: Vec::new(),
            before_compile_hooks: Vec::new(),
            after_compile_hooks: Vec::new(),
            before_save_hooks: Vec::new(),
            after_save_hooks: Vec::new(),
            before_load_hooks: Vec::new(),
            after_load_hooks: Vec::new(),
            on_event_hooks: Vec::new(),
            on_update_hooks: Vec::new(),
            on_render_hooks: Vec::new(),
            on_input_hooks: Vec::new(),
            on_focus_hooks: Vec::new(),
            on_blur_hooks: Vec::new(),
            config_changed_hooks: Vec::new(),
        }
    }

    /// Register init hook
    pub fn register_init<F>(&mut self, callback: F)
    where
        F: Fn(&PluginEvent) + Send + Sync + 'static,
    {
        self.init_hooks.push(Box::new(callback));
    }

    /// Register start hook
    pub fn register_start<F>(&mut self, callback: F)
    where
        F: Fn(&PluginEvent) + Send + Sync + 'static,
    {
        self.start_hooks.push(Box::new(callback));
    }

    /// Register shutdown hook
    pub fn register_shutdown<F>(&mut self, callback: F)
    where
        F: Fn(&PluginEvent) + Send + Sync + 'static,
    {
        self.shutdown_hooks.push(Box::new(callback));
    }

    /// Register destroy hook
    pub fn register_destroy<F>(&mut self, callback: F)
    where
        F: Fn(&PluginEvent) + Send + Sync + 'static,
    {
        self.destroy_hooks.push(Box::new(callback));
    }

    /// Register before_compile hook
    pub fn register_before_compile<F>(&mut self, callback: F)
    where
        F: Fn(&PluginEvent) + Send + Sync + 'static,
    {
        self.before_compile_hooks.push(Box::new(callback));
    }

    /// Register after_compile hook
    pub fn register_after_compile<F>(&mut self, callback: F)
    where
        F: Fn(&PluginEvent) + Send + Sync + 'static,
    {
        self.after_compile_hooks.push(Box::new(callback));
    }

    /// Register before_save hook
    pub fn register_before_save<F>(&mut self, callback: F)
    where
        F: Fn(&PluginEvent) + Send + Sync + 'static,
    {
        self.before_save_hooks.push(Box::new(callback));
    }

    /// Register after_save hook
    pub fn register_after_save<F>(&mut self, callback: F)
    where
        F: Fn(&PluginEvent) + Send + Sync + 'static,
    {
        self.after_save_hooks.push(Box::new(callback));
    }

    /// Register before_load hook
    pub fn register_before_load<F>(&mut self, callback: F)
    where
        F: Fn(&PluginEvent) + Send + Sync + 'static,
    {
        self.before_load_hooks.push(Box::new(callback));
    }

    /// Register after_load hook
    pub fn register_after_load<F>(&mut self, callback: F)
    where
        F: Fn(&PluginEvent) + Send + Sync + 'static,
    {
        self.after_load_hooks.push(Box::new(callback));
    }

    /// Register on_event hook
    pub fn register_on_event<F>(&mut self, callback: F)
    where
        F: Fn(&PluginEvent) + Send + Sync + 'static,
    {
        self.on_event_hooks.push(Box::new(callback));
    }

    /// Register on_update hook
    pub fn register_on_update<F>(&mut self, callback: F)
    where
        F: Fn(&PluginEvent) + Send + Sync + 'static,
    {
        self.on_update_hooks.push(Box::new(callback));
    }

    /// Register on_render hook
    pub fn register_on_render<F>(&mut self, callback: F)
    where
        F: Fn(&PluginEvent) + Send + Sync + 'static,
    {
        self.on_render_hooks.push(Box::new(callback));
    }

    /// Register on_input hook
    pub fn register_on_input<F>(&mut self, callback: F)
    where
        F: Fn(&PluginEvent) + Send + Sync + 'static,
    {
        self.on_input_hooks.push(Box::new(callback));
    }

    /// Register on_focus hook
    pub fn register_on_focus<F>(&mut self, callback: F)
    where
        F: Fn(&PluginEvent) + Send + Sync + 'static,
    {
        self.on_focus_hooks.push(Box::new(callback));
    }

    /// Register on_blur hook
    pub fn register_on_blur<F>(&mut self, callback: F)
    where
        F: Fn(&PluginEvent) + Send + Sync + 'static,
    {
        self.on_blur_hooks.push(Box::new(callback));
    }

    /// Register config_changed hook
    pub fn register_config_changed<F>(&mut self, callback: F)
    where
        F: Fn(&PluginEvent) + Send + Sync + 'static,
    {
        self.config_changed_hooks.push(Box::new(callback));
    }

    /// Execute init hooks
    pub fn execute_init(&self, event: &PluginEvent) {
        for callback in &self.init_hooks {
            callback(event);
        }
    }

    /// Execute start hooks
    pub fn execute_start(&self, event: &PluginEvent) {
        for callback in &self.start_hooks {
            callback(event);
        }
    }

    /// Execute shutdown hooks
    pub fn execute_shutdown(&self, event: &PluginEvent) {
        for callback in &self.shutdown_hooks {
            callback(event);
        }
    }

    /// Execute destroy hooks
    pub fn execute_destroy(&self, event: &PluginEvent) {
        for callback in &self.destroy_hooks {
            callback(event);
        }
    }

    /// Execute before_compile hooks
    pub fn execute_before_compile(&self, event: &PluginEvent) {
        for callback in &self.before_compile_hooks {
            callback(event);
        }
    }

    /// Execute after_compile hooks
    pub fn execute_after_compile(&self, event: &PluginEvent) {
        for callback in &self.after_compile_hooks {
            callback(event);
        }
    }

    /// Execute before_save hooks
    pub fn execute_before_save(&self, event: &PluginEvent) {
        for callback in &self.before_save_hooks {
            callback(event);
        }
    }

    /// Execute after_save hooks
    pub fn execute_after_save(&self, event: &PluginEvent) {
        for callback in &self.after_save_hooks {
            callback(event);
        }
    }

    /// Execute before_load hooks
    pub fn execute_before_load(&self, event: &PluginEvent) {
        for callback in &self.before_load_hooks {
            callback(event);
        }
    }

    /// Execute after_load hooks
    pub fn execute_after_load(&self, event: &PluginEvent) {
        for callback in &self.after_load_hooks {
            callback(event);
        }
    }

    /// Execute on_event hooks
    pub fn execute_on_event(&self, event: &PluginEvent) {
        for callback in &self.on_event_hooks {
            callback(event);
        }
    }

    /// Execute on_update hooks
    pub fn execute_on_update(&self, event: &PluginEvent) {
        for callback in &self.on_update_hooks {
            callback(event);
        }
    }

    /// Execute on_render hooks
    pub fn execute_on_render(&self, event: &PluginEvent) {
        for callback in &self.on_render_hooks {
            callback(event);
        }
    }

    /// Execute on_input hooks
    pub fn execute_on_input(&self, event: &PluginEvent) {
        for callback in &self.on_input_hooks {
            callback(event);
        }
    }

    /// Execute on_focus hooks
    pub fn execute_on_focus(&self, event: &PluginEvent) {
        for callback in &self.on_focus_hooks {
            callback(event);
        }
    }

    /// Execute on_blur hooks
    pub fn execute_on_blur(&self, event: &PluginEvent) {
        for callback in &self.on_blur_hooks {
            callback(event);
        }
    }

    /// Execute config_changed hooks
    pub fn execute_config_changed(&self, event: &PluginEvent) {
        for callback in &self.config_changed_hooks {
            callback(event);
        }
    }

    /// Clear all hooks
    pub fn clear(&mut self) {
        self.init_hooks.clear();
        self.start_hooks.clear();
        self.shutdown_hooks.clear();
        self.destroy_hooks.clear();
        self.before_compile_hooks.clear();
        self.after_compile_hooks.clear();
        self.before_save_hooks.clear();
        self.after_save_hooks.clear();
        self.before_load_hooks.clear();
        self.after_load_hooks.clear();
        self.on_event_hooks.clear();
        self.on_update_hooks.clear();
        self.on_render_hooks.clear();
        self.on_input_hooks.clear();
        self.on_focus_hooks.clear();
        self.on_blur_hooks.clear();
        self.config_changed_hooks.clear();
    }

    /// Get hook counts
    pub fn get_hook_counts(&self) -> HashMap<String, usize> {
        [
            ("init", &self.init_hooks),
            ("start", &self.start_hooks),
            ("shutdown", &self.shutdown_hooks),
            ("destroy", &self.destroy_hooks),
            ("before_compile", &self.before_compile_hooks),
            ("after_compile", &self.after_compile_hooks),
            ("before_save", &self.before_save_hooks),
            ("after_save", &self.after_save_hooks),
            ("before_load", &self.before_load_hooks),
            ("after_load", &self.after_load_hooks),
            ("on_event", &self.on_event_hooks),
            ("on_update", &self.on_update_hooks),
            ("on_render", &self.on_render_hooks),
            ("on_input", &self.on_input_hooks),
            ("on_focus", &self.on_focus_hooks),
            ("on_blur", &self.on_blur_hooks),
            ("config_changed", &self.config_changed_hooks),
        ]
        .iter()
        .map(|(name, hooks)| (name.to_string(), hooks.len()))
        .collect()
    }
}

/// Plugin hook registry
pub struct PluginHookRegistry {
    pub plugin_hooks: HashMap<String, HookLifecycleManager>,
    pub global_hooks: HookLifecycleManager,
    pub hook_registry: Arc<parking_lot::RwLock<HashMap<String, Vec<HookCallback>>>>,
}

impl PluginHookRegistry {
    pub fn new() -> Self {
        Self {
            plugin_hooks: HashMap::new(),
            global_hooks: HookLifecycleManager::new(),
            hook_registry: Arc::new(parking_lot::RwLock::new(HashMap::new())),
        }
    }

    /// Register plugin hooks
    pub fn register_plugin_hooks(&mut self, plugin_name: &str, manager: HookLifecycleManager) {
        self.plugin_hooks.insert(plugin_name.to_string(), manager);
    }

    /// Register global hooks
    pub fn register_global_hooks(&mut self, manager: HookLifecycleManager) {
        self.global_hooks = manager;
    }

    /// Execute plugin hooks
    pub fn execute_plugin_hooks(&self, plugin_name: &str, event: &PluginEvent) {
        if let Some(manager) = self.plugin_hooks.get(plugin_name) {
            match event.hook {
                crate::plugins::PluginHook::Init => manager.execute_init(event),
                crate::plugins::PluginHook::Start => manager.execute_start(event),
                crate::plugins::PluginHook::Shutdown => manager.execute_shutdown(event),
                crate::plugins::PluginHook::Destroy => manager.execute_destroy(event),
                crate::plugins::PluginHook::BeforeCompile => manager.execute_before_compile(event),
                crate::plugins::PluginHook::AfterCompile => manager.execute_after_compile(event),
                crate::plugins::PluginHook::BeforeSave => manager.execute_before_save(event),
                crate::plugins::PluginHook::AfterSave => manager.execute_after_save(event),
                crate::plugins::PluginHook::BeforeLoad => manager.execute_before_load(event),
                crate::plugins::PluginHook::AfterLoad => manager.execute_after_load(event),
                crate::plugins::PluginHook::OnEvent => manager.execute_on_event(event),
                crate::plugins::PluginHook::OnUpdate => manager.execute_on_update(event),
                crate::plugins::PluginHook::OnRender => manager.execute_on_render(event),
                crate::plugins::PluginHook::OnInput => manager.execute_on_input(event),
                crate::plugins::PluginHook::OnFocus => manager.execute_on_focus(event),
                crate::plugins::PluginHook::OnBlur => manager.execute_on_blur(event),
                crate::plugins::PluginHook::ConfigChanged => manager.execute_config_changed(event),
            }
        }
    }

    /// Execute global hooks
    pub fn execute_global_hooks(&self, event: &PluginEvent) {
        match event.hook {
            crate::plugins::PluginHook::Init => self.global_hooks.execute_init(event),
            crate::plugins::PluginHook::Start => self.global_hooks.execute_start(event),
            crate::plugins::PluginHook::Shutdown => self.global_hooks.execute_shutdown(event),
            crate::plugins::PluginHook::Destroy => self.global_hooks.execute_destroy(event),
            crate::plugins::PluginHook::BeforeCompile => self.global_hooks.execute_before_compile(event),
            crate::plugins::PluginHook::AfterCompile => self.global_hooks.execute_after_compile(event),
            crate::plugins::PluginHook::BeforeSave => self.global_hooks.execute_before_save(event),
            crate::plugins::PluginHook::AfterSave => self.global_hooks.execute_after_save(event),
            crate::plugins::PluginHook::BeforeLoad => self.global_hooks.execute_before_load(event),
            crate::plugins::PluginHook::AfterLoad => self.global_hooks.execute_after_load(event),
            crate::plugins::PluginHook::OnEvent => self.global_hooks.execute_on_event(event),
            crate::plugins::PluginHook::OnUpdate => self.global_hooks.execute_on_update(event),
            crate::plugins::PluginHook::OnRender => self.global_hooks.execute_on_render(event),
            crate::plugins::PluginHook::OnInput => self.global_hooks.execute_on_input(event),
            crate::plugins::PluginHook::OnFocus => self.global_hooks.execute_on_focus(event),
            crate::plugins::PluginHook::OnBlur => self.global_hooks.execute_on_blur(event),
            crate::plugins::PluginHook::ConfigChanged => self.global_hooks.execute_config_changed(event),
        }
    }

    /// Get plugin hook counts
    pub fn get_plugin_hook_counts(&self) -> HashMap<String, HashMap<String, usize>> {
        self.plugin_hooks
            .iter()
            .map(|(name, manager)| (name.clone(), manager.get_hook_counts()))
            .collect()
    }

    /// Get global hook counts
    pub fn get_global_hook_counts(&self) -> HashMap<String, usize> {
        self.global_hooks.get_hook_counts()
    }

    /// Clear all hooks
    pub fn clear(&mut self) {
        self.plugin_hooks.clear();
        self.global_hooks.clear();
    }
}

