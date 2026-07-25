//! Timeline System Editor - Adaptador para conectar con runtime

use crate::timeline::timeline_editor::TimelineEditor;

// Stub para TimelineSystem en editor
// Se usa TimelineEditor para la UI
#[derive(Debug, Clone)]
pub struct TimelineSystem {
    pub frame: u32,
    pub is_playing: bool,
    pub playback_speed: f32,
}

impl Default for TimelineSystem {
    fn default() -> Self {
        Self::new()
    }
}

impl TimelineSystem {
    pub fn new() -> Self {
        Self {
            frame: 0,
            is_playing: false,
            playback_speed: 60.0,
        }
    }

    pub fn register_entity(&mut self, _entity_id: u64, _frame: u32) {
        // Stub
    }

    pub fn set_playing(&mut self, is_playing: bool) {
        self.is_playing = is_playing;
    }

    pub fn update(&mut self, _dt: f32) {
        // Stub
    }

    pub fn set_frame(&mut self, frame: u32) {
        self.frame = frame;
    }

    pub fn next_frame(&mut self) {
        self.frame += 1;
    }

    pub fn prev_frame(&mut self) {
        self.frame = self.frame.saturating_sub(1);
    }

    pub fn reset(&mut self) {
        self.frame = 0;
        self.is_playing = false;
    }

    pub fn is_playing(&self) -> bool {
        self.is_playing
    }

    pub fn current_frame(&self) -> u32 {
        self.frame
    }

    pub fn set_playback_speed(&mut self, speed: f32) {
        self.playback_speed = speed;
    }
}

// Re-export Timeline y TimelineEvent del module timeline_event
pub use super::timeline_event::{Timeline, TimelineEvent};
