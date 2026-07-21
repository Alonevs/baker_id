//! Eventos para diálogos

use serde::{Deserialize, Serialize};

/// Tipo de evento de diálogo
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum DialogueEventType {
    Start,
    End,
    ShowLine,
    PlaySound,
    ShowCharacter,
}

impl Default for DialogueEventType {
    fn default() -> Self {
        DialogueEventType::Start
    }
}

impl std::fmt::Display for DialogueEventType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DialogueEventType::Start => write!(f, "Start"),
            DialogueEventType::End => write!(f, "End"),
            DialogueEventType::ShowLine => write!(f, "ShowLine"),
            DialogueEventType::PlaySound => write!(f, "PlaySound"),
            DialogueEventType::ShowCharacter => write!(f, "ShowCharacter"),
        }
    }
}

/// Evento de diálogo
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct DialogueEvent {
    pub id: u64,
    pub event_type: DialogueEventType,
    pub dialogue_id: u64,
    pub parameters: serde_json::Value,
}

impl DialogueEvent {
    pub fn new(event_type: DialogueEventType, dialogue_id: u64) -> Self {
        Self {
            id: 0,
            event_type,
            dialogue_id,
            parameters: serde_json::Value::Null,
        }
    }
}

/// Sistema de eventos de diálogo
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct DialogueEventSystem {
    pub events: Vec<DialogueEvent>,
    pub current_event: Option<u64>,
}

impl DialogueEventSystem {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_event(&mut self, event_type: DialogueEventType, dialogue_id: u64) -> u64 {
        let id = self.events.len() as u64;
        let event = DialogueEvent::new(event_type, dialogue_id);
        self.events.push(event);
        id
    }

    pub fn get_event(&self, id: u64) -> Option<&DialogueEvent> {
        self.events.iter().find(|e| e.id == id)
    }

    pub fn get_event_mut(&mut self, id: u64) -> Option<&mut DialogueEvent> {
        self.events.iter_mut().find(|e| e.id == id)
    }
}
