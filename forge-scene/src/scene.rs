use std::collections::HashMap;
use uuid::Uuid;
use crate::scene_node::NodeData;

#[derive(Debug, Clone)]
pub struct SceneConfig {
    pub name: String,
    pub path: String,
    pub is_main: bool,
    pub is_locked: bool,
    pub node_count: usize,
    pub group: String,
}

#[derive(Debug, Clone)]
pub struct SceneEditor {
    pub nodes: HashMap<String, NodeData>,
    pub scenes: HashMap<String, SceneConfig>,
}

/// Persistencia de nodos para drag & drop
#[derive(Debug, Clone, Default)]
pub struct ScenePersistence {
    pub nodes: HashMap<String, String>, // node_id -> asset_name
}

impl ScenePersistence {
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
        }
    }
    
    pub fn add_node(&mut self, node_id: String, asset_name: &str) {
        self.nodes.insert(node_id, asset_name.to_string());
    }
    
    pub fn get_node(&self, node_id: &str) -> Option<&String> {
        self.nodes.get(node_id)
    }
    
    pub fn remove_node(&mut self, node_id: &str) {
        self.nodes.remove(node_id);
    }
    
    /// Persistir todos los nodos
    pub fn persist_nodes(&self, path: &str) {
        println!("Persisting nodes to: {}", path);
    }
    
    /// Cargar nodos desde persistencia
    pub fn load_nodes(&mut self, path: &str) {
        println!("Loading nodes from: {}", path);
    }
}

impl SceneEditor {
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            scenes: HashMap::new(),
        }
    }
    
    pub fn create_node(&mut self, name: &str) -> String {
        let node_id = format!("node_{}", Uuid::new_v4());
        let node = NodeData::new(name, crate::EntityType::Node);
        
        self.nodes.insert(node_id.clone(), node);
        node_id
    }
    
    pub fn add_node_to_scene(&mut self, _node_id: String, scene_name: &str) {
        self.scenes
            .entry(scene_name.to_string())
            .or_insert_with(|| SceneConfig {
                name: scene_name.to_string(),
                path: format!("scenes/{}.scene", scene_name),
                is_main: false,
                is_locked: false,
                node_count: 0,
                group: "default".to_string(),
            })
            .node_count += 1;
    }

    pub fn add_component(&mut self, node_id: &str, component_type: crate::component::ComponentType) -> Option<String> {
        let node = self.nodes.get_mut(node_id)?;
        let component_id = format!("{}_{}", node_id, Uuid::new_v4());
        
        let component_data = match component_type {
            crate::component::ComponentType::Transform => crate::ComponentData::new_transform(),
            crate::component::ComponentType::Collider => crate::ComponentData::new_collider(),
            crate::component::ComponentType::Renderer => crate::ComponentData::new_renderer(),
            crate::component::ComponentType::Audio => crate::ComponentData::new_audio(),
            crate::component::ComponentType::Physics => crate::ComponentData::new_transform(),
            crate::component::ComponentType::Behavior => crate::ComponentData::new_transform(),
        };
        
        node.components.push(component_data);
        Some(component_id)
    }

    pub fn remove_component(&mut self, node_id: &str, component_index: usize) -> bool {
        let node = self.nodes.get_mut(node_id);
        if let Some(node) = node {
            node.components.remove(component_index);
            true
        } else {
            false
        }
    }

    pub fn export_scene(&mut self, path: &str) -> Result<(), std::io::Error> {
        let nodes_for_export: Vec<NodeData> = self.nodes.values().cloned().collect();
        let json = serde_json::to_string_pretty(&nodes_for_export)?;
        std::fs::write(path, json)?;
        Ok(())
    }
}
