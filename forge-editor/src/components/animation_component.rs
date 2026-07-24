//! AnimationComponent del editor

use serde::{Deserialize, Serialize};

/// Componente de animación
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct AnimationComponent {
    /// Nombre de la animación
    pub name: String,
    /// Duración en segundos
    pub duration: f32,
    /// Modo de loop
    pub loop_mode: AnimationLoopMode,
    /// Keyframes
    pub keyframes: Vec<Keyframe>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
pub enum AnimationLoopMode {
    #[default]
    Loop,
    Once,
    PingPong,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Keyframe {
    pub time: f32,
    pub value: f32,
    pub interpolation: KeyframeInterpolation,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
pub enum KeyframeInterpolation {
    #[default]
    Linear,
    EaseIn,
    EaseOut,
    EaseInOut,
    Step,
}

impl AnimationComponent {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_name(name: impl Into<String>) -> Self {
        let mut anim = Self::new();
        anim.name = name.into();
        anim
    }

    pub fn add_keyframe(&mut self, _property: &str, value: f32, time: f32) {
        self.keyframes
            .push(Keyframe {
                time,
                value,
                interpolation: KeyframeInterpolation::default(),
            });
    }

    pub fn get_keyframes(&self, _property: &str) -> Vec<&Keyframe> {
        self.keyframes.iter().collect()
    }

    pub fn set_loop_mode(&mut self, mode: AnimationLoopMode) {
        self.loop_mode = mode;
    }

    pub fn is_looped(&self) -> bool {
        matches!(self.loop_mode, AnimationLoopMode::Loop)
    }
}
