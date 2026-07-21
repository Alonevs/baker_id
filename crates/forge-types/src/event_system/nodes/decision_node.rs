//! Nodo de decisión multiopción en Event Forge

use serde::{Deserialize, Serialize};

/// Nodo de decisión con múltiples opciones
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecisionNode {
    /// ID único del nodo
    pub id: String,
    /// Texto de la pregunta/decisión
    pub decision_text: String,
    /// Máximo 4 opciones (para mando clásico)
    pub options: Vec<DecisionOption>,
    /// Conexiones de entrada
    pub incoming_connections: Vec<String>,
    /// Conexiones de salida (una por opción)
    pub outgoing_connections: Vec<String>,
}

/// Opción de decisión
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecisionOption {
    /// Texto de la opción
    pub text: String,
    /// Valor lógico asociado (para condiciones)
    pub value: String,
    /// Conexión de salida
    pub connection_id: String,
}

impl Default for DecisionOption {
    fn default() -> Self {
        Self {
            text: String::new(),
            value: String::new(),
            connection_id: String::new(),
        }
    }
}
