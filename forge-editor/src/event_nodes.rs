//! # Event Nodes Module
//! 
//! Módulo para nodos de evento.



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
    ShowDialog { dialog_id: String, language: String },
    PlayAnimation { target: String, animation: String, duration_ms: i32 },
    ChangeVariable { variable: String, operation: String, value: String },
    IfCondition { condition: String, then_branch: String, else_branch: String },
    AndCondition { conditions: Vec<String> },
    OrCondition { conditions: Vec<String> },
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

