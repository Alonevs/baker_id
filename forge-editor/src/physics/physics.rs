use crate::math::Vector2;

pub struct PhysicsConfig {
    pub gravity: f32,
    pub friction: f32,
    pub bounce: f32,
    pub collision_tolerance: f32,
}

impl Default for PhysicsConfig {
    fn default() -> Self {
        PhysicsConfig {
            gravity: 980.0,
            friction: 0.8,
            bounce: 0.5,
            collision_tolerance: 0.1,
        }
    }
}

pub struct PhysicsBody {
    pub position: Vector2,
    pub velocity: Vector2,
    pub mass: f32,
    pub is_static: bool,
    pub collision_shape: CollisionShape,
}

impl PhysicsBody {
    pub fn new(position: Vector2, mass: f32, shape: CollisionShape) -> Self {
        PhysicsBody {
            position,
            velocity: Vector2::new(0.0, 0.0),
            mass,
            is_static: false,
            collision_shape: shape,
        }
    }

    pub fn apply_force(&mut self, force: Vector2) {
        if self.is_static {
            return;
        }
        let acceleration = force / self.mass;
        self.velocity.x += acceleration.x;
        self.velocity.y += acceleration.y;
    }

    pub fn update(&mut self, dt: f32, config: &PhysicsConfig) {
        if self.is_static {
            return;
        }

        // Apply gravity
        self.velocity.y += config.gravity * dt;
        
        // Apply friction
        self.velocity *= config.friction;

        // Update position
        self.position.x += self.velocity.x * dt;
        self.position.y += self.velocity.y * dt;
    }
}

pub struct CollisionShape {
    pub width: f32,
    pub height: f32,
}

impl CollisionShape {
    pub fn new(width: f32, height: f32) -> Self {
        CollisionShape { width, height }
    }
}

pub enum CollisionResult {
    Collided { shape: CollisionShape },
    NoCollision,
}
