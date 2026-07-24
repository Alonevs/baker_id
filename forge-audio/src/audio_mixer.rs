use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Configuración del mixer
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MixerConfig {
    pub master_volume: f32,
    pub sample_rate: u32,
    pub channels: u16,
    pub buffer_size: u32,
}

impl Default for MixerConfig {
    fn default() -> Self {
        Self {
            master_volume: 1.0,
            sample_rate: 44100,
            channels: 2,
            buffer_size: 1024,
        }
    }
}

/// Canal de mezcla
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MixerChannel {
    pub id: String,
    pub name: String,
    pub volume: f32,
    pub pan: f32,
    pub pitch: f32,
    pub muted: bool,
    pub solo: bool,
    pub bus: Option<String>,
}

impl MixerChannel {
    pub fn new(id: String, name: String) -> Self {
        Self {
            id,
            name,
            volume: 1.0,
            pan: 0.0,
            pitch: 1.0,
            muted: false,
            solo: false,
            bus: None,
        }
    }

    pub fn set_volume(&mut self, volume: f32) {
        self.volume = volume.clamp(0.0, 1.0);
    }

    pub fn set_pan(&mut self, pan: f32) {
        self.pan = pan.clamp(-1.0, 1.0);
    }

    pub fn set_pitch(&mut self, pitch: f32) {
        self.pitch = pitch.max(0.1);
    }

    pub fn toggle_mute(&mut self) {
        self.muted = !self.muted;
    }

    pub fn toggle_solo(&mut self) {
        self.solo = !self.solo;
    }
}

/// Mixer de audio con múltiples canales
#[derive(Debug, Clone)]
pub struct AudioMixer {
    pub config: MixerConfig,
    pub channels: HashMap<String, MixerChannel>,
    pub master_volume: f32,
    pub output_buffer: Vec<f32>,
}

impl AudioMixer {
    pub fn new(config: Option<MixerConfig>) -> Self {
        let config = config.unwrap_or_default();
        Self {
            config: config.clone(),
            channels: HashMap::new(),
            master_volume: 1.0,
            output_buffer: Vec::with_capacity(config.buffer_size as usize),
        }
    }

    pub fn add_channel(&mut self, channel: MixerChannel) {
        self.channels.insert(channel.id.clone(), channel);
    }

    pub fn get_channel(&self, id: &str) -> Option<&MixerChannel> {
        self.channels.get(id)
    }

    pub fn get_channel_mut(&mut self, id: &str) -> Option<&mut MixerChannel> {
        self.channels.get_mut(id)
    }

    pub fn remove_channel(&mut self, id: &str) {
        self.channels.remove(id);
    }

    pub fn set_master_volume(&mut self, volume: f32) {
        self.master_volume = volume.clamp(0.0, 1.0);
    }

    pub fn mix(&mut self, _delta: f32) {
        let mut total_volume = 0.0;

        for channel in self.channels.values() {
            if channel.muted || channel.solo {
                continue;
            }

            let channel_volume = channel.volume * self.master_volume;
            total_volume += channel_volume;
        }

        // Mix channels into output buffer
        self.output_buffer.clear();
        for _ in 0..self.config.buffer_size {
            let mut sample = 0.0;
            for channel in self.channels.values() {
                if !channel.muted && !channel.solo {
                    sample += 0.5; // Simplified mixing
                }
            }
            self.output_buffer.push(sample * total_volume);
        }
    }

    pub fn play(&mut self) {
        self.mix(0.0);
    }

    pub fn get_channel_count(&self) -> usize {
        self.channels.len()
    }

    pub fn get_master_volume(&self) -> f32 {
        self.master_volume
    }
}

impl Default for AudioMixer {
    fn default() -> Self {
        Self::new(None)
    }
}
