//! # Explorer Panel API
//! 
//! Panel de exploración de proyecto con Tree View:
//! - Navegación recursiva de carpetas
//! - Selección de elementos
//! - Context menu (nuevo archivo/directorio)

use std::path::PathBuf;
use std::time::SystemTime;
use std::cell::RefCell;
use serde::{Serialize, Deserialize};

/// Tipo de nodo en el árbol
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NodeType {
    Directory,
    File,
}

/// Nodo del árbol de exploración
#[derive(Debug)]
pub struct ExplorerNode {
    /// Nombre del nodo
    pub name: String,
    /// Ruta completa
    pub path: PathBuf,
    /// Tipo de nodo
    pub node_type: NodeType,
    /// Si es directorio, hijos del árbol
    pub children: Vec<ExplorerNode>,
    /// Si es archivo, información del archivo
    pub file_info: Option<FileInfo>,
    /// Si el directorio está expandido
    is_expanded: RefCell<bool>,
    /// Si el nodo está seleccionado
    is_selected: RefCell<bool>,
}

impl ExplorerNode {
    /// Crea un nodo de directorio
    pub fn new_directory(name: &str, path: PathBuf) -> Self {
        Self {
            name: name.to_string(),
            path,
            node_type: NodeType::Directory,
            children: Vec::new(),
            file_info: None,
            is_expanded: RefCell::new(false),
            is_selected: RefCell::new(false),
        }
    }

    /// Crea un nodo de archivo
    pub fn new_file(name: &str, path: PathBuf, file_info: FileInfo) -> Self {
        Self {
            name: name.to_string(),
            path,
            node_type: NodeType::File,
            children: Vec::new(),
            file_info: Some(file_info),
            is_expanded: RefCell::new(false),
            is_selected: RefCell::new(false),
        }
    }

    /// Escanea directorio y añade hijos
    pub fn scan_directory(&mut self) {
        if self.node_type != NodeType::Directory {
            return;
        }

        if !self.path.exists() {
            return;
        }

        if let Ok(entries) = std::fs::read_dir(&self.path) {
            let mut children = Vec::new();
            
            for entry in entries.flatten() {
                let path = entry.path();
                let file_name = path.file_name().and_then(|n| n.to_str()).unwrap_or("unknown");
                
                if path.is_dir() {
                    children.push(ExplorerNode::new_directory(file_name, path.clone()));
                } else {
                    let file_info = FileInfo::new(&path);
                    children.push(ExplorerNode::new_file(file_name, path.clone(), file_info));
                }
            }
            
            // Ordenar: directorios primero, luego archivos
            children.sort_by(|a, b| {
                match (&a.node_type, &b.node_type) {
                    (NodeType::Directory, NodeType::File) => std::cmp::Ordering::Less,
                    (NodeType::File, NodeType::Directory) => std::cmp::Ordering::Greater,
                    _ => a.name.cmp(&b.name),
                }
            });
            
            self.children = children;
            *self.is_expanded.borrow_mut() = true;
        }
    }

    /// Busca archivo por nombre (recursivo)
    pub fn find_file(&self, name: &str) -> Option<&ExplorerNode> {
        if self.name == name {
            return Some(self);
        }
        
        if self.node_type == NodeType::Directory && *self.is_expanded.borrow() {
            for child in &self.children {
                if let Some(found) = child.find_file(name) {
                    return Some(found);
                }
            }
        }
        
        None
    }

    /// Selecciona este nodo
    pub fn select(&mut self) {
        *self.is_selected.borrow_mut() = true;
    }

    /// Deselecciona este nodo
    pub fn deselect(&mut self) {
        *self.is_selected.borrow_mut() = false;
    }

    /// Expande este directorio
    pub fn expand(&mut self) {
        if self.node_type == NodeType::Directory {
            *self.is_expanded.borrow_mut() = true;
            self.scan_directory();
        }
    }

    /// Colapsa este directorio
    pub fn collapse(&mut self) {
        if self.node_type == NodeType::Directory {
            *self.is_expanded.borrow_mut() = false;
        }
    }

    /// Toggle expand/collapse
    pub fn toggle_expand(&mut self) {
        if self.node_type == NodeType::Directory {
            {
                let mut expanded = self.is_expanded.borrow_mut();
                *expanded = !*expanded;
            }
            if *self.is_expanded.borrow() {
                self.scan_directory();
            }
        }
    }
}

impl Clone for ExplorerNode {
    fn clone(&self) -> Self {
        Self {
            name: self.name.clone(),
            path: self.path.clone(),
            node_type: self.node_type.clone(),
            children: self.children.clone(),
            file_info: self.file_info.clone(),
            is_expanded: RefCell::new(*self.is_expanded.borrow()),
            is_selected: RefCell::new(*self.is_selected.borrow()),
        }
    }
}

/// Información de archivo
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileInfo {
    /// Nombre de archivo
    pub name: String,
    /// Extensión
    pub extension: String,
    /// Tamaño en bytes
    pub size: u64,
    /// Fecha de modificación
    pub modified: u64,
    /// Categoría del archivo
    pub category: String,
}

impl FileInfo {
    /// Crea nueva información de archivo
    pub fn new(path: &PathBuf) -> Self {
        let file_name = path.file_name().and_then(|n| n.to_str()).unwrap_or("unknown").to_string();
        let extension = path.extension().and_then(|e| e.to_str()).unwrap_or("").to_string();
        let metadata = std::fs::metadata(path).ok();
        
        let size = metadata.as_ref().map(|m| m.len()).unwrap_or(0);
        let modified = metadata.as_ref()
            .map(|m| m.modified().ok().unwrap_or(SystemTime::UNIX_EPOCH).duration_since(SystemTime::UNIX_EPOCH).unwrap_or(std::time::Duration::ZERO).as_secs())
            .unwrap_or(0);
        
        let category = Self::get_category_for_extension(&extension);
        
        Self {
            name: file_name,
            extension,
            size,
            modified,
            category: category.to_string(),
        }
    }

    /// Obtiene categoría para una extensión
    fn get_category_for_extension(extension: &str) -> &'static str {
        match extension {
            "png" | "jpg" | "jpeg" | "gif" | "bmp" | "webp" | "tga" | "psd" | "svg" => "Sprites",
            "mp3" | "wav" | "ogg" | "flac" | "aiff" => "Audio",
            "csv" | "json" => "Data",
            "rs" | "lua" | "gdscript" | "js" | "ts" | "py" => "Scripts",
            "toml" | "yaml" | "yml" => "Config",
            "txt" | "md" | "doc" => "Text",
            "rs" | "c" | "cpp" | "h" => "Source",
            _ => "Other",
        }
    }
}

/// Panel de exploración
#[derive(Debug, Clone)]
pub struct ExplorerPanel {
    /// Raíz del árbol
    pub root: ExplorerNode,
    /// Carpeta actual seleccionada
    pub current_folder: Option<PathBuf>,
    /// Nodo seleccionado
    pub selected_node: Option<ExplorerNode>,
}

impl Default for ExplorerPanel {
    fn default() -> Self {
        Self::new()
    }
}

impl ExplorerPanel {
    /// Crea nuevo panel de exploración
    pub fn new() -> Self {
        let root = ExplorerNode::new_directory("Project", PathBuf::from("."));
        
        Self {
            root,
            current_folder: None,
            selected_node: None,
        }
    }

    /// Carga estructura de proyecto desde ProjectManager
    pub fn load_project(&mut self, project_path: &PathBuf) {
        let mut root = ExplorerNode::new_directory("Project", project_path.clone());
        root.scan_directory();
        
        self.root = root;
        self.current_folder = Some(project_path.clone());
        self.selected_node = None;
    }

    /// Actualiza estructura actual
    pub fn refresh(&mut self) {
        if let Some(ref path) = self.current_folder {
            self.root.scan_directory();
        }
    }

    /// Selecciona nodo por nombre
    pub fn select_node(&mut self, name: &str) -> bool {
        if let Some(node) = self.root.find_file(name) {
            self.selected_node = Some(node.clone());
            let mut node_clone = node.clone();
            node_clone.select();
            self.selected_node = Some(node_clone);
            true
        } else {
            false
        }
    }

    /// Expande todos los directorios
    pub fn expand_all(&mut self) {
        let mut root = self.root.clone();
        *root.is_expanded.borrow_mut() = true;
        root.scan_directory();
        for child in &mut root.children {
            *child.is_expanded.borrow_mut() = true;
            child.scan_directory();
            for grandchild in &mut child.children {
                *grandchild.is_expanded.borrow_mut() = true;
                grandchild.scan_directory();
            }
        }
        self.root = root;
    }

    /// Colapsa todos los directorios
    pub fn collapse_all(&mut self) {
        let mut root = self.root.clone();
        for child in &mut root.children {
            *child.is_expanded.borrow_mut() = false;
            for grandchild in &mut child.children {
                *grandchild.is_expanded.borrow_mut() = false;
            }
        }
        self.root = root;
    }

    /// Obtiene lista de archivos en carpeta actual
    pub fn get_files(&self) -> Vec<&ExplorerNode> {
        self.root.children.iter()
            .filter(|n| n.node_type == NodeType::File)
            .collect()
    }

    /// Obtiene lista de carpetas en carpeta actual
    pub fn get_folders(&self) -> Vec<&ExplorerNode> {
        self.root.children.iter()
            .filter(|n| n.node_type == NodeType::Directory)
            .collect()
    }

    /// Obtiene archivo seleccionado
    pub fn get_selected_file(&self) -> Option<&ExplorerNode> {
        self.selected_node.as_ref().filter(|n| n.node_type == NodeType::File)
    }

    /// Obtiene directorio seleccionado
    pub fn get_selected_folder(&self) -> Option<&ExplorerNode> {
        self.selected_node.as_ref().filter(|n| n.node_type == NodeType::Directory)
    }

    /// Cambia carpeta actual
    pub fn change_folder(&mut self, folder: &ExplorerNode) {
        self.current_folder = Some(folder.path.clone());
        self.selected_node = None;
    }

    /// Obtiene ruta de archivo seleccionado
    pub fn get_selected_path(&self) -> Option<PathBuf> {
        self.selected_node.as_ref().map(|n| n.path.clone())
    }

    /// Obtiene categoría del archivo seleccionado
    pub fn get_selected_category(&self) -> Option<String> {
        self.selected_node.as_ref().and_then(|n| n.file_info.as_ref()).map(|fi| fi.category.clone())
    }

    /// Carga estructura del proyecto desde ProjectManager
    pub fn load_from_project_manager(&mut self, project_path: &PathBuf) {
        let mut root = ExplorerNode::new_directory("Project", project_path.clone());
        root.scan_directory();
        self.root = root;
        self.current_folder = Some(project_path.clone());
    }
}

/// Widget UI del Explorer Panel
pub struct ExplorerPanelWidget {
    /// Panel de exploración
    pub panel: ExplorerPanel,
    /// Ancho del panel
    pub width: f32,
    /// Alto del panel
    pub height: f32,
}

impl Default for ExplorerPanelWidget {
    fn default() -> Self {
        Self::new()
    }
}

impl Clone for ExplorerPanelWidget {
    fn clone(&self) -> Self {
        Self {
            panel: self.panel.clone(),
            width: self.width,
            height: self.height,
        }
    }
}

impl ExplorerPanelWidget {
    /// Crea nuevo widget de Explorer Panel
    pub fn new() -> Self {
        Self {
            panel: ExplorerPanel::new(),
            width: 250.0,
            height: 600.0,
        }
    }

    /// Carga estructura del proyecto desde ProjectManager
    pub fn load_from_project_manager(&mut self, project_path: &std::path::PathBuf) {
        self.panel.load_from_project_manager(project_path);
    }

    /// Renderiza el Explorer Panel
    pub fn render(&mut self, ctx: &egui::Context) {
        egui::TopBottomPanel::top("explorer_panel").show(ctx, |ui| {
            ui.style_mut().wrap_mode = Some(egui::epaint::TextWrapMode::Truncate);
            
            // Título
            ui.heading("Explorer");
            
            // Tree View
            ui.horizontal(|ui| {
                if ui.button("Expand All").clicked() {
                    self.panel.expand_all();
                }
                if ui.button("Collapse All").clicked() {
                    self.panel.collapse_all();
                }
                if ui.button("Refresh").clicked() {
                    self.panel.refresh();
                }
            });
            
            ui.separator();
            
            // Renderizar árbol
            self.render_tree(ui, &self.panel.root, 0);
        });
    }

    /// Renderiza árbol recursivamente
    fn render_tree(&self, ui: &mut egui::Ui, node: &ExplorerNode, depth: usize) {
        let is_dir = node.node_type == NodeType::Directory;
        
        // Icono y nombre
        let icon = if is_dir { "📁" } else { "📄" };
        
        ui.indent(depth, |ui| {
            // Checkbox para expandir/colapsar
            if is_dir {
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.checkbox(&mut node.is_expanded.borrow_mut(), "");
                });
                
                if ui.button(format!("{} {}", icon, node.name)).clicked() {
                    let mut cloned = node.clone();
                    if *cloned.is_expanded.borrow() {
                        cloned.collapse();
                    } else {
                        cloned.expand();
                    }
                }
            } else {
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.checkbox(&mut node.is_selected.borrow_mut(), "");
                });
                
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label(format!("{} {}", icon, node.name));
                });
            }
            
            // Hijo si es directorio expandido
            if is_dir && *node.is_expanded.borrow() {
                for child in &node.children {
                    self.render_tree(ui, child, depth + 1);
                }
            }
        });
    }
}
