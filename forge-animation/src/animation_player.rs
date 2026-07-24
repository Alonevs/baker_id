use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::animation::{LoopMode};
use crate::animation_clip::{AnimationClip, AnimationEvent};
use crate::keyframe::Keyframe;


/// AnimationPlayer - Sistema de reproducción de animaciones en tiempo real
/// Permite reproducir, pausar, detener y gestionar múltiples clips de animación
pub struct AnimationPlayer {
    pub clips: Vec<AnimationClip>,
    pub current_clip: Option<Uuid>,
    pub state: AnimationState,
    pub active: bool,
    pub auto_play: bool,
    pub loop_mode: LoopMode,
    pub event_callbacks: Vec<(Uuid, Box<dyn Fn(&AnimationEvent) + Send + Sync>)>,
    pub last_event_time: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnimationState {
    pub current_time: f32,
    pub current_blend: BlendWeight,
    pub playing: bool,
    pub speed: f32,
    pub loop_mode: LoopMode,
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
                loop_mode: LoopMode::Loop,
            },
            active: false,
            auto_play: false,
            loop_mode: LoopMode::Loop,
            event_callbacks: Vec::new(),
            last_event_time: 0.0,
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
        let current_time = self.state.current_time;
        self.state.current_time += dt;

        // Handle loop (after mutable borrows)
        self.handle_loop();

        // Update blend weight and check events
        if let Some(current_clip_id) = self.current_clip {
            let clip = self.clips.iter().find(|c| c.id == current_clip_id);
            if let Some(clip) = clip {
                if let Some(anim) = clip.blend_tree.as_ref() {
                    self.state.current_blend = anim.interpolate(current_time);
                }

                // Check for events
                if self.state.current_time > self.last_event_time {
                    let events = clip.get_events_at_time(self.state.current_time);
                    for event in &events {
                        for callback in &self.event_callbacks {
                            if callback.0 == event.event_id {
                                callback.1(event);
                            }
                        }
                    }
                    self.last_event_time = self.state.current_time;
                }
            }
        }
    }

    pub fn handle_loop(&mut self) {
        if !self.state.playing {
            return;
        }

        if let Some(current_clip_id) = self.current_clip {
            if let Some(clip) = self.clips.iter_mut().find(|c| c.id == current_clip_id) {
                if let Some(duration) = clip.get_duration() {
                    let loop_duration = match self.loop_mode {
                        LoopMode::Loop => duration,
                        LoopMode::None => return, // Stop at end
                        LoopMode::PingPong => duration * 2.0,
                    };

                    if self.state.current_time >= loop_duration {
                        self.state.current_time = 0.0;
                        self.state.playing = false;
                    }
                }
            }
        }
    }



    pub fn add_event_callback(&mut self, event_id: Uuid, callback: Box<dyn Fn(&AnimationEvent) + Send + Sync>) {
        self.event_callbacks.push((event_id, callback));
    }

    pub fn remove_event_callback(&mut self, event_id: Uuid) {
        self.event_callbacks.retain(|(id, _)| *id != event_id);
    }

    pub fn get_current_time(&self) -> f32 {
        self.state.current_time
    }

    pub fn get_clip_duration(&self) -> Option<f32> {
        self.current_clip.and_then(|id| {
            self.clips.iter().find(|c| c.id == id).and_then(|c| c.get_duration())
        })
    }

    pub fn get_total_duration(&self) -> f32 {
        self.clips.iter()
            .map(|c| c.get_duration().unwrap_or(1.0))
            .sum()
    }

    pub fn scrub_to_time(&mut self, time: f32) {
        self.state.current_time = time;
        self.state.playing = false;
    }

    pub fn get_keyframe_at_time(&self, time: f32) -> Option<&Keyframe> {
        if let Some(clip) = self.clips.iter().find(|c| c.id == self.current_clip.unwrap()) {
            if let Some(anim) = clip.blend_tree.as_ref() {
                return anim.get_keyframe_at_time(time);
            }
        }
        None
    }

    pub fn get_transform_at_time(&self, time: f32, target: Uuid) -> Option<crate::keyframe::Transform> {
        if let Some(clip) = self.clips.iter().find(|c| c.id == self.current_clip.unwrap()) {
            if let Some(anim) = clip.blend_tree.as_ref() {
                return Some(anim.interpolate_transform(time, target));
            }
        }
        None
    }

    pub fn get_current_blend(&self) -> &BlendWeight {
        &self.state.current_blend
    }
}

impl Default for AnimationPlayer {
    fn default() -> Self {
        Self::new()
    }
}
