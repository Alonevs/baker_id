//! forge-panel-messaging - Sistema centralizado de eventos entre paneles

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Mutex;
use uuid::Uuid;

pub type EventCallback = Box<dyn Fn(&PanelMessage) + Send + Sync>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
    pub id: Uuid,
    pub event_type: EventType,
    pub source: String,
    pub target: String,
    pub payload: serde_json::Value,
    pub timestamp: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum EventType {
    PanelOpen,
    PanelClose,
    ViewportResize,
    AssetLoaded,
    SaveRequested,
    SceneLoaded,
    EntitySelected,
    PropertyChanged,
    // LiveSync Events
    SceneLoadedSync,
    EntityAdded,
    EntityRemoved,
    TransformChanged,
    ComponentChanged,
    Custom(String),
}

pub struct EventBus {
    subscribers: Mutex<HashMap<EventType, Vec<EventCallback>>>,
}

impl EventBus {
    pub fn new() -> Self {
        Self {
            subscribers: Mutex::new(HashMap::new()),
        }
    }

    pub fn subscribe(&self, event_type: EventType, callback: EventCallback) {
        let mut subscribers = self.subscribers.lock().unwrap();
        subscribers.entry(event_type).or_insert_with(Vec::new).push(callback);
    }

    pub fn unsubscribe(&self, event_type: EventType, callback: EventCallback) {
        let mut subscribers = self.subscribers.lock().unwrap();
        if let Some(subs) = subscribers.get_mut(&event_type) {
            subs.retain(|c| !std::ptr::eq(c, &callback));
        }
    }

    pub fn publish(&self, event: Event) {
        self.broadcast(event);
    }

    pub fn broadcast(&self, event: Event) {
        let message = PanelMessage {
            panel_id: "event_bus".to_string(),
            message_type: MessageType::Notification("event".to_string()),
            data: serde_json::to_value(&event).unwrap(),
        };
        self.broadcast_message(message);
    }

    // LiveSync helpers
    pub fn publish_scene_loaded_sync(&self, scene_id: Uuid, version: u32) {
        let event = Event {
            id: Uuid::new_v4(),
            event_type: EventType::SceneLoadedSync,
            source: "live_sync_manager".to_string(),
            target: "editor".to_string(),
            payload: serde_json::json!({ "scene_id": scene_id.to_string(), "version": version }),
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_millis() as u64,
        };
        self.broadcast(event);
    }

    pub fn publish_entity_added(&self, entity_id: Uuid, entity_type: &str, scene_id: Uuid) {
        let event = Event {
            id: Uuid::new_v4(),
            event_type: EventType::EntityAdded,
            source: "live_sync_manager".to_string(),
            target: "editor".to_string(),
            payload: serde_json::json!({ "entity_id": entity_id.to_string(), "entity_type": entity_type, "scene_id": scene_id.to_string() }),
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_millis() as u64,
        };
        self.broadcast(event);
    }

    pub fn publish_entity_removed(&self, entity_id: Uuid, scene_id: Uuid) {
        let event = Event {
            id: Uuid::new_v4(),
            event_type: EventType::EntityRemoved,
            source: "live_sync_manager".to_string(),
            target: "editor".to_string(),
            payload: serde_json::json!({ "entity_id": entity_id.to_string(), "scene_id": scene_id.to_string() }),
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_millis() as u64,
        };
        self.broadcast(event);
    }

    pub fn publish_transform_changed(&self, entity_id: Uuid, transform: &serde_json::Value) {
        let event = Event {
            id: Uuid::new_v4(),
            event_type: EventType::TransformChanged,
            source: "live_sync_manager".to_string(),
            target: "editor".to_string(),
            payload: serde_json::json!({ "entity_id": entity_id.to_string(), "transform": transform }),
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_millis() as u64,
        };
        self.broadcast(event);
    }

    pub fn publish_component_changed(&self, entity_id: Uuid, component_type: &str, data: &serde_json::Value) {
        let event = Event {
            id: Uuid::new_v4(),
            event_type: EventType::ComponentChanged,
            source: "live_sync_manager".to_string(),
            target: "editor".to_string(),
            payload: serde_json::json!({ "entity_id": entity_id.to_string(), "component_type": component_type, "data": data }),
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_millis() as u64,
        };
        self.broadcast(event);
    }

    pub fn broadcast_message(&self, message: PanelMessage) {
        let subscribers = self.subscribers.lock().unwrap();
        for (_event_type, callbacks) in subscribers.iter() {
            for callback in callbacks {
                callback(&message);
            }
        }
    }
}

impl Default for EventBus {
    fn default() -> Self {
        Self::new()
    }
}

pub type PanelId = String;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PanelMessage {
    pub panel_id: PanelId,
    pub message_type: MessageType,
    pub data: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MessageType {
    Request(String),
    Response(String),
    Notification(String),
}

pub struct PanelMessenger {
    panels: Mutex<HashMap<PanelId, Vec<EventCallback>>>,
}

impl PanelMessenger {
    pub fn new() -> Self {
        Self {
            panels: Mutex::new(HashMap::new()),
        }
    }

    pub fn register_panel(&self, panel_id: PanelId) {
        let mut panels = self.panels.lock().unwrap();
        panels.entry(panel_id).or_insert_with(Vec::new);
    }

    pub fn unregister_panel(&self, panel_id: PanelId) {
        let mut panels = self.panels.lock().unwrap();
        panels.remove(&panel_id);
    }

    pub fn send_message(&self, panel_id: PanelId, message_type: MessageType, data: serde_json::Value) {
        let panels = self.panels.lock().unwrap();
        if let Some(callbacks) = panels.get(&panel_id) {
            let message = PanelMessage {
                panel_id: panel_id.clone(),
                message_type,
                data,
            };
            for callback in callbacks {
                callback(&message);
            }
        }
    }

    pub fn broadcast_message(&self, message: PanelMessage) {
        let panels = self.panels.lock().unwrap();
        for (_panel_id, callbacks) in panels.iter() {
            for callback in callbacks {
                callback(&message);
            }
        }
    }
}

impl Default for PanelMessenger {
    fn default() -> Self {
        Self::new()
    }
}
