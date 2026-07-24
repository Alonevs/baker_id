//! Event Dialog - Gestión de diálogos de eventos

use eframe::egui;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct EventDialog {
    pub id: String,
    pub text: String,
    pub choices: Vec<String>,
    pub target: Option<String>,
}

impl Default for EventDialog {
    fn default() -> Self {
        Self {
            id: String::new(),
            text: String::new(),
            choices: Vec::new(),
            target: None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct EventDialogManager {
    pub dialogs: HashMap<String, EventDialog>,
    pub current_dialog: Option<String>,
    pub is_active: bool,
}

impl Default for EventDialogManager {
    fn default() -> Self {
        Self {
            dialogs: HashMap::new(),
            current_dialog: None,
            is_active: false,
        }
    }
}

impl EventDialogManager {
    pub fn new() -> Self {
        Self::default()
    }
    
    pub fn add_dialog(&mut self, id: String, text: String) {
        let dialog = EventDialog {
            id: id.clone(),
            text,
            choices: Vec::new(),
            target: None,
        };
        self.dialogs.insert(id, dialog);
    }
    
    pub fn get_dialog(&self, id: &str) -> Option<&EventDialog> {
        self.dialogs.get(id)
    }
    
    pub fn set_current_dialog(&mut self, id: String) {
        self.current_dialog = Some(id);
        self.is_active = true;
    }
    
    pub fn close_dialog(&mut self) {
        self.current_dialog = None;
        self.is_active = false;
    }
    
    pub fn show_dialog_panel(&mut self, ctx: &egui::Context, _ui: &mut egui::Ui) {
        egui::TopBottomPanel::top("event_dialog_panel").show(ctx, |ui| {
            if let Some(dialog_id) = &self.current_dialog {
                if let Some(dialog) = self.dialogs.get(dialog_id) {
                    ui.label(format!("Dialog: {}", dialog.id));
                    ui.label(&dialog.text);
                }
            }
        });
    }
}
