//! # Plugin Events Module
//! 
//! Sistema de eventos para plugins con:
//! - Plugin event system
//! - Event emitter
//! - Event listener
//! - Event dispatcher
//! - Event filters
//! - Event handlers
//! - Event types

use std::collections::HashMap;
use std::sync::Arc;

use crate::plugins::{PluginEvent, PluginHook, PluginId};

/// Event type
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum EventType {
    PluginLoaded,
    PluginUnloaded,
    PluginEnabled,
    PluginDisabled,
    PluginError,
    PluginConfigChanged,
    PluginDataChanged,
    PluginRequest,
    PluginResponse,
    PluginMessage,
    PluginShutdown,
    PluginUpdate,
    PluginInstall,
    PluginUninstall,
    PluginActivate,
    PluginDeactivate,
    PluginErrorOccurred,
    PluginReady,
    PluginNotReady,
}

impl From<PluginHook> for EventType {
    fn from(hook: PluginHook) -> Self {
        match hook {
            PluginHook::Init => EventType::PluginLoaded,
            PluginHook::Start => EventType::PluginReady,
            PluginHook::Shutdown => EventType::PluginShutdown,
            PluginHook::Destroy => EventType::PluginUnloaded,
            PluginHook::OnUpdate => EventType::PluginUpdate,
            PluginHook::ConfigChanged => EventType::PluginConfigChanged,
            _ => EventType::PluginMessage,
        }
    }
}

/// Event filter
#[derive(Debug, Clone)]
pub struct EventFilter {
    pub event_types: Vec<EventType>,
    pub plugin_ids: Vec<String>,
    pub hooks: Vec<PluginHook>,
    pub exclude_types: Vec<EventType>,
    pub exclude_plugins: Vec<String>,
}

impl EventFilter {
    pub fn new() -> Self {
        Self {
            event_types: Vec::new(),
            plugin_ids: Vec::new(),
            hooks: Vec::new(),
            exclude_types: Vec::new(),
            exclude_plugins: Vec::new(),
        }
    }

    /// Add event type
    pub fn add_event_type(&mut self, event_type: EventType) {
        if !self.event_types.contains(&event_type) {
            self.event_types.push(event_type);
        }
    }

    /// Add plugin ID
    pub fn add_plugin_id(&mut self, plugin_id: String) {
        if !self.plugin_ids.contains(&plugin_id) {
            self.plugin_ids.push(plugin_id);
        }
    }

    /// Add hook
    pub fn add_hook(&mut self, hook: PluginHook) {
        if !self.hooks.contains(&hook) {
            self.hooks.push(hook);
        }
    }

    /// Exclude event type
    pub fn exclude_event_type(&mut self, event_type: EventType) {
        if !self.exclude_types.contains(&event_type) {
            self.exclude_types.push(event_type);
        }
    }

    /// Exclude plugin ID
    pub fn exclude_plugin_id(&mut self, plugin_id: String) {
        if !self.exclude_plugins.contains(&plugin_id) {
            self.exclude_plugins.push(plugin_id);
        }
    }

    /// Check if event matches filter
    pub fn matches(&self, event: &PluginEvent) -> bool {
        // Check exclude types
        if !self.exclude_types.is_empty() {
            if self.exclude_types.contains(&EventType::from(event.hook)) {
                return false;
            }
        }

        // Check exclude plugins
        if !self.exclude_plugins.is_empty() {
            if self.exclude_plugins.contains(&event.plugin_id.name) {
                return false;
            }
        }

        // Check event types
        if !self.event_types.is_empty() {
            if !self.event_types.contains(&EventType::from(event.hook)) {
                return false;
            }
        }

        // Check hooks
        if !self.hooks.is_empty() {
            if !self.hooks.contains(&event.hook) {
                return false;
            }
        }

        true
    }

    /// Check if filter is empty
    pub fn is_empty(&self) -> bool {
        self.event_types.is_empty()
            && self.plugin_ids.is_empty()
            && self.hooks.is_empty()
            && self.exclude_types.is_empty()
            && self.exclude_plugins.is_empty()
    }
}

/// Event handler
pub struct EventHandler {
    pub handler: Box<dyn Fn(PluginEvent) + Send + Sync>,
    pub priority: i32,
    pub once: bool,
    pub fired: bool,
}

impl EventHandler {
    pub fn new<F>(handler: F, priority: i32, once: bool) -> Self
    where
        F: Fn(PluginEvent) + Send + Sync + 'static,
    {
        Self {
            handler: Box::new(handler),
            priority,
            once,
            fired: false,
        }
    }

    /// Execute handler
    pub fn execute(&mut self, event: &PluginEvent) {
        if self.fired {
            return;
        }

        (self.handler)(event.clone());

        if self.once {
            self.fired = true;
        }
    }

    /// Check if handler has fired
    pub fn has_fired(&self) -> bool {
        self.fired
    }

    /// Reset fired state
    pub fn reset(&mut self) {
        self.fired = false;
    }

    /// Get priority
    pub fn priority(&self) -> i32 {
        self.priority
    }

    /// Check if handler is once
    pub fn is_once(&self) -> bool {
        self.once
    }
}

impl PartialEq for EventHandler {
    fn eq(&self, other: &Self) -> bool {
        let self_ptr = self.handler.as_ref() as *const (dyn Fn(PluginEvent) + Send + Sync) as *const ();
        let other_ptr = other.handler.as_ref() as *const (dyn Fn(PluginEvent) + Send + Sync) as *const ();
        self_ptr == other_ptr && self.priority == other.priority && self.once == other.once && self.fired == other.fired
    }
}

/// Event dispatcher
pub struct EventDispatcher {
    pub listeners: HashMap<EventType, Vec<EventHandler>>,
    pub event_count: usize,
    pub processed_count: usize,
}

impl EventDispatcher {
    pub fn new() -> Self {
        Self {
            listeners: HashMap::new(),
            event_count: 0,
            processed_count: 0,
        }
    }

    /// Register listener
    pub fn register<F>(&mut self, event_type: EventType, handler: F, priority: i32, once: bool)
    where
        F: Fn(PluginEvent) + Send + Sync + 'static,
    {
        self.listeners
            .entry(event_type)
            .or_insert_with(Vec::new)
            .push(EventHandler::new(handler, priority, once));
    }

    /// Unregister listener
    pub fn unregister(&mut self, event_type: EventType, handler: &EventHandler) -> bool {
        if let Some(listeners) = self.listeners.get_mut(&event_type) {
            if let Some(pos) = listeners.iter().position(|h| h == handler) {
                listeners.remove(pos);
                self.event_count -= 1;
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    /// Emit event
    pub fn emit(&mut self, event: PluginEvent) {
        self.event_count += 1;

        if let Some(listeners) = self.listeners.get_mut(&EventType::from(event.hook)) {
            for listener in listeners {
                listener.execute(&event);
                self.processed_count += 1;
            }
        }
    }

    /// Emit filtered event
    pub fn emit_filtered(&mut self, event: PluginEvent, filter: &EventFilter) -> bool {
        if filter.matches(&event) {
            self.emit(event);
            true
        } else {
            false
        }
    }

    /// Get listener count
    pub fn listener_count(&self) -> usize {
        self.listeners.values().map(|l| l.len()).sum()
    }

    /// Get event count
    pub fn event_count(&self) -> usize {
        self.event_count
    }

    /// Get processed count
    pub fn processed_count(&self) -> usize {
        self.processed_count
    }

    /// Get listeners for event type
    pub fn get_listeners(&self, event_type: EventType) -> Option<&Vec<EventHandler>> {
        self.listeners.get(&event_type)
    }

    /// Get listeners for event type mutable
    pub fn get_listeners_mut(&mut self, event_type: EventType) -> Option<&mut Vec<EventHandler>> {
        self.listeners.get_mut(&event_type)
    }

    /// Clear all listeners
    pub fn clear(&mut self) {
        self.listeners.clear();
        self.event_count = 0;
        self.processed_count = 0;
    }

    /// Get stats
    pub fn get_stats(&self) -> HashMap<String, usize> {
        let mut stats = HashMap::new();
        stats.insert("total_listeners".to_string(), self.listener_count());
        stats.insert("total_events".to_string(), self.event_count);
        stats.insert("processed_events".to_string(), self.processed_count);

        for (event_type, listeners) in &self.listeners {
            stats.insert(format!("{:?}", event_type), listeners.len());
        }

        stats
    }
}

/// Event system
pub struct EventSystem {
    pub dispatcher: EventDispatcher,
    pub filters: Vec<EventFilter>,
    pub default_filter: Option<EventFilter>,
    pub event_history: Vec<PluginEvent>,
    pub max_history: usize,
}

impl EventSystem {
    pub fn new(max_history: usize) -> Self {
        Self {
            dispatcher: EventDispatcher::new(),
            filters: Vec::new(),
            default_filter: None,
            event_history: Vec::new(),
            max_history,
        }
    }

    /// Add event filter
    pub fn add_filter(&mut self, filter: EventFilter) {
        self.filters.push(filter);
    }

    /// Set default filter
    pub fn set_default_filter(&mut self, filter: EventFilter) {
        self.default_filter = Some(filter);
    }

    /// Emit event
    pub fn emit(&mut self, event: PluginEvent) {
        // Apply filters
        if !self.filters.is_empty() {
            let mut should_emit = true;

            for filter in &self.filters {
                if !filter.matches(&event) {
                    should_emit = false;
                    break;
                }
            }

            if !should_emit {
                return;
            }
        }

        // Apply default filter
        if let Some(ref filter) = self.default_filter {
            if !filter.matches(&event) {
                return;
            }
        }

        // Dispatch event
        self.dispatcher.emit(event.clone());

        // Add to history
        self.event_history.push(event.clone());
        if self.event_history.len() > self.max_history {
            self.event_history.remove(0);
        }
    }

    /// Emit filtered event
    pub fn emit_filtered(&mut self, event: PluginEvent, filter: &EventFilter) {
        self.dispatcher.emit_filtered(event, filter);
    }

    /// Register listener
    pub fn register<F>(&mut self, event_type: EventType, handler: F, priority: i32, once: bool)
    where
        F: Fn(PluginEvent) + Send + Sync + 'static,
    {
        self.dispatcher.register(event_type, handler, priority, once);
    }

    /// Unregister listener
    pub fn unregister(&mut self, event_type: EventType, handler: &EventHandler) -> bool {
        self.dispatcher.unregister(event_type, handler)
    }

    /// Get event history
    pub fn get_event_history(&self) -> &[PluginEvent] {
        &self.event_history
    }

    /// Get event count
    pub fn event_count(&self) -> usize {
        self.event_history.len()
    }

    /// Clear event history
    pub fn clear_history(&mut self) {
        self.event_history.clear();
    }

    /// Get dispatcher
    pub fn dispatcher(&self) -> &EventDispatcher {
        &self.dispatcher
    }

    /// Get dispatcher mutable
    pub fn dispatcher_mut(&mut self) -> &mut EventDispatcher {
        &mut self.dispatcher
    }

    /// Get filters
    pub fn get_filters(&self) -> &[EventFilter] {
        &self.filters
    }

    /// Get default filter
    pub fn default_filter(&self) -> Option<&EventFilter> {
        self.default_filter.as_ref()
    }

    /// Get stats
    pub fn get_stats(&self) -> HashMap<String, usize> {
        self.dispatcher.get_stats()
    }

    /// Clear all
    pub fn clear(&mut self) {
        self.dispatcher.clear();
        self.filters.clear();
        self.default_filter = None;
        self.event_history.clear();
    }
}

/// Event manager
pub struct EventManager {
    pub events: Vec<PluginEvent>,
    pub listeners: HashMap<String, Vec<Box<dyn Fn(PluginEvent) + Send + Sync>>>,
    pub event_count: usize,
    pub unique_event_count: usize,
}

impl EventManager {
    pub fn new() -> Self {
        Self {
            events: Vec::new(),
            listeners: HashMap::new(),
            event_count: 0,
            unique_event_count: 0,
        }
    }

    /// Add event
    pub fn add(&mut self, event: PluginEvent) {
        self.events.push(event.clone());
        self.event_count += 1;
        if !self.events.windows(2).any(|w| w[0] == event) {
            self.unique_event_count += 1;
        }
    }

    /// Register listener
    pub fn register<F>(&mut self, plugin_id: String, handler: F)
    where
        F: Fn(PluginEvent) + Send + Sync + 'static,
    {
        self.listeners
            .entry(plugin_id)
            .or_insert_with(Vec::new)
            .push(Box::new(handler));
    }

    /// Unregister listener
    pub fn unregister(&mut self, plugin_id: &str) -> usize {
        self.listeners.remove(plugin_id).map_or(0, |listeners| listeners.len())
    }

    /// Dispatch event
    pub fn dispatch(&mut self, event: PluginEvent) {
        if let Some(listeners) = self.listeners.get(&event.plugin_id.name) {
            for listener in listeners {
                listener(event.clone());
            }
        }
    }

    /// Get events
    pub fn get_events(&self) -> &[PluginEvent] {
        &self.events
    }

    /// Get event count
    pub fn event_count(&self) -> usize {
        self.event_count
    }

    /// Get unique event count
    pub fn unique_event_count(&self) -> usize {
        self.unique_event_count
    }

    /// Get listeners
    pub fn get_listeners(&self, plugin_id: &str) -> Option<&Vec<Box<dyn Fn(PluginEvent) + Send + Sync>>> {
        self.listeners.get(plugin_id)
    }

    /// Clear all
    pub fn clear(&mut self) {
        self.events.clear();
        self.listeners.clear();
        self.event_count = 0;
        self.unique_event_count = 0;
    }
}

/// Event bus
pub struct EventBus {
    pub subscribers: HashMap<String, Vec<Box<dyn Fn(PluginEvent) + Send + Sync>>>,
    pub publishers: HashMap<String, Arc<parking_lot::RwLock<Vec<Box<dyn Fn(PluginEvent) + Send + Sync>>>>>,
    pub channel_size: usize,
    pub message_queue: Vec<PluginEvent>,
}

impl EventBus {
    pub fn new(channel_size: usize) -> Self {
        Self {
            subscribers: HashMap::new(),
            publishers: HashMap::new(),
            channel_size,
            message_queue: Vec::new(),
        }
    }

    /// Subscribe to events
    pub fn subscribe<F>(&mut self, channel: String, handler: F)
    where
        F: Fn(PluginEvent) + Send + Sync + 'static,
    {
        self.subscribers
            .entry(channel)
            .or_insert_with(Vec::new)
            .push(Box::new(handler));
    }

    /// Publish event
    pub fn publish(&mut self, channel: String, event: PluginEvent) {
        if let Some(subscribers) = self.subscribers.get(&channel) {
            for subscriber in subscribers {
                subscriber(event.clone());
            }
        }
    }

    /// Get subscribers count
    pub fn subscriber_count(&self) -> usize {
        self.subscribers.values().map(|s| s.len()).sum()
    }

    /// Get message queue
    pub fn get_message_queue(&self) -> &[PluginEvent] {
        &self.message_queue
    }

    /// Clear message queue
    pub fn clear_message_queue(&mut self) {
        self.message_queue.clear();
    }

    /// Clear all
    pub fn clear(&mut self) {
        self.subscribers.clear();
        self.publishers.clear();
        self.message_queue.clear();
    }
}

/// Event types helper
pub mod event_types {
    use super::*;

    pub fn plugin_loaded() -> PluginEvent {
        PluginEvent::new(
            PluginHook::Init,
            PluginId::default_(),
            Some(serde_json::json!({ "event": "plugin_loaded" })),
        )
    }

    pub fn plugin_unloaded() -> PluginEvent {
        PluginEvent::new(
            PluginHook::Shutdown,
            PluginId::default_(),
            Some(serde_json::json!({ "event": "plugin_unloaded" })),
        )
    }

    pub fn plugin_enabled() -> PluginEvent {
        PluginEvent::new(
            PluginHook::Init,
            PluginId::default_(),
            Some(serde_json::json!({ "event": "plugin_enabled" })),
        )
    }

    pub fn plugin_disabled() -> PluginEvent {
        PluginEvent::new(
            PluginHook::Shutdown,
            PluginId::default_(),
            Some(serde_json::json!({ "event": "plugin_disabled" })),
        )
    }

    pub fn plugin_error() -> PluginEvent {
        PluginEvent::new(
            PluginHook::Init,
            PluginId::default_(),
            Some(serde_json::json!({ "event": "plugin_error" })),
        )
    }

    pub fn plugin_config_changed() -> PluginEvent {
        PluginEvent::new(
            PluginHook::ConfigChanged,
            PluginId::default_(),
            Some(serde_json::json!({ "event": "config_changed" })),
        )
    }

    pub fn plugin_data_changed() -> PluginEvent {
        PluginEvent::new(
            PluginHook::OnUpdate,
            PluginId::default_(),
            Some(serde_json::json!({ "event": "data_changed" })),
        )
    }

    pub fn plugin_request() -> PluginEvent {
        PluginEvent::new(
            PluginHook::OnInput,
            PluginId::default_(),
            Some(serde_json::json!({ "event": "request" })),
        )
    }

    pub fn plugin_response() -> PluginEvent {
        PluginEvent::new(
            PluginHook::OnInput,
            PluginId::default_(),
            Some(serde_json::json!({ "event": "response" })),
        )
    }

    pub fn plugin_message() -> PluginEvent {
        PluginEvent::new(
            PluginHook::OnUpdate,
            PluginId::default_(),
            Some(serde_json::json!({ "event": "message" })),
        )
    }

    pub fn plugin_shutdown() -> PluginEvent {
        PluginEvent::new(
            PluginHook::Shutdown,
            PluginId::default_(),
            Some(serde_json::json!({ "event": "shutdown" })),
        )
    }

    pub fn plugin_update() -> PluginEvent {
        PluginEvent::new(
            PluginHook::Init,
            PluginId::default_(),
            Some(serde_json::json!({ "event": "update" })),
        )
    }

    pub fn plugin_install() -> PluginEvent {
        PluginEvent::new(
            PluginHook::Init,
            PluginId::default_(),
            Some(serde_json::json!({ "event": "install" })),
        )
    }

    pub fn plugin_uninstall() -> PluginEvent {
        PluginEvent::new(
            PluginHook::Shutdown,
            PluginId::default_(),
            Some(serde_json::json!({ "event": "uninstall" })),
        )
    }

    pub fn plugin_activate() -> PluginEvent {
        PluginEvent::new(
            PluginHook::Start,
            PluginId::default_(),
            Some(serde_json::json!({ "event": "activate" })),
        )
    }

    pub fn plugin_deactivate() -> PluginEvent {
        PluginEvent::new(
            PluginHook::Shutdown,
            PluginId::default_(),
            Some(serde_json::json!({ "event": "deactivate" })),
        )
    }

    pub fn plugin_error_occurred() -> PluginEvent {
        PluginEvent::new(
            PluginHook::Init,
            PluginId::default_(),
            Some(serde_json::json!({ "event": "error_occurred" })),
        )
    }

    pub fn plugin_ready() -> PluginEvent {
        PluginEvent::new(
            PluginHook::Start,
            PluginId::default_(),
            Some(serde_json::json!({ "event": "ready" })),
        )
    }

    pub fn plugin_not_ready() -> PluginEvent {
        PluginEvent::new(
            PluginHook::Init,
            PluginId::default_(),
            Some(serde_json::json!({ "event": "not_ready" })),
        )
    }
}

