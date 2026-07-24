use std::collections::HashMap;
use std::hash::Hash;
use std::sync::Arc;
use uuid::Uuid;

use serde::{Serialize, Deserialize};
use thiserror::Error;

use crate::physics::PhysicsBody;
use crate::animation::Animation;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EntityType {
    Node,
    GameObject,
    Sprite,
    Mesh,
    Camera,
    Light,
    Audio,
    Particle,
    PhysicsBody,
    Joint,
    TileMap,
    TileLayer,
    TileSet,
    Animation,
    AnimationPlayer,
    Marker2D,
    Marker3D,
    Line2D,
    Line3D,
    Curve2D,
    Curve3D,
    CharacterBody2D,
    CharacterBody3D,
    Area2D,
    Area3D,
    StaticBody2D,
    StaticBody3D,
    CharacterBody,
    Spatial,
    Node2D,
    Node3D,
    Control,
    SubViewport,
    SubViewportContainer,
    CanvasLayer,
    MarginContainer,
    HBoxContainer,
    VBoxContainer,
    PanelContainer,
    ColorRect,
    Label,
    RichTextLabel,
    Button,
    TextureButton,
    ColorPickerButton,
    ProgressBar,
    TextureProgressBar,
    ScrollContainer,
    ScrollBar,
    TabBar,
    TabContainer,
    NinePatchRect,
    OptionButton,
    CheckBox,
    ButtonGroup,
    MenuButton,
    TooltipPopup,
    Popup,
    PopupMenu,
    Window,
    FileSystemControl,
    FileDialog,
    Tree,
    ItemList,
    ItemEdit,
    ListView,
    ListBox,
    ListContainer,
    GridContainer,
    BoxContainer,
    CenterContainer,
    AnchorLayout,
    MarginLayout,
    BoxLayout,
    AbsoluteAnchor,
    BoxLayoutContainer,
    HSplitContainer,
    VSplitContainer,
    Accordion,
    AccordionContainer,
    NinePatch,
    TextureRect,
}

impl AsRef<str> for EntityType {
    fn as_ref(&self) -> &str {
        match self {
            EntityType::Node => "Node",
            EntityType::GameObject => "GameObject",
            EntityType::Sprite => "Sprite",
            EntityType::Mesh => "Mesh",
            EntityType::Camera => "Camera",
            EntityType::Light => "Light",
            EntityType::Audio => "Audio",
            EntityType::Particle => "Particle",
            EntityType::PhysicsBody => "PhysicsBody",
            EntityType::Joint => "Joint",
            EntityType::TileMap => "TileMap",
            EntityType::TileLayer => "TileLayer",
            EntityType::TileSet => "TileSet",
            EntityType::Animation => "Animation",
            EntityType::AnimationPlayer => "AnimationPlayer",
            EntityType::Marker2D => "Marker2D",
            EntityType::Marker3D => "Marker3D",
            EntityType::Line2D => "Line2D",
            EntityType::Line3D => "Line3D",
            EntityType::Curve2D => "Curve2D",
            EntityType::Curve3D => "Curve3D",
            EntityType::CharacterBody2D => "CharacterBody2D",
            EntityType::CharacterBody3D => "CharacterBody3D",
            EntityType::Area2D => "Area2D",
            EntityType::Area3D => "Area3D",
            EntityType::StaticBody2D => "StaticBody2D",
            EntityType::StaticBody3D => "StaticBody3D",
            EntityType::CharacterBody => "CharacterBody",
            EntityType::Spatial => "Spatial",
            EntityType::Node2D => "Node2D",
            EntityType::Node3D => "Node3D",
            EntityType::Control => "Control",
            EntityType::SubViewport => "SubViewport",
            EntityType::SubViewportContainer => "SubViewportContainer",
            EntityType::CanvasLayer => "CanvasLayer",
            EntityType::MarginContainer => "MarginContainer",
            EntityType::HBoxContainer => "HBoxContainer",
            EntityType::VBoxContainer => "VBoxContainer",
            EntityType::PanelContainer => "PanelContainer",
            EntityType::ColorRect => "ColorRect",
            EntityType::Label => "Label",
            EntityType::RichTextLabel => "RichTextLabel",
            EntityType::Button => "Button",
            EntityType::TextureButton => "TextureButton",
            EntityType::ColorPickerButton => "ColorPickerButton",
            EntityType::ProgressBar => "ProgressBar",
            EntityType::TextureProgressBar => "TextureProgressBar",
            EntityType::ScrollContainer => "ScrollContainer",
            EntityType::ScrollBar => "ScrollBar",
            EntityType::TabBar => "TabBar",
            EntityType::TabContainer => "TabContainer",
            EntityType::NinePatchRect => "NinePatchRect",
            EntityType::OptionButton => "OptionButton",
            EntityType::CheckBox => "CheckBox",
            EntityType::ButtonGroup => "ButtonGroup",
            EntityType::MenuButton => "MenuButton",
            EntityType::TooltipPopup => "TooltipPopup",
            EntityType::Popup => "Popup",
            EntityType::PopupMenu => "PopupMenu",
            EntityType::Window => "Window",
            EntityType::FileSystemControl => "FileSystemControl",
            EntityType::FileDialog => "FileDialog",
            EntityType::Tree => "Tree",
            EntityType::ItemList => "ItemList",
            EntityType::ItemEdit => "ItemEdit",
            EntityType::ListView => "ListView",
            EntityType::ListBox => "ListBox",
            EntityType::ListContainer => "ListContainer",
            EntityType::GridContainer => "GridContainer",
            EntityType::BoxContainer => "BoxContainer",
            EntityType::CenterContainer => "CenterContainer",
            EntityType::AnchorLayout => "AnchorLayout",
            EntityType::MarginLayout => "MarginLayout",
            EntityType::BoxLayout => "BoxLayout",
            EntityType::BoxLayoutContainer => "BoxLayoutContainer",
            EntityType::HSplitContainer => "HSplitContainer",
            EntityType::VSplitContainer => "VSplitContainer",
            EntityType::Accordion => "Accordion",
            EntityType::AccordionContainer => "AccordionContainer",
            EntityType::NinePatch => "NinePatch",
            EntityType::TextureRect => "TextureRect",
            _ => "Unknown",
        }
    }
}

impl EntityType {
    pub fn default() -> Self {
        Self::Node
    }
}

use crate::component_data::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transform {
    pub transform: TransformData,
}

impl Default for Transform {
    fn default() -> Self {
        Self {
            transform: TransformData::default(),
        }
    }
}

impl PartialEq for Transform {
    fn eq(&self, other: &Self) -> bool {
        self.transform == other.transform
    }
}

impl Eq for Transform {}

impl Hash for Transform {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.transform.hash(state);
    }
}
  
  
  #[derive(Debug, Clone, Serialize, Deserialize)]
  pub struct NodeData {
    pub id: Uuid,
    pub name: String,
    pub entity_type: EntityType,
    pub parent_id: Option<Uuid>,
    pub transform: Transform,
    pub properties: HashMap<String, serde_json::Value>,
    pub signals: Vec<Signal>,
    pub scripts: Vec<Uuid>,
    pub children: Vec<Uuid>,
    pub physics_body: Option<Arc<PhysicsBody>>,
    pub animation: Option<Arc<Animation>>,
    pub components: Vec<crate::component_data::ComponentData>,
    pub is_group: bool,
}

impl NodeData {
    pub fn new(name: &str, entity_type: EntityType) -> Self {
        Self {
            id: Uuid::new_v4(),
            name: name.to_string(),
            entity_type,
            parent_id: None,
            transform: Transform::default(),
            properties: HashMap::new(),
            signals: Vec::new(),
            scripts: Vec::new(),
            children: Vec::new(),
            physics_body: None,
            animation: None,
            components: Vec::new(),
            is_group: false,
        }
    }
    
    pub fn as_group(name: &str) -> Self {
        Self {
            id: Uuid::new_v4(),
            name: name.to_string(),
            entity_type: EntityType::Node,
            parent_id: None,
            transform: Transform::default(),
            properties: HashMap::new(),
            signals: Vec::new(),
            scripts: Vec::new(),
            children: Vec::new(),
            physics_body: None,
            animation: None,
            components: Vec::new(),
            is_group: true,
        }
    }

    pub fn update(&mut self, _delta: f64) {
        // Update logic
    }

    pub fn set_physics_body(&mut self, physics_body: Arc<PhysicsBody>) {
        self.physics_body = Some(physics_body);
    }

    pub fn get_physics_body(&self) -> Option<Arc<PhysicsBody>> {
        self.physics_body.clone()
    }

    pub fn set_animation(&mut self, animation: Arc<Animation>) {
        self.animation = Some(animation);
    }

    pub fn get_animation(&self) -> Option<Arc<Animation>> {
        self.animation.clone()
    }
}

impl Default for NodeData {
    fn default() -> Self {
        Self::new("Node", EntityType::Node)
    }
}

impl NodeData {
    pub fn with_physics(&self) -> Self {
        Self {
            id: Uuid::new_v4(),
            name: "Node".to_string(),
            entity_type: EntityType::Node,
            parent_id: None,
            transform: Transform::default(),
            properties: HashMap::new(),
            signals: Vec::new(),
            scripts: Vec::new(),
            children: Vec::new(),
            physics_body: None,
            animation: None,
            components: Vec::new(),
            is_group: false,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Signal {
    pub name: String,
    pub arguments: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Scene {
    pub root_id: Option<Uuid>,
    pub nodes: HashMap<Uuid, NodeData>,
    pub groups: HashMap<String, Vec<Uuid>>,
    pub animations: HashMap<String, Vec<Keyframe>>,
}

impl Scene {
    pub fn new() -> Self {
        Self {
            root_id: None,
            nodes: HashMap::new(),
            groups: HashMap::new(),
            animations: HashMap::new(),
        }
    }

    pub fn add_node(&mut self, node: NodeData) -> Uuid {
        let id = node.id;
        self.nodes.insert(id, node);
        id
    }

    pub fn get_node(&self, id: &Uuid) -> Option<&NodeData> {
        self.nodes.get(id)
    }

    pub fn get_node_mut(&mut self, id: &Uuid) -> Option<&mut NodeData> {
        self.nodes.get_mut(id)
    }

    pub fn get_children(&self, parent_id: &Uuid) -> Vec<&NodeData> {
        self.nodes
            .values()
            .filter(|n| n.parent_id == Some(*parent_id))
            .collect()
    }

    pub fn get_root(&self) -> Option<&NodeData> {
        self.nodes.values().find(|n| n.parent_id.is_none())
    }

    pub fn save(&self, _path: &str) -> Result<(), SaveError> {
        // Implementar serialización
        Ok(())
    }

    pub fn load(_path: &str) -> Result<Self, LoadError> {
        // Implementar deserialización
        Ok(Self::new())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Keyframe {
    pub time: f64,
    pub value: serde_json::Value,
}

#[derive(Debug, Error)]
pub enum SaveError {
    #[error("Error saving scene: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),
}

#[derive(Debug, Error)]
pub enum LoadError {
    #[error("Error loading scene: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Deserialization error: {0}")]
    DeserializationError(#[from] serde_json::Error),
    #[error("Invalid scene format")]
    InvalidFormat,
}

