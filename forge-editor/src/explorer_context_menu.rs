//! # Explorer Context Menu
//! 
//! Menú contextual para Explorer Panel:
//! - Crear nuevo archivo
//! - Crear nuevo directorio
//! - Renombrar
//! - Eliminar
//! - Propiedades

use crate::explorer_panel::ExplorerPanelWidget;
use eframe::egui;

/// Contexto del menú
#[derive(Debug, Clone)]
pub enum ExplorerContextAction {
    /// Crear nuevo archivo
    CreateFile {
        /// Nombre del archivo
        name: String,
    },
    /// Crear nuevo directorio
    CreateDirectory {
        /// Nombre del directorio
        name: String,
    },
    /// Renombrar
    Rename {
        /// Ruta actual
        current_path: std::path::PathBuf,
        /// Nuevo nombre
        new_name: String,
    },
    /// Eliminar
    Delete {
        /// Ruta a eliminar
        path: std::path::PathBuf,
    },
    /// Propiedades
    Properties {
        /// Ruta
        path: std::path::PathBuf,
    },
}

impl std::fmt::Display for ExplorerContextAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CreateFile { name } => write!(f, "Create File: {}", name),
            Self::CreateDirectory { name } => write!(f, "Create Directory: {}", name),
            Self::Rename { .. } => write!(f, "Rename"),
            Self::Delete { .. } => write!(f, "Delete"),
            Self::Properties { .. } => write!(f, "Properties"),
        }
    }
}

/// Menú contextual del Explorer Panel
pub struct ExplorerContextMenu {
    /// Panel de exploración
    pub explorer: ExplorerPanelWidget,
    /// Acción seleccionada
    pub selected_action: Option<ExplorerContextAction>,
    /// Ruta del nodo seleccionado
    pub selected_path: Option<std::path::PathBuf>,
}

impl Default for ExplorerContextMenu {
    fn default() -> Self {
        Self::new()
    }
}

impl ExplorerContextMenu {
    /// Crea nuevo menú contextual
    pub fn new() -> Self {
        Self {
            explorer: ExplorerPanelWidget::new(),
            selected_action: None,
            selected_path: None,
        }
    }

    /// Renderiza el menú contextual
    pub fn render(&mut self, ctx: &egui::Context) {
        if let Some(action) = &self.selected_action {
            egui::menu::menu(ctx, |ui| {
                match action {
                    ExplorerContextAction::CreateFile { name } => {
                        ui.menu_button(format!("📄 Create File: {}", name), |ui| {
                            ui.horizontal(|ui| {
                                ui.label("Name:");
                                ui.text_edit_singleline(name);
                            });
                        });
                    }
                    ExplorerContextAction::CreateDirectory { name } => {
                        ui.menu_button(format!("📁 Create Directory: {}", name), |ui| {
                            ui.horizontal(|ui| {
                                ui.label("Name:");
                                ui.text_edit_singleline(name);
                            });
                        });
                    }
                    ExplorerContextAction::Rename { current_path, new_name } => {
                        ui.menu_button(format!("✏️ Rename: {}", new_name), |ui| {
                            ui.horizontal(|ui| {
                                ui.label("New Name:");
                                ui.text_edit_singleline(new_name);
                            });
                        });
                    }
                    ExplorerContextAction::Delete { path } => {
                        ui.menu_button(format!("🗑️ Delete: {}", path.display()), |ui| {
                            if ui.button("Confirm Delete").clicked() {
                                self.delete_path(path);
                                ui.close();
                            }
                        });
                    }
                    ExplorerContextAction::Properties { path } => {
                        ui.menu_button(format!("⚙️ Properties: {}", path.display()), |ui| {
                            ui.label("File Properties");
                            ui.label(format!("Path: {}", path.display()));
                            ui.label(format!("Size: {}", std::fs::metadata(path).map(|m| m.len()).unwrap_or(0)));
                            ui.label(format!("Modified: {}", std::fs::metadata(path).map(|m| m.modified().unwrap_or(std::time::SystemTime::now())).map(|t| t.format("%Y-%m-%d %H:%M:%S").to_string()).unwrap_or_else(|_| "Unknown".to_string())));
                        });
                    }
                }
            });
        }
    }

    /// Elimina ruta
    fn delete_path(&mut self, path: &std::path::PathBuf) {
        if path.exists() {
            if path.is_dir() {
                std::fs::remove_dir_all(path).ok();
            } else {
                std::fs::remove_file(path).ok();
            }
            self.explorer.panel.refresh();
        }
        self.selected_action = None;
    }

    /// Maneja acción del menú
    pub fn handle_action(&mut self, action: ExplorerContextAction) {
        self.selected_action = Some(action);
    }

    /// Obtiene ruta seleccionada
    pub fn get_selected_path(&self) -> Option<&std::path::PathBuf> {
        &self.selected_path.as_ref().map(|p| p.as_ref())
    }

    /// Selecciona ruta
    pub fn select_path(&mut self, path: std::path::PathBuf) {
        self.selected_path = Some(path.clone());
        // Determinar tipo de nodo
        if path.is_dir() {
            self.handle_directory_context(&path);
        } else {
            self.handle_file_context(&path);
        }
    }

    /// Maneja contexto de directorio
    fn handle_directory_context(&self, path: &std::path::PathBuf) {
        // Crear nuevo archivo en directorio
        let file_name = "new_file.bf";
        let full_path = path.join(file_name);
        self.selected_action = Some(ExplorerContextAction::CreateFile {
            name: file_name.to_string(),
        });
    }

    /// Maneja contexto de archivo
    fn handle_file_context(&self, path: &std::path::PathBuf) {
        // Crear nuevo directorio
        let dir_name = "new_folder";
        self.selected_action = Some(ExplorerContextAction::CreateDirectory {
            name: dir_name.to_string(),
        });
    }
}
