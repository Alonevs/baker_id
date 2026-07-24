//! # Event Nodes Module
//! 
//! Módulo para nodos de evento.
use std::collections::{HashMap, VecDeque};



/// Event Node Type
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum EventType {
    Trigger,
    Condition,
    Action,
    Variable,
    
    // Triggers
    OnStart,
    OnClick,
    OnTimer,
    OnEnter,
    OnExit,
    OnCollision,
    OnTrigger,
    OnKeyPress,
    OnMouseOver,
    OnMouseClick,
    
    // Actions
    ShowDialog,
    PlayAnimation,
    StopAnimation,
    PauseAnimation,
    ResumeAnimation,
    ChangeVariable,
    CallFunction,
    SetFlag,
    ClearFlag,
    SpawnEntity,
    RemoveEntity,
    PlaySound,
    ChangeColor,
    ChangePosition,
    Rotate,
    Scale,
    
    // Conditionals
    IfCondition,
    WhileLoop,
    ForLoop,
    Switch,
    AndCondition,
    OrCondition,
    NotCondition,
    
    // System
    Empty,
}

/// Node Data
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum NodeData {
    OnStart { auto_execute: bool },
    OnClick { target: String, button: i32 },
    OnTimer { interval_ms: i32, count: i32 },
    OnEnter { target: String },
    OnExit { target: String },
    OnCollision { target: String, normal: [f32; 2] },
    OnTrigger { target: String },
    OnKeyPress { key: String },
    OnMouseOver { target: String },
    OnMouseClick { target: String, button: i32 },
    ShowDialog { dialog_id: String, language: String },
    PlayAnimation { target: String, animation: String, duration_ms: i32 },
    StopAnimation { target: String, animation: String },
    PauseAnimation { target: String, animation: String },
    ResumeAnimation { target: String, animation: String },
    ChangeVariable { variable: String, operation: String, value: String },
    CallFunction { function_name: String, args: Vec<String> },
    SetFlag { flag: String },
    ClearFlag { flag: String },
    SpawnEntity { entity_type: String, position: [f32; 2] },
    RemoveEntity { entity_id: String },
    PlaySound { sound_name: String, volume: f32 },
    ChangeColor { r: f32, g: f32, b: f32, a: f32 },
    ChangePosition { x: f32, y: f32 },
    Rotate { angle_degrees: f32 },
    Scale { x: f32, y: f32 },
    IfCondition { condition: String, then_branch: String, else_branch: String },
    WhileLoop { condition: String, body_branch: String },
    ForLoop { variable: String, start: String, end: String, body_branch: String },
    Switch { value: String, cases: Vec<(String, String)> },
    AndCondition { conditions: Vec<String> },
    OrCondition { conditions: Vec<String> },
    NotCondition { condition: String },
    Empty,
}

mod pos2_serde {
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    pub fn serialize<S>(pos: &eframe::egui::Pos2, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        [pos.x, pos.y].serialize(serializer)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<eframe::egui::Pos2, D::Error>
    where
        D: Deserializer<'de>,
    {
        let arr = <[f32; 2]>::deserialize(deserializer)?;
        Ok(eframe::egui::Pos2::new(arr[0], arr[1]))
    }
}

/// Event Node
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct EventNode {
    pub id: String,
    pub event_type: EventType,
    #[serde(with = "pos2_serde")]
    pub position: eframe::egui::Pos2,
    pub execution_count: u32,
    pub is_active: bool,
    pub data: NodeData,
    pub group_id: Option<String>,
}

/// Edge
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Edge {
    pub from: String,
    pub to: String,
    pub condition: Option<String>,
}

/// Event Graph
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct EventGraph {
    pub nodes: Vec<EventNode>,
    pub edges: Vec<Edge>,
}

/// Runtime Context for execution
#[derive(Debug, Clone, Default)]
pub struct RuntimeContext {
    /// Variables de estado (clave-valor)
    pub variables: HashMap<String, String>,
    /// Callbacks para eventos
    pub callbacks: HashMap<String, VecDeque<String>>,
    /// Flag para controlar ejecución
    pub is_running: bool,
}

impl RuntimeContext {
    /// Crea un nuevo contexto de runtime vacío
    pub fn new() -> Self {
        Self::default()
    }

    /// Obtiene o crea una variable
    pub fn get_or_create_var(&mut self, name: &str, default: String) -> String {
        if let Some(var) = self.variables.get(name) {
            var.clone()
        } else {
            self.variables.insert(name.to_string(), default.clone());
            default
        }
    }

    /// Obtiene el valor de una variable
    pub fn get_var(&self, name: &str) -> Option<String> {
        self.variables.get(name).cloned()
    }

    /// Establece el valor de una variable
    pub fn set_var(&mut self, name: &str, value: String) {
        self.variables.insert(name.to_string(), value);
    }

    /// Elimina una variable
    pub fn remove_var(&mut self, name: &str) {
        self.variables.remove(name);
    }

    /// Registra un callback para un evento
    pub fn register_callback(&mut self, event_name: &str, callback_id: &str) {
        self.callbacks
            .entry(event_name.to_string())
            .or_insert_with(VecDeque::new)
            .push_front(callback_id.to_string());
    }

    /// Obtiene los callbacks para un evento
    pub fn get_callbacks(&self, event_name: &str) -> Vec<String> {
        self.callbacks.get(event_name).map(|v| v.iter().cloned().collect()).unwrap_or_default()
    }

    /// Evalúa una condición simple
    pub fn evaluate_condition(&self, condition: &str) -> bool {
        // Evaluar condiciones básicas
        let condition_lower = condition.to_lowercase();
        
        // Compara de igualdad
        if condition_lower.starts_with("var:") {
            if let Some(var_name) = condition_lower.strip_prefix("var:") {
                if let Some(var_value) = self.variables.get(var_name) {
                    return var_value.to_lowercase() == "true";
                }
                return false;
            }
        }
        
        // Compara con variables
        if condition_lower.starts_with("variable:") {
            if let Some(var_name) = condition_lower.strip_prefix("variable:") {
                if let Some(var_value) = self.variables.get(var_name) {
                    return var_value.to_lowercase() == "true";
                }
                return false;
            }
        }
        
        // Compara con flags
        if condition_lower.starts_with("flag:") {
            if let Some(flag_name) = condition_lower.strip_prefix("flag:") {
                return self.variables.get(flag_name).map_or(false, |v| v.to_lowercase() == "true");
            }
        }
        
        // Compara con valores numéricos
        if condition_lower.starts_with("count:") {
            if let Some(count_name) = condition_lower.strip_prefix("count:") {
                if let Some(count_value) = self.variables.get(count_name) {
                    if let Ok(num) = count_value.parse::<i32>() {
                        return num > 0;
                    }
                }
            }
        }
        
        // Default: condición siempre verdadera
        true
    }
}

/// Variable Operation
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum VariableOperation {
    Set,
    Increment,
    Decrement,
    Add,
    Subtract,
    Multiply,
    Divide,
}

/// Grupo de nodos
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct NodeGroup {
    pub id: String,
    pub name: String,
    pub is_collapsed: bool,
    pub children: Vec<String>, // IDs de grupos hijos (para anidación)
}

impl NodeGroup {
    pub fn new(id: String, name: String) -> Self {
        Self {
            id,
            name,
            is_collapsed: false,
            children: Vec::new(),
        }
    }
}

