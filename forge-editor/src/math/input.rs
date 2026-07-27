use std::collections::HashMap;
use crate::math::Vector2;

pub struct Input {
    keys: Vec<KeyCode>,
    key_states: HashMap<KeyCode, bool>,
    mouse_position: Vector2,
    delta_time: f32,
}

impl Input {
    pub fn new() -> Self {
        Input {
            keys: Vec::new(),
            key_states: HashMap::new(),
            mouse_position: Vector2::ZERO(),
            delta_time: 0.016,
        }
    }

    pub fn poll_events(&mut self) {
        // Simular eventos de teclado
        for key in self.keys.iter() {
            self.key_states.insert(*key, true);
        }
    }

    pub fn is_down(&self, key: KeyCode) -> bool {
        self.key_states.get(&key).copied().unwrap_or(false)
    }

    pub fn get_key(&self) -> KeyCode {
        KeyCode::None
    }

    pub fn get_delta_time(&self) -> f32 {
        self.delta_time
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum KeyCode {
    None,
    W,
    A,
    S,
    D,
    Space,
    Enter,
    Up,
    Down,
    Left,
    Right,
    N,
}
