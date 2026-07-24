//! Bitacora Manager - Gestión de notas y observaciones

use eframe::egui;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum LinkType {
    Event(u64),
    Dialog(u64),
    Actor(u64),
    Variable(u64),
    Scene(u64),
    Note(u64),
    Unknown(u64),
}

#[derive(Debug, Clone)]
pub struct BitacoraEntry {
    pub id: String,
    pub text: String,
    pub links: Vec<LinkType>,
    pub related_to: Option<String>,
}

impl BitacoraEntry {
    pub fn new(id: String, text: String) -> Self {
        Self {
            id,
            text,
            links: Vec::new(),
            related_to: None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct BitacoraManager {
    pub entries: HashMap<String, BitacoraEntry>,
    pub selected_entry_id: Option<String>,
    pub is_edit_mode: bool,
    pub current_filter: String,
}

impl Default for BitacoraManager {
    fn default() -> Self {
        Self {
            entries: HashMap::new(),
            selected_entry_id: None,
            is_edit_mode: false,
            current_filter: String::new(),
        }
    }
}

impl BitacoraManager {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn count(&self) -> usize {
        self.entries.len()
    }

    pub fn add_entry(&mut self, text: &str, related_to: Option<String>) -> String {
        let id = format!("note_{}", self.entries.len());
        let entry = BitacoraEntry::new(id.clone(), text.to_string());
        self.entries.insert(id.clone(), entry);
        self.selected_entry_id = Some(id.clone());
        self.is_edit_mode = false;
        id
    }

    pub fn get_entry(&self, id: &str) -> Option<&BitacoraEntry> {
        self.entries.get(id)
    }

    pub fn get_entry_mut(&mut self, id: &str) -> Option<&mut BitacoraEntry> {
        self.entries.get_mut(id)
    }

    pub fn remove_entry(&mut self, id: &str) {
        self.entries.remove(id);
        if self.selected_entry_id.as_ref().map(|s| s.as_str()) == Some(id) {
            self.selected_entry_id = None;
        }
    }

    pub fn deactivate_entry(&mut self, id: &str) {
        if let Some(entry) = self.entries.get_mut(id) {
            entry.related_to = None;
        }
        if self.selected_entry_id.as_ref().map(|s| s.as_str()) == Some(id) {
            self.selected_entry_id = None;
        }
    }

    pub fn log(&mut self, message: String) {
        println!("[Bitacora] {}", message);
    }

    pub fn show_bitacora_panel(&mut self, ctx: &egui::Context, ui: &mut egui::Ui) {
        egui::TopBottomPanel::top("bitacora_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label(egui::RichText::new("Bitacora Interactiva").color(egui::Color32::WHITE));
                ui.separator();
                ui.label(format!("Total: {}", self.count()));
            });

            ui.add_space(5.0);

            egui::ScrollArea::vertical().show(ui, |ui| {
                for (id, entry) in &self.entries {
                    ui.label(format!("{}: {}", id, entry.text));
                }
            });
        });
    }
}
