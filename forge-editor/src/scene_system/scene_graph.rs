//! # Scene Graph
//! 
//! Sistema de jerarquía de escenas con:
//! - Nodos de escena
//! - Hijos y padres
//! - Activación por cascada
//! - Desactivación por cascada

use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

/// Tipo de nodo en el grafo de escenas
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NodeType {
    Scene,
    Group,
    Layer,
    Transition,
}

/// Nodo en el grafo de escenas
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SceneGraphNode {
    pub id: String,
    pub name: String,
    pub node_type: NodeType,
    pub parent_id: Option<String>,
    pub children: Vec<String>,
    pub is_active: bool,
    pub metadata: HashMap<String, String>,
}

impl SceneGraphNode {
    /// Crea un nuevo nodo de escena
    pub fn new(id: String, name: String, node_type: NodeType) -> Self {
        Self {
            id,
            name,
            node_type,
            parent_id: None,
            children: Vec::new(),
            is_active: false,
            metadata: HashMap::new(),
        }
    }

    /// Obtiene hijos activos
    pub fn get_active_children(&self) -> Vec<&SceneGraphNode> {
        self.children
            .iter()
            .filter(|child_id| {
                self.scene_graph
                    .get_node(*child_id)
                    .map(|n| n.is_active)
                    .unwrap_or(false)
            })
            .map(|child_id| self.scene_graph.get_node(*child_id).unwrap())
            .collect()
    }
}

/// Grafo de escenas
pub struct SceneGraph {
    /// Nodos del grafo
    pub nodes: HashMap<String, SceneGraphNode>,
    /// Escena raíz
    pub root_id: Option<String>,
    /// Escena actual
    pub current_node_id: Option<String>,
    /// Referencias al SceneManager
    pub scene_manager: Option<super::scene_manager::SceneManager>,
}

impl Default for SceneGraph {
    fn default() -> Self {
        Self::new()
    }
}

impl SceneGraph {
    /// Crea un nuevo grafo de escenas
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            root_id: None,
            current_node_id: None,
            scene_manager: None,
        }
    }

    /// Crea grafo con SceneManager
    pub fn with_scene_manager(scene_manager: super::scene_manager::SceneManager) -> Self {
        Self {
            nodes: HashMap::new(),
            root_id: None,
            current_node_id: None,
            scene_manager: Some(scene_manager),
        }
    }

    /// Crea nodo en el grafo
    pub fn create_node(&mut self, id: String, name: String, node_type: NodeType) {
        let node = SceneGraphNode::new(id, name, node_type);
        self.nodes.insert(id, node);
        println!("[SCENE GRAPH] Created node: {}", name);
    }

    /// Crea nodo hijo de otro
    pub fn create_child(&mut self, parent_id: &str, child_id: String, child_name: String) {
        if let Some(parent) = self.nodes.get_mut(parent_id) {
            parent.children.push(child_id.clone());
            self.nodes.insert(child_id, SceneGraphNode::new(child_id, child_name, NodeType::Scene));
            println!("[SCENE GRAPH] Created child: {} under {}", child_name, parent_name);
        }
    }

    /// Obtiene nodo por ID
    pub fn get_node(&self, id: &str) -> Option<&SceneGraphNode> {
        self.nodes.get(id)
    }

    /// Obtiene nodo mutado por ID
    pub fn get_node_mut(&mut self, id: &str) -> Option<&mut SceneGraphNode> {
        self.nodes.get_mut(id)
    }

    /// Obtiene padre de un nodo
    pub fn get_parent(&self, node_id: &str) -> Option<String> {
        if let Some(node) = self.nodes.get(node_id) {
            node.parent_id.clone()
        } else {
            None
        }
    }

    /// Obtiene hijos de un nodo
    pub fn get_children(&self, parent_id: &str) -> Vec<String> {
        self.nodes
            .get(parent_id)
            .map(|n| n.children.clone())
            .unwrap_or_default()
    }

    /// Activa nodo y sus hijos
    pub fn activate_node(&mut self, node_id: &str) {
        if let Some(node) = self.nodes.get_mut(node_id) {
            node.is_active = true;
            
            // Activar hijos recursivamente
            for child_id in &node.children {
                if let Some(child) = self.nodes.get_mut(child_id) {
                    child.is_active = true;
                    self.activate_node(child_id);
                }
            }
            
            println!("[SCENE GRAPH] Activated node: {}", node_id);
        }
    }

    /// Desactiva nodo y sus hijos
    pub fn deactivate_node(&mut self, node_id: &str) {
        if let Some(node) = self.nodes.get_mut(node_id) {
            node.is_active = false;
            
            // Desactivar hijos recursivamente
            for child_id in &node.children {
                if let Some(child) = self.nodes.get_mut(child_id) {
                    child.is_active = false;
                    self.deactivate_node(child_id);
                }
            }
            
            println!("[SCENE GRAPH] Deactivated node: {}", node_id);
        }
    }

    /// Cambia estado activo/inactivo de nodo
    pub fn set_active(&mut self, node_id: &str, active: bool) {
        if let Some(node) = self.nodes.get_mut(node_id) {
            node.is_active = active;
            println!("[SCENE GRAPH] Set {} to {}", node.name, if active { "active" } else { "inactive" });
        }
    }

    /// Establece padre de un nodo
    pub fn set_parent(&mut self, node_id: &str, parent_id: Option<String>) {
        if let Some(node) = self.nodes.get_mut(node_id) {
            node.parent_id = parent_id;
            
            // Actualizar hijos
            if let Some(parent_id) = &parent_id {
                if let Some(parent) = self.nodes.get_mut(parent_id) {
                    if !parent.children.contains(node_id) {
                        parent.children.push(node_id.clone());
                    }
                }
            }
        }
    }

    /// Elimina nodo del grafo
    pub fn remove_node(&mut self, node_id: &str) {
        // Desactivar nodo y sus hijos
        self.deactivate_node(node_id);
        
        // Eliminar nodo
        self.nodes.remove(node_id);
        
        // Eliminar referencias en otros nodos
        for node in self.nodes.values_mut() {
            node.children.retain(|child| child != node_id);
        }
        
        println!("[SCENE GRAPH] Removed node: {}", node_id);
    }

    /// Obtiene camino desde raíz hasta nodo
    pub fn get_path_to_node(&self, node_id: &str) -> Vec<String> {
        let mut path = Vec::new();
        let mut current_id = Some(node_id.to_string());
        
        while let Some(id) = current_id {
            if let Some(node) = self.nodes.get(&id) {
                path.push(node.name.clone());
                current_id = node.parent_id.clone();
            } else {
                break;
            }
        }
        
        path.reverse();
        path
    }

    /// Cuenta nodos activos
    pub fn count_active_nodes(&self) -> usize {
        self.nodes
            .values()
            .filter(|n| n.is_active)
            .count()
    }

    /// Cuenta nodos totales
    pub fn count_nodes(&self) -> usize {
        self.nodes.len()
    }

    /// Obtiene lista de nodos activos
    pub fn get_active_nodes(&self) -> Vec<&SceneGraphNode> {
        self.nodes
            .values()
            .filter(|n| n.is_active)
            .collect()
    }

    /// Obtiene lista de nodos por tipo
    pub fn get_nodes_by_type(&self, node_type: NodeType) -> Vec<&SceneGraphNode> {
        self.nodes
            .values()
            .filter(|n| n.node_type == node_type)
            .collect()
    }

    /// Establece escena actual
    pub fn set_current_node(&mut self, node_id: &str) {
        self.current_node_id = Some(node_id.to_string());
        println!("[SCENE GRAPH] Set current node: {}", node_id);
    }

    /// Obtiene escena actual
    pub fn get_current_node(&self) -> Option<&SceneGraphNode> {
        self.current_node_id.as_ref().and_then(|id| self.nodes.get(id))
    }

    /// Obtiene nombre de escena actual
    pub fn get_current_node_name(&self) -> Option<String> {
        self.current_node_id.as_ref().and_then(|id| self.nodes.get(id).map(|n| n.name.clone()))
    }

    /// Limpia grafo
    pub fn clear(&mut self) {
        self.nodes.clear();
        self.root_id = None;
        self.current_node_id = None;
        println!("[SCENE GRAPH] Cleared graph");
    }

    /// Crea grafo de escenas de ejemplo
    pub fn create_sample_graph(&mut self) {
        // Crear raíz
        self.create_node("root".to_string(), "Root Scene".to_string(), NodeType::Scene);
        
        // Crear grupos
        self.create_node("group1".to_string(), "Game Scenes".to_string(), NodeType::Group);
        self.create_node("group2".to_string(), "Menu Scenes".to_string(), NodeType::Group);
        
        // Crear escenas
        self.create_node("scene1".to_string(), "Main Menu".to_string(), NodeType::Scene);
        self.create_node("scene2".to_string(), "Game".to_string(), NodeType::Scene);
        self.create_node("scene3".to_string(), "Level 1".to_string(), NodeType::Scene);
        self.create_node("scene4".to_string(), "Level 2".to_string(), NodeType::Scene);
        self.create_node("scene5".to_string(), "Pause Menu".to_string(), NodeType::Scene);
        self.create_node("scene6".to_string(), "Settings".to_string(), NodeType::Scene);
        
        // Crear conexiones
        self.set_parent("scene1".to_string(), Some("group1".to_string()));
        self.set_parent("scene2".to_string(), Some("group1".to_string()));
        self.set_parent("scene3".to_string(), Some("group1".to_string()));
        self.set_parent("scene4".to_string(), Some("group1".to_string()));
        self.set_parent("scene5".to_string(), Some("group2".to_string()));
        self.set_parent("scene6".to_string(), Some("group2".to_string()));
        
        println!("[SCENE GRAPH] Created sample graph with {} nodes", self.count_nodes());
    }
}

/// Implementación de SceneGraph con SceneManager
impl SceneGraph {
    /// Obtiene SceneManager si está disponible
    fn get_scene_manager(&self) -> Option<&super::scene_manager::SceneManager> {
        self.scene_manager.as_ref()
    }

    /// Obtiene SceneManager mutada si está disponible
    fn get_scene_manager_mut(&mut self) -> Option<&mut super::scene_manager::SceneManager> {
        self.scene_manager.as_mut()
    }

    /// Navega a escena en SceneManager
    pub fn navigate_to_scene(&mut self, scene_id: &str) {
        if let Some(scene_manager) = self.get_scene_manager_mut() {
            scene_manager.set_current_scene(scene_id);
        }
    }

    /// Obtiene nombre de escena actual de SceneManager
    pub fn get_current_scene_name(&self) -> Option<String> {
        self.get_scene_manager()
            .and_then(|sm| sm.get_current_scene_name())
    }
}
