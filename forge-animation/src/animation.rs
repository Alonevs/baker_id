use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::keyframe::Keyframe;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Animation {
    pub id: Uuid,
    pub name: String,
    pub duration: f32,
    pub fps: f32,
    pub blend_space: bool,
    pub loop_mode: LoopMode,
    pub keyframes: Vec<Keyframe>,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum LoopMode {
    None,
    Loop,
    PingPong,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnimationState {
    pub current_time: f32,
    pub current_blend: BlendWeight,
    pub playing: bool,
    pub speed: f32,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct BlendWeight {
    pub weight: f32,
    pub target: Uuid,
}

impl Animation {
    pub fn new(id: Uuid, name: String, duration: f32, fps: f32) -> Self {
        Self {
            id,
            name,
            duration,
            fps,
            blend_space: false,
            loop_mode: LoopMode::Loop,
            keyframes: Vec::new(),
        }
    }

    pub fn add_keyframe(&mut self, keyframe: Keyframe) {
        self.keyframes.push(keyframe);
    }

    pub fn get_keyframe_at_time(&self, time: f32) -> Option<&Keyframe> {
        self.keyframes.iter().find(|k| k.time >= time)
    }

    pub fn interpolate(&self, time: f32) -> BlendWeight {
        let t = time % self.duration;
        
        if let (Some(prev), Some(next)) = (
            self.keyframes.iter().rev().find(|k| k.time <= t),
            self.keyframes.iter().find(|k| k.time >= t)
        ) {
            let progress = (t - prev.time) / (next.time - prev.time);
            BlendWeight {
                weight: progress,
                target: next.target,
            }
        } else {
            BlendWeight {
                weight: 0.0,
                target: Uuid::nil(),
            }
        }
    }
}

impl Default for Animation {
    fn default() -> Self {
        Self::new(Uuid::nil(), String::new(), 1.0, 30.0)
    }
}
