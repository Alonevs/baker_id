use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::animation::InterpolationType;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Keyframe {
    pub time: f32,
    pub target: Uuid,
    pub transform: Transform,
    pub blend_weight: f32,
    pub interpolation: InterpolationType,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Transform {
    pub position: [f32; 3],
    pub rotation: [f32; 3],
    pub scale: [f32; 3],
}

impl Keyframe {
    pub fn new(time: f32, target: Uuid, transform: Transform, blend_weight: f32) -> Self {
        Self {
            time,
            target,
            transform,
            blend_weight,
            interpolation: InterpolationType::default(),
        }
    }

    pub fn new_with_interpolation(time: f32, target: Uuid, transform: Transform, blend_weight: f32, interpolation: InterpolationType) -> Self {
        Self {
            time,
            target,
            transform,
            blend_weight,
            interpolation,
        }
    }

    pub fn set_interpolation(&mut self, interpolation: InterpolationType) {
        self.interpolation = interpolation;
    }

    pub fn interpolate_value(&self, a: f32, b: f32, t: f32) -> f32 {
        self.interpolation.interpolate(a, b, t)
    }

    pub fn get_interpolation(&self) -> InterpolationType {
        self.interpolation
    }
}

impl Default for Transform {
    fn default() -> Self {
        Self {
            position: [0.0, 0.0, 0.0],
            rotation: [0.0, 0.0, 0.0],
            scale: [1.0, 1.0, 1.0],
        }
    }
}
