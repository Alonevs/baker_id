

/// Script Editor UI para edición de scripts
#[derive(Debug, Clone)]
pub struct ScriptEditor {
    pub current_script: Option<String>,
    pub show_editor: bool,
    pub show_preview: bool,
    pub current_line: usize,
    pub current_column: usize,
    pub editor_text: String,
}

impl Default for ScriptEditor {
    fn default() -> Self {
        Self {
            current_script: None,
            show_editor: true,
            show_preview: false,
            current_line: 0,
            current_column: 0,
            editor_text: String::new(),
        }
    }
}

impl ScriptEditor {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn load_script(&mut self, script: String) {
        self.current_script = Some(script.clone());
        self.editor_text = script;
    }

    pub fn save_script(&mut self) {
        // Placeholder
    }
}

