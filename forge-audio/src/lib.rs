//! forge-audio - Sistema de audio completo para Forge SDK 2D
//! 
//! # Módulos
//! - `audio_source` - Fuentes de audio con control de reproducción
//! - `audio_mixer` - Mixer con volumen, pan, pitch
//! - `audio_bus` - Sistema de buses para routing
//! - `audio_effects` - Efectos de audio (reverb, delay, etc.)
//! - `spatial_audio` - Audio espacial 3D
//! - `audio_manager` - Gestión global de audio

pub mod audio_source;
pub mod audio_mixer;
pub mod audio_bus;
pub mod audio_effects;
pub mod spatial_audio;
pub mod audio_manager;
pub mod audio_sample;

pub use audio_source::{AudioSource, SourceState, PlaybackMode};
pub use audio_mixer::{AudioMixer, MixerConfig, MixerChannel};
pub use audio_bus::{AudioBus, AudioBusSystem, BusType};
pub use audio_effects::{AudioEffects, AudioEffect, EffectType};
pub use spatial_audio::{SpatialAudio, SpatialAudioSource, AudioPosition};
pub use audio_manager::AudioManager;
pub use audio_sample::{AudioSample, AudioFormat};
