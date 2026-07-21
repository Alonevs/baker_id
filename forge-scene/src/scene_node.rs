use std::collections::HashMap;
use std::sync::Arc;
use uuid::Uuid;

use serde::{Serialize, Deserialize};
use thiserror::Error;

use crate::physics::PhysicsBody;
use crate::animation::Animation;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
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

