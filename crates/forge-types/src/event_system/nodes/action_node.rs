//! Nodo de acción en Event Forge

use serde::{Deserialize, Serialize};

/// Nodo de acción
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionNode {
    /// ID único del nodo
    pub id: String,
    /// Nombre de la acción
    pub action_name: String,
    /// Tipo de acción
    pub action_type: ActionType,
    /// Parámetros de la acción
    pub parameters: serde_json::Value,
    /// Conexiones de entrada
    pub incoming_connections: Vec<String>,
    /// Conexiones de salida
    pub outgoing_connections: Vec<String>,
}

/// Tipos de acciones
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ActionType {
    /// Dar item
    GiveItem,
    /// Cambiar estado
    ChangeState,
    /// Reproducir animación
    PlayAnimation,
    /// Activar trigger
    ActivateTrigger,
    /// Cambiar música
    ChangeMusic,
    /// Guardar partida
    SaveGame,
    /// Cargar partida
    LoadGame,
}

impl Default for ActionType {
    fn default() -> Self {
        Self::GiveItem
    }
}
