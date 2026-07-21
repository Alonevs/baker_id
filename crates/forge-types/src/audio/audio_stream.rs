//! Tipos de audio y canales

use serde::{Deserialize, Serialize};

/// Tipo de canal de audio
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum AudioChannelType {
    /// Canal principal (BGM)
    Bgm,
    /// Efectos de sonido (SFX)
    Sfx,
    /// Voz de diálogo
    Voice,
    /// Música ambiental
    Ambient,
    /// Sonidos de interfaz (UI)
    Ui,
    /// Sonidos de menú
    Menu,
}

impl Default for AudioChannelType {
    fn default() -> Self {
        Self::Bgm
    }
}

/// Stream de audio en el sistema
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioStream {
    /// Nombre del stream
    pub name: String,
    /// Tipo de canal
    pub channel_type: AudioChannelType,
    /// Archivo de audio
    pub audio_file: String,
    /// Volumen (0.0 - 1.0)
    pub volume: f32,
    /// Estado (playing, paused, stopped)
    pub state: AudioState,
    /// Loop (true/false)
    pub is_looping: bool,
    /// Posición 3D (para audio posicional)
    pub position: Option<(f32, f32, f32)>,
    /// Radio de escucha
    pub listen_radius: f32,
}

/// Estado del stream de audio
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum AudioState {
    /// Reproduciendo
    Playing,
    /// Pausado
    Paused,
    /// Detenido
    Stopped,
}

impl Default for AudioState {
    fn default() -> Self {
        Self::Stopped
    }
}
