use serde::{Deserialize, Serialize};

/// Component type discriminant
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ComponentType {
    Transform,
    Collider,
    Renderer,
    Sprite,
    Audio,
    Script,
    Dialogue,
    Behavior,
}

impl std::fmt::Display for ComponentType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ComponentType::Transform => write!(f, "transform"),
            ComponentType::Collider => write!(f, "collider"),
            ComponentType::Renderer => write!(f, "renderer"),
            ComponentType::Sprite => write!(f, "sprite"),
            ComponentType::Audio => write!(f, "audio"),
            ComponentType::Script => write!(f, "script"),
            ComponentType::Dialogue => write!(f, "dialogue"),
            ComponentType::Behavior => write!(f, "behavior"),
        }
    }
}

/// Unified component data wrapper
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentData {
    pub component_type: ComponentType,
    #[serde(flatten)]
    pub data: serde_json::Value,
}

impl ComponentData {
    pub fn new_transform() -> Self {
        ComponentData {
            component_type: ComponentType::Transform,
            data: serde_json::json!({
                "position": [0.0, 0.0, 0.0],
                "rotation": 0.0,
                "scale": [1.0, 1.0, 1.0]
            }),
        }
    }
    
    pub fn new_collider() -> Self {
        ComponentData {
            component_type: ComponentType::Collider,
            data: serde_json::json!({
                "shape": "box",
                "size": [1.0, 1.0],
                "enabled": true
            }),
        }
    }
    
    pub fn new_renderer() -> Self {
        ComponentData {
            component_type: ComponentType::Renderer,
            data: serde_json::json!({
                "mesh": null,
                "material": null,
                "color": [1.0, 1.0, 1.0],
                "visible": true
            }),
        }
    }
    
    pub fn new_sprite(texture_path: String) -> Self {
        ComponentData {
            component_type: ComponentType::Sprite,
            data: serde_json::json!({
                "texture_path": texture_path,
                "width": 64.0,
                "height": 64.0,
                "color": [1.0, 1.0, 1.0]
            }),
        }
    }
    
    pub fn new_audio() -> Self {
        ComponentData {
            component_type: ComponentType::Audio,
            data: serde_json::json!({
                "source": null,
                "volume": 1.0,
                "pitch": 1.0,
                "is_loop": false
            }),
        }
    }
    
    pub fn new_script(script_path: String) -> Self {
        ComponentData {
            component_type: ComponentType::Script,
            data: serde_json::json!({
                "script_path": script_path,
                "is_enabled": true
            }),
        }
    }
    
    pub fn new_dialogue(dialogue_path: String) -> Self {
        ComponentData {
            component_type: ComponentType::Dialogue,
            data: serde_json::json!({
                "dialogue_path": dialogue_path,
                "is_active": false
            }),
        }
    }

    pub fn new_behavior() -> Self {
        ComponentData {
            component_type: ComponentType::Behavior,
            data: serde_json::json!({
                "behavior_type": "patrol",
                "speed": 100.0,
                "range": 200.0,
                "is_active": true
            }),
        }
    }
}

/// Transform component data
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TransformData {
    pub position: [f32; 3],
    pub rotation: f32,
    pub scale: [f32; 3],
}

impl Default for TransformData {
    fn default() -> Self {
        TransformData {
            position: [0.0, 0.0, 0.0],
            rotation: 0.0,
            scale: [1.0, 1.0, 1.0],
        }
    }
}

impl Eq for TransformData {}

impl std::hash::Hash for TransformData {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        for &val in &self.position {
            val.to_bits().hash(state);
        }
        self.rotation.to_bits().hash(state);
        for &val in &self.scale {
            val.to_bits().hash(state);
        }
    }
}

/// Collider component data
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ColliderData {
    pub shape: ColliderShape,
    pub size: [f32; 2],
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ColliderShape {
    Box,
    Circle,
}

impl Default for ColliderData {
    fn default() -> Self {
        ColliderData {
            shape: ColliderShape::Box,
            size: [1.0, 1.0],
            enabled: true,
        }
    }
}

/// Renderer component data
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RendererData {
    pub mesh: Option<String>,
    pub material: Option<String>,
    pub color: [f32; 3],
    pub visible: bool,
}

impl Default for RendererData {
    fn default() -> Self {
        RendererData {
            mesh: None,
            material: None,
            color: [1.0, 1.0, 1.0],
            visible: true,
        }
    }
}

/// Audio component data
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AudioData {
    pub source: Option<String>,
    pub volume: f32,
    pub pitch: f32,
    pub is_loop: bool,
}

impl Default for AudioData {
    fn default() -> Self {
        AudioData {
            source: None,
            volume: 1.0,
            pitch: 1.0,
            is_loop: false,
        }
    }
}

/// Sprite component data
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SpriteData {
    pub texture_path: String,
    pub width: f32,
    pub height: f32,
    pub color: [f32; 3],
}

impl Default for SpriteData {
    fn default() -> Self {
        SpriteData {
            texture_path: String::new(),
            width: 64.0,
            height: 64.0,
            color: [1.0, 1.0, 1.0],
        }
    }
}

/// Script component data
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ScriptData {
    pub script_path: String,
    pub is_enabled: bool,
}

impl Default for ScriptData {
    fn default() -> Self {
        ScriptData {
            script_path: String::new(),
            is_enabled: true,
        }
    }
}

/// Dialogue component data
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DialogueData {
    pub dialogue_path: String,
    pub is_active: bool,
}

impl Default for DialogueData {
    fn default() -> Self {
        DialogueData {
            dialogue_path: String::new(),
            is_active: false,
        }
    }
}
