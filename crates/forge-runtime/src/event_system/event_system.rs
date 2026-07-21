//! Sistema de eventos y nodos lógicos

use serde::{Deserialize, Serialize};

/// Tipo de evento
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum EventType {
    Spawn,
    Destroy,
    MoveTo,
    ChangeState,
    TriggerDialogue,
    PlaySound,
    ChangeVariable,
}

impl Default for EventType {
    fn default() -> Self {
        EventType::Spawn
    }
}

impl std::fmt::Display for EventType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EventType::Spawn => write!(f, "Spawn"),
            EventType::Destroy => write!(f, "Destroy"),
            EventType::MoveTo => write!(f, "MoveTo"),
            EventType::ChangeState => write!(f, "ChangeState"),
            EventType::TriggerDialogue => write!(f, "TriggerDialogue"),
            EventType::PlaySound => write!(f, "PlaySound"),
            EventType::ChangeVariable => write!(f, "ChangeVariable"),
        }
    }
}

/// Nodo de evento
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct EventNode {
    pub id: u64,
    pub event_type: EventType,
    pub entity_id: Option<u64>,
    pub parameters: serde_json::Value,
    pub enabled: bool,
}

impl EventNode {
    pub fn new(event_type: EventType) -> Self {
        Self {
            id: 0,
            event_type,
            entity_id: None,
            parameters: serde_json::Value::Null,
            enabled: true,
        }
    }
}

/// Sistema de eventos
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct EventSystem {
    pub nodes: Vec<EventNode>,
    pub current_event: Option<u64>,
}

impl EventSystem {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_node(&mut self, event_type: EventType) -> u64 {
        let id = self.nodes.len() as u64;
        let node = EventNode::new(event_type);
        self.nodes.push(node);
        id
    }

    pub fn get_node(&self, id: u64) -> Option<&EventNode> {
        self.nodes.iter().find(|n| n.id == id)
    }

    pub fn get_node_mut(&mut self, id: u64) -> Option<&mut EventNode> {
        self.nodes.iter_mut().find(|n| n.id == id)
    }

    pub fn execute_event(&self, id: u64) -> Option<&EventNode> {
        self.nodes.iter().find(|n| n.id == id && n.enabled)
    }
}
