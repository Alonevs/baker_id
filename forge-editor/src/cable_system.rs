//! # Cable System Module
//! 
//! Módulo para sistema de cables.

/// Cable System
#[derive(Debug, Clone, Default)]
pub struct CableSystem {
    pub cables: Vec<Cable>,
    pub nodes: Vec<Node>,
}

impl CableSystem {
    pub fn new() -> Self {
        Self::default()
    }
    
    pub fn add_cable(&mut self, cable: Cable) {
        self.cables.push(cable);
    }
    
    pub fn add_node(&mut self, node: Node) {
        self.nodes.push(node);
    }
}

/// Cable
#[derive(Debug, Clone)]
pub struct Cable {
    pub from: String,
    pub to: String,
    pub data: serde_json::Value,
}

/// Node
#[derive(Debug, Clone)]
pub struct Node {
    pub id: String,
    pub ty: String,
    pub position: (f32, f32),
}

