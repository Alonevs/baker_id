use std::sync::Arc;
use std::collections::HashMap;
use uuid::Uuid;
use thiserror::Error;

use crate::scene_node::{NodeData, Scene, LoadError};

#[derive(Debug, Clone)]
pub struct SceneTree {
    pub root: Option<Arc<NodeData>>,
    pub nodes: HashMap<Uuid, Arc<NodeData>>,
    pub scenes: HashMap<Uuid, Arc<Scene>>,
    pub current_scene: Option<Uuid>,
    pub active_nodes: Vec<Arc<NodeData>>,
}

impl SceneTree {
    pub fn new() -> Self {
        Self {
            root: None,
            nodes: HashMap::new(),
            scenes: HashMap::new(),
            current_scene: None,
            active_nodes: Vec::new(),
        }
    }

    pub fn add_scene(&mut self, scene: Arc<Scene>) {
        let scene_id = Uuid::new_v4();
        self.scenes.insert(scene_id, scene);
    }

    pub fn load_scene(&mut self, scene_id: &Uuid) -> Result<(), LoadError> {
        if let Some(_scene) = self.scenes.get(scene_id) {
            // Implementar carga de escena
            Ok(())
        } else {
            Err(LoadError::InvalidFormat)
        }
    }

    pub fn switch_scene(&mut self, scene_id: &Uuid) {
        self.current_scene = Some(*scene_id);
    }

    pub fn get_node(&self, node_id: &Uuid) -> Option<Arc<NodeData>> {
        self.nodes.get(node_id).cloned()
    }

    pub fn get_root(&self) -> Option<Arc<NodeData>> {
        self.root.clone()
    }

    pub fn add_node(&mut self, node: Arc<NodeData>) -> Uuid {
        let id = node.id;
        self.nodes.insert(id, node.clone());
        if self.root.is_none() {
            self.root = Some(node);
        }
        id
    }

    pub fn set_parent(&mut self, node_id: &Uuid, parent_id: Option<Uuid>) -> Result<(), crate::scene_tree::ParentError> {
        // Remove from old parent's children
        if let Some(old_parent) = self.nodes.get(node_id).and_then(|n| n.parent_id) {
                if let Some(old_parent_node) = self.nodes.get(&old_parent) {
                    let _old_node = old_parent_node.as_ref().clone();
                    let new_children = _old_node.children.iter().filter(|child| !child.eq(&node_id)).cloned().collect();
                    let new_node = NodeData {
                        id: _old_node.id,
                        name: _old_node.name.clone(),
                        entity_type: _old_node.entity_type,
                        parent_id: _old_node.parent_id,
                        transform: _old_node.transform.clone(),
                        properties: _old_node.properties.clone(),
                        signals: _old_node.signals.clone(),
                        scripts: _old_node.scripts.clone(),
                        children: new_children,
                        physics_body: _old_node.physics_body.clone(),
                        animation: _old_node.animation.clone(),
                        components: _old_node.components.clone(),
                        is_group: _old_node.is_group,
                    };
                    self.nodes.insert(old_parent.clone(), Arc::new(new_node));
                }
        }
        
        // Add to new parent's children
        if let Some(ref pid) = parent_id {
            if let Some(parent_node) = self.nodes.get(pid) {
                let parent = parent_node.as_ref().clone();
                let mut new_children = parent.children.clone();
                new_children.push(*node_id);
                let new_node = NodeData {
                    id: parent.id,
                    name: parent.name.clone(),
                    entity_type: parent.entity_type,
                    parent_id: parent.parent_id,
                    transform: parent.transform.clone(),
                    properties: parent.properties.clone(),
                    signals: parent.signals.clone(),
                    scripts: parent.scripts.clone(),
                    children: new_children,
                    physics_body: parent.physics_body.clone(),
                    animation: parent.animation.clone(),
                    components: parent.components.clone(),
                    is_group: parent.is_group,
                };
                self.nodes.insert(pid.clone(), Arc::new(new_node));
            }
        }
        
        Ok(())
    }

    pub fn get_children(&self, parent_id: &Uuid) -> Vec<Arc<NodeData>> {
        self.nodes
            .values()
            .filter(|n| n.parent_id.as_ref() == Some(parent_id))
            .cloned()
            .collect()
    }



    pub fn get_active_nodes(&self) -> Vec<Arc<NodeData>> {
        self.active_nodes.clone()
    }

    pub fn update(&mut self, delta: f64) {
        // Actualizar todos los nodos activos
        for node in &self.active_nodes {
            let node_clone = node.clone();
            let mut node_clone = node_clone.as_ref().clone();
            node_clone.update(delta);
        }
    }
}

#[derive(Debug, Clone)]
pub struct SceneTreeEditor {
    pub tree: SceneTree,
    pub selected_nodes: Vec<Uuid>,
    pub hovered_node: Option<Uuid>,
    pub edit_mode: bool,
    pub drag_operation: DragOperation,
}

#[derive(Debug, Clone)]
pub enum DragOperation {
    None,
    Move { source: Uuid, target: Uuid },
    Copy { source: Uuid },
    Delete { target: Uuid },
}

impl SceneTreeEditor {
    pub fn new() -> Self {
        Self {
            tree: SceneTree::new(),
            selected_nodes: Vec::new(),
            hovered_node: None,
            edit_mode: true,
            drag_operation: DragOperation::None,
        }
    }

    pub fn select_node(&mut self, node_id: Uuid) {
        if self.selected_nodes.contains(&node_id) {
            self.selected_nodes.retain(|id| *id != node_id);
        } else {
            self.selected_nodes.push(node_id);
        }
        self.hovered_node = Some(node_id);
    }

    pub fn add_node_to_scene(&mut self, node: NodeData) -> Uuid {
        let arc_node = Arc::new(node);
        let id = self.tree.add_node(arc_node.clone());
        self.tree.active_nodes.push(arc_node);
        id
    }

    pub fn delete_selected(&mut self) {
        for node_arc in self.tree.nodes.values_mut() {
            if self.selected_nodes.iter().any(|id| id == &node_arc.id) {
                let parent_id = node_arc.parent_id.as_ref();
                let mut parent_clone = node_arc.as_ref().clone();
                parent_clone.signals.push(crate::scene_node::Signal {
                    name: "node_deleted".to_string(),
                    arguments: vec![node_arc.id.to_string()],
                });
                if let Some(_pid) = parent_id {
                    *node_arc = Arc::new(parent_clone);
                }
            }
        }
        self.selected_nodes.clear();
    }

    pub fn move_node(&mut self, source_id: Uuid, target_id: Uuid) -> Result<(), crate::scene_tree::MoveError> {
        if let (Some(source), Some(target)) = (
            self.tree.nodes.get(&source_id),
            self.tree.nodes.get(&target_id),
        ) {
            let source_clone = source.clone();
            let mut source_clone = source_clone.as_ref().clone();
            if source_clone.parent_id == Some(target_id.clone()) {
                source_clone.parent_id = target.parent_id;
            } else {
                source_clone.parent_id = Some(target_id);
            }
            if let Some(_old_node) = self.tree.nodes.remove(&source_id) {
                self.tree.nodes.insert(source_id, Arc::new(source_clone));
            }
            Ok(())
        } else {
            Err(crate::scene_tree::MoveError::NodeNotFound)
        }
    }
}

#[derive(Debug, Error)]
pub enum MoveError {
    #[error("Node not found")]
    NodeNotFound,
    #[error("Invalid parent")]
    InvalidParent,
}

#[derive(Debug, Error)]
pub enum ParentError {
    #[error("Node not found")]
    NodeNotFound,
    #[error("Invalid parent")]
    InvalidParent,
}

impl Default for SceneTreeEditor {
    fn default() -> Self {
        Self::new()
    }
}
