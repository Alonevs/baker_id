//! Script Resource - Gestión de scripts de usuario

use serde::{Deserialize, Serialize};
use std::path::Path;

// Importar trait Resource del módulo padre
use super::resource_manager::{Resource, ResourceType};

/// Script Resource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScriptResource {
    pub name: String,
    pub code: String,
    pub language: ScriptLanguage,
    pub is_compiled: bool,
    pub compiled_data: Option<serde_json::Value>,
    pub dependencies: Vec<String>,
    pub metadata: Option<serde_json::Value>,
}

impl ScriptResource {
    /// Crea nuevo script
    pub fn new(name: &str, code: &str, language: ScriptLanguage) -> Self {
        Self {
            name: name.to_string(),
            code: code.to_string(),
            language,
            is_compiled: false,
            compiled_data: None,
            dependencies: Vec::new(),
            metadata: None,
        }
    }

    /// Crea script desde JSON
    pub fn from_json(name: &str, data: &serde_json::Value) -> Result<Self, String> {
        let code = data["code"]
            .as_str()
            .ok_or_else(|| format!("Missing code in script: {}", name))?
            .to_string();

        let language = ScriptLanguage::from_json(&data["language"]);

        Ok(Self {
            name: name.to_string(),
            code,
            language,
            is_compiled: data["is_compiled"]
                .as_bool()
                .unwrap_or(false),
            compiled_data: data.get("compiled_data").cloned(),
            dependencies: data["dependencies"]
                .as_array()
                .map(|d| d.iter().map(|s| s.as_str().unwrap_or("").to_string()).collect())
                .unwrap_or_default(),
            metadata: data.get("metadata").cloned(),
        })
    }

    /// Obtiene código del script
    pub fn code(&self) -> &str {
        &self.code
    }

    /// Obtiene lenguaje del script
    pub fn language(&self) -> &ScriptLanguage {
        &self.language
    }

    /// Verifica si está compilado
    pub fn is_compiled(&self) -> bool {
        self.is_compiled
    }
}

impl Default for ScriptResource {
    fn default() -> Self {
        Self::new("default", "// empty script", ScriptLanguage::Lua)
    }
}

/// Lenguaje de script
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ScriptLanguage {
    Lua,
    Rust,
    Python,
    Custom(String),
}

impl ScriptLanguage {
    /// Crea lenguaje desde JSON
    pub fn from_json(data: &serde_json::Value) -> Self {
        match data.as_str() {
            Some("lua") | Some("Lua") => ScriptLanguage::Lua,
            Some("rust") | Some("Rust") => ScriptLanguage::Rust,
            Some("python") | Some("Python") => ScriptLanguage::Python,
            Some(custom) => ScriptLanguage::Custom(custom.to_string()),
            None => ScriptLanguage::Lua,
        }
    }

    /// Obtiene extensión del archivo
    pub fn extension(&self) -> &str {
        match self {
            ScriptLanguage::Lua => "lua",
            ScriptLanguage::Rust => "rs",
            ScriptLanguage::Python => "py",
            ScriptLanguage::Custom(name) => name,
        }
    }
}

/// Script Module para scripts compilados
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScriptModule {
    pub name: String,
    pub functions: Vec<ScriptFunction>,
    pub variables: Vec<ScriptVariable>,
    pub is_active: bool,
}

impl Default for ScriptModule {
    fn default() -> Self {
        Self {
            name: String::new(),
            functions: Vec::new(),
            variables: Vec::new(),
            is_active: false,
        }
    }
}

/// Función de script
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScriptFunction {
    pub name: String,
    pub parameters: Vec<String>,
    pub return_type: Option<String>,
    pub is_builtin: bool,
}

impl Default for ScriptFunction {
    fn default() -> Self {
        Self {
            name: String::new(),
            parameters: Vec::new(),
            return_type: None,
            is_builtin: false,
        }
    }
}

/// Variable de script
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScriptVariable {
    pub name: String,
    pub value: serde_json::Value,
    pub is_readonly: bool,
}

impl Default for ScriptVariable {
    fn default() -> Self {
        Self {
            name: String::new(),
            value: serde_json::Value::Null,
            is_readonly: false,
        }
    }
}

// Implementación del trait Resource
impl Resource for ScriptResource {
    fn name(&self) -> &str { &self.name }
    fn resource_type(&self) -> &ResourceType { &ResourceType::Script }
    fn load(&mut self) -> Result<(), String> { Ok(()) }
    fn unload(self) where Self: Sized {}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_script_resource_new() {
        let script = ScriptResource::new("test", "print('hello')", ScriptLanguage::Lua);
        assert_eq!(script.name, "test");
        assert_eq!(script.code, "print('hello')");
        assert_eq!(script.language, ScriptLanguage::Lua);
    }

    #[test]
    fn test_script_resource_default() {
        let script = ScriptResource::default();
        assert_eq!(script.name, "default");
        assert_eq!(script.language, ScriptLanguage::Lua);
    }

    #[test]
    fn test_script_language_lua() {
        let lang = ScriptLanguage::Lua;
        assert_eq!(lang.extension(), "lua");
    }

    #[test]
    fn test_script_language_rust() {
        let lang = ScriptLanguage::Rust;
        assert_eq!(lang.extension(), "rs");
    }

    #[test]
    fn test_script_language_python() {
        let lang = ScriptLanguage::Python;
        assert_eq!(lang.extension(), "py");
    }

    #[test]
    fn test_script_module_default() {
        let module = ScriptModule::default();
        assert!(!module.is_active);
        assert!(module.functions.is_empty());
    }

    #[test]
    fn test_script_function_default() {
        let func = ScriptFunction::default();
        assert_eq!(func.name, "");
        assert!(func.parameters.is_empty());
    }

    #[test]
    fn test_script_variable_default() {
        let var = ScriptVariable::default();
        assert_eq!(var.name, "");
        assert!(!var.is_readonly);
    }
}
