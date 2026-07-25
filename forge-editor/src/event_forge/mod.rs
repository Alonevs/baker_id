use std::collections::HashMap;
use std::sync::Arc;
use std::sync::RwLock;
use serde::{Serialize, Deserialize};

/// Resultado de ejecución de evento
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventResult {
    pub node_id: u32,
    pub success: bool,
    pub output_data: Option<String>,
}

/// Socket de conexión
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct SocketId {
    pub node_id: u32,
    pub socket_index: u32,
    pub is_output: bool,
}

impl SocketId {
    pub fn new(node_id: u32, socket_index: u32, is_output: bool) -> Self {
        Self {
            node_id,
            socket_index,
            is_output,
        }
    }

    pub fn output(node_id: u32, socket_index: u32) -> Self {
        Self::new(node_id, socket_index, true)
    }

    pub fn input(node_id: u32, socket_index: u32) -> Self {
        Self::new(node_id, socket_index, false)
    }
}

impl SocketId {
    pub fn from_id(socket_id: SocketId) -> Self {
        socket_id
    }
}

/// Nodo de evento
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Node {
    pub id: u32,
    pub node_type: NodeType,
    pub position: (f32, f32),
    pub properties: HashMap<String, String>,
    pub execution_count: u32,
}

impl Node {
    pub fn new(id: u32, node_type: NodeType) -> Self {
        Self {
            id,
            node_type,
            position: (0.0, 0.0),
            properties: HashMap::new(),
            execution_count: 0,
        }
    }
}

/// Tipo de nodo
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum NodeType {
    TriggerZone,
    Dialogue,
    Conditional,
    Cinematic,
    Action,
}

/// Conexión entre nodos
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Connection {
    pub from: SocketId,
    pub to: SocketId,
    pub bezier_path: Vec<(f32, f32)>,
}

/// Grafo de eventos
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventGraph {
    pub nodes: HashMap<u32, Node>,
    pub connections: Vec<Connection>,
}

impl EventGraph {
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            connections: Vec::new(),
        }
    }

    pub fn add_node(&mut self, node: Node) {
        self.nodes.insert(node.id, node);
    }

    pub fn get_node(&self, id: u32) -> Option<&Node> {
        self.nodes.get(&id)
    }

    pub fn get_node_mut(&mut self, id: u32) -> Option<&mut Node> {
        self.nodes.get_mut(&id)
    }

    pub fn remove_node(&mut self, id: u32) -> bool {
        let mut removed = false;
        
        // Eliminar conexiones salientes
        self.connections.retain(|conn| {
            if conn.from.node_id == id {
                removed = true;
                false
            } else {
                true
            }
        });

        // Eliminar conexiones entrantes
        self.connections.retain(|conn| {
            if conn.to.node_id == id {
                removed = true;
                false
            } else {
                true
            }
        });

        // Eliminar nodo
        self.nodes.remove(&id);
        removed
    }

    pub fn add_connection(&mut self, from: SocketId, to: SocketId) {
        let bezier_path = self.calculate_bezier(&from, &to);
        self.connections.push(Connection {
            from,
            to,
            bezier_path,
        });
    }

    fn calculate_bezier(&self, from: &SocketId, to: &SocketId) -> Vec<(f32, f32)> {
        // Simplificación: línea recta
        let node_from = self.nodes.get(&from.node_id);
        let node_to = self.nodes.get(&to.node_id);

        if let (Some(node_from), Some(node_to)) = (node_from, node_to) {
            let start = (node_from.position.0 + 100.0, node_from.position.1 + 50.0);
            let end = (node_to.position.0 - 100.0, node_to.position.1 + 50.0);

            // Curva de Bézier cuadrática simple
            let mid_x = (start.0 + end.0) / 2.0;
            let mid_y = (start.1 + end.1) / 2.0;

            vec![
                (start.0, start.1),
                (mid_x, mid_y),
                (end.0, end.1),
            ]
        } else {
            vec![]
        }
    }

    pub fn save_json(&self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let json = serde_json::to_string_pretty(self)?;
        std::fs::write(path, json)?;
        Ok(())
    }

    pub fn load_json(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let json = std::fs::read_to_string(path)?;
        let graph: Self = serde_json::from_str(&json)?;
        Ok(graph)
    }
}

/// Gestor de nodos de eventos
#[derive(Debug, Clone)]
pub struct EventNodeManager {
    pub graph: EventGraph,
    pub selected_node: Option<u32>,
    pub runtime_context: RuntimeContext,
}

impl Default for EventNodeManager {
    fn default() -> Self {
        Self::new()
    }
}

impl EventNodeManager {
    pub fn new() -> Self {
        Self {
            graph: EventGraph::new(),
            selected_node: None,
            runtime_context: RuntimeContext::new(),
        }
    }

    pub fn add_node(&mut self, node: Node) {
        self.graph.add_node(node);
    }

    pub fn get_node(&self, id: u32) -> Option<&Node> {
        self.graph.get_node(id)
    }

    pub fn get_node_mut(&mut self, id: u32) -> Option<&mut Node> {
        self.graph.get_node_mut(id)
    }

    pub fn select_node(&mut self, id: u32) {
        self.selected_node = Some(id);
    }

    pub fn deselect_node(&mut self) {
        self.selected_node = None;
    }

    pub fn create_connection(&mut self, from: SocketId, to: SocketId) {
        self.graph.add_connection(from, to);
    }

    pub fn remove_connection(&mut self, from: SocketId, to: SocketId) {
        self.graph.connections.retain(|conn| {
            !(conn.from == from && conn.to == to)
        });
    }

    pub fn execute(&mut self) -> Vec<EventResult> {
        let results = Vec::new();
        results
    }

    pub fn get_connections_for_node(&self, node_id: u32) -> Vec<&Connection> {
        self.graph.connections.iter().filter(|conn| {
            conn.from.node_id == node_id || conn.to.node_id == node_id
        }).collect()
    }

    pub fn save_json(&self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        self.graph.save_json(path)
    }

    pub fn load_json(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let graph = EventGraph::load_json(path)?;
        Ok(Self {
            graph,
            selected_node: None,
            runtime_context: RuntimeContext::new(),
        })
    }
}

/// Contexto de runtime para variables de estado
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuntimeContext {
    pub variables: HashMap<String, f32>,
    pub flags: HashMap<String, bool>,
    pub counters: HashMap<String, u32>,
}

impl RuntimeContext {
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
            flags: HashMap::new(),
            counters: HashMap::new(),
        }
    }

    pub fn set_variable(&mut self, name: &str, value: f32) {
        self.variables.insert(name.to_string(), value);
    }

    pub fn get_variable(&self, name: &str) -> Option<f32> {
        self.variables.get(name).copied()
    }

    pub fn set_flag(&mut self, name: &str, value: bool) {
        self.flags.insert(name.to_string(), value);
    }

    pub fn get_flag(&self, name: &str) -> Option<bool> {
        self.flags.get(name).copied()
    }

    pub fn increment_counter(&mut self, name: &str) -> u32 {
        let current = self.counters.get(name).copied().unwrap_or(0);
        let new_value = current + 1;
        self.counters.insert(name.to_string(), new_value);
        new_value
    }

    pub fn get_counter(&self, name: &str) -> u32 {
        *self.counters.get(name).unwrap_or(&0)
    }

    pub fn clear(&mut self) {
        self.variables.clear();
        self.flags.clear();
        self.counters.clear();
    }
}

/// Widget UI para EventForge
pub struct EventForgeWidget {
    pub manager: EventNodeManager,
    pub width: f32,
    pub height: f32,
}

impl Default for EventForgeWidget {
    fn default() -> Self {
        Self::new()
    }
}

impl EventForgeWidget {
    pub fn new() -> Self {
        Self {
            manager: EventNodeManager::new(),
            width: 800.0,
            height: 600.0,
        }
    }

    pub fn render(&self) {
        println!("EventForgeWidget: {} nodes, {} connections", 
            self.manager.graph.nodes.len(),
            self.manager.graph.connections.len()
        );
    }

    pub fn add_node(&mut self) {
        let node = Node::new(
            self.manager.graph.nodes.len() as u32 + 1,
            NodeType::TriggerZone,
        );
        self.manager.add_node(node);
    }
}
