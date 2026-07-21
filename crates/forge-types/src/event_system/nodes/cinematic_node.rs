//! Nodo de cinemática en Event Forge

use serde::{Deserialize, Serialize};

/// Nodo de cinemática
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CinematicNode {
    /// ID único del nodo
    pub id: String,
    /// Nombre de la cinemática
    pub cinematic_name: String,
    /// Timeline asociada
    pub timeline_id: String,
    /// Conexiones de entrada
    pub incoming_connections: Vec<String>,
    /// Conexiones de salida
    pub outgoing_connections: Vec<String>,
}
