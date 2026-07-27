use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Representación de una muestra de audio
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioSample {
    pub id: String,
    pub name: String,
    pub path: PathBuf,
    pub format: AudioFormat,
    pub duration: f32,
    pub sample_rate: u32,
    pub channels: u16,
    pub data: Vec<u8>,
}

impl AudioSample {
    pub fn new(id: String, name: String, path: PathBuf) -> Self {
        Self {
            id,
            name,
            path,
            format: AudioFormat::default(),
            duration: 0.0,
            sample_rate: 44100,
            channels: 1,
            data: Vec::new(),
        }
    }

    pub fn set_data(&mut self, data: Vec<u8>) {
        self.data = data;
    }

    pub fn set_duration(&mut self, duration: f32) {
        self.duration = duration;
    }
}

/// Formato de audio soportado
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum AudioFormat {
    /// WAV - Puro, sin compresión
    Wav,
    /// MP3 - Comprimido
    Mp3,
    /// OGG - Comprimido con Vorbis
    Ogg,
    /// FLAC - Sin pérdidas
    Flac,
    /// AIFF - Formato de intercambio de audio
    Aiff,
}

impl Default for AudioFormat {
    fn default() -> Self {
        Self::Wav
    }
}

impl std::fmt::Display for AudioFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Wav => write!(f, "WAV"),
            Self::Mp3 => write!(f, "MP3"),
            Self::Ogg => write!(f, "OGG"),
            Self::Flac => write!(f, "FLAC"),
            Self::Aiff => write!(f, "AIFF"),
        }
    }
}

/// Canal de audio (mono, stereo, surround)
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum AudioChannel {
    Mono,
    Stereo,
    Surround51,
    Surround71,
}

impl Default for AudioChannel {
    fn default() -> Self {
        Self::Stereo
    }
}

impl std::fmt::Display for AudioChannel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Mono => write!(f, "Mono"),
            Self::Stereo => write!(f, "Stereo"),
            Self::Surround51 => write!(f, "Surround51"),
            Self::Surround71 => write!(f, "Surround71"),
        }
    }
}
