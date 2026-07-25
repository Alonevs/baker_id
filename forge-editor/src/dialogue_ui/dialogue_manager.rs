use std::collections::HashMap;

/// Identificador único para diálogos
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DialogueId(pub u64);

impl DialogueId {
    pub fn new(id: u64) -> Self {
        Self(id)
    }
}

/// Línea de diálogo
#[derive(Debug, Clone)]
pub struct DialogueLine {
    pub text: String,
    pub speaker: Option<String>,
    pub is_emote: bool,
    pub is_system: bool,
    pub is_choice: bool,
}

impl DialogueLine {
    pub fn new(text: &str) -> Self {
        Self {
            text: text.to_string(),
            speaker: None,
            is_emote: false,
            is_system: false,
            is_choice: false,
        }
    }
    
    pub fn with_speaker(text: &str, speaker: &str) -> Self {
        Self {
            text: text.to_string(),
            speaker: Some(speaker.to_string()),
            is_emote: false,
            is_system: false,
            is_choice: false,
        }
    }
    
    pub fn emote(text: &str) -> Self {
        Self {
            text: text.to_string(),
            speaker: None,
            is_emote: true,
            is_system: false,
            is_choice: false,
        }
    }
    
    pub fn system(text: &str) -> Self {
        Self {
            text: text.to_string(),
            speaker: None,
            is_emote: false,
            is_system: true,
            is_choice: false,
        }
    }
}

/// Opción de diálogo
#[derive(Debug, Clone)]
pub struct DialogueChoice {
    pub text: String,
    pub dialogue_id: Option<DialogueId>,
    pub variables: HashMap<String, DialogueVariable>,
    pub metadata: HashMap<String, String>,
}

impl DialogueChoice {
    pub fn new(text: &str) -> Self {
        Self {
            text: text.to_string(),
            dialogue_id: None,
            variables: HashMap::new(),
            metadata: HashMap::new(),
        }
    }
    
    pub fn set_dialogue_id(&mut self, dialogue_id: DialogueId) {
        self.dialogue_id = Some(dialogue_id);
    }
    
    pub fn add_variable(&mut self, key: &str, variable: DialogueVariable) {
        self.variables.insert(key.to_string(), variable);
    }
    
    pub fn get_variable(&self, key: &str) -> Option<&DialogueVariable> {
        self.variables.get(key)
    }
}

/// Variable de diálogo
#[derive(Debug, Clone)]
pub struct DialogueVariable {
    pub name: String,
    pub value: DialogueValue,
    pub is_local: bool,
}

impl DialogueVariable {
    pub fn new(name: &str, value: DialogueValue) -> Self {
        Self {
            name: name.to_string(),
            value,
            is_local: false,
        }
    }
    
    pub fn local(name: &str, value: DialogueValue) -> Self {
        Self {
            name: name.to_string(),
            value,
            is_local: true,
        }
    }
}

/// Valor de variable de diálogo
#[derive(Debug, Clone)]
pub enum DialogueValue {
    String(String),
    Number(f64),
    Boolean(bool),
    None,
}

/// Contexto de diálogo
#[derive(Debug, Clone)]
pub struct DialogueContext {
    pub dialogue_id: DialogueId,
    pub speaker: Option<String>,
    pub lines: Vec<DialogueLine>,
    pub choices: Vec<DialogueChoice>,
    pub variables: HashMap<String, DialogueVariable>,
    pub is_active: bool,
    pub current_line_index: usize,
    pub created_at: std::time::Instant,
    pub last_updated: std::time::Instant,
}

impl DialogueContext {
    pub fn new(dialogue_id: DialogueId, lines: Vec<DialogueLine>, choices: Vec<DialogueChoice>) -> Self {
        Self {
            dialogue_id,
            speaker: None,
            lines,
            choices,
            variables: HashMap::new(),
            is_active: false,
            current_line_index: 0,
            created_at: std::time::Instant::now(),
            last_updated: std::time::Instant::now(),
        }
    }
    
    /// Línea actual
    pub fn current_line(&self) -> Option<&DialogueLine> {
        self.lines.get(self.current_line_index)
    }
    
    /// Línea actual mutante
    pub fn current_line_mut(&mut self) -> Option<&mut DialogueLine> {
        self.lines.get_mut(self.current_line_index)
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
        self.variables.insert(variable.name.clone(), variable);
        self.last_updated = std::time::Instant::now();
    }
    
    /// Obtener variable
    pub fn get_variable(&self, name: &str) -> Option<&DialogueVariable> {
        self.variables.get(name)
    }
    
    /// Obtener valor de variable
    pub fn get_variable_value(&self, name: &str) -> Option<&DialogueValue> {
        self.variables.get(name).map(|v| &v.value)
    }
    
    /// Verificar si variable existe
    pub fn has_variable(&self, name: &str) -> bool {
        self.variables.contains_key(name)
    }
    
    /// Obtener opciones disponibles
    pub fn choices(&self) -> &[DialogueChoice] {
        &self.choices
    }
    
    /// Tiempo de vida en segundos
    pub fn age(&self) -> f32 {
        self.created_at.elapsed().as_secs_f32()
    }
}

/// Gestión de contexto de diálogos
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
    pub fn get_context(&self, dialogue_id: DialogueId) -> Option<&DialogueContext> {
        self.context_stack.iter().find(|c| c.dialogue_id == dialogue_id)
    }
    
    /// Obtener contexto mutante por ID
    pub fn get_context_mut(&mut self, dialogue_id: DialogueId) -> Option<&mut DialogueContext> {
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

/// Gestor principal de diálogos
#[derive(Debug, Default)]
pub struct DialogueManager {
    pub dialogues: HashMap<DialogueId, DialogueContext>,
    pub context_manager: DialogueContextManager,
    pub next_context_id: u64,
    pub next_dialogue_id: u64,
    pub is_active: bool,
    pub last_dialogue_id: Option<DialogueId>,
}

impl DialogueManager {
    /// Crear nuevo gestor de diálogos
    pub fn new() -> Self {
        Self {
            dialogues: HashMap::new(),
            context_manager: DialogueContextManager::new(),
            next_context_id: 1,
            next_dialogue_id: 1,
            is_active: false,
            last_dialogue_id: None,
        }
    }
    
    /// Crear nuevo diálogo
    pub fn create_dialogue(&mut self, id: u64, lines: Vec<DialogueLine>, choices: Vec<DialogueChoice>) -> DialogueId {
        let dialogue_id = DialogueId::new(id);
        let context = DialogueContext::new(dialogue_id, lines, choices);
        self.dialogues.insert(dialogue_id, context);
        
        self.next_dialogue_id = id + 1;
        dialogue_id
    }
    
    /// Obtener diálogo por ID
    pub fn get_dialogue(&self, dialogue_id: DialogueId) -> Option<&DialogueContext> {
        self.dialogues.get(&dialogue_id)
    }
    
    /// Obtener diálogo mutante por ID
    pub fn get_dialogue_mut(&mut self, dialogue_id: DialogueId) -> Option<&mut DialogueContext> {
        self.dialogues.get_mut(&dialogue_id)
    }
    
    /// Mostrar diálogo
    pub fn show_dialogue(&mut self, dialogue_id: DialogueId) -> bool {
        if let Some(context) = self.dialogues.get(&dialogue_id) {
            self.context_manager.push(context.clone());
            self.context_manager.active_context_mut().unwrap().is_active = true;
            self.is_active = true;
            self.last_dialogue_id = Some(dialogue_id);
            return true;
        }
        false
    }
    
    /// Ocultar diálogo actual
    pub fn hide_dialogue(&mut self) {
        if let Some(context) = self.context_manager.pop() {
            context.is_active = false;
            self.is_active = !self.context_manager.context_count() > 0;
        }
    }
    
    /// Siguiente línea de diálogo
    pub fn next_line(&mut self) {
        if let Some(context) = self.context_manager.active_context_mut() {
            context.next_line();
        }
    }
    
    /// Línea anterior
    pub fn prev_line(&mut self) {
        if let Some(context) = self.context_manager.active_context_mut() {
            context.prev_line();
        }
    }
    
    /// Seleccionar opción
    pub fn select_choice(&mut self, choice_index: usize) {
        if let Some(context) = self.context_manager.active_context_mut() {
            if choice_index < context.choices.len() {
                let choice = &context.choices[choice_index];
                
                // Aplicar variables de la opción
                for (key, variable) in &choice.variables {
                    context.add_variable(variable.clone());
                }
                
                // Cambiar a siguiente diálogo si existe
                if let Some(dialogue_id) = choice.dialogue_id {
                    self.hide_dialogue();
                    self.show_dialogue(dialogue_id);
                }
            }
        }
    }
    
    /// Cerrar diálogo
    pub fn close_dialogue(&mut self) {
        self.hide_dialogue();
    }
    
    /// Añadir línea al diálogo actual
    pub fn add_line(&mut self, line: DialogueLine) {
        if let Some(context) = self.context_manager.active_context_mut() {
            context.lines.push(line);
            context.last_updated = std::time::Instant::now();
        }
    }
    
    /// Añadir opción al diálogo actual
    pub fn add_choice(&mut self, choice: DialogueChoice) {
        if let Some(context) = self.context_manager.active_context_mut() {
            context.choices.push(choice);
            context.last_updated = std::time::Instant::now();
        }
    }
    
    /// Obtener línea actual
    pub fn current_line(&self) -> Option<&DialogueLine> {
        self.context_manager.active_context().and_then(|c| c.current_line())
    }
    
    /// Obtener líneas restantes
    pub fn remaining_lines(&self) -> usize {
        self.context_manager.active_context().map(|c| c.remaining_lines()).unwrap_or(0)
    }
    
    /// Obtener opciones
    pub fn choices(&self) -> Option<&Vec<DialogueChoice>> {
        self.context_manager.active_context().map(|c| &c.choices)
    }
    
    /// Obtener variable por nombre
    pub fn get_variable(&self, name: &str) -> Option<&DialogueVariable> {
        self.context_manager.active_context().and_then(|c| c.get_variable(name))
    }
    
    /// Obtener valor de variable
    pub fn get_variable_value(&self, name: &str) -> Option<&DialogueValue> {
        self.context_manager.active_context().and_then(|c| c.get_variable_value(name))
    }
    
    /// Verificar si existe variable
    pub fn has_variable(&self, name: &str) -> bool {
        self.context_manager.active_context().map(|c| c.has_variable(name)).unwrap_or(false)
    }
    
    /// Verificar si diálogo está activo
    pub fn is_active(&self) -> bool {
        self.is_active && self.context_manager.context_count() > 0
    }
    
    /// Verificar si hay líneas restantes
    pub fn has_more_lines(&self) -> bool {
        self.context_manager.active_context().map(|c| c.has_more_lines()).unwrap_or(false)
    }
    
    /// Verificar si diálogo está completo
    pub fn is_complete(&self) -> bool {
        self.context_manager.active_context().map(|c| c.is_complete()).unwrap_or(false)
    }
    
    /// Obtener número total de diálogos
    pub fn dialogue_count(&self) -> usize {
        self.dialogues.len()
    }
    
    /// Obtener número total de contextos
    pub fn context_count(&self) -> usize {
        self.context_manager.context_count()
    }
    
    /// Limpiar todos los diálogos
    pub fn clear_all(&mut self) {
        self.dialogues.clear();
        self.context_manager.clear();
        self.is_active = false;
        self.last_dialogue_id = None;
    }
}
