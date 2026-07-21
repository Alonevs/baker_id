//! # Keyframe Stub
//! 
//! Módulo temporal para keyframes.

/// Keyframe stub
#[derive(Debug, Clone)]
pub struct Keyframe {
    pub time: f32,
    pub value: f32,
}

/// Keyframe Editor stub
#[derive(Debug, Clone, Default)]
pub struct KeyframeEditor {
    pub keyframes: Vec<Keyframe>,
}

impl KeyframeEditor {
    pub fn new() -> Self {
        Self::default()
    }
    
    pub fn add_keyframe(&mut self, time: f32, value: f32) {
        self.keyframes.push(Keyframe { time, value });
    }
    
    pub fn get_keyframe(&self, time: f32) -> Option<&Keyframe> {
        self.keyframes.iter().find(|k| k.time == time)
    }
}

