use crate::physics::PhysicsBody;
use crate::animation_system::AnimationSystem;
use crate::sprite_system::sprite_renderer::SpriteRenderer;
use crate::math::Vector2;

pub struct PlayerController {
    position: Vector2,
    velocity: Vector2,
    is_jumping: bool,
    is_attacking: bool,
    animation_system: AnimationSystem,
    sprite_renderer: SpriteRenderer,
    physics_body: PhysicsBody,
}

impl PlayerController {
    pub fn new(sprite_renderer: SpriteRenderer, animation_system: AnimationSystem) -> Self {
        let physics_body = PhysicsBody::new(
            Vector2::new(0.0, 0.0),
            10.0,
            CollisionShape::new(32.0, 64.0)
        );
        
        PlayerController {
            position: Vector2::new(0.0, 0.0),
            velocity: Vector2::new(0.0, 0.0),
            is_jumping: false,
            is_attacking: false,
            animation_system,
            sprite_renderer,
            physics_body,
        }
    }

    pub fn update(&mut self, dt: f32, input: &Input, collisions: &[CollisionResult]) {
        // Handle input
        if input.is_down(KeyCode::W) {
            self.jump();
        }
        if input.is_down(KeyCode::A) {
            self.move_left();
        }
        if input.is_down(KeyCode::D) {
            self.move_right();
        }
        if input.is_down(KeyCode::SPACE) {
            self.attack();
        }

        // Update physics
        self.physics_body.update(dt, &PhysicsConfig::default());
        self.position = self.physics_body.position;

        // Check collisions
        for collision in collisions.iter() {
            if let CollisionResult::Collided { shape } = collision {
                self.resolve_collision(shape);
            }
        }

        // Update animations
        self.animation_system.update(dt);
    }

    fn jump(&mut self) {
        if !self.is_jumping {
            self.velocity.y = -400.0;
            self.is_jumping = true;
        }
    }

    fn move_left(&mut self) {
        self.velocity.x = -200.0;
    }

    fn move_right(&mut self) {
        self.velocity.x = 200.0;
    }

    fn attack(&mut self) {
        self.is_attacking = true;
        // Play attack animation
    }

    fn resolve_collision(&mut self, shape: &CollisionShape) {
        // Simple collision resolution
        if self.position.y < shape.y + shape.height {
            self.velocity.y = 0.0;
            self.position.y = shape.y + shape.height;
        }
    }

    pub fn draw(&self, time: f32) {
        if let Some(sprite) = self.animation_system.get_current_frame(time) {
            self.sprite_renderer.draw_isometric_sprite(
                sprite, 
                self.position.x, 
                self.position.y, 
                0.0
            );
        }
    }
}
