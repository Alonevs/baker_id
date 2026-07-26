use crate::physics::{PhysicsBody, CollisionShape, CollisionResult};
use crate::math::Vector2;

/// Sistema de colisiones genérico (funciona con cualquier tipo de cámara)
pub struct CollisionSystem {
    bodies: Vec<PhysicsBody>,
}

impl CollisionSystem {
    pub fn new() -> Self {
        CollisionSystem {
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
        let distance = self.calculate_distance(
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

    /// Calcula distancia entre dos posiciones (funciona en cualquier sistema de coordenadas)
    fn calculate_distance(&self, pos1: &Vec2, pos2: &Vec2) -> f32 {
        // Distancia euclidiana estándar (funciona en cartesianas, isométricas, etc.)
        let dx = pos1.x - pos2.x;
        let dy = pos1.y - pos2.y;
        
        (dx.powi(2) + dy.powi(2)).sqrt()
    }
}
