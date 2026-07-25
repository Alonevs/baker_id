//! # Drag & Drop Integration
//! 
//! Módulo para Drag & Drop de archivos del Explorer Panel al Viewport.

use crate::project_manager::ProjectManager;
use std::path::PathBuf;

/// Sistema de Drag & Drop para archivos
pub struct DragDropSystem {
    pub drag_data: Option<DragDropData>,
    pub supported_extensions: Vec<String>,
}

impl Default for DragDropSystem {
    fn default() -> Self {
        Self::new()
    }
}

impl DragDropSystem {
    /// Crea un nuevo sistema de Drag & Drop
    pub fn new() -> Self {
        Self {
            drag_data: None,
            supported_extensions: vec![
                "bf".to_string(),
                "rs".to_string(),
                "lua".to_string(),
                "gdscript".to_string(),
                "js".to_string(),
                "ts".to_string(),
                "json".to_string(),
                "toml".to_string(),
                "yaml".to_string(),
                "png".to_string(),
                "jpg".to_string(),
                "jpeg".to_string(),
                "gif".to_string(),
                "bmp".to_string(),
                "webp".to_string(),
                "mp3".to_string(),
                "wav".to_string(),
                "ogg".to_string(),
                "flac".to_string(),
            ],
        }
    }

    /// Inicia el drag con datos
    pub fn start_drag(&mut self, data: DragDropData) {
        self.drag_data = Some(data);
    }

    /// Finaliza el drag
    pub fn end_drag(&mut self, target_path: Option<PathBuf>) {
        if let Some(ref data) = self.drag_data {
            if let Some(target) = target_path {
                self.handle_drop(data, &target);
            }
        }
        self.drag_data = None;
    }

    /// Maneja el drop en una ruta objetivo
    pub fn handle_drop(&mut self, data: &DragDropData, target_path: &PathBuf) {
        match data {
            DragDropData::File(path) => {
                self.handle_file_drop(path, target_path);
            }
            DragDropData::Directory(path) => {
                self.handle_directory_drop(path, target_path);
            }
        }
    }

    /// Maneja el drop de un archivo
    pub fn handle_file_drop(&mut self, source_path: &PathBuf, target_path: &PathBuf) {
        // Validar que el archivo existe
        if !source_path.exists() {
            return;
        }

        // Validar extensión
        if let Some(ext) = source_path.extension().and_then(|e| e.to_str()) {
            if !self.supported_extensions.contains(&ext.to_lowercase()) {
                self.log(format!("❌ Extension no soportada: {}", ext));
                return;
            }
        }

        // Copiar archivo a la ruta objetivo
        if let Some(parent) = target_path.parent() {
            std::fs::create_dir_all(parent).ok();
            
            let dest_path = target_path.join(source_path.file_name().unwrap_or(&source_path.clone()));
            std::fs::copy(source_path, &dest_path).ok();
            
            self.log(format!("✅ File dropped: {} → {}", source_path.display(), dest_path.display()));
        }
    }

    /// Maneja el drop de un directorio
    pub fn handle_directory_drop(&mut self, source_path: &PathBuf, target_path: &PathBuf) {
        // Validar que el directorio existe
        if !source_path.is_dir() {
            return;
        }

        // Copiar directorio a la ruta objetivo
        if let Some(parent) = target_path.parent() {
            std::fs::create_dir_all(parent).ok();
            
            let dest_path = target_path.join(source_path.file_name().unwrap_or(&source_path.clone()));
            
            if std::fs::copy(source_path, &dest_path).is_ok() {
                // Copiar contenido recursivamente
                self.copy_directory_recursive(source_path, &dest_path);
                
                self.log(format!("✅ Directory dropped: {} → {}", source_path.display(), dest_path.display()));
            }
        }
    }

    /// Copia directorio recursivamente
    fn copy_directory_recursive(&self, src: &PathBuf, dest: &PathBuf) {
        if let Ok(entries) = std::fs::read_dir(src) {
            for entry in entries.flatten() {
                let src_path = entry.path();
                let dest_path = dest.join(entry.file_name());
                
                if src_path.is_dir() {
                    if let Some(parent) = dest_path.parent() {
                        std::fs::create_dir_all(parent).ok();
                    }
                    self.copy_directory_recursive(&src_path, &dest_path);
                } else {
                    std::fs::copy(&src_path, &dest_path).ok();
                }
            }
        }
    }

    /// Registra mensaje
    pub fn log(&mut self, message: String) {
        println!("{}", message);
    }

    /// Verifica si una extensión es soportada
    pub fn is_supported(&self, extension: &str) -> bool {
        self.supported_extensions.contains(&extension.to_lowercase())
    }

    /// Obtiene archivos soportados
    pub fn get_supported_extensions(&self) -> &Vec<String> {
        &self.supported_extensions
    }
}

/// Datos de un drag & drop
#[derive(Debug, Clone)]
pub enum DragDropData {
    /// Ruta de un archivo
    File(PathBuf),
    /// Ruta de un directorio
    Directory(PathBuf),
}

impl DragDropData {
    /// Crea datos de archivo
    pub fn file(path: PathBuf) -> Self {
        Self::File(path)
    }

    /// Crea datos de directorio
    pub fn directory(path: PathBuf) -> Self {
        Self::Directory(path)
    }

    /// Obtiene la ruta
    pub fn path(&self) -> Option<&PathBuf> {
        match self {
            Self::File(path) => Some(path),
            Self::Directory(path) => Some(path),
        }
    }
}
