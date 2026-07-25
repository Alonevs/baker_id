//! Timeline System para sincronizar Editor con Runtime

use crate::timeline::timeline_manager::TimelineManager;
use crate::components::AnimationComponent;
use std::collections::HashMap;

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
    pub fn update(&mut self, _dt: f32) {
        // Actualizar timeline manager
        self.manager.update(_dt);
        
        // Actualizar frame del runtime
        self.runtime_frame += 1;
    }

    /// Aplica los eventos del frame actual
    fn apply_frame_events(&mut self, _frame: u32) {
        // Aplicar eventos del frame
        // (implementación pendiente)
    }

    /// Establece si está reproduciendo
    pub fn set_playing(&mut self, playing: bool) {
        self.manager.is_playing = playing;
    }

    /// Obtiene si está reproduciendo
    pub fn is_playing(&self) -> bool {
        self.manager.is_playing
    }

    /// Establece la velocidad de reproducción
    pub fn set_playback_speed(&mut self, speed: f32) {
        self.manager.playback_speed = speed;
    }
}
