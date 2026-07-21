//! Canales de audio individuales

use serde::{Deserialize, Serialize};

/// Canal de audio individual
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioChannel {
    /// ID único del canal
    pub id: String,
    /// Nombre del canal
    pub name: String,
    /// Tipo de canal (BGM, SFX, Voice, etc.)
    pub channel_type: crate::audio::AudioChannelType,
    /// Volumen (0.0 - 1.0)
    pub volume: f32,
    /// Estado (playing, paused, stopped)
    pub state: crate::audio::AudioState,
    /// Loop (true/false)
    pub is_looping: bool,
    /// Listeners activos en este canal
    pub listeners: Vec<String>,
    /// Lista de streams en este canal
    pub streams: Vec<crate::audio::AudioStream>,
}

impl Default for AudioChannel {
    fn default() -> Self {
        Self {
            id: String::new(),
            name: String::new(),
            channel_type: crate::audio::AudioChannelType::default(),
            volume: 1.0,
            state: crate::audio::AudioState::default(),
            is_looping: false,
            listeners: Vec::new(),
            streams: Vec::new(),
        }
    }
}

/// Sistema de audio posicional (3D)
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PositionalAudio {
    /// Posición del sonido en el mundo
    pub position: (f32, f32, f32),
    /// Rotación (para efectos direccionales)
    pub rotation: (f32, f32),
    /// Radio de escucha
    pub listen_radius: f32,
    /// Radio de escucha máximo
    pub max_listen_radius: f32,
    /// Atenuación (1.0 = sin atenuación)
    pub attenuation: f32,
    /// Volumen base
    pub base_volume: f32,
}
