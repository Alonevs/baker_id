//! Comandos desde el editor al runtime

use serde::{Deserialize, Serialize};


/// Mensajes desde el editor al runtime
#[derive(Serialize, Deserialize)]
pub enum EditorCommand {
    CreateEntity {
        entity_id: u64,
    },
    UpdateEntity {
        entity_id: u64,
    },
    DeleteEntity {
        entity_id: u64,
    },
    SetGameSpeed(f32),
    TogglePause,
    ExportProject,
}
