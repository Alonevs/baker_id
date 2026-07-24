//! Dialogue Editor UI - UI para editor de diálogos

use eframe::egui;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct DialogueEditorRenderer;

impl DialogueEditorRenderer {
    pub fn new() -> Self {
        Self
    }
    
    pub fn render_dialogue_editor(&self, ui: &mut egui::Ui, _dialogues: &HashMap<String, String>) {
        ui.label("Dialogue Editor UI");
    }
}
