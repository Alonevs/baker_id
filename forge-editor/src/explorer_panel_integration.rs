//! # Explorer Panel Integration
//! 
//! Integración de ExplorerPanel con ProjectManager:
//! - Cargar estructura de proyecto
//! - Actualizar en tiempo real
//! - Mostrar assets del proyecto

use crate::project_manager::ProjectManager;
use crate::explorer_panel::ExplorerPanelWidget;
use eframe::egui;

/// Widget integrado de Explorer Panel
pub struct ExplorerPanelIntegration {
    /// Panel de exploración
    pub explorer: ExplorerPanelWidget,
    /// ProjectManager
    pub project_manager: ProjectManager,
    /// Ruta del proyecto actual
    pub current_project_path: Option<std::path::PathBuf>,
}

impl Default for ExplorerPanelIntegration {
    fn default() -> Self {
        Self::new()
    }
}

impl ExplorerPanelIntegration {
    /// Crea nuevo widget integrado
    pub fn new() -> Self {
        Self {
            explorer: ExplorerPanelWidget::new(),
            project_manager: ProjectManager::new(),
            current_project_path: None,
        }
    }

    /// Configura con proyecto actual
    pub fn set_current_project(&mut self, project_path: std::path::PathBuf) {
        self.current_project_path = Some(project_path.clone());
        
        // Cargar estructura del proyecto
        if let Some(ref path) = self.current_project_path.clone() {
            let mut clone = self.explorer.clone();
            clone.load_from_project_manager(path);
            self.explorer = clone;
        }
    }

    /// Actualiza desde ProjectManager
    pub fn update(&mut self) {
        if let Some(ref path) = self.current_project_path {
            let mut clone = self.explorer.clone();
            clone.load_from_project_manager(path);
            self.explorer = clone;
        }
    }

    /// Renderiza el widget integrado
    pub fn ui(&mut self, ctx: &egui::Context) {
        egui::TopBottomPanel::top("explorer_panel").show(ctx, |ui| {
            ui.heading("Explorer Panel - Integrated");
            
            // Botones de control
            ui.horizontal(|ui| {
                if ui.button("Load Project").clicked() {
                    if let Some(ref path) = self.current_project_path {
                        let mut clone = self.explorer.clone();
                        clone.load_from_project_manager(path);
                        self.explorer = clone;
                    }
                }
                
                if ui.button("Refresh").clicked() {
                    self.explorer.panel.refresh();
                }
                
                if ui.button("Expand All").clicked() {
                    let mut clone = self.explorer.clone();
                    clone.panel.expand_all();
                    self.explorer = clone;
                }
                
                if ui.button("Collapse All").clicked() {
                    let mut clone = self.explorer.clone();
                    clone.panel.collapse_all();
                    self.explorer = clone;
                }
            });
            
            ui.separator();
            
            // Renderizar árbol
            self.explorer.render(ctx);
        });
    }

    /// Obtiene archivo seleccionado
    pub fn get_selected_file(&self) -> Option<&crate::explorer_panel::ExplorerNode> {
        self.explorer.panel.get_selected_file()
    }

    /// Obtiene directorio seleccionado
    pub fn get_selected_folder(&self) -> Option<&crate::explorer_panel::ExplorerNode> {
        self.explorer.panel.get_selected_folder()
    }

    /// Obtiene ruta del archivo seleccionado
    pub fn get_selected_path(&self) -> Option<std::path::PathBuf> {
        self.explorer.panel.get_selected_path()
    }
}
