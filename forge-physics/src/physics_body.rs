use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::constraints::Constraint;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhysicsWorld {
    pub bodies: Vec<PhysicsBody>,
    pub constraints: Vec<Constraint>,
    pub gravity: [f32; 2],
    pub dt: f32,
    pub max_velocity: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhysicsBody {
    pub id: Uuid,
    pub name: String,
    pub body_type: BodyType,
    pub transform: Transform,
    pub velocity: [f32; 2],
    pub angular_velocity: f32,
    pub mass: f32,
    pub friction: f32,
    pub restitution: f32,
    pub is_active: bool,
    pub radius: f32,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum BodyType {
    Static,
    Kinematic,
    Dynamic,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Transform {
    pub position: [f32; 2],
    pub rotation: f32,
}

#[derive(Debug, Clone)]
pub struct CollisionEvent {
    pub body1_id: Uuid,
    pub body2_id: Uuid,
    pub position: [f32; 2],
}

impl PhysicsWorld {
    pub fn new() -> Self {
        Self {
            bodies: Vec::new(),
            constraints: Vec::new(),
            gravity: [0.0, -9.81],
            dt: 1.0 / 60.0,
            max_velocity: 100.0,
        }
    }

    pub fn add_body(&mut self, body: PhysicsBody) -> Uuid {
        let id = body.id;
        self.bodies.push(body);
        id
    }

    pub fn get_body(&self, id: &Uuid) -> Option<&PhysicsBody> {
        self.bodies.iter().find(|b| &b.id == id)
    }

    pub fn get_body_mut(&mut self, id: &Uuid) -> Option<&mut PhysicsBody> {
        self.bodies.iter_mut().find(|b| &b.id == id)
    }

    pub fn get_all_bodies(&self) -> Vec<&PhysicsBody> {
        self.bodies.iter().collect()
    }

    pub fn update(&mut self, delta: f64) -> Vec<CollisionEvent> {
        let dt = delta as f32 * self.dt;
        let mut collisions = Vec::new();

        let steps = (dt / self.dt).max(1.0) as u32;
        
        for _ in 0..steps {
            let step_dt = dt / steps as f32;
            
            for body in &mut self.bodies {
                if body.is_active && body.body_type == BodyType::Dynamic {
                    body.velocity[1] += self.gravity[1] * step_dt;
                }
            }

            for body in &mut self.bodies {
                if body.is_active {
                    body.transform.position[0] += body.velocity[0] * step_dt;
                    body.transform.position[1] += body.velocity[1] * step_dt;
                    
                    let speed = (body.velocity[0].powi(2) + body.velocity[1].powi(2)).sqrt();
                    if speed > self.max_velocity {
                        let scale = self.max_velocity / speed;
                        body.velocity[0] *= scale;
                        body.velocity[1] *= scale;
                    }
                }
            }

            // Detect collisions
            let all_bodies: Vec<&PhysicsBody> = self.bodies.iter().collect();
            
            for body in all_bodies.iter() {
                if !body.is_active { continue; }
                
                for other in all_bodies.iter() {
                    if !other.is_active { continue; }
                    if body.id == other.id { continue; }
                    
                    let dist = ((other.transform.position[0] - body.transform.position[0]).powi(2) + 
                               (other.transform.position[1] - body.transform.position[1]).powi(2)).sqrt();
                    let min_dist = body.radius + other.radius;
                    
                    if dist < min_dist {
                        collisions.push(CollisionEvent {
                            body1_id: body.id,
                            body2_id: other.id,
                            position: [
                                (body.transform.position[0] + other.transform.position[0]) / 2.0,
                                (body.transform.position[1] + other.transform.position[1]) / 2.0,
                            ],
                        });
                    }
                }
            }
            
            // Resolve collisions - separated into two steps
            let mut bodies_to_resolve: Vec<(Uuid, Uuid)> = Vec::new();
            
            for collision in &collisions {
                bodies_to_resolve.push((collision.body1_id, collision.body2_id));
            }
            
            for (body1_id, body2_id) in bodies_to_resolve {
                let body = match self.get_body(&body1_id) {
                    Some(b) => b.clone(),
                    None => continue,
                };
                let other = match self.get_body(&body2_id) {
                    Some(b) => b.clone(),
                    None => continue,
                };
                
                let dist = ((other.transform.position[0] - body.transform.position[0]).powi(2) + 
                           (other.transform.position[1] - body.transform.position[1]).powi(2)).sqrt();
                
                if dist == 0.0 { continue; }
                
                let overlap = (body.radius + other.radius) - dist;
                let normal_x = (other.transform.position[0] - body.transform.position[0]) / dist;
                let normal_y = (other.transform.position[1] - body.transform.position[1]) / dist;
                
                if body.body_type == BodyType::Dynamic {
                    if let Some(body_mut) = self.get_body_mut(&body1_id) {
                        body_mut.velocity[0] -= normal_x * overlap * 0.5;
                        body_mut.velocity[1] -= normal_y * overlap * 0.5;
                    }
                }
                if other.body_type == BodyType::Dynamic {
                    if let Some(other_mut) = self.get_body_mut(&body2_id) {
                        other_mut.velocity[0] += normal_x * overlap * 0.5;
                        other_mut.velocity[1] += normal_y * overlap * 0.5;
                    }
                }
            }
        }

        collisions
    }
}

impl Default for PhysicsWorld {
    fn default() -> Self {
        Self::new()
    }
}
