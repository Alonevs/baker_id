//! Delta Sync - Sincronización incremental

use crate::property_panel::EntityId;

#[derive(Debug, Clone)]
pub struct DeltaEncoder;

impl DeltaEncoder {
    pub fn new() -> Self {
        Self
    }
    
    pub fn encode_deltas(&self, old_state: &EntityId, new_state: &EntityId) -> Vec<u8> {
        Vec::new()
    }
}

#[derive(Debug, Clone)]
pub struct DeltaDecoder;

impl DeltaDecoder {
    pub fn new() -> Self {
        Self
    }
    
    pub fn decode(&self, data: &[u8]) -> Result<EntityId, String> {
        Ok(EntityId::default())
    }
}
