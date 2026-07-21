//! Módulo de tracks del secuenciador

use serde::{Deserialize, Serialize};

use super::{TrackType, TimelineClip, TrackMetadata};

/// Track individual con clips
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Track {
    pub track_type: TrackType,
    pub name: String,
    pub metadata: TrackMetadata,
    pub clips: Vec<TimelineClip>,
}

impl Default for Track {
    fn default() -> Self {
        Self {
            track_type: TrackType::Audio,
            name: String::new(),
            metadata: TrackMetadata::default(),
            clips: Vec::new(),
        }
    }
}
