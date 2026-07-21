//! Definición lógica de nodos y cables de Event Forge

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventNode {
    pub id: Uuid,
    pub node_type: NodeType,
    pub properties: serde_json::Value,
    pub inputs: Vec<String>,
    pub outputs: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NodeType {
    Trigger,
    Action,
    Condition,
    Variable,
    System,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventGraph {
    pub nodes: Vec<EventNode>,
    pub connections: Vec<EventConnection>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventConnection {
    pub from: Uuid,
    pub from_output: String,
    pub to: Uuid,
    pub to_input: String,
}
