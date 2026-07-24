//! Timeline para diálogos

use serde::{Deserialize, Serialize};

/// Evento de diálogo en la timeline
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct DialogueTimelineEvent {
    pub frame: u32,
    pub dialogue_id: u64,
    pub action: String,
}

impl DialogueTimelineEvent {
    pub fn new(frame: u32, dialogue_id: u64, action: String) -> Self {
        Self {
            frame,
            dialogue_id,
            action,
        }
    }
}

/// Timeline de diálogos
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct DialogueTimeline {
    pub events: Vec<DialogueTimelineEvent>,
    pub current_frame: u32,
}

impl DialogueTimeline {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_event(&mut self, dialogue_id: u64, action: String) {
        let event = DialogueTimelineEvent::new(
            self.current_frame,
            dialogue_id,
            action,
        );
        self.events.push(event);
    }

    pub fn get_events_at_frame(&self, frame: u32) -> Vec<&DialogueTimelineEvent> {
        self.events
            .iter()
            .filter(|e| e.frame == frame)
            .collect()
    }
}
