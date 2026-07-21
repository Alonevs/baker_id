use std::path::PathBuf;
use std::collections::HashMap;
use uuid::Uuid;
use crate::script::{Script, ScriptError, ScriptType};
use crate::syntax_highlighter::SyntaxHighlighter;

pub struct ScriptEditor {
    pub current_script: Option<Script>,
    pub open_scripts: HashMap<Uuid, Script>,
    pub script_history: Vec<Script>,
    pub syntax_highlighter: SyntaxHighlighter,
}

impl ScriptEditor {
    pub fn new() -> Self {
        Self {
            current_script: None,
            open_scripts: HashMap::new(),
            script_history: Vec::new(),
            syntax_highlighter: SyntaxHighlighter::new(),
        }
    }

    pub fn open_script(&mut self, path: &PathBuf) -> Result<Script, ScriptError> {
        let script = Script {
            id: uuid::Uuid::new_v4(),
            path: path.clone(),
            name: path.file_name().and_then(|n| n.to_str()).unwrap_or("").to_string(),
            script_type: self.detect_script_type(path)?,
            content: std::fs::read_to_string(path)?,
            variables: HashMap::new(),
            functions: HashMap::new(),
            is_loading: false,
        };

        self.open_scripts.insert(script.id, script.clone());
        self.current_script = Some(script.clone());
        Ok(script)
    }

    pub fn save_script(&mut self) -> Result<(), ScriptError> {
        if let Some(ref script) = self.current_script {
            std::fs::write(&script.path, &script.content)?;
            Ok(())
        } else {
            Err(ScriptError::ScriptNotFound("No script open".to_string()))
        }
    }

    pub fn close_script(&mut self, script_id: &uuid::Uuid) -> Result<(), ScriptError> {
        self.open_scripts.remove(script_id);
        if self.current_script.as_ref().map(|s| &s.id == script_id).unwrap_or(false) {
            self.current_script = self.open_scripts.values().next().cloned();
        }
        Ok(())
    }

    pub fn detect_script_type(&self, path: &PathBuf) -> Result<ScriptType, ScriptError> {
        let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("");
        match ext {
            "gd" | "gdshader" => Ok(ScriptType::GDScript),
            "cs" => Ok(ScriptType::CSharp),
            "cpp" | "h" | "hpp" => Ok(ScriptType::Cpp),
            "hlsl" | "fx" | "compute" => Ok(ScriptType::HLSL),
            "glsl" | "vert" | "frag" | "comp" => Ok(ScriptType::GLSL),
            "shader" => Ok(ScriptType::Shader),
            "bt" => Ok(ScriptType::Blueprint),
            "vs" => Ok(ScriptType::VisualScript),
            _ => Ok(ScriptType::Resource),
        }
    }

    pub fn get_current_script(&self) -> Option<&Script> {
        self.current_script.as_ref()
    }

    pub fn get_current_script_mut(&mut self) -> Option<&mut Script> {
        self.current_script.as_mut()
    }
}

impl Default for ScriptEditor {
    fn default() -> Self {
        Self::new()
    }
}
