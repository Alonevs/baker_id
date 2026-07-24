//! Paperdoll - Sistema de capas para personajes

use serde::{Serialize, Deserialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum PaperdollLayer {
    Background,
    Character,
    Accessories,
    Effects,
}

impl Default for PaperdollLayer {
    fn default() -> Self {
        Self::Character
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaperdollItem {
    pub layer: PaperdollLayer,
    pub item_id: String,
    pub position: (f32, f32),
    pub scale: f32,
    pub rotation: f32,
}

impl Default for PaperdollItem {
    fn default() -> Self {
        Self {
            layer: PaperdollLayer::Character,
            item_id: String::new(),
            position: (0.0, 0.0),
            scale: 1.0,
            rotation: 0.0,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Paperdoll {
    pub layers: HashMap<PaperdollLayer, Vec<PaperdollItem>>,
    pub current_skin: Option<String>,
    pub current_outfit: Option<String>,
}

impl Default for Paperdoll {
    fn default() -> Self {
        Self {
            layers: HashMap::new(),
            current_skin: None,
            current_outfit: None,
        }
    }
}

impl Paperdoll {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_item(&mut self, layer: PaperdollLayer, item: PaperdollItem) {
        self.layers.entry(layer).or_default().push(item);
    }

    pub fn get_items(&self, layer: PaperdollLayer) -> Vec<&PaperdollItem> {
        self.layers.get(&layer).map(|v| v.iter().collect()).unwrap_or_default()
    }

    pub fn set_skin(&mut self, skin: String) {
        self.current_skin = Some(skin);
    }

    pub fn set_outfit(&mut self, outfit: String) {
        self.current_outfit = Some(outfit);
    }
}
