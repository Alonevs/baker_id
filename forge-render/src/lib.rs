//! # Forge Render Backend
//! 
//! Sistema de renderizado 2D para el SDK.

use egui::Color32;
use egui::{Rect, Ui};
use forge_runtime::resource::sprite_resource::SpriteResource;
use forge_types::Vec2;


/// Configuración del render backend
#[derive(Debug, Clone)]
pub struct RenderConfig {
    pub window_title: String,
    pub window_width: f32,
    pub window_height: f32,
    pub dpi_scale: f32,
}

impl Default for RenderConfig {
    fn default() -> Self {
        Self {
            window_title: "Forge SDK - Game Preview".to_string(),
            window_width: 1280.0,
            window_height: 720.0,
            dpi_scale: 1.0,
        }
    }
}

/// Representa una entidad renderizable
#[derive(Debug, Clone)]
pub struct RenderableEntity {
    pub position: Vec2,
    pub size: Vec2,
    pub sprite: Option<SpriteResource>,
    pub color: Color32,
    pub visible: bool,
}

impl RenderableEntity {
    pub fn new(position: Vec2, size: Vec2, color: Color32) -> Self {
        Self {
            position,
            size,
            sprite: None,
            color,
            visible: true,
        }
    }

    pub fn with_sprite(mut self, sprite: SpriteResource) -> Self {
        self.sprite = Some(sprite);
        self
    }
}

/// Sistema de renderizado
pub struct RenderSystem {
    config: RenderConfig,
    entities: Vec<RenderableEntity>,
}

impl RenderSystem {
    pub fn new(config: RenderConfig) -> Self {
        Self {
            config,
            entities: Vec::new(),
        }
    }

    /// Añade una entidad renderizable
    pub fn add_entity(&mut self, entity: RenderableEntity) {
        self.entities.push(entity);
    }

    /// Elimina una entidad por posición
    pub fn remove_entity(&mut self, pos: &Vec2, size: &Vec2) -> bool {
        let pos_vec = Vec2::new(pos.x, pos.y);
        let size_vec = Vec2::new(size.x, size.y);
        self.entities.retain(|e| {
            (e.position.x - pos_vec.x).abs() > 0.01 || (e.position.y - pos_vec.y).abs() > 0.01
                || (e.size.x - size_vec.x).abs() > 0.01 || (e.size.y - size_vec.y).abs() > 0.01
        });
        !self.entities.is_empty()
    }

    /// Renderiza el fondo
    pub fn render_background(&self, ui: &Ui, viewport: Rect) {
        let rect = Rect::from_min_size(egui::Pos2::new(0.0, 0.0), viewport.size());
        ui.painter().rect_filled(rect, 0.0, Color32::from_rgb(30, 30, 30));
    }

    /// Renderiza las entidades
    pub fn render_entities(&self, ui: &Ui, viewport: Rect) {
        for entity in &self.entities {
            if !entity.visible {
                continue;
            }

            let screen_pos = self.world_to_screen_pos(entity.position, viewport);
            let screen_size = self.world_to_screen_size(entity.size, viewport);

            if let Some(sprite) = &entity.sprite {
                self.render_sprite(ui, screen_pos, screen_size, viewport);
            } else {
                self.render_rect(ui, screen_pos, screen_size, entity.color);
            }
        }
    }

    /// Convierte posición del mundo a pantalla
    pub fn world_to_screen_pos(&self, world_pos: Vec2, viewport: Rect) -> Vec2 {
        let center = viewport.center();
        Vec2::new(
            center.x - world_pos.x * viewport.width() / 2.0,
            center.y - world_pos.y * viewport.height() / 2.0,
        )
    }

    /// Convierte tamaño del mundo a pantalla
    pub fn world_to_screen_size(&self, world_size: Vec2, viewport: Rect) -> Vec2 {
        Vec2::new(
            world_size.x * viewport.width() / 2.0,
            world_size.y * viewport.height() / 2.0,
        )
    }

    /// Renderiza un rectángulo
    pub fn render_rect(&self, ui: &Ui, pos: Vec2, size: Vec2, color: Color32) {
        let rect = Rect::from_center_size(
            egui::Pos2::new(pos.x, pos.y),
            egui::Vec2::new(size.x, size.y),
        );

        ui.painter().rect_filled(rect, 0.0, color);
    }

    /// Renderiza un sprite
    pub fn render_sprite(&self, ui: &Ui, pos: Vec2, size: Vec2, viewport: Rect) {
        let rect = Rect::from_center_size(
            egui::Pos2::new(pos.x, pos.y),
            egui::Vec2::new(size.x, size.y),
        );

        ui.painter().rect_filled(rect, 0.0, Color32::from_rgb(100, 100, 255));
    }

    /// Obtiene información del atlas
    fn get_atlas_info(&self, sprite: &SpriteResource) -> AtlasInfo {
        let mut total_width: f32 = 0.0;
        let mut total_height: f32 = 0.0;

        if let Some(frames) = &sprite.frames {
            for frame in frames {
                total_width = total_width.max(frame.width);
                total_height = total_height.max(frame.height);
            }
        }

        AtlasInfo {
            width: total_width,
            height: total_height,
            texture_path: sprite.texture_path.clone().unwrap_or_default(),
        }
    }
}

/// Información del atlas
#[derive(Debug, Clone)]
pub struct AtlasInfo {
    pub width: f32,
    pub height: f32,
    pub texture_path: String,
}

impl Default for RenderSystem {
    fn default() -> Self {
        Self::new(RenderConfig::default())
    }
}

// Función de utilidad para crear entidades
pub fn create_test_entity() -> RenderableEntity {
    RenderableEntity::new(
        Vec2::new(0.0, 0.0),
        Vec2::new(10.0, 10.0),
        Color32::from_rgb(100, 100, 200),
    )
}
