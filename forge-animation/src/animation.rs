use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::keyframe::{Keyframe, Transform};

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

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum InterpolationType {
    Linear,
    EaseIn,
    EaseOut,
    EaseInOut,
    Step,
}

impl Default for InterpolationType {
    fn default() -> Self {
        InterpolationType::Linear
    }
}

impl InterpolationType {
    pub fn ease_in(t: f32) -> f32 {
        t * t
    }

    pub fn ease_out(t: f32) -> f32 {
        t * (2.0 - t)
    }

    pub fn ease_in_out(t: f32) -> f32 {
        if t < 0.5 {
            2.0 * t * t
        } else {
            -1.0 + (4.0 - 2.0 * t) * t
        }
    }

    pub fn interpolate(&self, a: f32, b: f32, t: f32) -> f32 {
        match self {
            InterpolationType::Linear => a + (b - a) * t,
            InterpolationType::EaseIn => a + (b - a) * Self::ease_in(t),
            InterpolationType::EaseOut => a + (b - a) * Self::ease_out(t),
            InterpolationType::EaseInOut => a + (b - a) * Self::ease_in_out(t),
            InterpolationType::Step => {
                if t >= 0.5 { b } else { a }
            }
        }
    }
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

    pub fn interpolate_transform(&self, time: f32, target: Uuid) -> Transform {
        let t = time % self.duration;
        
        if let (Some(prev), Some(next)) = (
            self.keyframes.iter().rev().find(|k| k.time <= t && k.target == target),
            self.keyframes.iter().find(|k| k.time >= t && k.target == target)
        ) {
            let progress = (t - prev.time).max(0.0) / (next.time - prev.time).max(0.01);
            
            let interp_type = next.interpolation;
            
            let pos_x = interp_type.interpolate(prev.transform.position[0], next.transform.position[0], progress);
            let pos_y = interp_type.interpolate(prev.transform.position[1], next.transform.position[1], progress);
            let pos_z = interp_type.interpolate(prev.transform.position[2], next.transform.position[2], progress);
            
            let rot_x = interp_type.interpolate(prev.transform.rotation[0], next.transform.rotation[0], progress);
            let rot_y = interp_type.interpolate(prev.transform.rotation[1], next.transform.rotation[1], progress);
            let rot_z = interp_type.interpolate(prev.transform.rotation[2], next.transform.rotation[2], progress);
            
            let scl_x = interp_type.interpolate(prev.transform.scale[0], next.transform.scale[0], progress);
            let scl_y = interp_type.interpolate(prev.transform.scale[1], next.transform.scale[1], progress);
            let scl_z = interp_type.interpolate(prev.transform.scale[2], next.transform.scale[2], progress);
            
            Transform {
                position: [pos_x, pos_y, pos_z],
                rotation: [rot_x, rot_y, rot_z],
                scale: [scl_x, scl_y, scl_z],
            }
        } else {
            Transform::default()
        }
    }

    pub fn add_keyframe(&mut self, keyframe: Keyframe) {
        self.keyframes.push(keyframe);
        self.keyframes.sort_by(|a, b| a.time.partial_cmp(&b.time).unwrap());
    }

    pub fn remove_keyframe(&mut self, index: usize) -> Option<Keyframe> {
        Some(self.keyframes.remove(index))
    }

    pub fn get_keyframe_before(&self, time: f32) -> Option<&Keyframe> {
        self.keyframes.iter().rfind(|k| k.time <= time)
    }

    pub fn get_keyframe_after(&self, time: f32) -> Option<&Keyframe> {
        self.keyframes.iter().find(|k| k.time >= time)
    }

    pub fn get_keyframe_at(&self, time: f32) -> Option<&Keyframe> {
        self.keyframes.iter().find(|k| (k.time - time).abs() < 0.01)
    }

    pub fn set_keyframe_interpolation(&mut self, index: usize, interpolation: InterpolationType) {
        if index < self.keyframes.len() {
            self.keyframes[index].interpolation = interpolation;
        }
    }

    pub fn generate_keyframes(&mut self, start_time: f32, end_time: f32, fps: f32, targets: Vec<Uuid>) {
        let step = 1.0 / fps;
        let mut current_time = start_time;

        while current_time <= end_time {
            for target in &targets {
                let keyframe = Keyframe::new(
                    current_time,
                    *target,
                    Transform::default(),
                    0.0,
                );
                self.add_keyframe(keyframe);
            }
            current_time += step;
        }
    }
}

impl Default for Animation {
    fn default() -> Self {
        Self::new(Uuid::nil(), String::new(), 1.0, 30.0)
    }
}
