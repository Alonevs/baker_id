use crate::dialogue_editor::{Dialogue, DialogueLine, DialogueManager, DialogueAction};
use eframe::egui;
use std::collections::HashMap;

/// Renderizador del Editor de Diálogos
pub struct DialogueEditorRenderer {
    selected_dialogue: Option<String>,
    selected_line: Option<String>,
}

impl DialogueEditorRenderer {
    pub fn new() -> Self {
        Self {
            selected_dialogue: None,
            selected_line: None,
        }
    }
    
    pub fn render_dialogue_editor(&mut self, ui: &mut egui::Ui, dialogues: &HashMap<String, Dialogue>) -> egui::Response {
        let response = ui.allocate_response(ui.available_size(), egui::Sense::click());
        
        // Lista de diálogos
        if dialogues.is_empty() {
            ui.label(egui::RichText::new("No dialogues loaded").color(egui::Color32::GRAY));
            ui.label(egui::RichText::new("Load dialogues from Excel/CSV to start editing").color(egui::Color32::GRAY));
        } else {
            ui.heading(egui::RichText::new("💬 Dialogues").font(egui::FontId::proportional(14.0)));
            
            for (id, dialogue) in dialogues {
                let is_selected = self.selected_dialogue.as_deref() == Some(id);
                let label = ui.selectable_label(is_selected, format!("📝 {}", dialogue.name));
                if label.clicked() {
                    self.selected_dialogue = Some(id.clone());
                }
            }
        }
        
        // Editor de diálogo seleccionado
        if let Some(dialogue_id) = &self.selected_dialogue {
            if let Some(dialogue) = dialogues.get(dialogue_id) {
                self.render_dialogue_detail(ui, dialogue);
            }
        }
        
        response
    }
    
    fn render_dialogue_detail(&mut self, ui: &mut egui::Ui, dialogue: &Dialogue) {
        ui.heading(egui::RichText::new(&dialogue.name).font(egui::FontId::proportional(14.0)));
        
        // Información básica
        ui.horizontal(|ui| {
            ui.label(format!("Actor: {:?}", dialogue.actor_id));
            ui.label(format!("Language: {}", dialogue.language));
            ui.label(format!("Lines: {}", dialogue.lines.len()));
        });
        
        ui.add_space(10.0);
        
        // Lista de líneas
        ui.heading(egui::RichText::new("📝 Lines").font(egui::FontId::proportional(13.0)));
        
        if dialogue.lines.is_empty() {
            ui.label(egui::RichText::new("No lines yet").color(egui::Color32::GRAY));
        } else {
            for (index, line) in dialogue.lines.iter().enumerate() {
                let is_selected = self.selected_line.as_deref() == Some(line.id.clone());
                let label = ui.selectable_label(is_selected, format!("{}: {}", index + 1, line.text.truncate(50)));
                if label.clicked() {
                    self.selected_line = Some(line.id.clone());
                }
            }
        }
        
        ui.add_space(10.0);
        
        // Editor de línea seleccionada
        if let Some(line_id) = &self.selected_line {
            if let Some(line) = dialogue.lines.iter().find(|l| l.id == *line_id) {
                self.render_line_editor(ui, line);
            }
        }
        
        ui.add_space(10.0);
        
        // Variables del diálogo
        ui.heading(egui::RichText::new("🔧 Variables").font(egui::FontId::proportional(13.0)));
        if dialogue.variables.is_empty() {
            ui.label(egui::RichText::new("No variables defined").color(egui::Color32::GRAY));
        } else {
            for (var_id, var) in &dialogue.variables {
                ui.horizontal(|ui| {
                    ui.label(format!("{}: {:?}", var_id, var.variable_type));
                    ui.label(format!("Value: {:?}", var.value));
                });
            }
        }
        
        // Condiciones del diálogo
        ui.heading(egui::RichText::new("⚙️ Conditions").font(egui::FontId::proportional(13.0)));
        if dialogue.conditions.is_empty() {
            ui.label(egui::RichText::new("No conditions defined").color(egui::Color32::GRAY));
        } else {
            for (index, condition) in dialogue.conditions.iter().enumerate() {
                ui.horizontal(|ui| {
                    ui.label(format!("Condition {}: ", index + 1));
                    ui.label(format!("{:?}", condition.operator));
                });
            }
        }
    }
    
    fn render_line_editor(&mut self, ui: &mut egui::Ui, line: &DialogueLine) {
        ui.heading(egui::RichText::new("✏️ Edit Line").font(egui::FontId::proportional(13.0)));
        
        ui.horizontal(|ui| {
            ui.label(format!("ID: {}", line.id));
            ui.label(format!("Duration: {}ms", line.duration_ms));
        });
        
        ui.add_space(5.0);
        
        // Text
        ui.label("Text:");
        ui.text_edit_multiline(&mut line.text);
        
        ui.add_space(5.0);
        
        // Speaker
        ui.label("Speaker:");
        ui.text_edit_singleline(&mut line.speaker.unwrap_or_default());
        
        ui.add_space(5.0);
        
        // Action
        ui.label("Action:");
        let action_str = match line.action {
            DialogueAction::Speak => "Speak",
            DialogueAction::Wait => "Wait",
            DialogueAction::PlayAnimation => "PlayAnimation",
            DialogueAction::PlaySound => "PlaySound",
            DialogueAction::ShowImage => "ShowImage",
            DialogueAction::Hide => "Hide",
            DialogueAction::Branch => "Branch",
            DialogueAction::End => "End",
        };
        ui.label(action_str);
        
        ui.add_space(5.0);
        
        // Variables usadas
        ui.label("Variables:");
        ui.text_edit_singleline(&mut line.variables.join(", "));
    }
}

impl Default for DialogueEditorRenderer {
    fn default() -> Self {
        Self::new()
    }
}

