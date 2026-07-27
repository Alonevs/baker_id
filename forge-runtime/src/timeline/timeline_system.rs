//! Timeline System - Sistema central de línea de tiempo

use crate::timeline::timeline_manager::TimelineManager;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct TimelineSystem {
    pub manager: TimelineManager,
    pub entity_frames: HashMap<u64, u32>,
}

impl Default for TimelineSystem {
    fn default() -> Self {
        Self::new()
    }
}

impl TimelineSystem {
    pub fn new() -> Self {
        Self {
            manager: TimelineManager::new(),
            entity_frames: HashMap::new(),
        }
    }

    pub fn register_entity(&mut self, entity_id: u64, frame: u32) {
        self.entity_frames.insert(entity_id, frame);
        let animation = crate::timeline::timeline_manager::AnimationComponent::new("default".to_string(), 1.0, frame);
        self.manager.register_entity(entity_id, animation);
    }

    pub fn get_entity_frame(&self, entity_id: u64) -> Option<u32> {
        self.manager.get_entity_animation(entity_id).map(|anim| anim.current_frame)
    }

    pub fn set_playing(&mut self, is_playing: bool) {
        self.manager.is_playing = is_playing;
    }

    pub fn update(&mut self, dt: f32) {
        if self.manager.is_playing {
            self.manager.timeline.next_frame();
            let frame = self.manager.timeline.current_frame;
            for entity_id in self.entity_frames.keys() {
                if let Some(anim) = self.manager.get_entity_animation_mut(*entity_id) {
                    anim.current_frame = frame;
                }
            }
        }
    }

    pub fn set_frame(&mut self, frame: u32) {
        self.manager.timeline.current_frame = frame;
        for entity_id in self.entity_frames.keys() {
            if let Some(anim) = self.manager.get_entity_animation_mut(*entity_id) {
                anim.current_frame = frame;
            }
        }
    }

    pub fn set_playback_speed(&mut self, speed: f32) {
        self.manager.playback_speed = speed;
    }

    pub fn next_frame(&mut self) {
        self.manager.timeline.next_frame();
    }

    pub fn prev_frame(&mut self) {
        self.manager.timeline.prev_frame();
    }

    pub fn reset(&mut self) {
        self.manager.timeline.reset();
    }

    pub fn is_playing(&self) -> bool {
        self.manager.is_playing()
    }

    pub fn current_frame(&self) -> u32 {
        self.manager.current_frame()
    }

    pub fn get_event_at_frame(&self, frame: u32) -> Vec<&crate::timeline::timeline::TimelineEvent> {
        self.manager.timeline.get_event_at_frame(frame)
    }
}
