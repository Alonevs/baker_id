use crate::level_manager::{Level, Tile};
use crate::sprite_manager::SpriteManager;
use crate::math::Vector2;
use forge_runtime::camera::Camera;

pub struct TileSystem {
    level: Level,
    sprite_manager: SpriteManager,
    visible_tiles: Vec<Tile>,
}

impl TileSystem {
    pub fn new(level: Level, sprite_manager: SpriteManager) -> Self {
        TileSystem {
            level,
            sprite_manager,
            visible_tiles: Vec::new(),
        }
    }

    pub fn get_tile_at(&self, x: i32, y: i32) -> Option<&Tile> {
        self.level.get_tile_at(x, y)
    }

    pub fn get_platform_at(&self, x: i32, y: i32) -> Option<&crate::level_manager::Platform> {
        self.level.get_platform_at(x, y)
    }

    /// Actualiza los tiles visibles según la cámara
    pub fn update_visible_tiles(&mut self, camera: &Camera) {
        let camera_x = camera.position.x;
        let camera_y = camera.position.y;
        let view_distance = 100.0; // Aumentado para mejor rendimiento

        let visible_x_start = (camera_x - view_distance) as i32;
        let visible_x_end = (camera_x + view_distance) as i32;
        let visible_y_start = (camera_y - view_distance) as i32;
        let visible_y_end = (camera_y + view_distance) as i32;

        self.visible_tiles = self.level.tiles.iter()
            .filter(|t| t.x >= visible_x_start && t.x <= visible_x_end &&
                        t.y >= visible_y_start && t.y <= visible_y_end)
            .cloned()
            .collect();
    }

    /// Dibuja los tiles visibles según el tipo de cámara
    pub fn draw_tiles(&self, camera: &Camera) {
        match camera.camera_type {
            forge_runtime::camera::CameraType::Isometric => {
                self.draw_isometric_tiles(camera);
            }
            forge_runtime::camera::CameraType::TopDown |
            forge_runtime::camera::CameraType::SideScroller |
            forge_runtime::camera::CameraType::Free2D => {
                self.draw_2d_tiles(camera);
            }
            _ => {
                // Fallback a isométrico
                self.draw_isometric_tiles(camera);
            }
        }
    }

    /// Dibuja tiles en vista isométrica
    fn draw_isometric_tiles(&self, camera: &Camera) {
        for tile in &self.visible_tiles {
            let sprite = self.sprite_manager.get_sprite("environment", &tile.tile_type);
            if let Some(s) = sprite {
                // Proyección isométrica
                let iso_x = (tile.x - tile.y) * 20.0;
                let iso_y = (tile.x + tile.y) * 10.0;
                s.draw(&mut self.sprite_manager, iso_x, iso_y, 0.0);
            }
        }
    }

    /// Dibuja tiles en vista 2D (top-down, side-scroller, free-2d)
    fn draw_2d_tiles(&self, camera: &Camera) {
        for tile in &self.visible_tiles {
            let sprite = self.sprite_manager.get_sprite("environment", &tile.tile_type);
            if let Some(s) = sprite {
                // Posición 2D simple
                let x = tile.x * 32.0 + camera.position.x;
                let y = tile.y * 32.0 + camera.position.y;
                s.draw(&mut self.sprite_manager, x, y, 0.0);
            }
        }
    }
}
