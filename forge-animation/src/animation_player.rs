use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::animation_clip::AnimationClip;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnimationPlayer {
    pub clips: Vec<AnimationClip>,
    pub current_clip: Option<Uuid>,
    pub state: AnimationState,
    pub active: bool,
    pub auto_play: bool,
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

impl BlendWeight {
    pub fn new(weight: f32, target: Uuid) -> Self {
        Self { weight, target }
    }
}

impl AnimationPlayer {
    pub fn new() -> Self {
        Self {
            clips: Vec::new(),
            current_clip: None,
            state: AnimationState {
                current_time: 0.0,
                current_blend: BlendWeight { weight: 0.0, target: Uuid::nil() },
                playing: false,
                speed: 1.0,
            },
            active: false,
            auto_play: false,
        }
    }

    pub fn add_clip(&mut self, clip: AnimationClip) -> Uuid {
        let id = clip.id;
        self.clips.push(clip);
        id
    }

    pub fn set_clip(&mut self, clip_id: Uuid) {
        self.current_clip = Some(clip_id);
        self.state.current_time = 0.0;
        self.state.playing = false;
    }

    pub fn play(&mut self, clip_id: Option<Uuid>, speed: f32) {
        if let Some(id) = clip_id {
            self.set_clip(id);
        }
        self.state.playing = true;
        self.state.speed = speed;
        self.active = true;
    }

    pub fn pause(&mut self) {
        self.state.playing = false;
        self.active = true;
    }

    pub fn stop(&mut self) {
        self.state.playing = false;
        self.state.current_time = 0.0;
        self.active = false;
    }

    pub fn update(&mut self, delta: f64) {
        if !self.active || !self.state.playing {
            return;
        }

        let dt = delta as f32 * self.state.speed;
        self.state.current_time += dt;

        if let Some(clip) = self.clips.iter().find(|c| c.id == self.current_clip.unwrap()) {
            // Update blend weight
            if let Some(anim) = clip.blend_tree.as_ref() {
                self.state.current_blend = anim.interpolate(self.state.current_time);
            }
        }
    }

    pub fn is_playing(&self) -> bool {
        self.state.playing
    }

    pub fn get_current_time(&self) -> f32 {
        self.state.current_time
    }
}

impl Default for AnimationPlayer {
    fn default() -> Self {
        Self::new()
    }
}
