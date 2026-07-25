use crate::physics::{PhysicsBody, CollisionShape, CollisionResult};
use crate::math::Vector2;

pub struct IsometricCollisionSystem {
    bodies: Vec<PhysicsBody>,
}

impl IsometricCollisionSystem {
    pub fn new() -> Self {
        IsometricCollisionSystem {
            bodies: Vec::new(),
        }
    }

    pub fn add_body(&mut self, body: PhysicsBody) {
        self.bodies.push(body);
    }

    pub fn check_collisions(&self) -> Vec<CollisionResult> {
        let mut collisions = Vec::new();
        
        for i in 0..self.bodies.len() {
            for j in i+1..self.bodies.len() {
                if let CollisionResult::Collided { shape } = 
                    self.check_collision(&self.bodies[i], &self.bodies[j]) {
                    collisions.push(CollisionResult::Collided { shape });
                }
            }
        }
        
        collisions
    }

    fn check_collision(&self, body1: &PhysicsBody, body2: &PhysicsBody) -> CollisionResult {
        let distance = self.calculate_isometric_distance(
            &body1.position, 
            &body2.position
        );
        
        let combined_radius = (body1.collision_shape.width + body2.collision_shape.width) / 2.0;
        
        if distance < combined_radius {
            CollisionResult::Collided {
                shape: body1.collision_shape.clone(),
            }
        } else {
            CollisionResult::NoCollision
        }
    }

    fn calculate_isometric_distance(&self, pos1: &Vec2, pos2: &Vec2) -> f32 {
        // Convert isometric coordinates to cartesian for collision detection
        let iso_to_cartesian = |x: f32, y: f32| -> Vec2 {
            Vec2::new(
                (x - y) * 20.0,  // Scale factor for isometric projection
                (x + y) * 10.0   // Vertical scale
            )
        };
        
        let cart1 = iso_to_cartesian(pos1.x, pos1.y);
        let cart2 = iso_to_cartesian(pos2.x, pos2.y);
        
        ((cart1.x - cart2.x).powi(2) + (cart1.y - cart2.y).powi(2)).sqrt()
    }
}
