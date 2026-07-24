//! Timeline Manager para conectar Timeline Editor con Runtime

use crate::components::AnimationComponent;
use crate::timeline::Timeline;
use std::collections::HashMap;
use uuid::Uuid;

/// Manager de Timeline que conecta el editor con el runtime
#[derive(Debug, Clone)]
pub struct TimelineManager {
    /// Timeline actual
    pub timeline: Timeline,
    /// Mapa de entidades a sus AnimationComponents
    pub entity_animations: HashMap<u64, AnimationComponent>,
    /// Animaciones cargadas desde editor
    pub loaded_animations: HashMap<String, Vec<crate::animation::Keyframe>>,
    /// Estado de reproducción
    pub is_playing: bool,
    /// Velocidad de reproducción
    pub playback_speed: f32,
}

impl Default for TimelineManager {
    fn default() -> Self {
        Self {
            timeline: Timeline::default(),
            entity_animations: HashMap::new(),
            loaded_animations: HashMap::new(),
            is_playing: false,
            playback_speed: 1.0,
        }
    }
}

impl TimelineManager {
    /// Crea un nuevo TimelineManager
    pub fn new() -> Self {
        Self::default()
    }

    /// Registra un AnimationComponent para una entidad
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

    /// Obtiene los eventos en el frame actual
    pub fn get_events_at_frame(&self, frame: u32) -> Vec<&TimelineEvent> {
        self.timeline.get_event_at_frame(frame)
    }

    /// Actualiza el manager con delta time
    pub fn update(&mut self, dt: f32) {
        // Actualizar timeline
        if self.is_playing {
            let frame_duration = self.timeline.frame_duration as f32 * self.playback_speed;
            self.timeline.current_frame += 1;
            
            // Aplicar eventos del frame actual
            self.apply_frame_events(self.timeline.current_frame);
        }

        // Actualizar animaciones de entidades
        for (_, animation) in self.entity_animations.iter_mut() {
            animation.update(dt);
        }
    }

    /// Aplica los eventos del frame actual
    fn apply_frame_events(&self, frame: u32) {
        let events = self.get_events_at_frame(frame);
        for event in events {
            if let Some(entity_anim) = self.entity_animations.get_mut(&event.entity_id) {
                // Aplicar acción del evento
                match event.action.as_str() {
                    "play" => {
                        entity_anim.play(&event.action);
                    }
                    "stop" => {
                        entity_anim.stop();
                    }
                    "pause" => {
                        entity_anim.pause();
                    }
                    "set_value" => {
                        if let Some(anim) = entity_anim.get_animation_mut(&event.action) {
                            // Actualizar keyframe con nuevo valor
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    /// Inicia la reproducción
    pub fn play(&mut self) {
        self.is_playing = true;
        self.timeline.is_playing = true;
    }

    /// Pausa la reproducción
    pub fn pause(&mut self) {
        self.is_playing = false;
        self.timeline.is_playing = false;
    }

    /// Detiene la reproducción
    pub fn stop(&mut self) {
        self.is_playing = false;
        self.timeline.is_playing = false;
        self.timeline.reset();
    }

    /// Avanza al siguiente frame
    pub fn next_frame(&mut self) {
        self.timeline.next_frame();
    }

    /// Retrocede al frame anterior
    pub fn prev_frame(&mut self) {
        self.timeline.prev_frame();
    }

    /// Establece el frame actual
    pub fn set_frame(&mut self, frame: u32) {
        self.timeline.current_frame = frame;
        // Aplicar eventos del frame
        self.apply_frame_events(frame);
    }

    /// Cambia la velocidad de reproducción
    pub fn set_playback_speed(&mut self, speed: f32) {
        self.playback_speed = speed;
    }

    /// Carga una animación desde el editor
    pub fn load_animation(&mut self, name: &str, keyframes: Vec<crate::animation::Keyframe>) {
        self.loaded_animations.insert(name.to_string(), keyframes);
    }

    /// Obtiene las animaciones cargadas
    pub fn get_loaded_animations(&self) -> &HashMap<String, Vec<crate::animation::Keyframe>> {
        &self.loaded_animations
    }

    /// Serializa el estado del TimelineManager
    pub fn serialize(&self) -> String {
        serde_json::to_string_pretty(self).unwrap_or_default()
    }

    /// Deserializa desde JSON
    pub fn deserialize(data: &str) -> Self {
        serde_json::from_str(data).unwrap_or_else(|_| Self::default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_timeline_manager_with_animation_component() {
        let mut manager = TimelineManager::new();
        let mut animation = AnimationComponent::new();
        manager.register_entity(1, animation);
        manager.play();
        manager.update(0.016);
        assert_eq!(manager.timeline.current_frame, 1);
        manager.stop();
        assert!(!manager.is_playing);
    }

    #[test]
    fn test_timeline_manager_frame_navigation() {
        let mut manager = TimelineManager::new();
        manager.set_frame(10);
        assert_eq!(manager.timeline.current_frame, 10);
        manager.next_frame();
        assert_eq!(manager.timeline.current_frame, 11);
        manager.prev_frame();
        assert_eq!(manager.timeline.current_frame, 10);
    }

    #[test]
    fn test_timeline_manager_playback_speed() {
        let mut manager = TimelineManager::new();
        manager.play();
        manager.set_playback_speed(1.0);
        manager.update(0.016);
        assert_eq!(manager.timeline.current_frame, 1);
        manager.set_playback_speed(2.0);
        manager.update(0.016);
        assert_eq!(manager.timeline.current_frame, 3);
        manager.stop();
    }

    #[test]
    fn test_timeline_manager_multiple_entities() {
        let mut manager = TimelineManager::new();
        let mut anim1 = AnimationComponent::new();
        let mut anim2 = AnimationComponent::new();
        manager.register_entity(1, anim1);
        manager.register_entity(2, anim2);
        manager.play();
        manager.update(0.016);
        assert!(manager.get_entity_animation(1).is_some());
        assert!(manager.get_entity_animation(2).is_some());
        manager.stop();
    }

    #[test]
    fn test_timeline_manager_play() {
        let mut manager = TimelineManager::new();
        manager.play();
        assert!(manager.is_playing);
    }

    #[test]
    fn test_timeline_manager_update() {
        let mut manager = TimelineManager::new();
        manager.play();
        manager.update(0.016);
        assert_eq!(manager.timeline.current_frame, 1);
    }

    #[test]
    fn test_timeline_manager_register_entity() {
        let mut manager = TimelineManager::new();
        let mut animation = AnimationComponent::new();
        manager.register_entity(1, animation);
        assert!(manager.get_entity_animation(1).is_some());
    }

    #[test]
    fn test_timeline_manager_set_frame() {
        let mut manager = TimelineManager::new();
        manager.set_frame(10);
        assert_eq!(manager.timeline.current_frame, 10);
    }
}
