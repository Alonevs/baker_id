//! Delta Sync - Sincronización incremental

use crate::property_panel::EntityId;

#[derive(Debug, Clone)]
pub struct DeltaEncoder;

impl DeltaEncoder {
    pub fn new() -> Self {
        Self
    }
    
    pub fn encode_deltas(&self, _old_state: &EntityId, _new_state: &EntityId) -> Vec<u8> {
        Vec::new()
    }
}

#[derive(Debug, Clone)]
pub struct DeltaDecoder;

impl DeltaDecoder {
    pub fn new() -> Self {
        Self
    }
    
    pub fn decode(&self, _data: &[u8]) -> Result<EntityId, String> {
        Ok(EntityId::default())
    }
}
