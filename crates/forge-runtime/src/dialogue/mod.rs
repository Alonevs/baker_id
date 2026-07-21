//! Runtime de Diálogos y Eventos

pub use dialogue::{Dialogue, DialogueLine, DialogueSystem, DialogueType};
pub use timeline::{DialogueTimeline, DialogueTimelineEvent};
pub use event_system::{DialogueEvent, DialogueEventSystem, DialogueEventType};

pub mod dialogue;
pub mod timeline;
pub mod event_system;
