use crate::math::Vector2;

pub struct IsometricCamera {
    position: Vector2,
    target: Vector2,
    fov: f32,
    zoom: f32,
    is_following: bool,
    follow_smoothness: f32,
}

impl IsometricCamera {
    pub fn new(fov: f32, zoom: f32) -> Self {
        IsometricCamera {
            position: Vector2::ZERO,
            target: Vector2::ZERO,
            fov,
            zoom,
            is_following: false,
            follow_smoothness: 0.1,
        }
    }

    pub fn set_target(&mut self, target: Vector2) {
        self.target = target;
    }

    pub fn follow_player(&mut self, player_position: &Vector2) {
        self.is_following = true;
        self.target = *player_position;
    }

    pub fn update(&mut self, dt: f32) {
        if self.is_following {
            // Smooth follow
            let dx = (self.target.x - self.position.x) * self.follow_smoothness;
            let dy = (self.target.y - self.position.y) * self.follow_smoothness;
            self.position.x += dx;
            self.position.y += dy;
        }
    }

    pub fn get_view_matrix(&self) -> Matrix4x4 {
        // Isometric projection matrix
        let iso_scale = 1.0 / (self.fov * self.zoom);
        
        Matrix4x4::new(
            iso_scale,  0.0,  0.0,  0.0,
            0.0,        iso_scale, 0.0, 0.0,
            0.0,        0.0,    1.0,  0.0,
            -self.position.x * iso_scale,
            -self.position.y * iso_scale,
            0.0,        0.0,    0.0,  1.0
        )
    }

    pub fn get_screen_position(&self, iso_x: f32, iso_y: f32) -> (f32, f32) {
        let screen_x = (iso_x - iso_y) * 20.0 * self.zoom - self.position.x;
        let screen_y = (iso_x + iso_y) * 10.0 * self.zoom - self.position.y;
        (screen_x, screen_y)
    }
}
