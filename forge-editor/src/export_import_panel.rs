//! Export Import Panel - Panel para exportar e importar assets

use eframe::egui;
use crate::export_manager::ExportManager;
use crate::import_manager::ImportManager;
use std::path::PathBuf;

pub struct ExportImportPanel {
    pub export_manager: ExportManager,
    pub import_manager: ImportManager,
    pub export_path: PathBuf,
    pub import_path: PathBuf,
    pub last_exported: Option<String>,
    pub last_imported: Option<String>,
    pub project_summary: String,
}

impl Default for ExportImportPanel {
    fn default() -> Self {
        Self::new()
    }
}

impl ExportImportPanel {
    pub fn new() -> Self {
        Self {
            export_manager: ExportManager::new(),
            import_manager: ImportManager::new(),
            export_path: PathBuf::from("project_export.json"),
            import_path: PathBuf::from("project_import.json"),
            last_exported: None,
            last_imported: None,
            project_summary: String::new(),
        }
    }
    
    /// Actualiza el resumen del proyecto
    pub fn update_summary(&mut self) {
        self.project_summary = self.export_manager.get_project()
            .entities
            .iter()
            .map(|e| format!("- {}: {} (pos: {}, {}, {})", e.name, e.id, e.position.0, e.position.1, e.position.2))
            .collect::<Vec<_>>()
            .join("\n");
    }
    
    /// Exporta el proyecto
    pub fn export_project(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.export_manager.export(&self.export_path.to_str().unwrap())?;
        self.last_exported = Some(self.export_path.to_string_lossy().to_string());
        self.log(format!("✅ Exportado: {}", self.export_path.display()));
        Ok(())
    }
    
    /// Importa el proyecto
    pub fn import_project(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.import_manager.import(&self.import_path.to_str().unwrap())?;
        self.import_manager.validate()?;
        self.last_imported = Some(self.import_path.to_string_lossy().to_string());
        self.update_summary();
        self.log(format!("✅ Importado: {}", self.import_path.display()));
        Ok(())
    }
    
    /// Agrega entidad manualmente
    pub fn add_entity(&mut self, name: &str, position: (f32, f32)) {
        self.export_manager.add_entity(
            "entity_1",
            name,
            (position.0, position.1, 0.0),
            (0.0, 0.0, 0.0),
            (1.0, 1.0, 1.0),
            true,
            Vec::new(),
            Vec::new(),
        );
        self.update_summary();
    }
    
    /// Log de export/import
    pub fn log(&mut self, message: String) {
        println!("{}", message);
    }
    
    /// UI completa del Export Import Panel
    pub fn ui(&mut self, ctx: &egui::Context, ui: &mut egui::Ui) {
        egui::SidePanel::left("export_import_panel").show(ctx, |ui| {
            ui.heading("Export / Import");
            
            // Resumen del proyecto
            ui.label("Project Summary:");
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.label(format!("Name: {}", self.export_manager.get_name()));
                ui.label(format!("Version: {}", self.export_manager.get_project().version));
                ui.label(format!("Entities: {}", self.export_manager.get_project().entities.len()));
                ui.label(format!("Physics Bodies: {}", self.export_manager.get_project().physics.bodies.len()));
                ui.label(format!("Particles: {}", self.export_manager.get_project().particles.particles.len()));
                ui.label(format!("Animations: {}", self.export_manager.get_project().animations.animations.len()));
                ui.label(format!("Dialogues: {}", self.export_manager.get_project().dialogues.len()));
                ui.label(format!("Events: {}", self.export_manager.get_project().events.len()));
                
                ui.separator();
                ui.label("Entities:");
                if !self.project_summary.is_empty() {
                    ui.label(&self.project_summary);
                } else {
                    ui.label("(No entities)");
                }
            });
            
            ui.separator();
            
            // Exportar
            ui.label("Export Project:");
            ui.horizontal(|ui| {
                ui.label("Path:");
                ui.add(egui::TextEdit::singleline(&mut self.export_path.to_string_lossy()));
                if ui.button("Browse").clicked() {
                    // Placeholder para browse dialog
                    ui.add(egui::Label::new("📁 Browse dialog placeholder"));
                }
            });
            
            if ui.button("📤 Export Project").clicked() {
                if let Err(e) = self.export_project() {
                    ui.label(format!("❌ Export error: {}", e));
                } else {
                    ui.label("✅ Exported successfully!");
                }
            }
            
            ui.separator();
            
            // Importar
            ui.label("Import Project:");
            ui.horizontal(|ui| {
                ui.label("Path:");
                ui.add(egui::TextEdit::singleline(&mut self.import_path.to_string_lossy()));
                if ui.button("Browse").clicked() {
                    // Placeholder para browse dialog
                    ui.add(egui::Label::new("📁 Browse dialog placeholder"));
                }
            });
            
            if ui.button("📥 Import Project").clicked() {
                if let Err(e) = self.import_project() {
                    ui.label(format!("❌ Import error: {}", e));
                } else {
                    ui.label("✅ Imported successfully!");
                }
            }
            
            ui.separator();
            
            // Botones adicionales
            ui.horizontal(|ui| {
                if ui.button("🗑️ Clear Project").clicked() {
                    self.export_manager = ExportManager::new();
                    self.update_summary();
                    ui.label("✅ Cleared");
                }
                
                if ui.button("🔄 Reset to Default").clicked() {
                    self.export_manager = ExportManager::new();
                    self.update_summary();
                    ui.label("✅ Reset");
                }
            });
            
            ui.separator();
            
            // Historial
            ui.label("History:");
            egui::ScrollArea::vertical().show(ui, |ui| {
                if let Some(ref last_export) = self.last_exported {
                    ui.label(format!("📤 Last export: {}", last_export));
                }
                if let Some(ref last_import) = self.last_imported {
                    ui.label(format!("📥 Last import: {}", last_import));
                }
            });
        });
    }
}
