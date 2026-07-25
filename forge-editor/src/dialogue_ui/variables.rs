use crate::dialogue_ui::{DialogueVariable, DialogueValue};
use std::collections::HashMap;

/// Contexto de variables de diálogo
#[derive(Debug, Default)]
pub struct DialogueVariables {
    pub variables: HashMap<String, DialogueVariable>,
    pub local_variables: HashMap<String, DialogueVariable>,
    pub next_id: u64,
}

impl DialogueVariables {
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
            local_variables: HashMap::new(),
            next_id: 1,
        }
    }
    
    /// Crear nueva variable
    pub fn create(&mut self, name: &str, value: DialogueValue) -> DialogueVariable {
        let id = self.next_id;
        self.next_id += 1;
        
        let variable = DialogueVariable::local(name.to_string(), value);
        self.local_variables.insert(name.to_string(), variable);
        
        DialogueVariable {
            name: name.to_string(),
            value,
            is_local: true,
        }
    }
    
    /// Obtener variable por nombre
    pub fn get(&self, name: &str) -> Option<&DialogueVariable> {
        self.local_variables.get(name).or_else(|| self.variables.get(name))
    }
    
    /// Obtener valor de variable
    pub fn get_value(&self, name: &str) -> Option<&DialogueValue> {
        self.get(name).map(|v| &v.value)
    }
    
    /// Verificar si existe variable
    pub fn has(&self, name: &str) -> bool {
        self.local_variables.contains_key(name) || self.variables.contains_key(name)
    }
    
    /// Añadir variable al contexto global
    pub fn add_global(&mut self, variable: DialogueVariable) {
        self.variables.insert(variable.name.clone(), variable);
    }
    
    /// Remover variable
    pub fn remove(&mut self, name: &str) -> Option<DialogueVariable> {
        self.local_variables.remove(name).or_else(|| self.variables.remove(name))
    }
    
    /// Limpiar variables locales
    pub fn clear_local(&mut self) {
        self.local_variables.clear();
    }
    
    /// Limpiar todas las variables
    pub fn clear(&mut self) {
        self.variables.clear();
        self.local_variables.clear();
    }
    
    /// Obtener número total de variables
    pub fn count(&self) -> usize {
        self.variables.len() + self.local_variables.len()
    }
}

/// Contexto de diálogo completo
#[derive(Debug, Default)]
pub struct DialogueContext {
    pub dialogue_id: u64,
    pub speaker: Option<String>,
    pub lines: Vec<String>,
    pub choices: Vec<String>,
    pub variables: DialogueVariables,
    pub is_active: bool,
    pub current_line_index: usize,
    pub created_at: std::time::Instant,
    pub last_updated: std::time::Instant,
}

impl DialogueContext {
    pub fn new(dialogue_id: u64, lines: Vec<String>, choices: Vec<String>) -> Self {
        Self {
            dialogue_id,
            speaker: None,
            lines,
            choices,
            variables: DialogueVariables::new(),
            is_active: false,
            current_line_index: 0,
            created_at: std::time::Instant::now(),
            last_updated: std::time::Instant::now(),
        }
    }
    
    /// Línea actual
    pub fn current_line(&self) -> Option<&String> {
        self.lines.get(self.current_line_index)
    }
    
    /// Siguiente línea
    pub fn next_line(&mut self) {
        if self.current_line_index < self.lines.len() - 1 {
            self.current_line_index += 1;
            self.last_updated = std::time::Instant::now();
        }
    }
    
    /// Línea anterior
    pub fn prev_line(&mut self) {
        if self.current_line_index > 0 {
            self.current_line_index -= 1;
            self.last_updated = std::time::Instant::now();
        }
    }
    
    /// Obtener número total de líneas
    pub fn line_count(&self) -> usize {
        self.lines.len()
    }
    
    /// Obtener número de líneas restantes
    pub fn remaining_lines(&self) -> usize {
        self.lines.len() - self.current_line_index - 1
    }
    
    /// Verificar si hay líneas restantes
    pub fn has_more_lines(&self) -> bool {
        self.current_line_index < self.lines.len() - 1
    }
    
    /// Verificar si el diálogo está completo
    pub fn is_complete(&self) -> bool {
        self.current_line_index >= self.lines.len() - 1
    }
    
    /// Añadir variable
    pub fn add_variable(&mut self, variable: DialogueVariable) {
        self.variables.add_global(variable);
        self.last_updated = std::time::Instant::now();
    }
    
    /// Obtener variable por nombre
    pub fn get_variable(&self, name: &str) -> Option<&DialogueVariable> {
        self.variables.get(name)
    }
    
    /// Obtener valor de variable
    pub fn get_variable_value(&self, name: &str) -> Option<&DialogueValue> {
        self.variables.get_value(name)
    }
    
    /// Verificar si variable existe
    pub fn has_variable(&self, name: &str) -> bool {
        self.variables.has(name)
    }
    
    /// Obtener opciones
    pub fn choices(&self) -> &[String] {
        &self.choices
    }
    
    /// Tiempo de vida en segundos
    pub fn age(&self) -> f32 {
        self.created_at.elapsed().as_secs_f32()
    }
}

/// Gestor de contexto de diálogos
#[derive(Debug, Default)]
pub struct DialogueContextManager {
    pub active_contexts: Vec<DialogueContext>,
    pub context_stack: Vec<DialogueContext>,
    pub next_context_id: u64,
    pub next_dialogue_id: u64,
}

impl DialogueContextManager {
    pub fn new() -> Self {
        Self {
            active_contexts: Vec::new(),
            context_stack: Vec::new(),
            next_context_id: 1,
            next_dialogue_id: 1,
        }
    }
    
    /// Pushar contexto
    pub fn push(&mut self, context: DialogueContext) {
        self.context_stack.push(context);
        self.active_contexts.push(self.context_stack.last().unwrap().clone());
    }
    
    /// Pop contexto
    pub fn pop(&mut self) -> Option<DialogueContext> {
        if let Some(context) = self.context_stack.pop() {
            let index = self.active_contexts.len() - 1;
            if index >= 0 {
                self.active_contexts.pop();
                return Some(context);
            }
        }
        None
    }
    
    /// Obtener contexto activo
    pub fn active_context(&self) -> Option<&DialogueContext> {
        self.context_stack.last()
    }
    
    /// Obtener contexto activo mutante
    pub fn active_context_mut(&mut self) -> Option<&mut DialogueContext> {
        self.context_stack.last_mut()
    }
    
    /// Obtener contexto por ID
    pub fn get_context(&self, dialogue_id: u64) -> Option<&DialogueContext> {
        self.context_stack.iter().find(|c| c.dialogue_id == dialogue_id)
    }
    
    /// Obtener contexto mutante por ID
    pub fn get_context_mut(&mut self, dialogue_id: u64) -> Option<&mut DialogueContext> {
        self.context_stack.iter_mut().find(|c| c.dialogue_id == dialogue_id)
    }
    
    /// Limpiar todos los contextos
    pub fn clear(&mut self) {
        self.context_stack.clear();
        self.active_contexts.clear();
    }
    
    /// Obtener número total de contextos
    pub fn context_count(&self) -> usize {
        self.context_stack.len()
    }
}
