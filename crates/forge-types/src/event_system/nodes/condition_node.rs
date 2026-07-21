//! Nodo de condición en Event Forge

use serde::{Deserialize, Serialize};

/// Nodo de condición para bifurcaciones
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConditionNode {
    /// ID único del nodo
    pub id: String,
    /// Condición a evaluar
    pub condition: ConditionExpression,
    /// Conexiones de entrada
    pub incoming_connections: Vec<String>,
    /// Conexiones de salida (verdadero/falso)
    pub outgoing_connections: Vec<String>,
}

/// Expresión de condición
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ConditionExpression {
    /// Variable booleana
    #[serde(rename = "boolean")]
    Boolean {
        /// Nombre de la variable
        variable: String,
        /// Valor esperado (true/false)
        value: bool,
    },
    /// Variable numérica
    #[serde(rename = "numeric")]
    Numeric {
        /// Nombre de la variable
        variable: String,
        /// Operador
        operator: String,
        /// Valor
        value: f32,
    },
    /// Tiempo del juego
    #[serde(rename = "time")]
    Time {
        /// Operador temporal
        operator: String,
        /// Valor en segundos
        value: f32,
    },
    /// Input del jugador
    #[serde(rename = "input")]
    Input {
        /// Botón presionado
        button: String,
        /// Estado (pressed/hold/release)
        state: String,
    },
}

/// Operadores de comparación
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ConditionOperator {
    Equal,
    NotEqual,
    GreaterThan,
    LessThan,
    GreaterEqual,
    LessEqual,
}

impl Default for ConditionOperator {
    fn default() -> Self {
        Self::Equal
    }
}
