use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Collider {
    pub id: Uuid,
    pub shape: ColliderShape,
    pub position: [f32; 2],
    pub rotation: f32,
    pub radius: f32,
    pub is_static: bool,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum ColliderShape {
    Circle,
    Box,
    Capsule,
    Polygon,
}

impl Collider {
    pub fn new(id: Uuid, shape: ColliderShape, position: [f32; 2], rotation: f32, radius: f32) -> Self {
        Self {
            id,
            shape,
            position,
            rotation,
            radius,
            is_static: false,
        }
    }

    pub fn is_inside(&self, point: &[f32; 2]) -> bool {
        let dx = point[0] - self.position[0];
        let dy = point[1] - self.position[1];
        let dist_sq = dx.powi(2) + dy.powi(2);
        dist_sq <= self.radius.powi(2)
    }
}

impl Collider {
    pub fn to_collider(&self) -> Collider {
        self.clone()
    }
}
