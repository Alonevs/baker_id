//! Sockets de audio para entidades

use serde::{Deserialize, Serialize};

/// Socket de audio en una entidad
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioSocket {
    /// ID único del socket
    pub id: String,
    /// Nombre del socket
    pub name: String,
    /// Tipo de socket (Footstep, Attack, Ambient, etc.)
    pub socket_type: AudioSocketType,
    /// Archivo de audio asociado
    pub audio_file: String,
    /// Volumen del socket
    pub volume: f32,
    /// Prioridad (alta para ataques, baja para ambient)
    pub priority: u8,
    /// Estado (active, inactive, playing, stopped)
    pub state: AudioSocketState,
    /// Parámetros adicionales
    pub parameters: serde_json::Value,
}

/// Tipo de socket de audio
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum AudioSocketType {
    /// Pasos del personaje (Footstep)
    Footstep,
    /// Ataque (Attack)
    Attack,
    /// Sonido ambiental (Ambient)
    Ambient,
    /// Sonido de daño (Damage)
    Damage,
    /// Sonido de recolección (Collectible)
    Collectible,
    /// Sonido de uso de item (Use)
    Use,
    /// Sonido de apertura (Open)
    Open,
    /// Sonido de cierre (Close)
    Close,
    /// Sonido de éxito (Success)
    Success,
    /// Sonido de error (Error)
    Error,
    /// Sonido de habilidad (Skill)
    Skill,
    /// Sonido de muerte (Death)
    Death,
}

impl Default for AudioSocketType {
    fn default() -> Self {
        Self::Footstep
    }
}

/// Estado del socket de audio
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum AudioSocketState {
    /// Inactivo
    Inactive,
    /// Activo pero no sonando
    Active,
    /// Reproduciendo
    Playing,
    /// Detenido
    Stopped,
}

impl Default for AudioSocketState {
    fn default() -> Self {
        Self::Inactive
    }
}
