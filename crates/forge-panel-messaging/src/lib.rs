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
