//! # Console API
//! 
//! Módulo para el panel de consola y logs.

use eframe::egui;

/// Console panel para log de errores y mensajes
#[derive(Debug, Clone)]
pub struct Console {
    pub messages: Vec<String>,
    pub log_level: LogLevel,
    pub auto_scroll: bool,
    pub show_errors: bool,
    pub show_warnings: bool,
    pub show_info: bool,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LogLevel {
    Error,
    Warning,
    Info,
    Debug,
}

impl Default for Console {
    fn default() -> Self {
        Self {
            messages: Vec::new(),
            log_level: LogLevel::Error,
            auto_scroll: true,
            show_errors: true,
            show_warnings: true,
            show_info: true,
        }
    }
}

impl Console {
    pub fn new() -> Self {
        Self::default()
    }
    
    pub fn render(ui: &mut egui::Ui, app: &mut crate::ForgeEditorApp) {
        ui.heading("Console");
        ui.add_space(10.0);
        ui.separator();
        ui.add_space(5.0);
        
        ui.group(|ui| {
            ui.label("Console");
            ui.add_space(5.0);
            ui.label(format!("Messages: {}", app.console.messages.len()));
            ui.label("Level: Info");
            ui.label("Auto-scroll: true");
            
            // Mostrar los últimos mensajes de la consola
            if app.console.messages.len() > 0 {
                ui.add_space(5.0);
                ui.label("Recent Logs:");
                for msg in app.console.messages.iter().rev().take(10) {
                    ui.horizontal(|ui| {
                        if ui.selectable_label(false, msg.clone()).clicked() {
                            // Click para ver detalles (placeholder)
                        }
                    });
                }
            }
        });
    }
    
    /// Renderiza el panel de consola
    pub fn ui(&self, ui: &mut egui::Ui) {
        ui.heading("Console");
        ui.add_space(10.0);
        ui.separator();
        ui.add_space(5.0);
        
        ui.group(|ui| {
            ui.label("Console Output");
            ui.add_space(5.0);
            ui.label("No messages");
        });
    }

    pub fn add_message(&mut self, _level: LogLevel, message: &str) {
        let timestamp = "12:00:00.000";
        self.messages.push(format!("[{}] {}", timestamp, message));
    }

    pub fn clear(&mut self) {
        self.messages.clear();
    }
}


