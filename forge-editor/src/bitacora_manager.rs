use eframe::egui;
use std::collections::HashMap;
use std::sync::Arc;
use chrono::{DateTime, Utc};
use regex::Regex;
use crate::bitacora_singleton::get_validator_singleton;

/// Tipo de enlace en una nota
#[derive(Debug, Clone)]
pub enum LinkType {
    Event(String),       // {evt:ID}
    Dialog(String),      // {dlg:ID}
    Actor(String),       // {actor:ID}
    Variable(String),    // {var:ID}
    Scene(String),       // {scene:ID}
    Note(String),        // {note:text}
    Unknown(String),     // Otro patrón no reconocido
}

impl LinkType {
    pub fn parse(pattern: &str) -> Self {
        let pattern_lower = pattern.to_lowercase();
        
        if pattern_lower.starts_with("{evt:") {
            let id = &pattern[5..];
            LinkType::Event(id.to_string())
        } else if pattern_lower.starts_with("{dlg:") {
            let id = &pattern[5..];
            LinkType::Dialog(id.to_string())
        } else if pattern_lower.starts_with("{actor:") {
            let id = &pattern[7..];
            LinkType::Actor(id.to_string())
        } else if pattern_lower.starts_with("{var:") {
            let id = &pattern[5..];
            LinkType::Variable(id.to_string())
        } else if pattern_lower.starts_with("{scene:") {
            let id = &pattern[7..];
            LinkType::Scene(id.to_string())
        } else if pattern_lower.starts_with("{note:") {
            let text = &pattern[6..];
            LinkType::Note(text.to_string())
        } else {
            LinkType::Unknown(pattern.to_string())
        }
    }

    pub fn get_id(&self) -> &str {
        match self {
            LinkType::Event(id) | LinkType::Dialog(id) | LinkType::Actor(id) |
            LinkType::Variable(id) | LinkType::Scene(id) | LinkType::Note(id) |
            LinkType::Unknown(id) => id,
        }
    }
}

/// Estructura de una nota en la bitácora
#[derive(Debug, Clone)]
pub struct BitacoraEntry {
    pub id: String,
    pub text: String,
    pub links: Vec<LinkType>,
    pub link_positions: Vec<(usize, usize)>, // (start, end) en el texto
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub related_to: Option<String>, // ID del evento/diálogo relacionado
    pub tags: Vec<String>,
    pub is_active: bool,
}

impl BitacoraEntry {
    pub fn new(id: &str, text: &str) -> Self {
        let (links, link_positions) = Self::parse_links(text);
        Self {
            id: id.to_string(),
            text: text.to_string(),
            links,
            link_positions,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            related_to: None,
            tags: Vec::new(),
            is_active: true,
        }
    }

    /// Parsear enlaces del texto y extraer sus posiciones
    fn parse_links(text: &str) -> (Vec<LinkType>, Vec<(usize, usize)>) {
        let mut links = Vec::new();
        let mut positions = Vec::new();
        
        // Patrón para capturar {tipo:id}
        let pattern = Regex::new(r"\{(evt|dlg|actor|var|scene|note):([^}]+)\}").unwrap();
        
        for cap in pattern.captures_iter(text) {
            let start = cap.get(0).unwrap().start();
            let end = cap.get(0).unwrap().end();
            
            let link_type = LinkType::parse(&text[start..end]);
            links.push(link_type);
            positions.push((start, end));
        }
        
        (links, positions)
    }

    /// Actualizar texto y reparsear enlaces
    pub fn update_text(&mut self, new_text: &str) {
        self.text = new_text.to_string();
        let (links, link_positions) = Self::parse_links(new_text);
        self.links = links;
        self.link_positions = link_positions;
        self.updated_at = Utc::now();
    }

    pub fn update_text(&mut self, new_text: &str) {
        self.text = new_text.to_string();
        self.updated_at = Utc::now();
    }

    pub fn add_tag(&mut self, tag: &str) {
        if !self.tags.contains(tag) {
            self.tags.push(tag.to_string());
            self.updated_at = Utc::now();
        }
    }

    pub fn remove_tag(&mut self, tag: &str) {
        self.tags.retain(|t| t != tag);
        self.updated_at = Utc::now();
    }

    pub fn set_related_to(&mut self, related_to: Option<String>) {
        self.related_to = related_to;
        self.updated_at = Utc::now();
    }

    pub fn deactivate(&mut self) {
        self.is_active = false;
        self.updated_at = Utc::now();
    }
}

/// Gestor de la bitácora interactiva
pub struct BitacoraManager {
    pub entries: HashMap<String, BitacoraEntry>,
    pub current_filter: String,
    pub selected_entry_id: Option<String>,
    pub is_edit_mode: bool,
}

impl Default for BitacoraManager {
    fn default() -> Self {
        Self {
            entries: HashMap::new(),
            current_filter: "".to_string(),
            selected_entry_id: None,
            is_edit_mode: false,
        }
    }
}

impl BitacoraManager {
    /// Añadir una nueva nota a la bitácora
    pub fn add_entry(&mut self, text: &str, related_to: Option<String>) -> String {
        let id = Utc::now().format("%Y%m%d_%H%M%S_%f").to_string();
        let entry = BitacoraEntry::new(&id, text);
        entry.set_related_to(related_to);
        self.entries.insert(id.clone(), entry);
        id
    }

    /// Obtener una nota por ID
    pub fn get_entry(&self, id: &str) -> Option<&BitacoraEntry> {
        self.entries.get(id)
    }

    /// Obtener una nota mutada por ID
    pub fn get_entry_mut(&mut self, id: &str) -> Option<&mut BitacoraEntry> {
        self.entries.get_mut(id)
    }

    /// Actualizar texto de una nota
    pub fn update_entry(&mut self, id: &str, new_text: &str) {
        if let Some(entry) = self.entries.get_mut(id) {
            entry.update_text(new_text);
        }
    }

    /// Eliminar una nota
    pub fn remove_entry(&mut self, id: &str) {
        self.entries.remove(id);
    }

    /// Desactivar una nota (sin eliminar)
    pub fn deactivate_entry(&mut self, id: &str) {
        if let Some(entry) = self.entries.get_mut(id) {
            entry.deactivate();
        }
    }

    /// Obtener todas las entradas activas
    pub fn get_active_entries(&self) -> Vec<&BitacoraEntry> {
        self.entries.values()
            .filter(|e| e.is_active)
            .collect()
    }

    /// Obtener entradas filtradas
    pub fn get_filtered_entries(&self, filter: &str) -> Vec<&BitacoraEntry> {
        let filter_lower = filter.to_lowercase();
        self.get_active_entries()
            .into_iter()
            .filter(|e| {
                e.text.to_lowercase().contains(&filter_lower) ||
                e.tags.iter().any(|t| t.to_lowercase().contains(&filter_lower))
            })
            .collect()
    }

    /// Obtener entradas relacionadas con un evento
    pub fn get_related_entries(&self, event_id: &str) -> Vec<&BitacoraEntry> {
        self.get_active_entries()
            .into_iter()
            .filter(|e| e.related_to.as_ref() == Some(&event_id.to_string()))
            .collect()
    }

    /// Añadir tag a una entrada
    pub fn add_tag_to_entry(&mut self, id: &str, tag: &str) {
        if let Some(entry) = self.entries.get_mut(id) {
            entry.add_tag(tag);
        }
    }

    /// Remover tag de una entrada
    pub fn remove_tag_from_entry(&mut self, id: &str, tag: &str) {
        if let Some(entry) = self.entries.get_mut(id) {
            entry.remove_tag(tag);
        }
    }

    /// Limpiar bitácora
    pub fn clear(&mut self) {
        self.entries.clear();
    }

    /// Obtener cantidad de entradas
    pub fn count(&self) -> usize {
        self.entries.len()
    }
}

    /// Panel de visualización de la bitácora interactiva
    pub fn show_bitacora_panel(ctx: &egui::Context, ui: &mut egui::Ui, manager: &mut BitacoraManager) {
        let panel = egui::Panel::new("📚 Bitácora Interactiva")
            .frame(egui::Frame::none().fill(egui::Color32::from_rgb(40, 40, 60)))
            .show(ctx, ui);

        ui.horizontal(|ui| {
            ui.label(egui::RichText::new("📝 Notas y Observaciones").color(egui::Color32::WHITE));
            
            if ui.button("➕ Nueva Nota").clicked() {
                manager.selected_entry_id = None;
                manager.is_edit_mode = false;
            }

            ui.separator();
            
            ui.label(format!("📊 Total: {}", manager.count()));
        });

        ui.add_space(5.0);

        // Lista de entradas
        egui::ScrollArea::vertical().show(ui, |ui| {
            ui.heading("📋 Entradas");

    // Panel de entrada de nueva nota
    egui::TopBottomPanel::top("bitacora_input").show(ctx, |ui| {
        ui.horizontal(|ui| {
            ui.label(egui::RichText::new("📝 Nueva nota:").color(egui::Color32::YELLOW));
            
            let mut note_text = String::new();
            ui.text_edit_singleline(&mut note_text);
            
            ui.add_space(5.0);
            
            if ui.button("💾 Guardar").clicked() {
                let id = manager.add_entry(&note_text, None);
                manager.selected_entry_id = Some(id.clone());
                manager.is_edit_mode = true;
            }
        });
    });

    // Lista de entradas
    egui::ScrollArea::vertical().show(ui, |ui| {
        ui.heading("📋 Entradas");
        
        let entries: Vec<_> = manager.get_active_entries().into_iter().collect();
        
        if entries.is_empty() {
            ui.label(egui::RichText::new("No hay notas en la bitácora").color(egui::Color32::GRAY));
        } else {
            for entry in entries {
                let mut is_selected = manager.selected_entry_id.as_ref() == Some(&entry.id);
                
                let response = ui.selectable_label(is_selected, format!("🔹 {}", entry.id));
                if response.clicked() {
                    manager.selected_entry_id = Some(entry.id.clone());
                    manager.is_edit_mode = true;
                    manager.open_entry(&entry.id);
                }
                
                ui.label(format!("  📅 {}", entry.created_at.format("%Y-%m-%d %H:%M:%S UTC")));
                if !entry.text.is_empty() {
                    ui.label(format!("  📄 {}", entry.text[..100].trim_end_matches(&['.', '!', '?'][..]).to_string()));
                }
                if !entry.tags.is_empty() {
                    ui.label(format!("  🏷️ {}", entry.tags.join(", ")));
                }
                ui.add_space(2.0);
            }
        }
    });

    // Editor de nota seleccionada
    if let Some(entry_id) = &manager.selected_entry_id {
        if let Some(entry) = manager.get_entry_mut(entry_id) {
            egui::TopBottomPanel::top("bitacora_editor").show(ctx, |ui| {
                ui.heading(format!("✏️ Editar nota: {}", entry.id));
                
                ui.horizontal(|ui| {
                    if ui.button("📝 Editar texto").clicked() {
                        manager.is_edit_mode = true;
                    }
                    
                    ui.add_space(5.0);
                    
                    if ui.button("🔗 Vincular a evento").clicked() {
                        // Abrir diálogo para seleccionar evento
                        let mut linked_event = String::new();
                        ui.text_edit_singleline(&mut linked_event);
                        
                        if !linked_event.is_empty() {
                            entry.set_related_to(Some(linked_event));
                            manager.add_tag_to_entry(entry_id, &linked_event);
                        }
                    }
                    
                    ui.add_space(5.0);
                    
                    if ui.button("❌ Desactivar").clicked() {
                        manager.deactivate_entry(entry_id);
                        manager.selected_entry_id = None;
                    }
                    
                    ui.add_space(5.0);
                    
                    if ui.button("🗑️ Eliminar").clicked() {
                        manager.remove_entry(entry_id);
                        manager.selected_entry_id = None;
                    }
                });
                
                ui.add_space(10.0);
                
                // Renderizar texto con enlaces interactivos
                ui.label(egui::RichText::new("Texto de la nota:").color(egui::Color32::WHITE));
                ui.add_space(5.0);
                
                let mut text_to_edit = entry.text.clone();
                
                // Renderizar texto con enlaces clickeables
                let mut rendered_text = String::new();
                let mut current_pos = 0;
                let mut has_links = !entry.links.is_empty();
                
                if has_links {
                    for (i, link) in entry.links.iter().enumerate() {
                        // Añadir texto antes del enlace
                        if current_pos < entry.text.len() {
                            let chunk = &entry.text[current_pos..entry.text.link_positions[i].0];
                            rendered_text.push_str(&chunk);
                        }
                        
                        // Añadir botón para el enlace
                        let button_text = match link {
                            LinkType::Event(id) => format!("📄 {evt:{}", id),
                            LinkType::Dialog(id) => format!("💬 {dlg:{}", id),
                            LinkType::Actor(id) => format!("🎭 {actor:{}", id),
                            LinkType::Variable(id) => format!("📊 {var:{}", id),
                            LinkType::Scene(id) => format!("🎬 {scene:{}", id),
                            LinkType::Note(id) => format!("📝 {note:{}", id),
                            LinkType::Unknown(id) => format!("🔗 {}", id),
                        };
                        
                        let response = ui.add_sized(
                            egui::Vec2::new(500.0, ui.font_metrics().line_height()),
                            egui::Button::new(button_text).sense(egui::Sense::click())
                        );
                        
                        if response.clicked() {
                            // Aquí podrías redirigir al panel correspondiente
                            // Por ahora solo logueamos
                            manager.log(format!("🔗 Enlace clickeado: {}", button_text));
                        }
                        
                        current_pos = entry.text.link_positions[i].1;
                    }
                    
                    // Añadir texto restante
                    if current_pos < entry.text.len() {
                        rendered_text.push_str(&entry.text[current_pos..]);
                    }
                } else {
                    // Sin enlaces, renderizar texto plano
                    if ui.text_edit_multiline(&mut text_to_edit).height_in_lines(10.0).desired_width(500.0).ui().clicked() {
                        entry.update_text(&text_to_edit);
                    }
                    ui.label(egui::RichText::new(entry.text.clone()).color(egui::Color32::WHITE));
                }
                
                // Si no hay enlaces, permitir edición
                if !has_links && ui.text_edit_multiline(&mut text_to_edit).height_in_lines(10.0).desired_width(500.0).ui().clicked() {
                    entry.update_text(&text_to_edit);
                }
                
                ui.add_space(10.0);
                
                ui.label(egui::RichText::new("Etiquetas:").color(egui::Color32::YELLOW));
                
                if let Some(related) = &entry.related_to {
                    ui.label(format!("  🔗 Relacionado con: {}", related));
                }
                
                ui.add_space(5.0);
                
                ui.horizontal(|ui| {
                    ui.label(egui::RichText::new("+").color(egui::Color32::GRAY));
                    if ui.text_edit_singleline(&mut manager.current_filter).height_in_lines(1.0).desired_width(200.0).ui().clicked() {
                        manager.current_filter = manager.current_filter.clone();
                    }
                });
            });
        }
    }
}

