//! # Serialization Panel API
//! 
//! Módulo para panel de serialización.


/// Serialization Panel - panel para serialización
#[derive(Default)]
pub struct SerializationPanel {
    pub data: serde_json::Value,
}

impl SerializationPanel {
    pub fn new() -> Self {
        Self::default()
    }
}

