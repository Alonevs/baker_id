//! # Script Viewer API
//! 
//! Módulo para visualización de scripts.

use eframe::egui;

/// Script Viewer - visualización de scripts
#[derive(Default)]
pub struct ScriptViewer {
    pub current_script: Option<String>,
    pub script_path: Option<std::path::PathBuf>,
    pub line: usize,
    pub column: usize,
}

impl ScriptViewer {
    pub fn new() -> Self {
        Self::default()
    }
    
    pub fn render(&mut self, ui: &mut egui::Ui, console: &mut crate::ConsoleLog) {
        ui.heading("📝 Editor de Scripts");
        ui.add_space(5.0);

        if let Some(path) = &self.script_path {
            ui.horizontal(|ui| {
                ui.label(format!("Ruta activa: {}", path.display()));
                ui.add_space(10.0);
                if ui.button("💾 Guardar Script").clicked() {
                    if let Some(content) = &self.current_script {
                        match std::fs::write(path, content) {
                            Ok(_) => {
                                console.add_message(
                                    crate::debugger::LogLevel::Info,
                                    &format!("Script guardado exitosamente: {}", path.file_name().unwrap().to_string_lossy())
                                );
                            }
                            Err(e) => {
                                console.add_message(
                                    crate::debugger::LogLevel::Error,
                                    &format!("Error al guardar script: {}", e)
                                );
                            }
                        }
                    }
                }
            });
            ui.separator();
            ui.add_space(5.0);

            if let Some(content) = &mut self.current_script {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    let text_edit = egui::TextEdit::multiline(content)
                        .font(egui::TextStyle::Monospace)
                        .desired_width(f32::INFINITY)
                        .desired_rows(25)
                        .lock_focus(true);
                    ui.add(text_edit);
                });
            }
        } else {
            ui.colored_label(
                egui::Color32::from_rgb(180, 180, 180),
                "Haz doble clic en cualquier script (.lua o .rs) en el Asset Browser para comenzar a editar."
            );
        }
    }
}

