//! Nodo de diálogo en Event Forge

use serde::{Deserialize, Serialize};

/// Nodo de diálogo
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DialogueNode {
    /// ID único del nodo
    pub id: String,
    /// Texto del diálogo
    pub dialogue_text: String,
    /// Actor que habla
    pub speaker: String,
    /// Estilo del globo de texto
    pub style: DialogueStyle,
    /// Conexiones de entrada
    pub incoming_connections: Vec<String>,
    /// Conexiones de salida
    pub outgoing_connections: Vec<String>,
}

/// Estilo del globo de diálogo
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum DialogueStyle {
    /// Estilo estándar
    Standard,
    /// Estilo narrativo (cursiva)
    Narrative,
    /// Estilo pensamiento
    Thought,
    /// Estilo gritos
    Shout,
}

impl Default for DialogueStyle {
    fn default() -> Self {
        Self::Standard
    }
}
