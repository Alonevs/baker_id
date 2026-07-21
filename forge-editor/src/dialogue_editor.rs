use std::collections::HashMap;
use std::thread;
use serde::{Serialize, Deserialize};

/// Tipos de variables de diálogo
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum VariableType {
    String,
    Int,
    Float,
    Bool,
}

/// Variable de diálogo
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DialogueVariable {
    pub id: String,
    pub name: String,
    pub variable_type: VariableType,
    pub value: serde_json::Value,
    pub is_global: bool,
    pub description: String,
}

impl Default for DialogueVariable {
    fn default() -> Self {
        Self {
            id: format!("var_{}", std::process::id()),
            name: "new_variable".to_string(),
            variable_type: VariableType::String,
            value: serde_json::Value::String("".to_string()),
            is_global: false,
            description: "".to_string(),
        }
    }
}

/// Condición en diálogo
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DialogueCondition {
    pub id: String,
    pub operator: ConditionOperator,
    pub left_value: serde_json::Value,
    pub right_value: serde_json::Value,
    pub is_negated: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ConditionOperator {
    Equal,
    NotEqual,
    GreaterThan,
    LessThan,
    GreaterThanOrEqual,
    LessThanOrEqual,
    Contains,
    NotContains,
    StartsWith,
    EndsWith,
    IsTrue,
    IsFalse,
}

/// Diálogo
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dialogue {
    pub id: String,
    pub name: String,
    pub actor_id: Option<String>,
    pub language: String,
    pub lines: Vec<DialogueLine>,
    pub conditions: Vec<DialogueCondition>,
    pub variables: HashMap<String, DialogueVariable>,
    pub is_active: bool,
    pub priority: u32,
}

impl Default for Dialogue {
    fn default() -> Self {
        Self {
            id: format!("dialog_{}", std::process::id()),
            name: "new_dialogue".to_string(),
            actor_id: None,
            language: "es".to_string(),
            lines: Vec::new(),
            conditions: Vec::new(),
            variables: HashMap::new(),
            is_active: true,
            priority: 0,
        }
    }
}

/// Línea de diálogo
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DialogueLine {
    pub id: String,
    pub text: String,
    pub speaker: Option<String>,
    pub action: DialogueAction,
    pub duration_ms: u64,
    pub is_hidden: bool,
    pub variables: Vec<String>,
}

impl Default for DialogueLine {
    fn default() -> Self {
        Self {
            id: format!("line_{}", std::process::id()),
            text: "".to_string(),
            speaker: None,
            action: DialogueAction::Speak,
            duration_ms: 2000,
            is_hidden: false,
            variables: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DialogueAction {
    Speak,
    Wait,
    PlayAnimation,
    PlaySound,
    ShowImage,
    Hide,
    Branch,
    End,
}

/// Gestor de diálogos
pub struct DialogueManager {
    pub dialogues: HashMap<String, Dialogue>,
    pub actors: HashMap<String, Actor>,
    pub current_language: String,
    pub selected_dialogue: Option<String>,
    pub selected_line: Option<String>,
    pub variables: HashMap<String, DialogueVariable>,
    pub conditions: Vec<DialogueCondition>,
    pub is_watching: bool,
    pub watch_thread: Option<thread::JoinHandle<()>>,
}

impl Default for DialogueManager {
    fn default() -> Self {
        Self {
            dialogues: HashMap::new(),
            actors: HashMap::new(),
            current_language: "es".to_string(),
            selected_dialogue: None,
            selected_line: None,
            variables: HashMap::new(),
            conditions: Vec::new(),
            is_watching: false,
            watch_thread: None,
        }
    }
}

/// Actor en el diálogo
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Actor {
    pub id: String,
    pub name: String,
    pub dialogues: Vec<String>,
    pub variables: HashMap<String, DialogueVariable>,
    pub is_active: bool,
}

impl Default for Actor {
    fn default() -> Self {
        Self {
            id: format!("actor_{}", std::process::id()),
            name: "new_actor".to_string(),
            dialogues: Vec::new(),
            variables: HashMap::new(),
            is_active: true,
        }
    }
}

impl DialogueManager {
    pub fn new() -> Self {
        Self::default()
    }
    
    pub fn add_dialogue(&mut self, dialogue: Dialogue) {
        self.dialogues.insert(dialogue.id.clone(), dialogue);
    }
    
    pub fn remove_dialogue(&mut self, id: &str) -> bool {
        self.dialogues.remove(id).is_some()
    }
    
    pub fn get_dialogue(&self, id: &str) -> Option<&Dialogue> {
        self.dialogues.get(id)
    }
    
    pub fn get_dialogue_mut(&mut self, id: &str) -> Option<&mut Dialogue> {
        self.dialogues.get_mut(id)
    }
    
    pub fn get_all_dialogues(&self) -> &HashMap<String, Dialogue> {
        &self.dialogues
    }
    
    pub fn get_all_actors(&self) -> &HashMap<String, Actor> {
        &self.actors
    }
    
    pub fn get_variable(&self, name: &str) -> Option<&DialogueVariable> {
        self.variables.get(name)
    }
    
    pub fn set_variable(&mut self, variable: DialogueVariable) {
        self.variables.insert(variable.id.clone(), variable);
    }
    
    pub fn evaluate_condition(&self, condition: &DialogueCondition) -> bool {
        let left: serde_json::Value = serde_json::from_value(condition.left_value.clone()).unwrap_or(serde_json::Value::Null);
        let right: serde_json::Value = serde_json::from_value(condition.right_value.clone()).unwrap_or(serde_json::Value::Null);
        
        match condition.operator {
            ConditionOperator::Equal => left == right,
            ConditionOperator::NotEqual => left != right,
            ConditionOperator::GreaterThan => left.to_string() > right.to_string(),
            ConditionOperator::LessThan => left.to_string() < right.to_string(),
            ConditionOperator::GreaterThanOrEqual => left.to_string() >= right.to_string(),
            ConditionOperator::LessThanOrEqual => left.to_string() <= right.to_string(),
            ConditionOperator::Contains => left.as_str().map(|s| s.contains(&right.to_string())).unwrap_or(false) || left.to_string().contains(&right.to_string()),
            ConditionOperator::NotContains => !(left.as_str().map(|s| s.contains(&right.to_string())).unwrap_or(false) || left.to_string().contains(&right.to_string())),
            ConditionOperator::StartsWith => left.as_str().map(|s| s.starts_with(&right.to_string())).unwrap_or(false) || left.to_string().starts_with(&right.to_string()),
            ConditionOperator::EndsWith => left.as_str().map(|s| s.ends_with(&right.to_string())).unwrap_or(false) || left.to_string().ends_with(&right.to_string()),
            ConditionOperator::IsTrue => left == serde_json::Value::Bool(true),
            ConditionOperator::IsFalse => left == serde_json::Value::Bool(false),
        }
    }
    
    pub fn is_condition_met(&self, condition: &DialogueCondition) -> bool {
        let mut met = self.evaluate_condition(condition);
        if condition.is_negated {
            met = !met;
        }
        met
    }
    
    pub fn toggle_watch(&mut self) {
        self.is_watching = !self.is_watching;
    }

    pub fn render(&mut self) {
        // stub
    }
}

