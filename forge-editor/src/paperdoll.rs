use serde::{Serialize, Deserialize};
use std::collections::HashMap;

/// Capas del sistema Paperdoll
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum PaperdollLayer {
    Base,
    Face,
    Outfit,
    Accessory,
}

impl std::fmt::Display for PaperdollLayer {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            PaperdollLayer::Base => write!(f, "Base"),
            PaperdollLayer::Face => write!(f, "Face"),
            PaperdollLayer::Outfit => write!(f, "Outfit"),
            PaperdollLayer::Accessory => write!(f, "Accessory"),
        }
    }
}

/// Sockets de anclaje
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Socket {
    pub id: String,
    pub position: [f32; 2],
    pub name: String,
}

/// Personaje Paperdoll
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaperdollCharacter {
    pub id: String,
    pub name: String,
    pub base_layer: PaperdollBase,
    pub layers: HashMap<PaperdollLayer, PaperdollLayerData>,
    pub sockets: Vec<Socket>,
}

/// Datos de una capa
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaperdollLayerData {
    pub sprite_path: String,
    pub animation_frames: Vec<AnimationFrame>,
    pub is_enabled: bool,
}

/// Marco de animación
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnimationFrame {
    pub name: String,
    pub frames: Vec<String>,
    pub fps: f32,
    pub is_loop: bool,
}

/// Capa base del personaje
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaperdollBase {
    pub hitbox_vertices: Vec<[f32; 2]>,
    pub base_color: [u8; 4],
    pub default_outfit: Option<String>,
}

impl Default for PaperdollBase {
    fn default() -> Self {
        Self {
            hitbox_vertices: vec![
                [0.0, 0.0], [20.0, 0.0], [20.0, 30.0], [40.0, 30.0],
                [40.0, 50.0], [20.0, 50.0], [20.0, 80.0], [10.0, 80.0],
                [10.0, 50.0], [0.0, 50.0], [0.0, 30.0], [10.0, 30.0],
            ],
            base_color: [255, 255, 255, 255],
            default_outfit: None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct RenderedCharacter {
    pub id: String,
    pub layers_rendered: Vec<(PaperdollLayer, String)>,
    pub total_size: [u32; 2],
}

impl PaperdollCharacter {
    pub fn new(id: &str, name: &str) -> Self {
        Self {
            id: id.to_string(),
            name: name.to_string(),
            base_layer: PaperdollBase::default(),
            layers: HashMap::new(),
            sockets: Vec::new(),
        }
    }

    pub fn add_layer(&mut self, layer: PaperdollLayer, data: PaperdollLayerData) {
        self.layers.insert(layer, data);
    }

    pub fn add_socket(&mut self, socket: Socket) {
        self.sockets.push(socket);
    }

    pub fn render(&self) -> RenderedCharacter {
        let mut layers_rendered = Vec::new();
        let mut total_width = 0u32;
        let mut total_height = 0u32;

        layers_rendered.push((PaperdollLayer::Base, "base.png".to_string()));
        total_width = 40;
        total_height = 80;

        for (layer, data) in &self.layers {
            if data.is_enabled {
                layers_rendered.push((layer.clone(), data.sprite_path.clone()));
            }
        }

        RenderedCharacter {
            id: self.id.clone(),
            layers_rendered,
            total_size: [total_width, total_height],
        }
    }
}

