/// forge_scene stub para compilación
/// 
/// Este stub actúa como puente de compatibilidad entre la UI del editor y el crate real `forge-scene`.
/// 
/// **Estrategia:**
/// - Mantiene tipos simplificados en la UI del editor para asegurar 0 errores de compilación
/// - Implementa `.to_real()` y `.from_real()` hacia el crate real `forge-scene`
/// - Qwen usará estos métodos para alimentar `forge-physics` y `forge-animation` de forma segura
/// - Cuando todo esté conectado, el stub se eliminará de forma transparente

use std::collections::HashMap;
use std::sync::Arc;
use uuid::Uuid;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Asset {
    pub id: Uuid,
    pub path: String,
    pub data: Vec<u8>,
}

impl Asset {
    pub fn split(self) -> (String, Vec<u8>) {
        (self.path, self.data)
    }

    pub fn as_str(&self) -> &str {
        &self.path
    }

    /// Convierte Asset stub a Asset real de forge-scene
    pub fn to_real(&self) -> ::forge_scene::Asset {
        ::forge_scene::Asset {
            id: self.id.to_string(),
            name: self.path.clone(),
            path: self.path.clone(),
            asset_type: ::forge_scene::AssetType::Sprite,
            size: self.data.len() as u64,
            is_loaded: true,
        }
    }

    /// Crea Asset stub desde Asset real de forge-scene
    pub fn from_real(asset: &::forge_scene::Asset) -> Self {
        Self {
            id: Uuid::parse_str(&asset.id).unwrap_or_default(),
            path: asset.path.clone(),
            data: vec![],
        }
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Scene {
    pub name: String,
    pub entities: Vec<Arc<NodeData>>,
    pub selected: Vec<usize>,
}

impl Scene {
    /// Convierte Scene stub a Scene real de forge-scene
    pub fn to_real(&self) -> ::forge_scene::Scene {
        let mut nodes_map: HashMap<Uuid, ::forge_scene::NodeData> = HashMap::new();
        
        for entity in &self.entities {
            let real_node = entity.to_real();
            nodes_map.insert(real_node.id, real_node);
        }

        ::forge_scene::Scene {
            root_id: if !self.entities.is_empty() {
                Some(self.entities[0].id)
            } else {
                None
            },
            nodes: nodes_map,
            groups: HashMap::new(),
            animations: HashMap::new(),
        }
    }

    /// Crea Scene stub desde Scene real de forge-scene
    pub fn from_real(scene: &::forge_scene::Scene) -> Self {
        let mut entities: Vec<Arc<NodeData>> = Vec::new();
        
        for (_, node) in &scene.nodes {
            let stub_node = NodeData::from_real(node);
            entities.push(Arc::new(stub_node));
        }

        Self {
            name: "Scene".to_string(),
            entities,
            selected: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Default, Serialize, Deserialize)]
pub enum EntityType {
    #[default]
    Empty,
    Group,
    GameObject,
    Sprite,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeData {
    pub id: Uuid,
    pub name: String,
    pub entity_type: EntityType,
    pub parent_id: Option<Uuid>,
    pub transform: Transform,
    pub properties: HashMap<String, String>,
    pub components: Vec<::forge_scene::ComponentData>,
}

impl PartialEq for NodeData {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl NodeData {
    /// Convierte NodeData stub a NodeData real de forge-scene
    pub fn to_real(&self) -> ::forge_scene::NodeData {
        let real_entity_type = match self.entity_type {
            EntityType::GameObject => ::forge_scene::EntityType::GameObject,
            EntityType::Sprite => ::forge_scene::EntityType::Sprite,
            EntityType::Group => ::forge_scene::EntityType::Node,
            EntityType::Empty => ::forge_scene::EntityType::Node,
        };

        ::forge_scene::NodeData {
            id: self.id,
            name: self.name.clone(),
            entity_type: real_entity_type,
            parent_id: self.parent_id,
            transform: self.transform.to_real(),
            properties: self.properties
                .iter()
                .map(|(k, v)| (k.clone(), serde_json::Value::String(v.clone())))
                .collect(),
            signals: Vec::new(),
            scripts: Vec::new(),
            children: Vec::new(),
            physics_body: None,
            animation: None,
            components: self.components.clone(),
            is_group: self.entity_type == EntityType::Group,
        }
    }

    /// Crea NodeData stub desde NodeData real de forge-scene
    pub fn from_real(node: &::forge_scene::NodeData) -> Self {
        let stub_entity_type = match node.entity_type {
            ::forge_scene::EntityType::GameObject => EntityType::GameObject,
            ::forge_scene::EntityType::Sprite => EntityType::Sprite,
            _ => if node.is_group { EntityType::Group } else { EntityType::Empty },
        };

        Self {
            id: node.id,
            name: node.name.clone(),
            entity_type: stub_entity_type,
            parent_id: node.parent_id,
            transform: Transform::from_real(&node.transform),
            properties: node.properties
                .iter()
                .map(|(k, v)| (k.clone(), match v {
                    serde_json::Value::String(s) => s.clone(),
                    _ => "value".to_string(),
                }))
                .collect(),
            components: node.components.clone(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Default, Serialize, Deserialize)]
pub struct Transform {
    pub position: [f32; 3],
    pub rotation: f32,
    pub scale: [f32; 3],
}

impl Transform {
    /// Convierte Transform stub a Transform real de forge-scene
    pub fn to_real(&self) -> ::forge_scene::Transform {
        ::forge_scene::Transform {
            transform: ::forge_scene::TransformData {
                position: self.position,
                rotation: self.rotation,
                scale: self.scale,
            },
        }
    }

    /// Crea Transform stub desde Transform real de forge-scene
    pub fn from_real(transform: &::forge_scene::Transform) -> Self {
        Self {
            position: [transform.transform.position[0], transform.transform.position[1], 0.0],
            rotation: transform.transform.rotation,
            scale: [transform.transform.scale[0], transform.transform.scale[1], 1.0],
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct TransformProps {
    pub position: [f32; 3],
    pub rotation: f32,
    pub scale: [f32; 3],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum NodeType {
    #[default]
    Empty,
    GameObject,
    Sprite,
    Transform,
}

#[derive(Debug, Clone, Default)]
pub struct SceneTree {
    pub root: Option<NodeData>,
    pub tree: Vec<NodeData>,
    pub nodes: Vec<NodeData>,
    pub active_nodes: Vec<usize>,
    pub active_node_id: Option<Uuid>,
    pub filter: Option<String>,
}

impl SceneTree {
    pub fn new() -> Self {
        Self::default()
    }

    /// Convierte SceneTree stub a SceneTree real de forge-scene
    pub fn to_real(&self) -> ::forge_scene::SceneTree {
        let mut nodes_map: HashMap<Uuid, Arc<::forge_scene::NodeData>> = HashMap::new();
        
        if let Some(root) = &self.root {
            nodes_map.insert(root.id, Arc::new(root.to_real()));
        }
        
        for node in &self.tree {
            nodes_map.insert(node.id, Arc::new(node.to_real()));
        }

        ::forge_scene::SceneTree {
            root: self.root.as_ref().map(|n| Arc::new(n.to_real())),
            nodes: nodes_map,
            scenes: HashMap::new(),
            current_scene: None,
            active_nodes: self.active_nodes.iter().map(|&idx| {
                Arc::new(self.tree[idx].to_real())
            }).collect(),
        }
    }

    /// Crea SceneTree stub desde SceneTree real de forge-scene
    pub fn from_real(scene_tree: &::forge_scene::SceneTree) -> Self {
        let mut nodes: Vec<NodeData> = Vec::new();
        
        for (_, node) in &scene_tree.nodes {
            let stub_node = NodeData::from_real(node);
            nodes.push(stub_node);
        }

        let mut active_indices = Vec::new();
        for active in &scene_tree.active_nodes {
            if let Some(pos) = nodes.iter().position(|n| n.id == active.id) {
                active_indices.push(pos);
            }
        }

        Self {
            root: nodes.first().cloned(),
            tree: nodes.clone(),
            nodes,
            active_nodes: active_indices,
            active_node_id: None,
            filter: None,
        }
    }

    pub fn add_node(&mut self, _node: NodeData) -> Option<usize> {
        self.root = Some(_node.clone());
        self.tree.push(_node);
        Some(0)
    }

    pub fn add_node_to_scene(&mut self, node: NodeData) -> Option<usize> {
        if self.root.is_none() {
            self.root = Some(node.clone());
        }
        self.tree.push(node);
        Some(self.tree.len() - 1)
    }

    pub fn remove_node(&mut self, id: Uuid) {
        self.tree.retain(|n| n.id != id);
        self.nodes.retain(|n| n.id != id);
        if self.active_node_id == Some(id) {
            self.active_node_id = None;
        }
    }

    pub fn set_directory(&mut self, _dir: &str) {
        // stub
    }

    pub fn get_root(&self) -> Option<&NodeData> {
        self.root.as_ref()
    }

    pub fn get_root_mut(&mut self) -> Option<&mut NodeData> {
        self.root.as_mut()
    }

    pub fn get_tree(&self) -> &[NodeData] {
        &self.tree
    }

    pub fn get_tree_mut(&mut self) -> &mut Vec<NodeData> {
        &mut self.tree
    }

    pub fn nodes(&self) -> &Vec<NodeData> {
        &self.nodes
    }

    pub fn active_nodes(&self) -> &Vec<usize> {
        &self.active_nodes
    }

    pub fn set_nodes(&mut self, nodes: Vec<NodeData>) {
        self.nodes = nodes;
    }

    pub fn set_active_nodes(&mut self, active: Vec<usize>) {
        self.active_nodes = active;
    }

    pub fn set_filter(&mut self, filter: &str) {
        self.filter = Some(filter.to_string());
    }
}

// Re-exportar tipos reales del backend para simplificar el mapeo en el editor
pub use ::forge_scene::{ComponentData, ComponentType, TransformData};
