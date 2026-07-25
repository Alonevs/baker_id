//! Timeline Integration - Conexión entre editor y runtime

use crate::timeline::timeline_system::TimelineSystem;
use crate::components::AnimationComponent;

#[derive(Debug, Clone, Default)]
pub struct EditorPlaybackState {
    pub current_timeline: Option<TimelineSystem>,
    pub is_playing: bool,
    pub current_frame: u32,
    pub playback_speed: f32,
}

#[derive(Debug, Clone)]
pub struct TimelineIntegration {
    pub editor_state: EditorPlaybackState,
    pub timeline_manager: TimelineSystem,
}

impl TimelineIntegration {
    pub fn new() -> Self {
        Self {
            editor_state: EditorPlaybackState::default(),
            timeline_manager: TimelineSystem::new(),
        }
    }
    
    pub fn register_entity(&mut self, runtime_id: u64, _animation: AnimationComponent) {
        self.timeline_manager.register_entity(runtime_id, 0);
    }
    
    pub fn get_entity_animation(&self, _runtime_id: u64) -> Option<&AnimationComponent> {
        None
    }
    
    pub fn start_playback(&mut self) {
        self.editor_state.is_playing = true;
        self.timeline_manager.set_playing(true);
    }
    
    pub fn pause_playback(&mut self) {
        self.editor_state.is_playing = false;
        self.timeline_manager.set_playing(false);
    }
    
    pub fn stop_playback(&mut self) {
        self.editor_state.is_playing = false;
        self.timeline_manager.set_playing(false);
        self.timeline_manager.manager.reset();
    }
    
    pub fn update(&mut self, dt: f32) {
        self.editor_state.current_frame += 1;
        self.timeline_manager.update(dt);
    }
    
    pub fn set_frame(&mut self, frame: u32) {
        self.editor_state.current_frame = frame;
        self.timeline_manager.manager.set_frame(frame);
    }
    
    pub fn next_frame(&mut self) {
        self.editor_state.current_frame += 1;
        self.timeline_manager.manager.next_frame();
    }
    
    pub fn prev_frame(&mut self) {
        self.editor_state.current_frame = self.editor_state.current_frame.saturating_sub(1);
        self.timeline_manager.manager.prev_frame();
    }
    
    pub fn set_playback_speed(&mut self, speed: f32) {
        self.editor_state.playback_speed = speed;
        self.timeline_manager.manager.playback_speed = speed;
    }
    
    pub fn is_playing(&self) -> bool {
        self.editor_state.is_playing
    }
    
    pub fn get_current_frame(&self) -> u32 {
        self.editor_state.current_frame
    }
    
    pub fn get_playback_speed(&self) -> f32 {
        self.editor_state.playback_speed
    }
    
    pub fn serialize(&self) -> Vec<u8> {
        Vec::new()
    }
    
    pub fn deserialize(_data: &[u8]) -> Self {
        Self::new()
    }
}

