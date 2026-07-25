//! # Explorer Panel Integration
//! 
//! Integración de ExplorerPanel con ProjectManager:
//! - Cargar estructura de proyecto
//! - Actualizar en tiempo real
//! - Mostrar assets del proyecto
//! - Drag & Drop de archivos al viewport
//! - Context Menu para crear archivos/directorios

use crate::project_manager::ProjectManager;
use crate::explorer_panel::ExplorerPanelWidget;
use crate::drag_drop_integration::{DragDropSystem, DragDropData};
use crate::explorer_context_menu::{ExplorerContextMenu, ExplorerContextAction};
use eframe::egui;

/// Widget integrado de Explorer Panel
pub struct ExplorerPanelIntegration {
    /// Panel de exploración
    pub explorer: ExplorerPanelWidget,
    /// ProjectManager
    pub project_manager: ProjectManager,
    /// Ruta del proyecto actual
    pub current_project_path: Option<std::path::PathBuf>,
    /// Sistema de Drag & Drop
    pub drag_drop_system: DragDropSystem,
    /// Context Menu
    pub context_menu: ExplorerContextMenu,
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
            drag_drop_system: DragDropSystem::new(),
            context_menu: ExplorerContextMenu::new(),
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

    /// Renderiza el widget integrado con Drag & Drop y Context Menu
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
            
            // Renderizar árbol con Context Menu
            self.render_explorer_with_context_menu(ctx);
        });
        
        // Renderizar overlay de Drag & Drop en el viewport
        self.render_drag_drop_overlay(ctx);
    }
    
    /// Renderiza Explorer Panel con Context Menu
    fn render_explorer_with_context_menu(&mut self, ctx: &egui::Context) {
        // Renderizar menú contextual si está activo
        self.context_menu.render(ctx);
        
        // Renderizar árbol
        self.explorer.render(ctx);
    }
    
    /// Renderiza overlay de Drag & Drop
    fn render_drag_drop_overlay(&mut self, ctx: &egui::Context) {
        if let Some(ref data) = self.drag_drop_system.drag_data {
            egui::TopBottomPanel::top("drag_drop_overlay").show(ctx, |ui| {
                ui.heading("📁 Drop File Here");
                
                ui.label(format!("Dragging: {:?}", data));
                ui.label("Drag and drop to the viewport area");
            });
        }
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

    /// Inicia drag de archivo
    pub fn start_file_drag(&mut self, path: std::path::PathBuf) {
        self.drag_drop_system.start_drag(DragDropData::file(path));
    }

    /// Inicia drag de directorio
    pub fn start_directory_drag(&mut self, path: std::path::PathBuf) {
        self.drag_drop_system.start_drag(DragDropData::directory(path));
    }

    /// Finaliza drag y procesa drop
    pub fn end_drag(&mut self, viewport_path: Option<std::path::PathBuf>) {
        self.drag_drop_system.end_drag(viewport_path);
    }

    /// Verifica si extensión es soportada
    pub fn is_extension_supported(&self, extension: &str) -> bool {
        self.drag_drop_system.is_supported(extension)
    }

    /// Obtiene sistema de Drag & Drop
    pub fn get_drag_drop_system(&mut self) -> &mut DragDropSystem {
        &mut self.drag_drop_system
    }

    /// Maneja selección de ruta con Context Menu
    pub fn handle_context_menu(&mut self, path: std::path::PathBuf) {
        self.context_menu.select_path(path);
    }

    /// Obtiene context menu
    pub fn get_context_menu(&mut self) -> &mut ExplorerContextMenu {
        &mut self.context_menu
    }

    /// Renderiza menú contextual en posición
    pub fn render_context_menu_at(&mut self, ctx: &egui::Context, position: egui::Pos2) {
        if let Some(action) = &self.context_menu.selected_action {
            egui::menu::menu(ctx, |ui| {
                match action {
                    ExplorerContextAction::CreateFile { name } => {
                        if ui.button(format!("📄 Create File: {}", name)).clicked() {
                            self.handle_action(action.clone());
                        }
                    }
                    ExplorerContextAction::CreateDirectory { name } => {
                        if ui.button(format!("📁 Create Directory: {}", name)).clicked() {
                            self.handle_action(action.clone());
                        }
                    }
                    ExplorerContextAction::Rename { .. } => {
                        ui.label("Rename feature coming soon");
                    }
                    ExplorerContextAction::Delete { path } => {
                        if ui.button("🗑️ Delete").clicked() {
                            self.context_menu.delete_path(path);
                        }
                    }
                    ExplorerContextAction::Properties { path } => {
                        ui.label(format!("Path: {}", path.display()));
                    }
                }
            }).position(position);
        }
    }

    /// Maneja acción del menú
    fn handle_action(&mut self, action: ExplorerContextAction) {
        self.context_menu.handle_action(action);
        self.explorer.panel.refresh();
    }
}
