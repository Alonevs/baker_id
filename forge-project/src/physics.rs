use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct PhysicsSystem {
    pub bodies: HashMap<u64, serde_json::Value>,
}

impl PhysicsSystem {
    pub fn new() -> Self {
        Self { bodies: HashMap::new() }
    }

    pub fn add(&mut self, body_id: u64, body: serde_json::Value) {
        self.bodies.insert(body_id, body);
    }

    pub fn get(&self, body_id: u64) -> Option<&serde_json::Value> {
        self.bodies.get(&body_id)
    }
}

impl Default for PhysicsSystem {
    fn default() -> Self {
        Self::new()
    }
}