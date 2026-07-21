use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct AnimationSystem {
    pub animations: HashMap<String, serde_json::Value>,
}

impl AnimationSystem {
    pub fn new() -> Self {
        Self { animations: HashMap::new() }
    }

    pub fn add(&mut self, name: String, animation: serde_json::Value) {
        self.animations.insert(name, animation);
    }

    pub fn get(&self, name: &str) -> Option<&serde_json::Value> {
        self.animations.get(name)
    }
}

impl Default for AnimationSystem {
    fn default() -> Self {
        Self::new()
    }
}