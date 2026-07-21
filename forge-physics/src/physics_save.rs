use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::physics_body::{PhysicsWorld, Transform, PhysicsBody, BodyType};
use crate::constraints::{Constraint, ConstraintType};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhysicsWorldSaveData {
    pub bodies: Vec<PhysicsBodySaveData>,
    pub constraints: Vec<ConstraintSaveData>,
    pub gravity: [f32; 2],
    pub dt: f32,
    pub max_velocity: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhysicsBodySaveData {
    pub id: String,
    pub name: String,
    pub body_type: BodyType,
    pub position: [f32; 2],
    pub rotation: f32,
    pub velocity: [f32; 2],
    pub angular_velocity: f32,
    pub mass: f32,
    pub friction: f32,
    pub restitution: f32,
    pub is_active: bool,
    pub radius: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConstraintSaveData {
    pub id: String,
    pub body1: String,
    pub body2: String,
    pub constraint_type: ConstraintType,
    pub position: [f32; 2],
    pub stiffness: f32,
    pub damping: f32,
}

impl PhysicsBodySaveData {
    pub fn to_body(&self) -> PhysicsBody {
        PhysicsBody {
            id: Uuid::parse_str(&self.id).unwrap_or_else(|_| Uuid::new_v4()),
            name: self.name.clone(),
            body_type: self.body_type,
            transform: Transform {
                position: self.position,
                rotation: self.rotation,
            },
            velocity: self.velocity,
            angular_velocity: self.angular_velocity,
            mass: self.mass,
            friction: self.friction,
            restitution: self.restitution,
            is_active: self.is_active,
            radius: self.radius,
        }
    }
}

impl ConstraintSaveData {
    pub fn to_constraint(&self) -> Constraint {
        Constraint {
            id: Uuid::parse_str(&self.id).unwrap_or_else(|_| Uuid::new_v4()),
            body1: Uuid::parse_str(&self.body1).unwrap_or_else(|_| Uuid::new_v4()),
            body2: Uuid::parse_str(&self.body2).unwrap_or_else(|_| Uuid::new_v4()),
            constraint_type: self.constraint_type,
            position: self.position,
            stiffness: self.stiffness,
            damping: self.damping,
        }
    }
}

impl PhysicsWorldSaveData {
    pub fn to_world(&self) -> PhysicsWorld {
        PhysicsWorld {
            bodies: self.bodies.iter().map(|b| b.to_body()).collect(),
            constraints: self.constraints.iter().map(|c| c.to_constraint()).collect(),
            gravity: self.gravity,
            dt: self.dt,
            max_velocity: self.max_velocity,
        }
    }
}