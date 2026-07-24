use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::{LoopMode, Transform, Keyframe};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnimationClip {
    pub id: Uuid,
    pub name: String,
    pub layers: Vec<AnimationLayer>,
    pub events: Vec<AnimationEvent>,
    pub blend_tree: Option<BlendTree>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnimationLayer {
    pub name: String,
    pub animation: Uuid,
    pub weight: f32,
    pub offset: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnimationEvent {
    pub time: f32,
    pub event_id: Uuid,
    pub parameters: Vec<(String, serde_json::Value)>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlendTree {
    pub name: String,
    pub type_: BlendTreeType,
    pub root: Uuid,
    pub transitions: Vec<Transition>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum BlendTreeType {
    Linear,
    Radial,
    Tree,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transition {
    pub from: Uuid,
    pub to: Uuid,
    pub curve: Vec<(f32, f32)>,
}

impl AnimationClip {
    pub fn new(id: Uuid, name: String) -> Self {
        Self {
            id,
            name,
            layers: Vec::new(),
            events: Vec::new(),
            blend_tree: None,
        }
    }

    pub fn add_layer(&mut self, layer: AnimationLayer) {
        self.layers.push(layer);
    }

    pub fn add_event(&mut self, event: AnimationEvent) {
        self.events.push(event);
    }

    pub fn set_blend_tree(&mut self, blend_tree: BlendTree) {
        self.blend_tree = Some(blend_tree);
    }

    pub fn get_layer(&self, index: usize) -> Option<&AnimationLayer> {
        self.layers.get(index)
    }

    pub fn get_events_at_time(&self, time: f32) -> Vec<&AnimationEvent> {
        self.events.iter().filter(|e| e.time <= time).collect()
    }

    pub fn get_duration(&self) -> Option<f32> {
        if self.layers.is_empty() {
            return None;
        }
        Some(self.layers.iter().map(|l| l.animation).count() as f32)
    }
}

use crate::animation_player::BlendWeight;

impl BlendTree {
    pub fn interpolate(&self, time: f32) -> BlendWeight {
        let duration = self.get_duration();
        let progress = time % duration / duration;

        BlendWeight {
            weight: progress,
            target: self.root,
        }
    }

    pub fn get_keyframe_at_time(&self, _time: f32) -> Option<&Keyframe> {
        None
    }

    pub fn interpolate_transform(&self, _time: f32, _target: Uuid) -> Transform {
        Transform::default()
    }

    pub fn get_duration(&self) -> f32 {
        self.transitions.len() as f32
    }
}

impl Default for AnimationClip {
    fn default() -> Self {
        Self::new(Uuid::nil(), String::new())
    }
}
