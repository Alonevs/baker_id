use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct AudioManager {
    pub sounds: HashMap<String, serde_json::Value>,
    pub music: HashMap<String, serde_json::Value>,
}

impl AudioManager {
    pub fn new() -> Self {
        Self { sounds: HashMap::new(), music: HashMap::new() }
    }

    pub fn add_sound(&mut self, name: String, sound: serde_json::Value) {
        self.sounds.insert(name, sound);
    }

    pub fn add_music(&mut self, name: String, music: serde_json::Value) {
        self.music.insert(name, music);
    }

    pub fn play(&self, name: &str) -> bool {
        self.sounds.contains_key(name) || self.music.contains_key(name)
    }
}

impl Default for AudioManager {
    fn default() -> Self {
        Self::new()
    }
}