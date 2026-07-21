//! Secuenciador de eventos y timeline del juego

pub mod tracks;
pub mod commands;
pub mod vfx;

pub use crate::timeline::{
    Timeline, TimelineTrack, TrackType, TimelineClip, TrackMetadata, ClipData,
    ClipMetadata, TimeScale, PlayheadHotkey, CommonClipData, ClipType,
};
pub use commands::{CommandSignal, CommandType};
pub use vfx::{CameraEffect, CameraEffectType, ScreenVFX, VFXType, TextOverlay, TextEffect};
