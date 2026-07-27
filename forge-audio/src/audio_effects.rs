use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Tipo de efecto de audio
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum EffectType {
    /// Reverb (reverberación)
    Reverb,
    /// Delay (eco)
    Delay,
    /// Chorus (coro)
    Chorus,
    /// Flanger
    Flanger,
    /// Phaser
    Phaser,
    /// Distorsión
    Distortion,
    /// Compresor
    Compressor,
    /// Limiter
    Limiter,
    /// EQ (equalizador)
    Equalizer,
    /// Lowpass
    Lowpass,
    /// Highpass
    Highpass,
    /// Bandpass
    Bandpass,
}

impl Default for EffectType {
    fn default() -> Self {
        Self::Reverb
    }
}

impl std::fmt::Display for EffectType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Reverb => write!(f, "Reverb"),
            Self::Delay => write!(f, "Delay"),
            Self::Chorus => write!(f, "Chorus"),
            Self::Flanger => write!(f, "Flanger"),
            Self::Phaser => write!(f, "Phaser"),
            Self::Distortion => write!(f, "Distortion"),
            Self::Compressor => write!(f, "Compressor"),
            Self::Limiter => write!(f, "Limiter"),
            Self::Equalizer => write!(f, "Equalizer"),
            Self::Lowpass => write!(f, "Lowpass"),
            Self::Highpass => write!(f, "Highpass"),
            Self::Bandpass => write!(f, "Bandpass"),
        }
    }
}

/// Configuración de reverb
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReverbConfig {
    pub wet: f32,
    pub dry: f32,
    pub decay: f32,
    pub room_size: f32,
    pub damping: f32,
}

impl Default for ReverbConfig {
    fn default() -> Self {
        Self {
            wet: 0.5,
            dry: 1.0,
            decay: 2.0,
            room_size: 0.5,
            damping: 0.5,
        }
    }
}

/// Configuración de delay
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DelayConfig {
    pub time: f32,
    pub feedback: f32,
    pub wet: f32,
    pub dry: f32,
}

impl Default for DelayConfig {
    fn default() -> Self {
        Self {
            time: 0.3,
            feedback: 0.5,
            wet: 0.5,
            dry: 1.0,
        }
    }
}

/// Configuración de chorus
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChorusConfig {
    pub rate: f32,
    pub depth: f32,
    pub delay: f32,
    pub wet: f32,
    pub dry: f32,
}

impl Default for ChorusConfig {
    fn default() -> Self {
        Self {
            rate: 0.5,
            depth: 0.25,
            delay: 0.025,
            wet: 0.5,
            dry: 1.0,
        }
    }
}

/// Configuración de efectos
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EffectConfig {
    pub effect_type: EffectType,
    pub params: HashMap<String, f32>,
}

impl EffectConfig {
    pub fn new(effect_type: EffectType) -> Self {
        let mut params = HashMap::new();
        params.insert("wet".to_string(), 0.5);
        params.insert("dry".to_string(), 1.0);
        Self {
            effect_type,
            params,
        }
    }

    pub fn set_param(&mut self, key: String, value: f32) {
        self.params.insert(key, value);
    }
}

/// Efecto de audio
#[derive(Debug, Clone)]
pub struct AudioEffect {
    pub id: String,
    pub name: String,
    pub effect_type: EffectType,
    pub config: EffectConfig,
    pub enabled: bool,
    pub bypassed: bool,
}

impl AudioEffect {
    pub fn new(id: String, name: String, effect_type: EffectType) -> Self {
        Self {
            id,
            name,
            effect_type,
            config: EffectConfig::new(effect_type),
            enabled: true,
            bypassed: false,
        }
    }

    pub fn set_param(&mut self, key: String, value: f32) {
        self.config.params.insert(key, value);
    }

    pub fn toggle_bypass(&mut self) {
        self.bypassed = !self.bypassed;
    }
}

/// Sistema de efectos de audio
#[derive(Debug, Clone)]
pub struct AudioEffects {
    pub effects: HashMap<String, AudioEffect>,
}

impl AudioEffects {
    pub fn new() -> Self {
        Self {
            effects: HashMap::new(),
        }
    }

    pub fn add_effect(&mut self, effect: AudioEffect) {
        self.effects.insert(effect.id.clone(), effect);
    }

    pub fn get_effect(&self, id: &str) -> Option<&AudioEffect> {
        self.effects.get(id)
    }

    pub fn get_effect_mut(&mut self, id: &str) -> Option<&mut AudioEffect> {
        self.effects.get_mut(id)
    }

    pub fn remove_effect(&mut self, id: &str) {
        self.effects.remove(id);
    }

    pub fn process_sample(&self, input: f32) -> f32 {
        if self.effects.is_empty() {
            return input;
        }

        let mut output = input;
        for effect in self.effects.values() {
            if effect.enabled && !effect.bypassed {
                output = Self::apply_effect(effect, output);
            }
        }
        output
    }

    fn apply_effect(effect: &AudioEffect, input: f32) -> f32 {
        match effect.effect_type {
            EffectType::Reverb => {
                // Simplified reverb - devuelve input con atenuación
                input * effect.config.params.get("wet").copied().unwrap_or(0.5)
            }
            EffectType::Delay => {
                // Simplified delay - devuelve input con eco
                input * effect.config.params.get("wet").copied().unwrap_or(0.5)
            }
            EffectType::Chorus => {
                // Simplified chorus - devuelve input modulado
                input * effect.config.params.get("wet").copied().unwrap_or(0.5)
            }
            EffectType::Distortion => {
                // Simplified distortion - clips el input
                input.clamp(-1.0, 1.0) * 2.0
            }
            EffectType::Compressor => {
                // Simplified compressor - atenua picos
                input.max(-0.5).min(0.5)
            }
            EffectType::Limiter => {
                // Simplified limiter - limita el input
                input.clamp(-1.0, 1.0)
            }
            _ => input,
        }
    }

    pub fn get_effect_count(&self) -> usize {
        self.effects.len()
    }
}

impl Default for AudioEffects {
    fn default() -> Self {
        Self::new()
    }
}
