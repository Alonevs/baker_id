//! Cuaderno de Diseño GDD y Tablero de Ruta (Roadmap)

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Nota del Cuaderno GDD
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GDDNote {
    pub id: Uuid,
    pub title: String,
    pub content: String,
    pub created_at: String,
    pub updated_at: String,
    pub tags: Vec<String>,
    pub is_link: bool,
    pub linked_roadmap_task: Option<Uuid>,
}

/// Tarea del roadmap (hitos de desarrollo)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoadmapTask {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub status: TaskStatus,
    pub priority: TaskPriority,
    pub milestone: u32,
    pub linked_map: Option<String>,
    pub coordinates: Option<MapCoordinates>,
    pub due_date: Option<String>,
}

impl RoadmapTask {
    pub fn new(id: Uuid, title: impl Into<String>, milestone: u32) -> Self {
        Self {
            id,
            title: title.into(),
            description: String::new(),
            status: TaskStatus::Todo,
            priority: TaskPriority::Medium,
            milestone,
            linked_map: None,
            coordinates: None,
            due_date: None,
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum TaskStatus {
    Todo,
    InProgress,
    Done,
    Blocked,
}

impl std::fmt::Display for TaskStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TaskStatus::Todo => write!(f, "⏳ Todo"),
            TaskStatus::InProgress => write!(f, "🔄 En Progreso"),
            TaskStatus::Done => write!(f, "✅ Completado"),
            TaskStatus::Blocked => write!(f, "🚧 Bloqueado"),
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum TaskPriority {
    Low,
    Medium,
    High,
    Critical,
}

impl std::fmt::Display for TaskPriority {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TaskPriority::Low => write!(f, "Low"),
            TaskPriority::Medium => write!(f, "Medium"),
            TaskPriority::High => write!(f, "High"),
            TaskPriority::Critical => write!(f, "Critical"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MapCoordinates {
    pub x: f32,
    pub y: f32,
}

/// Bitácora de escenas de un personaje
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterSceneLog {
    pub character_id: String,
    pub entries: Vec<CharacterSceneEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterSceneEntry {
    pub scene_id: String,
    pub map_name: String,
    pub coordinates: MapCoordinates,
    pub dialogue_id: Option<String>,
    pub dialogue_text: Option<String>,
    pub notes: Vec<RevisionNote>,
    pub timestamp: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RevisionNote {
    pub note_id: Uuid,
    pub text: String,
    pub is_todo: bool,
    pub is_review: bool,
    pub author: Option<String>,
}

/// Bitácora global del proyecto (guion consolidado)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalScriptLog {
    pub entries: Vec<GlobalScriptEntry>,
    pub last_updated: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalScriptEntry {
    pub dialogue_id: String,
    pub character_id: String,
    pub map_name: String,
    pub text: String,
    pub is_editable: bool,
    pub linked_node_id: Option<Uuid>,
    pub metadata: ScriptEntryMetadata,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ScriptEntryMetadata {
    pub checkpoint: u32,
    pub progress_step: u32,
    pub flags: Vec<String>,
}

/// Ruta del roadmap en formato TOML
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Roadmap {
    pub version: u32,
    pub tasks: Vec<RoadmapTask>,
    pub milestones: Vec<Milestone>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Milestone {
    pub number: u32,
    pub title: String,
    pub description: String,
    pub tasks: Vec<Uuid>,
    pub target_date: Option<String>,
}

/// Notas de revisión en el inspector del personaje
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterNote {
    pub note_id: Uuid,
    pub text: String,
    pub is_todo: bool,
    pub is_review: bool,
    pub linked_scene: Option<String>,
}

/// Estado del indexador en tiempo real
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexerState {
    pub maps_indexed: u32,
    pub events_indexed: u32,
    pub dialogues_indexed: u32,
    pub last_scan_time: String,
    pub pending_changes: Vec<ChangePending>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChangePending {
    pub file_path: String,
    pub change_type: ChangeType,
    pub timestamp: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ChangeType {
    MapChanged,
    EventGraphChanged,
    DialogueChanged,
    AssetChanged,
}
