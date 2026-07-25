//! Timeline Manager para conectar Timeline Editor con Runtime

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::timeline::timeline::Timeline;

/// Animación de una entidad
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct AnimationComponent {
    pub name: String,
    pub duration: f32,
    pub loop_count: u32,
    pub current_frame: u32,
}

impl AnimationComponent {
    pub fn new(name: String, duration: f32, loop_count: u32) -> Self {
        Self {
            name,
            duration,
            loop_count,
            current_frame: 0,
        }
    }
}

/// Manager de Timeline que conecta el editor con el runtime
#[derive(Debug, Clone)]
pub struct TimelineManager {
    /// Timeline actual
    pub timeline: Timeline,
    /// Mapa de entidades a sus AnimationComponents
    pub entity_animations: HashMap<u64, AnimationComponent>,
    /// Animaciones cargadas desde editor
    pub loaded_animations: HashMap<String, Vec<String>>,
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
    pub fn get_events_at_frame(&self, frame: u32) -> Vec<&String> {
        Vec::new()
    }

    /// Actualiza el manager con delta time
    pub fn update(&mut self, dt: f32) {
        if self.is_playing {
            self.timeline.next_frame();
        }
    }

    /// Inicia reproducción
    pub fn start(&mut self) {
        self.is_playing = true;
    }

    /// Detiene reproducción
    pub fn stop(&mut self) {
        self.is_playing = false;
    }

    /// Obtiene si está reproduciendo
    pub fn is_playing(&self) -> bool {
        self.is_playing
    }

    /// Obtiene el frame actual
    pub fn current_frame(&self) -> u32 {
        self.timeline.current_frame
    }
}
