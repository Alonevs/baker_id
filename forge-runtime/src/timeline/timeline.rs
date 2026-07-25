//! Timeline y Sequencer del juego

use serde::{Deserialize, Serialize};


/// Evento en la timeline
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TimelineEvent {
    pub id: u64,
    pub frame: u32,
    pub entity_id: u64,
    pub action: String,
    pub value: f32,
}

impl Default for TimelineEvent {
    fn default() -> Self {
        Self {
            id: 0,
            frame: 0,
            entity_id: 0,
            action: String::new(),
            value: 0.0,
        }
    }
}

/// LÃ­nea de tiempo del juego
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct Timeline {
    pub events: Vec<TimelineEvent>,
    pub current_frame: u32,
    pub is_playing: bool,
}

impl Timeline {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_event(&mut self, entity_id: u64, action: String, value: f32) -> u64 {
        let id = self.events.len() as u64;
        let event = TimelineEvent {
            id,
            frame: self.current_frame,
            entity_id,
            action,
            value,
        };
        self.events.push(event);
        id
    }

    pub fn get_event_at_frame(&self, frame: u32) -> Vec<&TimelineEvent> {
        self.events
            .iter()
            .filter(|e| e.frame == frame)
            .collect()
    }

    pub fn next_frame(&mut self) {
        self.current_frame += 1;
    }

    pub fn prev_frame(&mut self) {
        self.current_frame = self.current_frame.saturating_sub(1);
    }

    pub fn reset(&mut self) {
        self.current_frame = 0;
        self.is_playing = false;
    }
}



