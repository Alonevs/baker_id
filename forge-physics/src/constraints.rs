use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Constraint {
    pub id: Uuid,
    pub body1: Uuid,
    pub body2: Uuid,
    pub constraint_type: ConstraintType,
    pub position: [f32; 2],
    pub stiffness: f32,
    pub damping: f32,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum ConstraintType {
    Fixed,
    Hinge,
    Distance,
    Spring,
}

impl Constraint {
    pub fn new(
        id: Uuid,
        body1: Uuid,
        body2: Uuid,
        constraint_type: ConstraintType,
        position: [f32; 2],
        stiffness: f32,
        damping: f32,
    ) -> Self {
        Self {
            id,
            body1,
            body2,
            constraint_type,
            position,
            stiffness,
            damping,
        }
    }

    pub fn update(&self, _body1: &[f32; 2], _body2: &[f32; 2], _dt: f32) {
        // Implementation pending
    }
    
    pub fn to_constraint(&self) -> Constraint {
        self.clone()
    }
}
