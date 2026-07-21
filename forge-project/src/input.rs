use std::collections::HashMap;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum InputAction {
    Button { name: String },
    Axis { name: String, value: f32 },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InputAxis {
    pub name: String,
    pub key_bindings: Vec<String>,
    pub deadzone: f32,
}

impl InputAxis {
    pub fn new(name: String, key_bindings: Vec<String>, deadzone: f32) -> Self {
        Self { name, key_bindings, deadzone }
    }
}

impl Default for InputAxis {
    fn default() -> Self {
        Self {
            name: "Unknown".to_string(),
            key_bindings: Vec::new(),
            deadzone: 0.5,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InputButton {
    pub name: String,
    pub key_bindings: Vec<String>,
}

impl InputButton {
    pub fn new(name: String, key_bindings: Vec<String>) -> Self {
        Self { name, key_bindings }
    }
}

impl Default for InputButton {
    fn default() -> Self {
        Self {
            name: "Unknown".to_string(),
            key_bindings: Vec::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct InputSystem {
    pub axes: HashMap<String, InputAxis>,
    pub buttons: HashMap<String, InputButton>,
    pub current_state: HashMap<String, f32>,
}

impl InputSystem {
    pub fn new() -> Self {
        Self {
            axes: HashMap::new(),
            buttons: HashMap::new(),
            current_state: HashMap::new(),
        }
    }

    pub fn add_axis(&mut self, axis: InputAxis) {
        self.axes.insert(axis.name.clone(), axis);
    }

    pub fn add_button(&mut self, button: InputButton) {
        self.buttons.insert(button.name.clone(), button);
    }

    pub fn get_value(&self, action_name: &str) -> f32 {
        self.current_state.get(action_name).copied().unwrap_or(0.0)
    }

    pub fn is_pressed(&self, action_name: &str) -> bool {
        self.current_state.get(action_name).map(|&v| v > 0.5).unwrap_or(false)
    }
}

impl Default for InputSystem {
    fn default() -> Self {
        Self::new()
    }
}