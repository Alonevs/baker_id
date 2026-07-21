//! Datos para proyecto.toml

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub id: Uuid,
    pub name: String,
    pub game_type: GameType,
    pub limits: ProjectLimits,
    pub sdk_version: String,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum GameType {
    Iso,
    Platformer,
    Shooter,
    Adventure,
    Puzzle,
}

impl std::fmt::Display for GameType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GameType::Iso => write!(f, "Iso"),
            GameType::Platformer => write!(f, "Platformer"),
            GameType::Shooter => write!(f, "Shooter"),
            GameType::Adventure => write!(f, "Adventure"),
            GameType::Puzzle => write!(f, "Puzzle"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectLimits {
    pub max_entities: u32,
    pub max_events: u32,
    pub max_audio_tracks: u16,
    pub max_dialogue_nodes: u32,
}

impl Default for Project {
    fn default() -> Self {
        Self {
            id: Uuid::new_v4(),
            name: "New Project".to_string(),
            game_type: GameType::Platformer,
            limits: ProjectLimits {
                max_entities: 500,
                max_events: 1000,
                max_audio_tracks: 16,
                max_dialogue_nodes: 500,
            },
            sdk_version: "0.1.0".to_string(),
        }
    }
}
