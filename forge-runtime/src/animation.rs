//! AnimationComponent con interpolación de keyframes

use crate::timeline::{Timeline, TimelineEvent};
use std::collections::HashMap;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum LoopMode {
    Loop,
    PingPong,
    Once,
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
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnimationComponent {
    pub name: String,
    pub timeline: Option<Timeline>,
    pub state: AnimationState,
    pub loop_mode: LoopMode,
    pub playback_speed: f32,
    pub properties: HashMap<String, Vec<Keyframe>>,
}

impl AnimationComponent {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            timeline: None,
            state: AnimationState {
                current_time: 0.0,
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

    pub fn play(&mut self) {
        self.state.is_playing = true;
    }

    pub fn pause(&mut self) {
        self.state.is_playing = false;
    }

    pub fn stop(&mut self) {
        self.state.is_playing = false;
        self.state.current_time = 0.0;
        self.state.current_frame = 0;
    }

    pub fn update(&mut self, delta_time: f32) {
        if self.state.is_playing {
            self.state.current_time += delta_time * self.playback_speed;
            self.advance_frame();
        }
    }

    pub fn advance_frame(&mut self) {
        self.state.current_frame += 1;
        if self.state.current_frame >= self.state.total_frames {
            match self.loop_mode {
                LoopMode::Loop => {
                    self.state.current_frame = 0;
                    self.state.current_time = 0.0;
                }
                LoopMode::PingPong => {
                    self.state.is_playing = false;
                }
                LoopMode::Once => {
                    self.state.is_playing = false;
                }
            }
        }
    }

    pub fn set_frame(&mut self, frame: u32) {
        self.state.current_frame = frame;
        self.state.current_time = frame as f32;
    }

    pub fn next_frame(&mut self) {
        if self.state.current_frame < self.state.total_frames {
            self.state.current_frame += 1;
            self.state.current_time = self.state.current_frame as f32;
        }
    }

    pub fn prev_frame(&mut self) {
        if self.state.current_frame > 0 {
            self.state.current_frame -= 1;
            self.state.current_time = self.state.current_frame as f32;
        }
    }

    pub fn get_value(&self, property: &str) -> Option<f32> {
        if let Some(keyframes) = self.properties.get(property) {
            if let Some(frame_keyframes) = keyframes.iter().find(|kf| kf.time as u32 == self.state.current_frame) {
                return Some(frame_keyframes.value);
            }
        }
        None
    }

    pub fn get_interpolated_value(&self, property: &str, frame: u32) -> Option<f32> {
        if let Some(keyframes) = self.properties.get(property) {
            if keyframes.is_empty() {
                return None;
            }

            let current = frame as f32;
            
            if current <= keyframes[0].time {
                return Some(keyframes[0].value);
            }

            if current >= keyframes.last()?.time {
                return Some(keyframes.last()?.value);
            }

            for i in 0..keyframes.len() - 1 {
                let kf1 = &keyframes[i];
                let kf2 = &keyframes[i + 1];

                if current >= kf1.time && current <= kf2.time {
                    let ratio = (current - kf1.time) / (kf2.time - kf1.time);
                    let value = kf1.value + (kf2.value - kf1.value) * ratio;
                    return Some(value);
                }
            }

            None
        } else {
            None
        }
    }

    pub fn apply_frame_events(&self, frame: u32) {
        if let Some(events) = self.timeline.as_ref().and_then(|t| t.events.get(&frame)) {
            for event in events {
                event.apply_frame_events(frame);
            }
        }
    }

    pub fn set_timeline(&mut self, timeline: Timeline) {
        self.timeline = Some(timeline);
        if let Some(t) = &self.timeline {
            self.state.total_frames = t.duration;
        }
    }

    pub fn get_timeline(&self) -> Option<&Timeline> {
        self.timeline.as_ref()
    }

    pub fn get_total_frames(&self) -> u32 {
        self.state.total_frames
    }

    pub fn set_total_frames(&mut self, frames: u32) {
        self.state.total_frames = frames;
    }

    pub fn add_property_keyframes(&mut self, property: &str, keyframes: Vec<Keyframe>) {
        self.properties.insert(property.to_string(), keyframes);
    }

    pub fn get_properties(&self) -> &HashMap<String, Vec<Keyframe>> {
        &self.properties
    }

    pub fn serialize(&self) -> String {
        serde_json::to_string_pretty(self).unwrap_or_default()
    }

    pub fn deserialize(data: &str) -> Option<Self> {
        serde_json::from_str(data).ok()
    }
}

impl Default for AnimationComponent {
    fn default() -> Self {
        Self::new("default")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_animation_component_new() {
        let comp = AnimationComponent::new("test");
        assert_eq!(comp.name, "test");
        assert_eq!(comp.loop_mode, LoopMode::Loop);
    }

    #[test]
    fn test_animation_component_play_pause_stop() {
        let mut comp = AnimationComponent::new("test");
        assert!(!comp.state.is_playing);
        comp.play();
        assert!(comp.state.is_playing);
        comp.pause();
        assert!(!comp.state.is_playing);
        comp.stop();
        assert_eq!(comp.state.current_time, 0.0);
    }

    #[test]
    fn test_animation_component_update() {
        let mut comp = AnimationComponent::new("test");
        comp.set_total_frames(100);
        comp.play();
        comp.update(0.016);
        assert!(comp.state.is_playing);
    }

    #[test]
    fn test_animation_component_set_frame() {
        let mut comp = AnimationComponent::new("test");
        comp.set_frame(10);
        assert_eq!(comp.state.current_frame, 10);
        assert_eq!(comp.state.current_time, 10.0);
    }

    #[test]
    fn test_animation_component_get_interpolated_value() {
        let mut comp = AnimationComponent::new("test");
        let keyframes = vec![
            Keyframe { time: 0.0, value: 0.0, interpolation: "Linear".to_string() },
            Keyframe { time: 10.0, value: 100.0, interpolation: "Linear".to_string() },
        ];
        comp.add_property_keyframes("position_x", keyframes);
        comp.set_total_frames(20);
        
        let value = comp.get_interpolated_value("position_x", 5);
        assert_eq!(value, Some(50.0));
    }

    #[test]
    fn test_animation_component_serialize_deserialize() {
        let comp = AnimationComponent::new("test");
        let json = comp.serialize();
        let loaded = AnimationComponent::deserialize(&json);
        assert!(loaded.is_some());
    }

    #[test]
    fn test_animation_component_loop_modes() {
        let mut comp = AnimationComponent::new("test");
        comp.set_total_frames(100);
        comp.loop_mode = LoopMode::Loop;
        
        comp.play();
        for _ in 0..100 {
            comp.advance_frame();
        }
        // After 100 iterations, current_frame should be 0 (loop triggered once)
        assert_eq!(comp.state.current_frame, 0, "Loop mode should restart at 0");
    }
}
