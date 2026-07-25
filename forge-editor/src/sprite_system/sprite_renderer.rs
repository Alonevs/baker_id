use crate::sprite_manager::SpriteManager;

pub struct SpriteRenderer {
    sprite_manager: SpriteManager,
    isometric_scale: f32,
}

impl SpriteRenderer {
    pub fn new(sprite_manager: SpriteManager) -> Self {
        SpriteRenderer {
            sprite_manager,
            isometric_scale: 1.0,
        }
    }

    pub fn set_isometric_scale(&mut self, scale: f32) {
        self.isometric_scale = scale;
    }

    pub fn draw_isometric_sprite(&self, sprite: &Sprite, x: f32, y: f32, 
                                  rotation: f32) {
        let scaled_x = x * self.isometric_scale;
        let scaled_y = y * self.isometric_scale;
        sprite.draw(&mut self.sprite_manager, scaled_x, scaled_y, rotation);
    }

    pub fn draw_isometric_animation(&self, animation: &Animation, x: f32, y: f32,
                                     time: f32) -> Option<()> {
        if let Some(sprite) = animation.get_current_frame(time) {
            self.draw_isometric_sprite(sprite, x, y, 0.0);
            Some(())
        } else {
            None
        }
    }
}
