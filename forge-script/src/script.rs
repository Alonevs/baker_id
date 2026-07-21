use std::path::PathBuf;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use thiserror::Error;
use uuid::{Uuid, Uuid as UuidType};
use crate::syntax_highlighter::SyntaxHighlighter;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Script {
    pub id: Uuid,
    pub path: PathBuf,
    pub name: String,
    pub script_type: ScriptType,
    pub content: String,
    pub variables: HashMap<String, ScriptVariable>,
    pub functions: HashMap<String, FunctionInfo>,
    pub is_loading: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ScriptType {
    GDScript,
    CSharp,
    Cpp,
    HLSL,
    GLSL,
    Shader,
    Blueprint,
    VisualScript,
    Resource,
    Scene,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScriptVariable {
    pub name: String,
    pub r#type: String,
    pub value: serde_json::Value,
    pub is_public: bool,
    pub is_constant: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionInfo {
    pub name: String,
    pub return_type: String,
    pub parameters: Vec<FunctionParameter>,
    pub is_static: bool,
    pub is_abstract: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionParameter {
    pub name: String,
    pub r#type: String,
    pub is_vararg: bool,
}

#[derive(Debug, Error)]
pub enum ScriptError {
    #[error("Script not found: {0}")]
    ScriptNotFound(String),
    #[error("Invalid script format")]
    InvalidFormat,
    #[error("Script is loading")]
    ScriptLoading,
    #[error("Syntax error: {0}")]
    SyntaxError(String),
    #[error("Type error: {0}")]
    TypeError(String),
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}
