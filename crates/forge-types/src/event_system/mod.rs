//! Sistema de Eventos y Nodos Lógicos (Event Forge / Cine Graph)

pub mod nodes;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Nodo en el grafo de eventos
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventNode {
    pub id: Uuid,
    pub node_type: NodeType,
    pub label: String,
    pub data: NodeData,
    pub inputs: Vec<NodePort>,
    pub outputs: Vec<NodePort>,
    pub metadata: NodeMetadata,
}

impl EventNode {
    pub fn new(id: Uuid, node_type: NodeType, label: impl Into<String>) -> Self {
        Self {
            id,
            node_type,
            label: label.into(),
            data: NodeData::default(),
            inputs: Vec::new(),
            outputs: Vec::new(),
            metadata: NodeMetadata::default(),
        }
    }
}

/// Datos específicos del nodo
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct NodeData {
    #[serde(flatten)]
    pub common: CommonNodeData,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CommonNodeData {
    pub value: Option<f64>,
    pub text: Option<String>,
    pub enabled: bool,
}

/// Puerto de conexión del nodo
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodePort {
    pub id: Uuid,
    pub port_type: PortType,
    pub label: Option<String>,
}

/// Tipo de puerto de conexión
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum PortType {
    /// Entrada (input)
    Input,
    /// Salida (output)
    Output,
    /// Entrada de datos
    Data,
    /// Salida de datos
    Result,
}

impl std::fmt::Display for PortType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PortType::Input => write!(f, "input"),
            PortType::Output => write!(f, "output"),
            PortType::Data => write!(f, "data"),
            PortType::Result => write!(f, "result"),
        }
    }
}

/// Línea de conexión entre nodos (cable de Bézier)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Connection {
    pub from: (Uuid, Uuid),  // (node_id, port_id)
    pub to: (Uuid, Uuid),
    pub color: String,
    pub label: Option<String>,
}

/// Grafo completo de eventos
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventGraph {
    pub nodes: Vec<EventNode>,
    pub connections: Vec<Connection>,
    pub active_path: Vec<Uuid>,
    pub metadata: GraphMetadata,
}

impl EventGraph {
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            connections: Vec::new(),
            active_path: Vec::new(),
            metadata: GraphMetadata::default(),
        }
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GraphMetadata {
    pub map_reference: Option<String>,
    pub entity_reference: Option<String>,
    pub description: Option<String>,
    pub is_cinematic: bool,
    pub is_interactive: bool,
}

/// Pestaña de lógica (para organizar el grafo)
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum LogicTab {
    /// Pestaña de narrativa y diálogos
    Narrative,
    /// Pestaña de combate y habilidades
    Combat,
    /// Pestaña de inventario y restricciones
    Inventory,
    /// Pestaña de IA y comportamiento
    Ai,
    /// Pestaña de eventos de mundo
    WorldEvents,
}

impl std::fmt::Display for LogicTab {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LogicTab::Narrative => write!(f, "Narrativa"),
            LogicTab::Combat => write!(f, "Combate"),
            LogicTab::Inventory => write!(f, "Inventario"),
            LogicTab::Ai => write!(f, "IA"),
            LogicTab::WorldEvents => write!(f, "Mundo"),
        }
    }
}

/// Metadatos del nodo
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct NodeMetadata {
    pub author: Option<String>,
    pub description: Option<String>,
    pub tags: Vec<String>,
    pub custom_data: serde_json::Value,
}

/// Estado de ejecución del grafo (para depuración visual)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionState {
    pub current_node: Option<Uuid>,
    pub active_connections: Vec<(Uuid, Uuid)>,
    pub execution_time: f64,
    pub flags: Vec<String>,
}

/// Nodo de decisión multiopción (hasta 4 opciones)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecisionNode {
    pub conditions: Vec<Condition>,
    pub options: Vec<DecisionOption>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecisionOption {
    pub label: String,
    pub next_node: Option<Uuid>,
    pub action: Option<NodeAction>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Condition {
    pub variable: String,
    pub operator: ConditionOperator,
    pub value: f64,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ConditionOperator {
    Equal,
    NotEqual,
    GreaterThan,
    LessThan,
    GreaterEqual,
    LessEqual,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeAction {
    pub action_type: ActionType,
    pub parameters: serde_json::Value,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum NodeType {
    /// Nodo de evento de texto
    Event,
    /// Nodo de decisión multiopción
    Decision,
    /// Nodo de acción
    Action,
    /// Nodo de condición
    Condition,
    /// Nodo de cinemática
    Cinematic,
    /// Nodo de fin de evento
    EndEvent,
}

impl std::fmt::Display for NodeType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NodeType::Event => write!(f, "🟡 EVENTO"),
            NodeType::Decision => write!(f, "🔷 DECISIÓN"),
            NodeType::Action => write!(f, "🔵 ACCIÓN"),
            NodeType::Condition => write!(f, "🟣 CONDICIÓN"),
            NodeType::Cinematic => write!(f, "🎬 CINEMÁTICA"),
            NodeType::EndEvent => write!(f, "⏹️ FIN"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ActionType {
    PlayAnimation { animation_id: String },
    TriggerSound { sound_id: String },
    SpawnParticle { particle_id: String },
    SetVariable { variable: String, value: f64 },
    ChangeState { state_id: String },
    ExecuteEvent { event_id: String },
}
