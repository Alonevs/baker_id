//! Nodo final en Event Forge

use serde::{Deserialize, Serialize};

/// Nodo de fin en Event Forge
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EndNode {
    /// ID único del nodo
    pub id: String,
    /// Tipo de fin
    pub end_type: EndType,
    /// Conexiones de entrada
    pub incoming_connections: Vec<String>,
    /// Conexiones de salida (generalmente vacío)
    pub outgoing_connections: Vec<String>,
}

/// Tipos de fin
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum EndType {
    /// Fin normal
    Normal,
    /// Reiniciar
    Restart,
    /// Cargar mapa
    LoadMap,
    /// Fin de diálogo
    DialogueEnd,
}

impl Default for EndType {
    fn default() -> Self {
        Self::Normal
    }
}
