use crate::scene_node::{SaveError, LoadError};

pub struct ResourceSave {}

impl ResourceSave {
    pub fn save(&self, data: &[u8]) -> Result<(), SaveError> {
        // Implementation
        Ok(())
    }
}
