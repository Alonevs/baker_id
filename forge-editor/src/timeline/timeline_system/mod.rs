//! Timeline System del editor

use crate::timeline::{Timeline, TimelineEvent};
use crate::components::AnimationComponent;
use std::collections::HashMap;

/// Sistema de Timeline del editor
#[derive(Debug, Clone)]
pub struct TimelineSystem {
    pub manager: Timeline,
    pub active_entities: HashMap<u64, u64>,
    pub runtime_frame: u32,
}

impl Default for TimelineSystem {
    fn default() -> Self {
        Self {
            manager: Timeline::new(),
            active_entities: HashMap::new(),
            runtime_frame: 0,
        }
    }
}

impl TimelineSystem {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register_entity(&mut self, runtime_id: u64, editor_id: u64) {
        self.active_entities.insert(runtime_id, editor_id);
    }

    pub fn get_editor_entity(&self, runtime_id: u64) -> Option<u64> {
        self.active_entities.get(&runtime_id).copied()
    }

    pub fn sync_animations(&mut self) {
    }

    pub fn update(&mut self, dt: f32) {
        if self.manager.is_playing {
            self.manager.current_frame += 1;
        }
        
        let frame_duration = (self.manager.current_frame as f32 + 1.0) *
            self.manager.playback_speed;
        if dt >= frame_duration {
            self.runtime_frame += 1;
            self.apply_frame_events(self.runtime_frame);
        }
    }

    fn apply_frame_events(&mut self, frame: u32) {
    }

    pub fn set_playing(&mut self, playing: bool) {
        self.manager.is_playing = playing;
    }

    pub fn is_playing(&self) -> bool {
        self.manager.is_playing
    }

    pub fn set_playback_speed(&mut self, speed: f32) {
        self.manager.playback_speed = speed;
    }
}
