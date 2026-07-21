use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize)]
#[allow(dead_code)]
pub struct CollisionEvent {
    pub body1_id: Uuid,
    pub body2_id: Uuid,
    pub position: [f32; 2],
}

#[allow(dead_code)]
impl CollisionEvent {
    pub fn new(body1_id: Uuid, body2_id: Uuid, position: [f32; 2]) -> Self {
        Self {
            body1_id,
            body2_id,
            position,
        }
    }
}
