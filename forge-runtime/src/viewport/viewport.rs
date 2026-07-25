//! # Viewport
//! 
//! Viewport para renderizado en tiempo real de entidades.

use crate::play_mode::{PlayMode, PlayModeState};
use crate::entities::Entity;

/// Configuración del Viewport
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ViewportConfig {
    pub width: u32,
    pub height: u32,
    pub background_color: (f32, f32, f32, f32),
}

impl Default for ViewportConfig {
    fn default() -> Self {
        Self {
            width: 800,
            height: 600,
            background_color: (0.1, 0.1, 0.15, 1.0),
        }
    }
}

/// Viewport para renderizado en tiempo real
#[derive(Debug)]
pub struct Viewport {
    pub config: ViewportConfig,
    pub entities: Vec<Entity>,
    pub play_mode: Option<PlayMode>,
    pub camera_scale: f32,
    pub camera_offset: (f32, f32),
    pub is_rendering: bool,
}

impl Default for Viewport {
    fn default() -> Self {
        Self {
            config: ViewportConfig::default(),
            entities: Vec::new(),
            play_mode: None,
            camera_scale: 1.0,
            camera_offset: (0.0, 0.0),
            is_rendering: false,
        }
    }
}

impl Viewport {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_entities(&mut self, entities: Vec<Entity>) {
        self.entities = entities;
    }

    pub fn connect_play_mode(&mut self, play_mode: PlayMode) {
        self.play_mode = Some(play_mode);
    }

    pub fn start_rendering(&mut self) {
        self.is_rendering = true;
    }

    pub fn stop_rendering(&mut self) {
        self.is_rendering = false;
    }

    pub fn update(&mut self, delta: f32) {
        if let Some(ref mut mode) = self.play_mode {
            mode.update(delta);
        }
    }

    pub fn render(&self) {
        if !self.is_rendering {
            return;
        }

        // Dibujar grid
        self.draw_grid();

        // Dibujar entidades
        for entity in &self.entities {
            self.draw_entity(entity);
        }
    }

    fn draw_grid(&self) {
        // Grid placeholder
    }

    fn draw_entity(&self, entity: &Entity) {
        // Entity placeholder
    }

    pub fn get_play_state(&self) -> PlayModeState {
        if let Some(ref mode) = self.play_mode {
            mode.get_state()
        } else {
            PlayModeState::Stopped
        }
    }
}
