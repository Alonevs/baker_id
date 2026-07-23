//! # Export/Import Panel
//! 
//! Panel para exportar e importar proyectos en formato .map y JSON

use crate::ForgeEditorApp;

/// Panel de exportación/importación
pub struct ExportImportPanel {
    export_path: String,
    import_path: String,
    #[allow(dead_code)]
    file_format: String,
}

impl ExportImportPanel {
    /// Crea un nuevo panel de exportación/importación
    pub fn new() -> Self {
        Self {
            export_path: "project.map".to_string(),
            import_path: String::new(),
            file_format: "Map".to_string(),
        }
    }

    /// Renderiza el panel de exportación/importación
    pub fn render(&mut self, ui: &mut egui::Ui, app: &mut ForgeEditorApp) {
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            ui.heading("Export / Import Project");
            ui.add_space(10.0);

            // Selector de formato
            egui::ComboBox::from_id_salt("file_format")
                .selected_text(app.export_mgr.get_name())
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut app.export_mgr.get_project_mut().name, "New Project".to_string(), "New Project");
                    ui.separator();
                    
                    // Botón de exportar
                    if ui.button("Export Project").clicked() {
                        if let Err(e) = app.export_project(&self.export_path) {
                            ui.label(format!("Error: {}", e));
                        } else {
                            ui.label("Project exported successfully!");
                        }
                    }

                    // Botón de importar
                    if ui.button("Import Project").clicked() {
                        if let Err(e) = app.import_project(&self.import_path) {
                            ui.label(format!("Error: {}", e));
                        } else {
                            ui.label("Project imported successfully!");
                        }
                    }

                    // Botón de exportar JSON
                    if ui.button("Export JSON").clicked() {
                        if let Err(e) = app.export_json(&self.export_path) {
                            ui.label(format!("Error: {}", e));
                        } else {
                            ui.label("Project exported to JSON!");
                        }
                    }

                    // Botón de importar JSON
                    if ui.button("Import JSON").clicked() {
                        if let Err(e) = app.import_json(&self.import_path) {
                            ui.label(format!("Error: {}", e));
                        } else {
                            ui.label("Project imported from JSON!");
                        }
                    }
                });

            ui.add_space(15.0);

            // Path de exportación
            ui.label("Export Path:");
            ui.add(egui::TextEdit::singleline(&mut self.export_path).desired_width(300.0));

            // Path de importación
            ui.label("Import Path:");
            ui.add(egui::TextEdit::singleline(&mut self.import_path).desired_width(300.0));

            ui.add_space(10.0);

            // Información del proyecto
            ui.label("Project Info:");
            ui.label(format!("Name: {}", app.export_mgr.get_name()));
            ui.label(format!("Version: {}", app.export_mgr.get_project().version));
            ui.label(format!("Entities: {}", app.export_mgr.get_project().entities.len()));
            ui.label(format!("Physics Bodies: {}", app.export_mgr.get_project().physics.bodies.len()));
            ui.label(format!("Particles: {}", app.export_mgr.get_project().particles.particles.len()));
            ui.label(format!("Animations: {}", app.export_mgr.get_project().animations.animations.len()));
            ui.label(format!("Dialogues: {}", app.export_mgr.get_project().dialogues.len()));
            ui.label(format!("Events: {}", app.export_mgr.get_project().events.len()));
        });
    }
}

