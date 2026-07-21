//! # Script Properties API
//! 
//! Módulo para propiedades de scripts.

use eframe::egui;

/// Script Properties - propiedades de scripts
#[derive(Debug, Clone)]
pub struct ScriptProperties {
    pub script_name: String,
    pub script_path: String,
    pub script_lines: Vec<String>,
    pub current_line: usize,
    pub is_compiled: bool,
}

impl Default for ScriptProperties {
    fn default() -> Self {
        Self {
            script_name: "Untitled".to_string(),
            script_path: String::new(),
            script_lines: Vec::new(),
            current_line: 0,
            is_compiled: false,
        }
    }
}

impl ScriptProperties {
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Renderiza el editor de scripts en la UI
    /// 
    /// Conecta con: script_editor
    pub fn render(ui: &mut egui::Ui, app: &mut crate::ForgeEditorApp) {
        ui.heading("Script Properties");
        ui.add_space(10.0);
        ui.separator();
        ui.add_space(5.0);
        
        ui.group(|ui| {
            ui.label("Script Properties");
            ui.add_space(5.0);
            ui.label(format!("Script: {}", app.script_props.script_name));
            ui.label(format!("Path: {}", app.script_props.script_path));
            ui.label(format!("Compiled: {}", app.script_props.is_compiled));
        });
    }
    
    /// Renderiza el panel de scripts
    pub fn ui(&mut self, ui: &mut egui::Ui) {
        ui.heading("Script Properties");
        ui.add_space(10.0);
        ui.separator();
        ui.add_space(5.0);
        
        ui.group(|ui| {
            ui.label("Script:");
            ui.text_edit_singleline(&mut self.script_name);
            ui.add_space(5.0);
            ui.label("Path:");
            ui.text_edit_singleline(&mut self.script_path);
            ui.add_space(5.0);
            
            if !self.script_lines.is_empty() {
                ui.label("Code:");
                ui.add_space(5.0);
                ui.label("Line 1:");
                ui.text_edit_singleline(&mut self.script_lines[0]);
            }
        });
    }

    pub fn set_script_name(&mut self, name: &str) {
        self.script_name = name.to_string();
    }

    pub fn add_line(&mut self, line: &str) {
        self.script_lines.push(line.to_string());
    }

    pub fn remove_line(&mut self, index: usize) {
        if index < self.script_lines.len() {
            self.script_lines.remove(index);
        }
    }

    pub fn clear(&mut self) {
        self.script_lines.clear();
    }
}


