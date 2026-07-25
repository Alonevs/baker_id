//! # Viewport
//! 
//! Viewport para renderizado en tiempo real de entidades.

use crate::play_mode::{PlayMode, PlayModeState, Entity};

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
    /// Configuración
    pub config: ViewportConfig,
    /// Entidades renderizadas
    pub entities: Vec<Entity>,
    /// Play Mode
    pub play_mode: Option<PlayMode>,
    /// Escala de cámara
    pub camera_scale: f32,
    /// Offset de cámara
    pub camera_offset: (f32, f32),
    /// Estado de renderizado
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
    /// Crea un nuevo Viewport
    pub fn new() -> Self {
        Self::default()
    }

    /// Configura las entidades
    pub fn set_entities(&mut self, entities: Vec<Entity>) {
        self.entities = entities;
    }

    /// Conecta con Play Mode
    pub fn connect_play_mode(&mut self, play_mode: PlayMode) {
        self.play_mode = Some(play_mode);
    }

    /// Inicia renderizado
    pub fn start_rendering(&mut self) {
        self.is_rendering = true;
    }

    /// Detiene renderizado
    pub fn stop_rendering(&mut self) {
        self.is_rendering = false;
    }

    /// Actualiza el viewport
    pub fn update(&mut self, delta: f32) {
        if let Some(ref mut mode) = self.play_mode {
            mode.update(delta);
        }
    }

    /// Renderiza el viewport
    pub fn render(&self, ui: &mut dyn FnMut()) {
        if !self.is_rendering {
            return;
        }

        // Dibujar fondo
        ui.call(|_| {
            egui::Color32::from_srgba(self.config.background_color).paint(
                egui::Rect::from_min_size(egui::vec2(0.0, 0.0), egui::vec2(self.config.width as f32, self.config.height as f32)),
                egui::CornerRadius::ZERO,
            );
        });

        // Dibujar grid
        self.draw_grid(ui);

        // Dibujar entidades
        for entity in &self.entities {
            self.draw_entity(entity, ui);
        }
    }

    /// Dibuja el grid de fondo
    fn draw_grid(&self, ui: &mut dyn FnMut()) {
        ui.call(|_| {
            let grid_color = egui::Color32::from_srgba((0.2, 0.2, 0.25, 1.0));
            let grid_line_width = 1.0;

            // Grid vertical
            for x in (0..self.config.width).step_by(50) {
                let x_pos = (x as f32 * self.camera_scale) + self.camera_offset.0;
                egui::Line::new(egui::vec2(x_pos, 0.0), egui::vec2(x_pos, self.config.height as f32))
                    .color(grid_color)
                    .stroke_width(grid_line_width)
                    .paint(
                        egui::Rect::from_min_size(egui::vec2(x_pos, 0.0), egui::vec2(0.0, self.config.height as f32)),
                        egui::CornerRadius::ZERO,
                    );
            }

            // Grid horizontal
            for y in (0..self.config.height).step_by(50) {
                let y_pos = (y as f32 * self.camera_scale) + self.camera_offset.1;
                egui::Line::new(egui::vec2(0.0, y_pos), egui::vec2(self.config.width as f32, y_pos))
                    .color(grid_color)
                    .stroke_width(grid_line_width)
                    .paint(
                        egui::Rect::from_min_size(egui::vec2(0.0, y_pos), egui::vec2(self.config.width as f32, 0.0)),
                        egui::CornerRadius::ZERO,
                    );
            }
        });
    }

    /// Dibuja una entidad
    fn draw_entity(&self, entity: &Entity, ui: &mut dyn FnMut()) {
        ui.call(|_| {
            let size = egui::vec2(20.0 * self.camera_scale, 20.0 * self.camera_scale);
            let position = egui::vec2(entity.position.0, entity.position.1);
            
            egui::Shape::rect_filled(
                egui::Rect::from_center_size(position, size),
                egui::Rounding::NONE,
                egui::Color32::from_srgba((0.3, 0.6, 0.9, 0.8)),
            )
            .paint(
                egui::Rect::from_min_size(position, size),
                egui::CornerRadius::ZERO,
            );
        });
    }

    /// Obtiene el estado de Play Mode
    pub fn get_play_state(&self) -> PlayModeState {
        if let Some(ref mode) = self.play_mode {
            mode.get_state()
        } else {
            PlayModeState::Stopped
        }
    }
}
