//! Audio Resource - Gestión de audio y música

use serde::{Deserialize, Serialize};

// Importar trait Resource del módulo padre
use super::resource_manager::{Resource, ResourceType};

/// Audio Resource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioResource {
    pub name: String,
    pub path: Option<String>,
    pub duration: f32,
    pub channels: u16,
    pub sample_rate: u32,
    pub format: AudioFormat,
    pub volume: f32,
    pub metadata: Option<serde_json::Value>,
}

impl AudioResource {
    /// Crea nuevo audio
    pub fn new(name: &str, duration: f32) -> Self {
        Self {
            name: name.to_string(),
            path: None,
            duration,
            channels: 1,
            sample_rate: 44100,
            format: AudioFormat::Stereo,
            volume: 1.0,
            metadata: None,
        }
    }

    /// Crea audio desde JSON
    pub fn from_json(name: &str, data: &serde_json::Value) -> Result<Self, String> {
        let duration = data["duration"]
            .as_f64()
            .ok_or_else(|| format!("Missing duration in audio: {}", name))?
            as f32;

        let path = data["path"].as_str().map(|s| s.to_string());

        Ok(Self {
            name: name.to_string(),
            path,
            duration,
            channels: data["channels"]
                .as_u64()
                .map(|c| c as u16)
                .unwrap_or(1),
            sample_rate: data["sample_rate"]
                .as_u64()
                .map(|s| s as u32)
                .unwrap_or(44100),
            format: AudioFormat::from_json(&data["format"]),
            volume: data["volume"]
                .as_f64()
                .map(|v| v as f32)
                .unwrap_or(1.0),
            metadata: data.get("metadata").cloned(),
        })
    }

    /// Obtiene duración en segundos
    pub fn duration(&self) -> f32 {
        self.duration
    }

    /// Obtiene volumen actual
    pub fn volume(&self) -> f32 {
        self.volume
    }

    /// Establece volumen
    pub fn set_volume(&mut self, volume: f32) {
        self.volume = volume.clamp(0.0, 1.0);
    }
}

impl Default for AudioResource {
    fn default() -> Self {
        Self::new("default", 30.0)
    }
}

/// Formato de audio
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum AudioFormat {
    Mono,
    Stereo,
    Surround,
}

impl AudioFormat {
    /// Crea formato desde JSON
    pub fn from_json(data: &serde_json::Value) -> Self {
        match data.as_str() {
            Some("mono") | Some("1") => AudioFormat::Mono,
            Some("stereo") | Some("2") => AudioFormat::Stereo,
            Some("surround") | Some("5.1") => AudioFormat::Surround,
            _ => AudioFormat::Mono,
        }
    }

    /// Obtiene número de canales
    pub fn channels(&self) -> u16 {
        match self {
            AudioFormat::Mono => 1,
            AudioFormat::Stereo => 2,
            AudioFormat::Surround => 6,
        }
    }
}

/// Audio Channel para playback
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioChannel {
    pub name: String,
    pub resource: String,
    pub volume: f32,
    pub pitch: f32,
    pub is_playing: bool,
    pub playback_time: f32,
    pub is_loop: bool,
}

impl Default for AudioChannel {
    fn default() -> Self {
        Self {
            name: String::new(),
            resource: String::new(),
            volume: 1.0,
            pitch: 1.0,
            is_playing: false,
            playback_time: 0.0,
            is_loop: false,
        }
    }
}

// Implementación del trait Resource
impl Resource for AudioResource {
    fn name(&self) -> &str { &self.name }
    fn resource_type(&self) -> &ResourceType { &ResourceType::Audio }
    fn load(&mut self) -> Result<(), String> { Ok(()) }
    fn unload(self) where Self: Sized {}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_audio_resource_new() {
        let audio = AudioResource::new("test", 5.0);
        assert_eq!(audio.duration, 5.0);
        assert_eq!(audio.volume, 1.0);
    }

    #[test]
    fn test_audio_resource_default() {
        let audio = AudioResource::default();
        assert_eq!(audio.duration, 30.0);
        assert_eq!(audio.volume, 1.0);
    }

    #[test]
    fn test_audio_format_mono() {
        let format = AudioFormat::Mono;
        assert_eq!(format.channels(), 1);
    }

    #[test]
    fn test_audio_format_stereo() {
        let format = AudioFormat::Stereo;
        assert_eq!(format.channels(), 2);
    }

    #[test]
    fn test_audio_format_surround() {
        let format = AudioFormat::Surround;
        assert_eq!(format.channels(), 6);
    }

    #[test]
    fn test_audio_channel_default() {
        let channel = AudioChannel::default();
        assert!(!channel.is_playing);
        assert_eq!(channel.volume, 1.0);
    }
}
