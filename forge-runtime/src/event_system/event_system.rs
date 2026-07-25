//! Sistema de Eventos y Nodos LÃ³gicos

use crate::entities::Component;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Tipo de evento
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum EventType {
    Update,
    Animation,
    Input,
    Collision,
    Custom(String),
}

/// Nodo de evento
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct EventNode {
    pub id: u64,
    pub event_type: EventType,
    pub inputs: Vec<String>,
    pub outputs: Vec<String>,
    pub data: HashMap<String, serde_json::Value>,
}

impl Default for EventNode {
    fn default() -> Self {
        Self {
            id: 0,
            event_type: EventType::Update,
            inputs: Vec::new(),
            outputs: Vec::new(),
            data: HashMap::new(),
        }
    }
}

impl EventNode {
    pub fn new(event_type: EventType) -> Self {
        Self {
            id: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_millis() as u64,
            event_type,
            ..Default::default()
        }
    }

    pub fn add_input(&mut self, input: String) {
        self.inputs.push(input);
    }

    pub fn add_output(&mut self, output: String) {
        self.outputs.push(output);
    }

    pub fn has_input(&self, input: &str) -> bool {
        self.inputs.contains(&input.to_string())
    }

    pub fn has_output(&self, output: &str) -> bool {
        self.outputs.contains(&output.to_string())
    }
}

/// Sistema de eventos
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct EventSystem {
    pub nodes: HashMap<u64, EventNode>,
    pub active_node_id: Option<u64>,
    pub event_queue: Vec<EventNode>,
}

impl EventSystem {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_node(&mut self, node: EventNode) -> u64 {
        let id = node.id;
        self.nodes.insert(id, node);
        id
    }

    pub fn get_node(&self, id: u64) -> Option<&EventNode> {
        self.nodes.get(&id)
    }

    pub fn get_node_mut(&mut self, id: u64) -> Option<&mut EventNode> {
        self.nodes.get_mut(&id)
    }

    pub fn activate_node(&mut self, node_id: u64) {
        self.active_node_id = Some(node_id);
    }

    pub fn get_active_node(&self) -> Option<&EventNode> {
        self.active_node_id.and_then(|id| self.nodes.get(&id))
    }

    pub fn process_event(&mut self, event_type: EventType, data: HashMap<String, serde_json::Value>) {
        // Procesar evento en nodos activos
        if let Some(active_node) = self.get_active_node() {
            if active_node.event_type == event_type {
                // Ejecutar nodo
            }
        }
    }

    pub fn clear(&mut self) {
        self.nodes.clear();
        self.event_queue.clear();
        self.active_node_id = None;
    }
}

