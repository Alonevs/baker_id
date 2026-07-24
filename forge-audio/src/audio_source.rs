use serde::{Deserialize, Serialize};
use crate::audio_sample::AudioSample;

/// Estado de reproducción de una fuente de audio
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum SourceState {
    /// Detenido
    Stopped,
    /// Reproduciendo
    Playing,
    /// Pausado
    Paused,
    /// Terminado
    Finished,
}

/// Modo de reproducción
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum PlaybackMode {
    /// Una sola pasada
    Once,
    /// Repetir indefinidamente
    Loop,
    /// Ping-pong (adelante-atrás)
    PingPong,
}

/// Fuente de audio con control de reproducción
pub struct AudioSource {
    pub sample: AudioSample,
    pub state: SourceState,
    pub playback_mode: PlaybackMode,
    pub position: f32,
    pub total_duration: f32,
    pub volume: f32,
    pub pitch: f32,
    pub pan: f32,
    pub loop_start: f32,
    pub loop_end: f32,
    pub callback: Option<Box<dyn Fn() + Send + Sync + 'static>>,
}

impl std::fmt::Debug for AudioSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AudioSource")
            .field("sample", &self.sample)
            .field("state", &self.state)
            .field("playback_mode", &self.playback_mode)
            .field("position", &self.position)
            .field("total_duration", &self.total_duration)
            .field("volume", &self.volume)
            .field("pitch", &self.pitch)
            .field("pan", &self.pan)
            .field("loop_start", &self.loop_start)
            .field("loop_end", &self.loop_end)
            .field("callback", &"...")
            .finish()
    }
}

impl AudioSource {
    pub fn new(sample: AudioSample) -> Self {
        Self {
            sample,
            state: SourceState::Stopped,
            playback_mode: PlaybackMode::Once,
            position: 0.0,
            total_duration: 0.0,
            volume: 1.0,
            pitch: 1.0,
            pan: 0.0,
            loop_start: 0.0,
            loop_end: 0.0,
            callback: None,
        }
    }

    pub fn set_volume(&mut self, volume: f32) {
        self.volume = volume.clamp(0.0, 1.0);
    }

    pub fn set_pitch(&mut self, pitch: f32) {
        self.pitch = pitch.max(0.1);
    }

    pub fn set_pan(&mut self, pan: f32) {
        self.pan = pan.clamp(-1.0, 1.0);
    }

    pub fn set_loop(&mut self, start: f32, end: f32) {
        self.loop_start = start;
        self.loop_end = end;
    }

    pub fn play(&mut self, callback: Option<Box<dyn Fn() + Send + Sync>>) {
        self.state = SourceState::Playing;
        self.position = 0.0;
        self.callback = callback;
    }

    pub fn pause(&mut self) {
        self.state = SourceState::Paused;
    }

    pub fn stop(&mut self) {
        self.state = SourceState::Stopped;
        self.position = 0.0;
        self.callback = None;
    }

    pub fn update(&mut self, delta: f32, sample_rate: u32) {
        if self.state != SourceState::Playing {
            return;
        }

        let samples_per_second = sample_rate as f32;
        let samples_per_frame = delta * samples_per_second;
        self.position += samples_per_frame * self.pitch;

        // Check for loop
        if self.loop_end > 0.0 && self.position >= self.loop_end {
            if self.playback_mode == PlaybackMode::Loop {
                self.position = self.loop_start;
            } else {
                self.state = SourceState::Finished;
            }
        } else if self.playback_mode == PlaybackMode::Loop {
            if self.position >= self.total_duration {
                self.position = self.loop_start;
            }
        }

        // Check for finish
        if self.position >= self.total_duration && self.playback_mode != PlaybackMode::Loop {
            self.state = SourceState::Finished;
        }

        // Execute callback on frames
        if self.callback.is_some() && self.position > 0.0 {
            (self.callback.as_ref().unwrap())();
        }
    }

    pub fn is_playing(&self) -> bool {
        self.state == SourceState::Playing
    }

    pub fn is_paused(&self) -> bool {
        self.state == SourceState::Paused
    }

    pub fn get_position(&self) -> f32 {
        self.position
    }

    pub fn get_duration(&self) -> f32 {
        self.total_duration
    }

    pub fn get_volume(&self) -> f32 {
        self.volume
    }

    pub fn get_pitch(&self) -> f32 {
        self.pitch
    }

    pub fn get_pan(&self) -> f32 {
        self.pan
    }
}

impl Default for AudioSource {
    fn default() -> Self {
        Self::new(AudioSample::new(String::new(), String::new(), std::path::PathBuf::new()))
    }
}
