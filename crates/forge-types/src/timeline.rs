//! Módulo de timeline del secuenciador

use serde::{Deserialize, Serialize};

/// Configuración de escala de tiempo
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Default)]
#[repr(u8)]
pub enum TimeScale {
    /// Marcas en segundos (0.0s, 1.5s, 3.0s)
    #[default]
    Seconds = 0,
    /// Marcas en frames (Frame 0, Frame 90, Frame 180)
    Frames = 1,
}

/// Atajos de teclado para mover el cabezal
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Default)]
#[repr(u8)]
pub enum PlayheadHotkey {
    #[default]
    Left = 0,
    Right = 1,
    Up = 2,
    Down = 3,
}

/// Tipo de pista
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Default)]
#[repr(u8)]
pub enum TrackType {
    #[default]
    Audio = 0,
    Video = 1,
    Midi = 2,
    Marker = 3,
}

/// Metadatos de pista
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TrackMetadata {
    pub name: String,
    pub color: String,
}

/// Metadatos de clip
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ClipMetadata {
    pub start_time: f32,
    pub duration: f32,
    pub r#loop: bool,
}

/// Datos de clip
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ClipData {
    pub metadata: ClipMetadata,
    pub content: Vec<u8>,
}

/// Datos de clip comunes
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CommonClipData {
    pub id: String,
    pub name: String,
}

/// Pista de timeline
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimelineTrack {
    pub id: String,
    pub name: String,
    pub track_type: TrackType,
    pub metadata: TrackMetadata,
    pub clips: Vec<TimelineClip>,
}

/// Clip de timeline
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimelineClip {
    pub id: String,
    pub name: String,
    pub clip_type: ClipType,
    pub data: ClipData,
    pub metadata: ClipMetadata,
}

/// Tipo de clip
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Default)]
#[repr(u8)]
pub enum ClipType {
    #[default]
    Audio = 0,
    Video = 1,
    Midi = 2,
    Marker = 3,
}

/// Timeline
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Timeline {
    pub id: String,
    pub name: String,
    pub time_scale: TimeScale,
    pub duration: f32,
    pub tracks: Vec<TimelineTrack>,
    pub common_clips: Vec<CommonClipData>,
}
