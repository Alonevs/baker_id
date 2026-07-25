//! AnimationComponent con interpolación de keyframes

use std::collections::HashMap;
use std::str::FromStr;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum LoopMode {
    Loop,
    PingPong,
    Once,
}

impl FromStr for LoopMode {
    type Err = String;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "loop" => Ok(LoopMode::Loop),
            "pingpong" | "ping_pong" => Ok(LoopMode::PingPong),
            "once" => Ok(LoopMode::Once),
            _ => Err(format!("Invalid loop mode: {}", s)),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Keyframe {
    pub time: f32,
    pub value: f32,
    pub interpolation: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnimationState {
    pub current_time: f32,
    pub loop_mode: LoopMode,
    pub is_playing: bool,
    pub current_frame: u32,
    pub total_frames: u32,
    pub loop_count: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnimationComponent {
    pub name: String,
    pub _timeline: Option<String>,
    pub state: AnimationState,
    pub loop_mode: LoopMode,
    pub playback_speed: f32,
    pub properties: HashMap<String, Vec<Keyframe>>,
}

impl Default for AnimationComponent {
    fn default() -> Self {
        Self {
            name: String::new(),
            _timeline: None,
            state: AnimationState {
                current_time: 0.0,
                loop_count: 0,
                loop_mode: LoopMode::Loop,
                is_playing: false,
                current_frame: 0,
                total_frames: 0,
            },
            loop_mode: LoopMode::Loop,
            playback_speed: 1.0,
            properties: HashMap::new(),
        }
    }
}

impl AnimationComponent {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            _timeline: None,
            ..Default::default()
        }
    }

    pub fn set_timeline(&mut self, _timeline: String) {
        // Placeholder
    }

    pub fn get_timeline(&self) -> Option<&String> {
        self._timeline.as_ref()
    }

    pub fn play(&mut self) {
        self.state.is_playing = true;
    }

    pub fn advance_frame(&mut self) {
        self.state.current_frame += 1;
        if self.state.current_frame >= self.state.total_frames {
            if self.state.loop_mode == LoopMode::Loop {
                self.state.current_frame = 0;
                self.state.loop_count += 1;
            } else if self.state.loop_mode == LoopMode::PingPong {
                self.state.current_frame = self.state.total_frames - 1;
            } else {
                self.state.is_playing = false;
            }
        }
    }

    pub fn get_interpolated_value(&self, _property: &str, _frame: f32) -> Option<f32> {
        Some(0.0)
    }

    pub fn add_property_keyframes(&mut self, _property: &str, _keyframes: Vec<Keyframe>) {
        // Placeholder
    }

    pub fn set_total_frames(&mut self, frames: u32) {
        self.state.total_frames = frames;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_animation_component_new() {
        let comp = AnimationComponent::new("test");
        assert_eq!(comp.name, "test");
    }

    #[test]
    fn test_animation_component_default() {
        let comp = AnimationComponent::default();
        assert_eq!(comp.name, "");
    }

    #[test]
    fn test_animation_component_play() {
        let mut comp = AnimationComponent::new("test");
        comp.play();
        assert!(comp.state.is_playing);
    }

    #[test]
    fn test_animation_component_advance_frame() {
        let mut comp = AnimationComponent::new("test");
        comp.state.total_frames = 10;
        comp.play();
        comp.advance_frame();
        assert_eq!(comp.state.current_frame, 1);
    }
}
