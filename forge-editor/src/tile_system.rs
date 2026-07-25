use crate::level_manager::{Level, Tile};
use crate::sprite_manager::SpriteManager;
use crate::math::Vector2;

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

    pub fn update_visible_tiles(&mut self, camera_x: f32, camera_y: f32, view_distance: f32) {
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

    pub fn draw_tiles(&self, camera_x: f32, camera_y: f32) {
        for tile in &self.visible_tiles {
            let sprite = self.sprite_manager.get_sprite("environment", &tile.tile_type);
            if let Some(s) = sprite {
                let iso_x = (tile.x - tile.y) * 20.0;
                let iso_y = (tile.x + tile.y) * 10.0;
                s.draw(&mut self.sprite_manager, iso_x, iso_y, 0.0);
            }
        }
    }
}
