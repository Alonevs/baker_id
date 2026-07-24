//! Timeline Integration: conectar Editor con Runtime

use crate::timeline::{TimelineManager, TimelineEvent};
use crate::components::AnimationComponent;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};

/// Estado de reproducción en el editor
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct EditorPlaybackState {
    /// Timeline actual en el editor
    pub current_timeline: Option<Timeline>,
    /// Reproduciendo
    pub is_playing: bool,
    /// Frame actual
    pub current_frame: u32,
    /// Velocidad de reproducción
    pub playback_speed: f32,
}

/// Integración de Timeline entre Editor y Runtime
#[derive(Debug, Clone)]
pub struct TimelineIntegration {
    /// Manager de timeline principal
    pub timeline_manager: TimelineManager,
    /// Animaciones de entidades
    pub entity_animations: HashMap<u64, AnimationComponent>,
    /// Estado de reproducción del editor
    pub editor_state: EditorPlaybackState,
}

impl TimelineIntegration {
    /// Crea una nueva integración de Timeline
    pub fn new() -> Self {
        Self {
            timeline_manager: TimelineManager::new(),
            entity_animations: HashMap::new(),
            editor_state: EditorPlaybackState::default(),
        }
    }

    /// Registra una entidad con su AnimationComponent
    pub fn register_entity(&mut self, entity_id: u64, animation: AnimationComponent) {
        self.entity_animations.insert(entity_id, animation);
    }

    /// Obtiene el AnimationComponent de una entidad
    pub fn get_entity_animation(&self, entity_id: u64) -> Option<&AnimationComponent> {
        self.entity_animations.get(&entity_id)
    }

    /// Obtiene el AnimationComponent mutado de una entidad
    pub fn get_entity_animation_mut(&mut self, entity_id: u64) -> Option<&mut AnimationComponent> {
        self.entity_animations.get_mut(&entity_id)
    }

    /// Establece la timeline actual
    pub fn set_timeline(&mut self, timeline: Timeline) {
        self.editor_state.current_timeline = Some(timeline);
    }

    /// Inicia reproducción en el editor
    pub fn start_playback(&mut self) {
        self.editor_state.is_playing = true;
        self.timeline_manager.play();
    }

    /// Pausa reproducción en el editor
    pub fn pause_playback(&mut self) {
        self.editor_state.is_playing = false;
        self.timeline_manager.pause();
    }

    /// Detiene reproducción en el editor
    pub fn stop_playback(&mut self) {
        self.editor_state.is_playing = false;
        self.timeline_manager.stop();
    }

    /// Actualiza la integración con delta time
    pub fn update(&mut self, dt: f32) {
        // Actualizar timeline manager
        self.timeline_manager.update(dt);
        
        // Actualizar estado del editor si está reproduciendo
        if self.editor_state.is_playing {
            self.editor_state.current_frame += 1;
        }
    }

    /// Establece el frame actual
    pub fn set_frame(&mut self, frame: u32) {
        self.editor_state.current_frame = frame;
        self.timeline_manager.set_frame(frame);
    }

    /// Siguiente frame
    pub fn next_frame(&mut self) {
        self.editor_state.current_frame += 1;
        self.timeline_manager.next_frame();
    }

    /// Frame anterior
    pub fn prev_frame(&mut self) {
        self.editor_state.current_frame = self.editor_state.current_frame.saturating_sub(1);
        self.timeline_manager.prev_frame();
    }

    /// Establece la velocidad de reproducción
    pub fn set_playback_speed(&mut self, speed: f32) {
        self.editor_state.playback_speed = speed;
        self.timeline_manager.set_playback_speed(speed);
    }

    /// Obtiene si está reproduciendo
    pub fn is_playing(&self) -> bool {
        self.editor_state.is_playing
    }

    /// Obtiene el frame actual
    pub fn get_current_frame(&self) -> u32 {
        self.editor_state.current_frame
    }

    /// Obtiene la velocidad de reproducción
    pub fn get_playback_speed(&self) -> f32 {
        self.editor_state.playback_speed
    }

    /// Serializa el estado de integración
    pub fn serialize(&self) -> String {
        serde_json::to_string_pretty(self).unwrap_or_default()
    }

    /// Deserializa desde JSON
    pub fn deserialize(data: &str) -> Self {
        serde_json::from_str(data).unwrap_or_else(|_| Self::new())
    }
}

impl Default for TimelineIntegration {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_timeline_integration_new() {
        let integration = TimelineIntegration::new();
        assert!(!integration.is_playing());
        assert_eq!(integration.get_current_frame(), 0);
    }

    #[test]
    fn test_timeline_integration_register_entity() {
        let mut integration = TimelineIntegration::new();
        let mut animation = AnimationComponent::new();
        integration.register_entity(1, animation);
        assert!(integration.get_entity_animation(1).is_some());
        assert!(integration.get_entity_animation(2).is_none());
    }

    #[test]
    fn test_timeline_integration_playback_control() {
        let mut integration = TimelineIntegration::new();
        integration.start_playback();
        assert!(integration.is_playing());
        integration.pause_playback();
        assert!(!integration.is_playing());
        integration.stop_playback();
    }

    #[test]
    fn test_timeline_integration_frame_navigation() {
        let mut integration = TimelineIntegration::new();
        integration.set_frame(10);
        assert_eq!(integration.get_current_frame(), 10);
        integration.next_frame();
        assert_eq!(integration.get_current_frame(), 11);
        integration.prev_frame();
        assert_eq!(integration.get_current_frame(), 10);
    }

    #[test]
    fn test_timeline_integration_update() {
        let mut integration = TimelineIntegration::new();
        integration.set_frame(0);
        integration.update(0.016);
        assert_eq!(integration.get_current_frame(), 1);
    }

    #[test]
    fn test_timeline_integration_playback_speed() {
        let mut integration = TimelineIntegration::new();
        integration.set_playback_speed(1.0);
        assert_eq!(integration.get_playback_speed(), 1.0);
        integration.set_playback_speed(2.0);
        assert_eq!(integration.get_playback_speed(), 2.0);
    }

    #[test]
    fn test_timeline_integration_serialize_deserialize() {
        let integration = TimelineIntegration::new();
        let serialized = integration.serialize();
        let deserialized: TimelineIntegration = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized.get_current_frame(), integration.get_current_frame());
    }

    #[test]
    fn test_timeline_integration_default() {
        let integration = TimelineIntegration::default();
        assert!(!integration.is_playing());
        assert_eq!(integration.get_current_frame(), 0);
    }
}
