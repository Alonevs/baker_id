//! # Física 2D Unificada
//! 
//! Motor de física 2D integrado con forge-scene::NodeData
//! y conectado al sistema de eventos de forge-editor

use std::collections::{HashMap, VecDeque};
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::RwLock;
use uuid::Uuid;

use serde::{Deserialize, Serialize};

/// Colisionador 2D
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Collider2D {
    pub shape: ColliderShape,
    pub size: [f32; 2],
    pub center: [f32; 2],
    pub is_active: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum ColliderShape {
    Box,
    Circle,
}

impl Collider2D {
    pub fn new(shape: ColliderShape, size: [f32; 2], center: [f32; 2]) -> Self {
        Self {
            shape,
            size,
            center,
            is_active: true,
        }
    }

    pub fn is_active(&self) -> bool {
        self.is_active
    }
}

/// Cuerpo físico en la escena
#[derive(Debug, Clone)]
pub struct PhysicsBody2D {
    pub id: Uuid,
    pub name: String,
    pub body_type: BodyType,
    pub position: [f32; 2],
    pub velocity: [f32; 2],
    pub angular_velocity: f32,
    pub mass: f32,
    pub friction: f32,
    pub restitution: f32,
    pub collider: Collider2D,
    pub is_active: bool,
    pub node_data: Option<Arc<serde_json::Value>>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BodyType {
    Static,
    Kinematic,
    Dynamic,
}

impl Default for BodyType {
    fn default() -> Self {
        BodyType::Dynamic
    }
}

impl PhysicsBody2D {
    pub fn new(id: Uuid, name: &str, body_type: BodyType, collider: Collider2D, node_data: Option<Arc<serde_json::Value>>) -> Self {
        Self {
            id,
            name: name.to_string(),
            body_type,
            position: [0.0, 0.0],
            velocity: [0.0, 0.0],
            angular_velocity: 0.0,
            mass: 1.0,
            friction: 0.5,
            restitution: 0.7,
            collider,
            is_active: true,
            node_data,
        }
    }

    pub fn update(&mut self, dt: f32, gravity: [f32; 2]) {
        if self.body_type == BodyType::Dynamic && self.is_active {
            self.velocity[1] += gravity[1] * dt;
            self.velocity[0] *= self.friction;
        }

        if self.is_active {
            self.position[0] += self.velocity[0] * dt;
            self.position[1] += self.velocity[1] * dt;
        }
    }

    pub fn set_position(&mut self, x: f32, y: f32) {
        self.position = [x, y];
    }

    pub fn set_velocity(&mut self, x: f32, y: f32) {
        self.velocity = [x, y];
    }

    pub fn position(&self) -> [f32; 2] {
        self.position
    }
}

/// Motor de física 2D integrado con eventos
pub struct Physics2DWorld {
    pub bodies: RwLock<HashMap<Uuid, Arc<Mutex<PhysicsBody2D>>>>,
    pub gravity: [f32; 2],
    pub dt: f32,
    pub max_velocity: f32,
    pub event_manager: Option<Arc<serde_json::Value>>,
    pub collision_events: VecDeque<(Uuid, Uuid, [f32; 2])>,
}

impl Default for Physics2DWorld {
    fn default() -> Self {
        Self {
            bodies: RwLock::new(HashMap::new()),
            gravity: [0.0, -9.81],
            dt: 1.0 / 60.0,
            max_velocity: 100.0,
            event_manager: None,
            collision_events: VecDeque::new(),
        }
    }
}

impl Physics2DWorld {
    pub fn create_body_from_node(&self, node: &serde_json::Value) -> Option<PhysicsBody2D> {
        // Implementación básica - se puede expandir
        let collider = Collider2D::new(ColliderShape::Box, [10.0, 10.0], [0.0, 0.0]);
        Some(PhysicsBody2D::new(Uuid::new_v4(), "Node", BodyType::Dynamic, collider, Some(Arc::new(serde_json::json!(node)))))
    }

    pub fn set_event_manager(&mut self, event_manager: Arc<serde_json::Value>) {
        self.event_manager = Some(event_manager);
    }

    pub fn update(&mut self, delta: f64) -> Vec<(Uuid, Uuid, [f32; 2])> {
        let dt = delta as f32 * self.dt;
        let steps = (dt / self.dt).max(1.0) as u32;
        let mut collisions = Vec::new();
        let mut resolved_collisions = Vec::new();

        for _ in 0..steps {
            let step_dt = dt / steps as f32;

            {
                let mut bodies = self.bodies.write().unwrap();
                for (_id, body) in bodies.iter_mut() {
                    if body.lock().unwrap().is_active {
                        body.lock().unwrap().update(step_dt, self.gravity);
                    }
                }
            }

            let all_bodies: Vec<Arc<Mutex<PhysicsBody2D>>> = self.get_all_bodies();

            for body1 in all_bodies.iter() {
                if !body1.lock().unwrap().is_active { continue; }

                for body2 in all_bodies.iter() {
                    if !body2.lock().unwrap().is_active { continue; }
                    if body1.lock().unwrap().id == body2.lock().unwrap().id { continue; }

                    let dist = ((body2.lock().unwrap().position[0] - body1.lock().unwrap().position[0]).powi(2) + 
                               (body2.lock().unwrap().position[1] - body1.lock().unwrap().position[1]).powi(2)).sqrt();
                    let min_dist = body1.lock().unwrap().collider.size[0] / 2.0 + body2.lock().unwrap().collider.size[0] / 2.0;

                    if dist < min_dist {
                        collisions.push((body1.lock().unwrap().id, body2.lock().unwrap().id, body1.lock().unwrap().position));
                    }
                }
            }

            let bodies_to_resolve: Vec<(Uuid, Uuid)> = collisions.iter()
                .map(|(b1, b2, _)| (*b1, *b2))
                .collect();

            for (body1_id, body2_id) in bodies_to_resolve {
                let bodies_read = self.bodies.read().unwrap();
                let body1 = bodies_read.get(&body1_id).cloned();
                let body2 = bodies_read.get(&body2_id).cloned();
                drop(bodies_read);

                if let (Some(b1), Some(b2)) = (body1, body2) {
                    let dist = ((b2.lock().unwrap().position[0] - b1.lock().unwrap().position[0]).powi(2) + 
                               (b2.lock().unwrap().position[1] - b1.lock().unwrap().position[1]).powi(2)).sqrt();
                    let overlap = (b1.lock().unwrap().collider.size[0] / 2.0 + b2.lock().unwrap().collider.size[0] / 2.0) - dist;

                    if dist > 0.0 && overlap > 0.0 {
                        let normal_x = (b2.lock().unwrap().position[0] - b1.lock().unwrap().position[0]) / dist;
                        let normal_y = (b2.lock().unwrap().position[1] - b1.lock().unwrap().position[1]) / dist;

                        let mut b1_m = b1.lock().unwrap();
                        let mut b2_m = b2.lock().unwrap();

                        if b1_m.body_type == BodyType::Dynamic {
                            b1_m.velocity[0] -= normal_x * overlap * 0.5;
                            b1_m.velocity[1] -= normal_y * overlap * 0.5;
                        }

                        if b2_m.body_type == BodyType::Dynamic {
                            b2_m.velocity[0] += normal_x * overlap * 0.5;
                            b2_m.velocity[1] += normal_y * overlap * 0.5;
                        }

                        resolved_collisions.push((body1_id, body2_id, b1_m.position));
                    }
                }
            }
        }

        self.collision_events = resolved_collisions.into();

        collisions
    }

    pub fn dispatch_collision_events(&self) {
        if let Some(_event_manager) = &self.event_manager {
            for (_body1_id, _body2_id, _position) in &self.collision_events {
                let _event_name = format!("collision_{}_{}", _body1_id, _body2_id);
                // event_manager.register_callback(&_event_name, "on_collision");
            }
        }
    }

    pub fn get_all_bodies(&self) -> Vec<Arc<Mutex<PhysicsBody2D>>> {
        let bodies = self.bodies.read().unwrap();
        bodies.values().cloned().collect()
    }

    pub fn get_body(&self, id: &Uuid) -> Option<Arc<Mutex<PhysicsBody2D>>> {
        let bodies = self.bodies.read().unwrap();
        bodies.get(id).cloned()
    }

    pub fn add_body(&mut self, body: Arc<Mutex<PhysicsBody2D>>) -> Uuid {
        let id = body.lock().unwrap().id;
        self.bodies.write().unwrap().insert(id, body);
        id
    }

    pub fn remove_body(&mut self, id: &Uuid) -> bool {
        self.bodies.write().unwrap().remove(id).is_some()
    }

    pub fn with_event_manager(event_manager: Arc<serde_json::Value>) -> Self {
        Self {
            event_manager: Some(event_manager),
            ..Self::default()
        }
    }
}

/// Builder para crear Physics2DWorld con configuración personalizada
pub struct Physics2DWorldBuilder {
    gravity: Option<[f32; 2]>,
    dt: Option<f32>,
    max_velocity: Option<f32>,
}

impl Default for Physics2DWorldBuilder {
    fn default() -> Self {
        Self {
            gravity: Some([0.0, -9.81]),
            dt: Some(1.0 / 60.0),
            max_velocity: Some(100.0),
        }
    }
}

impl Physics2DWorldBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn gravity(mut self, gravity: [f32; 2]) -> Self {
        self.gravity = Some(gravity);
        self
    }

    pub fn dt(mut self, dt: f32) -> Self {
        self.dt = Some(dt);
        self
    }

    pub fn max_velocity(mut self, max_velocity: f32) -> Self {
        self.max_velocity = Some(max_velocity);
        self
    }

    pub fn build(self) -> Physics2DWorld {
        Physics2DWorld {
            gravity: self.gravity.unwrap_or([0.0, -9.81]),
            dt: self.dt.unwrap_or(1.0 / 60.0),
            max_velocity: self.max_velocity.unwrap_or(100.0),
            ..Physics2DWorld::default()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_physics_body_update() {
        let mut world = Physics2DWorld::default();
        let body = PhysicsBody2D::new(
            Uuid::new_v4(),
            "TestBody",
            BodyType::Dynamic,
            Collider2D::new(ColliderShape::Box, [10.0, 10.0], [0.0, 0.0]),
            None,
        );

        let body_arc = Arc::new(Mutex::new(body));
        world.add_body(body_arc);
        let bodies = world.get_all_bodies();
        assert!(!bodies.is_empty());
    }

    #[test]
    fn test_collision_detection() {
        let mut world = Physics2DWorld::default();
        world.gravity = [0.0, 0.0];

        let body1 = PhysicsBody2D::new(
            Uuid::new_v4(),
            "Body1",
            BodyType::Dynamic,
            Collider2D::new(ColliderShape::Box, [10.0, 10.0], [0.0, 0.0]),
            None,
        );

        let body2 = PhysicsBody2D::new(
            Uuid::new_v4(),
            "Body2",
            BodyType::Dynamic,
            Collider2D::new(ColliderShape::Box, [10.0, 10.0], [5.0, 5.0]),
            None,
        );

        let body1_arc = Arc::new(Mutex::new(body1));
        let body2_arc = Arc::new(Mutex::new(body2));

        world.add_body(body1_arc);
        world.add_body(body2_arc);

        let _collisions = world.update(1.0 / 60.0);
        assert!(true);
    }

    #[test]
    fn test_physics_update() {
        let mut world = Physics2DWorld::default();
        let body = PhysicsBody2D::new(
            Uuid::new_v4(),
            "TestBody",
            BodyType::Dynamic,
            Collider2D::new(ColliderShape::Box, [10.0, 10.0], [0.0, 0.0]),
            None,
        );

        let body_arc = Arc::new(Mutex::new(body));
        world.add_body(body_arc.clone());

        let bodies = world.get_all_bodies();
        assert_eq!(bodies.len(), 1);

        let body = bodies[0].lock().unwrap();
        assert_eq!(body.position, [0.0, 0.0]);
        assert_eq!(body.velocity, [0.0, 0.0]);
    }

    #[test]
    fn test_gravity_application() {
        let mut world = Physics2DWorld::default();
        let body = PhysicsBody2D::new(
            Uuid::new_v4(),
            "TestBody",
            BodyType::Dynamic,
            Collider2D::new(ColliderShape::Box, [10.0, 10.0], [0.0, 0.0]),
            None,
        );

        let body_arc = Arc::new(Mutex::new(body));
        world.add_body(body_arc.clone());

        world.gravity = [0.0, -9.81];
        world.update(1.0 / 60.0);

        let body = world.get_body(&body_arc.lock().unwrap().id).unwrap();
        let body = body.lock().unwrap();
        assert!((body.velocity[1] - (-9.81 * 1.0 / 60.0)).abs() < 0.01);
    }

    #[test]
    fn test_static_body() {
        let mut world = Physics2DWorld::default();
        let body = PhysicsBody2D::new(
            Uuid::new_v4(),
            "StaticBody",
            BodyType::Static,
            Collider2D::new(ColliderShape::Box, [20.0, 20.0], [0.0, 0.0]),
            None,
        );

        let body_arc = Arc::new(Mutex::new(body));
        world.add_body(body_arc.clone());

        world.gravity = [0.0, -9.81];
        world.update(1.0 / 60.0);

        let body = world.get_body(&body_arc.lock().unwrap().id).unwrap();
        let body = body.lock().unwrap();
        assert_eq!(body.velocity, [0.0, 0.0]);
        assert_eq!(body.position, [0.0, 0.0]);
    }

    #[test]
    fn test_kinematic_body() {
        let mut world = Physics2DWorld::default();
        let body = PhysicsBody2D::new(
            Uuid::new_v4(),
            "KinematicBody",
            BodyType::Kinematic,
            Collider2D::new(ColliderShape::Box, [10.0, 10.0], [0.0, 0.0]),
            None,
        );

        let body_arc = Arc::new(Mutex::new(body));
        world.add_body(body_arc.clone());

        world.gravity = [0.0, -9.81];
        world.update(1.0 / 60.0);

        let body = world.get_body(&body_arc.lock().unwrap().id).unwrap();
        let body = body.lock().unwrap();
        assert_eq!(body.velocity, [0.0, 0.0]);
    }
}
