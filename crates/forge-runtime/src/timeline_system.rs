//! Timeline System para sincronizar Editor con Runtime

use crate::timeline::{TimelineManager, TimelineEvent};
use crate::components::AnimationComponent;
use std::collections::HashMap;
use uuid::Uuid;

/// Sistema de Timeline que sincroniza el editor con el runtime
pub struct TimelineSystem {
    /// Manager de timeline principal
    pub manager: TimelineManager,
    /// Entidades activas en runtime
    pub active_entities: HashMap<u64, u64>, // runtime_id -> editor_id
    /// Frame actual del runtime
    pub runtime_frame: u32,
}

impl Default for TimelineSystem {
    fn default() -> Self {
        Self {
            manager: TimelineManager::new(),
            active_entities: HashMap::new(),
            runtime_frame: 0,
        }
    }
}

impl TimelineSystem {
    /// Crea un nuevo TimelineSystem
    pub fn new() -> Self {
        Self::default()
    }

    /// Registra una entidad del runtime con su ID de editor
    pub fn register_entity(&mut self, runtime_id: u64, editor_id: u64) {
        self.active_entities.insert(runtime_id, editor_id);
    }

    /// Obtiene el ID de editor para un runtime_id
    pub fn get_editor_entity(&self, runtime_id: u64) -> Option<u64> {
        self.active_entities.get(&runtime_id).copied()
    }

    /// Sincroniza animaciones del editor con el runtime
    pub fn sync_animations(&mut self) {
        // Sincronizar entidades del editor con runtime
        for (editor_id, animation) in self.manager.entity_animations.iter() {
            if let Some(runtime_id) = self.active_entities.get(editor_id) {
                // Obtener AnimationComponent del runtime
                // (en implementación real, esto buscaría en entities del runtime)
            }
        }
    }

    /// Actualiza el sistema con delta time
    pub fn update(&mut self, dt: f32) {
        // Actualizar timeline manager
        self.manager.update(dt);
        
        // Actualizar frame del runtime
        let frame_duration = self.manager.timeline.frame_duration as f32 * self.manager.playback_speed;
        if dt >= frame_duration {
            self.runtime_frame += 1;
            // Aplicar eventos del frame
            self.apply_frame_events(self.runtime_frame);
        }
    }

    /// Aplica los eventos del frame actual
    fn apply_frame_events(&self, frame: u32) {
        let events = self.manager.get_events_at_frame(frame);
        for event in events {
            // Aplicar acción del evento en runtime
            if let Some(entity_id) = self.active_entities.get(&event.entity_id) {
                // En implementación real, esto llamaría a métodos de la entidad
                // para aplicar el evento
            }
        }
    }

    /// Obtiene el frame actual del runtime
    pub fn get_runtime_frame(&self) -> u32 {
        self.runtime_frame
    }

    /// Obtiene el estado de reproducción
    pub fn is_playing(&self) -> bool {
        self.manager.is_playing
    }

    /// Cambia el estado de reproducción
    pub fn set_playing(&mut self, playing: bool) {
        self.manager.is_playing = playing;
        self.manager.timeline.is_playing = playing;
    }

    /// Obtiene el playback speed
    pub fn get_playback_speed(&self) -> f32 {
        self.manager.playback_speed
    }

    /// Cambia el playback speed
    pub fn set_playback_speed(&mut self, speed: f32) {
        self.manager.set_playback_speed(speed);
    }

    /// Serializa el estado del sistema
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
    use crate::timeline::Timeline;

    #[test]
    fn test_timeline_system_basic() {
        let mut system = TimelineSystem::new();
        system.manager.play();
        system.update(0.016);
        assert_eq!(system.runtime_frame, 1);
        system.set_playing(false);
        assert!(!system.is_playing());
    }

    #[test]
    fn test_timeline_system_entity_sync() {
        let mut system = TimelineSystem::new();
        system.register_entity(1, 100);
        system.manager.register_entity(100, AnimationComponent::new());
        system.sync_animations();
        assert_eq!(system.get_editor_entity(1), Some(100));
    }

    #[test]
    fn test_timeline_system_playback_speed() {
        let mut system = TimelineSystem::new();
        system.manager.play();
        system.set_playback_speed(2.0);
        system.update(0.016);
        assert_eq!(system.runtime_frame, 2);
        system.set_playing(false);
    }
}