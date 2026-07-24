//! Comandos desde el editor al runtime

use serde::{Deserialize, Serialize};
use crate::entities::Component;

/// Mensajes desde el editor al runtime
#[derive(Serialize, Deserialize)]
pub enum EditorCommand {
    CreateEntity {
        entity_id: u64,
        components: Vec<Component>,
    },
    UpdateEntity {
        entity_id: u64,
        components: Vec<Component>,
    },
    DeleteEntity {
        entity_id: u64,
    },
    SetGameSpeed(f32),
    TogglePause,
    ExportProject,
}
