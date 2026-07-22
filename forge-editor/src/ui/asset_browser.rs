//! Asset Browser para lectura de archivos del disco

use std::fs;
use std::path::{Path, PathBuf};
use std::collections::HashMap;
use pollster::FutureExt;
use crate::debugger::LogLevel;
use crate::{DragOperation, DropTarget};
use crate::ui::asset_preview::AssetPreviewType;

/// Tipos de asset por extensión
pub struct AssetType {
    pub name: &'static str,
    pub extensions: Vec<&'static str>,
}

/// Estructura de carpeta para árbol jerárquico
#[derive(Clone)]
pub struct FolderTree {
    pub name: String,
    pub files: Vec<String>,
    pub subfolders: Vec<FolderTree>,
}

/// Asset Browser para navegación de assets
#[derive(Debug, Clone)]
pub struct AssetBrowser {
    pub assets: Vec<String>,
    pub selected_asset: Option<usize>,
    pub current_category: String,
    pub show_categories: bool,
    pub search_filter: String,
    pub categories: Vec<String>,
    pub assets_directory: PathBuf,
    pub asset_tree: HashMap<String, Vec<String>>,
    pub pending_import: Option<(PathBuf, String)>,
}

impl Default for AssetBrowser {
    fn default() -> Self {
        Self::new()
    }
}

impl AssetBrowser {
    pub fn new() -> Self {
        Self {
            assets: Vec::new(),
            selected_asset: None,
            current_category: "All".to_string(),
            show_categories: true,
            search_filter: String::new(),
            categories: vec![
                "Sprites".to_string(),
                "Animations".to_string(),
                "Audio".to_string(),
                "Scripts".to_string(),
                "Materials".to_string(),
                "Dialogues".to_string(),
            ],
            assets_directory: PathBuf::from("./assets"),
            asset_tree: HashMap::new(),
            pending_import: None,
        }
    }

    /// Cargar assets desde el directorio de assets del proyecto
    pub fn load_from_project(&mut self, project_path: &PathBuf) {
        let assets_path = project_path.join("assets");
        self.load_assets(&assets_path);
    }

    /// Cargar assets desde la ruta de assets actual
    pub fn load_from_current_assets_path(&mut self, assets_path: &PathBuf) {
        self.load_assets(assets_path);
    }

    /// Agregar un asset seleccionado a la escena actual
    /// Buscar el path de un asset en el árbol
    pub fn find_asset_path(&self, asset_name: &str) -> Option<std::path::PathBuf> {
        let extension = asset_name.split('.').last().unwrap_or("").to_lowercase();
        
        // Verificar si es un sprite (imagen)
        if ["png", "jpg", "jpeg", "gif", "bmp", "webp"].contains(&extension.as_str()) {
            // Buscar el path en el árbol de assets
            for (folder_name, files) in &self.asset_tree {
                if files.iter().any(|f| f.contains(asset_name)) {
                    let mut path = self.assets_directory.clone();
                    if folder_name != "root" {
                        path.push(folder_name);
                    }
                    path.push(asset_name);
                    if path.exists() {
                        return Some(path);
                    }
                }
            }
        }
        
        None
    }

    /// Cargar assets desde un directorio
    pub fn load_assets(&mut self, directory: &Path) {
        if !directory.exists() {
            return;
        }
        
        let mut assets = Vec::new();
        
        if let Ok(entries) = directory.read_dir() {
            for entry in entries {
                if let Ok(entry) = entry {
                    let path = entry.path();
                    
                    if path.is_file() {
                        let extension = path.extension().and_then(|e| e.to_str()).unwrap_or("");
                        
                        // Soporte para imágenes y assets comunes
                        if ["png", "jpg", "jpeg", "gif", "webp", "tga", "psd", "svg", "fnt", "json", "script"].contains(&extension) {
                            let asset_name = path.file_name().and_then(|n| n.to_str()).unwrap_or("unknown").to_string();
                            assets.push(asset_name.clone());
                        }
                    }
                }
            }
        }
        
        self.assets = assets;
    }

    /// Obtener directorio de assets por defecto
    pub fn get_default_assets_directory() -> PathBuf {
        PathBuf::from("./assets")
    }

    /// Cargar assets desde un directorio (recursivo)
    pub fn load_from_directory(&mut self, dir_path: &Path) {
        self.assets_directory = dir_path.to_path_buf();
        self.asset_tree.clear();
        self.assets.clear();
        self.pending_import = None;
        
        if !dir_path.exists() {
            println!("Directorio de assets no existe: {:?}", dir_path);
            return;
        }
        
        // Recopilar todos los archivos recursivamente
        let mut all_files: Vec<PathBuf> = Vec::new();
        self.collect_files_recursive(dir_path, &mut all_files);
        
        // Organizar en árbol por carpetas
        let mut tree: HashMap<String, Vec<String>> = HashMap::new();
        for file_path in &all_files {
            if file_path.is_file() {
                let file_name = file_path.file_name().unwrap().to_str().unwrap().to_string();
                let extension = file_path.extension().and_then(|e| e.to_str()).unwrap_or("").to_lowercase();
                
                // Determinar categoría
                let category = self.get_category_for_extension(&extension);
                
                // Buscar carpeta padre
                if let Some(parent) = file_path.parent() {
                    let parent_name = parent.file_name().unwrap().to_str().unwrap_or("root");
                    tree.entry(parent_name.to_string()).or_insert_with(Vec::new).push(format!("{} ({})", file_name, category));
                }
            }
        }
        
        self.asset_tree = tree;
        self.load_flat_assets_from_slice(&all_files);
    }
    
    /// Recopilar archivos recursivamente
    fn collect_files_recursive(&self, path: &Path, files: &mut Vec<PathBuf>) {
        if !path.exists() {
            return;
        }
        
        if let Ok(entries) = fs::read_dir(path) {
            for entry in entries {
                if let Ok(entry) = entry {
                    let path = entry.path();
                    if path.is_dir() {
                        // Recursivamente cargar subcarpetas
                        self.collect_files_recursive(&path, files);
                    } else if path.is_file() {
                        files.push(path);
                    }
                }
            }
        }
    }

    /// Extensiones soportadas por categoría
    fn get_supported_extensions() -> HashMap<&'static str, Vec<&'static str>> {
        let mut ext_map: HashMap<&'static str, Vec<&'static str>> = HashMap::new();
        
        // Sprites
        ext_map.insert("Sprites", vec!["png", "jpg", "jpeg", "gif", "bmp", "webp"]);
        // Audio
        ext_map.insert("Audio", vec!["mp3", "wav", "ogg", "flac", "aiff"]);
        // Dialogues
        ext_map.insert("Dialogues", vec!["csv", "json"]);
        // Scripts
        ext_map.insert("Scripts", vec!["rs", "lua", "gdscript", "js", "ts"]);
        // Materials
        ext_map.insert("Materials", vec!["mat", "mtl", "obj"]);
        
        ext_map
    }
    
    /// Verificar si una extensión es soportada
    fn is_extension_supported(&self, extension: &str, category: &str) -> bool {
        let supported = Self::get_supported_extensions();
        supported.get(category).map(|exts| exts.contains(&extension)).unwrap_or(false)
    }
    
    /// Cargar assets en formato plano (no organizado por carpetas)
    pub fn load_flat_assets(&mut self, files: &[PathBuf]) {
        for file_path in files {
            if file_path.is_file() {
                let file_name = file_path.file_name().unwrap().to_str().unwrap().to_string();
                let extension = file_path.extension().and_then(|e| e.to_str()).unwrap_or("").to_lowercase();
                
                // Verificar si la extensión es soportada
                if self.is_extension_supported(&extension, &self.current_category) {
                    self.assets.push(file_name);
                }
            }
        }
    }

    /// Cargar assets desde slice (evita borrow después de move)
    pub fn load_flat_assets_from_slice(&mut self, files: &[PathBuf]) {
        for file_path in files {
            if file_path.is_file() {
                let file_name = file_path.file_name().unwrap().to_str().unwrap().to_string();
                let extension = file_path.extension().and_then(|e| e.to_str()).unwrap_or("").to_lowercase();
                
                // Verificar si la extensión es soportada
                if self.is_extension_supported(&extension, &self.current_category) {
                    self.assets.push(file_name);
                }
            }
        }
    }

    /// Construir árbol jerárquico de carpetas
    fn build_hierarchical_tree(&self, dir_path: &Path) -> FolderTree {
        if !dir_path.exists() {
            return FolderTree {
                name: dir_path.file_name().unwrap().to_str().unwrap().to_string(),
                files: Vec::new(),
                subfolders: Vec::new(),
            };
        }
        
        let mut root = FolderTree {
            name: dir_path.file_name().unwrap().to_str().unwrap().to_string(),
            files: Vec::new(),
            subfolders: Vec::new(),
        };
        
        if let Ok(entries) = fs::read_dir(dir_path) {
            for entry in entries {
                if let Ok(entry) = entry {
                    let path = entry.path();
                    if path.is_dir() {
                        let _folder_name = path.file_name().unwrap().to_str().unwrap().to_string();
                        root.subfolders.push(self.build_hierarchical_tree(&path));
                    } else if path.is_file() {
                        let file_name = path.file_name().unwrap().to_str().unwrap().to_string();
                        let extension = path.extension().and_then(|e| e.to_str()).unwrap_or("").to_lowercase();
                        
                        // Solo agregar archivos soportados
                        if self.is_extension_supported(&extension, &self.current_category) {
                            root.files.push(format!("{} ({})", file_name, self.get_category_for_extension(&extension)));
                        }
                    }
                }
            }
        }
        
        root
    }
    
    /// Obtener categoría para una extensión
    fn get_category_for_extension(&self, extension: &str) -> &'static str {
        match extension {
            "png" | "jpg" | "jpeg" | "gif" | "bmp" | "webp" => "Sprites",
            "mp3" | "wav" | "ogg" | "flac" | "aiff" => "Audio",
            "csv" | "json" => "Dialogues",
            "rs" | "lua" | "gdscript" | "js" | "ts" => "Scripts",
            "mat" | "mtl" | "obj" => "Materials",
            _ => "Other",
        }
    }

    /// Filtrar assets por categoría
    pub fn get_assets_by_category(&self, category: &str) -> Vec<&String> {
        self.assets.iter()
            .filter(|asset| self.get_category_for_asset(asset) == category)
            .collect()
    }

    /// Obtener categoría de un asset
    fn get_category_for_asset(&self, asset: &str) -> &'static str {
        let extension = asset.split('.').last().unwrap_or("").to_lowercase();
        self.get_category_for_extension(&extension)
    }

    /// Renderizar árbol de carpetas recursivamente
    pub fn render_folder_tree(&self, ui: &mut egui::Ui, folder: &FolderTree, depth: usize) {
        ui.group(|ui| {
            ui.horizontal(|ui| {
                // Indentación
                for _ in 0..depth {
                    ui.add_space(10.0);
                    ui.label("└── ");
                }
                
                // Nombre de carpeta
                ui.label(format!("📁 {}", folder.name));
                
                // Contador de archivos
                if !folder.files.is_empty() {
                    ui.label(format!(" ({} files)", folder.files.len()));
                }
                
                // Renderizar archivos
                if !folder.files.is_empty() {
                    ui.indent(&folder.name, |ui| {
                        for file in folder.files.iter().take(5) {
                            ui.horizontal(|ui| {
                                ui.label("  •");
                                ui.label(file);
                            });
                        }
                        if folder.files.len() > 5 {
                            ui.label(format!("  ... and {} more files", folder.files.len() - 5));
                        }
                    });
                }
                
                // Renderizar subcarpetas
                for subfolder in &folder.subfolders {
                    self.render_folder_tree(ui, subfolder, depth + 1);
                }
            });
        });
    }
    
    /// Cambiar categoría actual
    pub fn change_category(&mut self, category: &str) {
        self.current_category = category.to_string();
    }

    /// Obtener asset seleccionado
    pub fn get_selected_asset(&self) -> Option<&String> {
        self.selected_asset.and_then(|index| self.assets.get(index))
    }

    /// Obtener ruta completa del asset seleccionado
    pub fn get_selected_asset_path(&self) -> Option<PathBuf> {
        self.get_selected_asset().and_then(|asset| {
            // Buscar en el árbol de carpetas
            for (folder_name, files) in &self.asset_tree {
                if files.iter().any(|f| f.contains(asset)) {
                    // Construir ruta aproximada
                    let mut path = self.assets_directory.clone();
                    if folder_name != "root" {
                        path.push(folder_name);
                    }
                    path.push(asset);
                    return Some(path);
                }
            }
            None
        })
    }

    /// Obtener ruta de imagen para preview
    pub fn get_image_path(&self, asset_name: &str) -> Option<PathBuf> {
        let extension = asset_name.split('.').last().unwrap_or("").to_lowercase();
        
        if ["png", "jpg", "jpeg", "gif", "bmp", "webp"].contains(&extension.as_str()) {
            // Buscar en el árbol de carpetas
            for (folder_name, files) in &self.asset_tree {
                if files.iter().any(|f| f.contains(asset_name)) {
                    let mut path = self.assets_directory.clone();
                    if folder_name != "root" {
                        path.push(folder_name);
                    }
                    path.push(asset_name);
                    return Some(path);
                }
            }
        }
        None
    }

    /// Filtrar assets por búsqueda
    pub fn filter_assets(&mut self, filter: &str) {
        self.search_filter = filter.to_string();
    }

    /// Obtener assets filtrados
    pub fn get_filtered_assets(&self) -> Vec<&String> {
        if self.search_filter.is_empty() {
            return self.assets.iter().collect();
        }
        
        self.assets.iter()
            .filter(|asset| asset.to_lowercase().contains(&self.search_filter.to_lowercase()))
            .collect()
    }
    
    /// Asignar sprite a entidad seleccionada
    pub fn assign_sprite_to_entity(&mut self, asset_name: &str, app: &mut crate::ForgeEditorApp) {
        if let Some(sprite_path) = self.get_image_path(asset_name) {
            app.selected_entity_sprite_path = Some(sprite_path.to_string_lossy().to_string());
            app.console.add_message(crate::debugger::LogLevel::Info, &format!("Sprite assigned: {}", asset_name));
        }
    }

    /// Renderizar el Asset Browser en la UI
    pub fn render(ui: &mut egui::Ui, app: &mut crate::ForgeEditorApp) {
        ui.heading("Asset Browser");
        ui.add_space(10.0);
        ui.separator();
        ui.add_space(5.0);
        
        ui.group(|ui| {
            ui.label("Asset Browser");
            ui.add_space(5.0);
            
            // Mostrar directorio actual
            ui.horizontal(|ui| {
                if ui.button("Load Default Assets...").clicked() {
                    let dir = app.asset_browser.assets_directory.clone();
                    app.asset_browser.load_assets(&dir);
                    app.console.add_message(LogLevel::Info, &format!("Loaded {} assets", app.asset_browser.assets.len()));
                }
                ui.add_space(5.0);
                if ui.button("Change Directory...").clicked() {
                    if let Some(path) = pollster::block_on(rfd::AsyncFileDialog::new().pick_folder()) {
                        let path = path.path();
                        app.asset_browser.assets_directory = path.to_path_buf();
                        let dir = app.asset_browser.assets_directory.clone();
                        app.asset_browser.load_assets(&dir);
                        app.console.add_message(LogLevel::Info, &format!("Loaded {} assets from: {}", app.asset_browser.assets.len(), path.display()));
                    }
                }
            });
            ui.add_space(5.0);
            
            // Mostrar árbol de carpetas jerárquico
            ui.label("📁 Folders:");
            if !app.asset_browser.asset_tree.is_empty() {
                ui.add_space(2.0);
                
                // Construir árbol jerárquico
                let asset_browser = &app.asset_browser;
                let root = asset_browser.build_hierarchical_tree(&asset_browser.assets_directory);
                asset_browser.render_folder_tree(ui, &root, 0);
            } else {
                ui.add_space(2.0);
                ui.label("  No folders found");
            }
            
            ui.add_space(5.0);
            ui.horizontal(|ui| {
                if ui.button("🔄 Refresh Assets").clicked() {
                    let dir = app.asset_browser.assets_directory.clone();
                    app.asset_browser.load_assets(&dir);
                    app.console.add_message(LogLevel::Info, &format!("Refreshed: {} assets loaded", app.asset_browser.assets.len()));
                }
            });
            
            ui.add_space(10.0);
            ui.separator();
            ui.add_space(5.0);
            
            // Mostrar estadísticas
            let total_assets = app.asset_browser.assets.len();
            let filtered_assets = app.asset_browser.get_filtered_assets().len();
            let _current_category_assets = app.asset_browser.get_assets_by_category(&app.asset_browser.current_category).len();
            
            ui.horizontal(|ui| {
                ui.label(format!("Total: {}", total_assets));
                ui.label(format!("Filtered: {}", filtered_assets));
                ui.label(format!("Category: {}", app.asset_browser.current_category));
            });
            
            ui.add_space(10.0);
            
            // Control de búsqueda
            ui.horizontal(|ui| {
                ui.label("Search:");
                ui.text_edit_singleline(&mut app.asset_browser.search_filter);
                if ui.button("Clear").clicked() {
                    app.asset_browser.search_filter.clear();
                }
            });
            
            ui.add_space(10.0);
            
            // Categorías
            ui.horizontal(|ui| {
                if ui.button("All Categories").clicked() {
                    app.asset_browser.change_category("All");
                }
                ui.add_space(5.0);
                if ui.button("Sprites").clicked() {
                    app.asset_browser.change_category("Sprites");
                }
                ui.add_space(5.0);
                if ui.button("Audio").clicked() {
                    app.asset_browser.change_category("Audio");
                }
                ui.add_space(5.0);
                if ui.button("Scripts").clicked() {
                    app.asset_browser.change_category("Scripts");
                }
                ui.add_space(5.0);
                if ui.button("Dialogues").clicked() {
                    app.asset_browser.change_category("Dialogues");
                }
            });
            
            ui.add_space(10.0);
            
            // Botones de acción
            ui.horizontal(|ui| {
                if ui.button("🔄 Refresh").clicked() {
                    let dir = app.asset_browser.assets_directory.clone();
                    app.asset_browser.load_from_directory(&dir);
                    app.console.add_message(LogLevel::Warning, "Refreshing assets...");
                }
                ui.add_space(5.0);
                if ui.button("📥 Import Selected").clicked() {
                    let selected_asset = app.asset_browser.get_selected_asset();
                    let asset_browser_dir = app.asset_browser.assets_directory.clone();
                    
                    if let Some(asset_name) = selected_asset {
                        // Intentar importar (esto fallará por préstamo mutuo, se manejará en update)
                        app.console.add_message(LogLevel::Info, &format!("Import requested for: {}", asset_name));
                        app.pending_import = Some((asset_browser_dir, asset_name.clone()));
                    } else {
                        app.console.add_message(LogLevel::Warning, "No asset selected");
                    }
                }
                ui.add_space(5.0);
                if ui.button("🎯 Assign to Selected Entity").clicked() {
                    // Leer datos de asset_browser primero
                    let asset_browser = &app.asset_browser;
                    let selected_idx = asset_browser.selected_asset;
                    
                    if let Some(idx) = selected_idx {
                        let asset_name = asset_browser.assets.get(idx).map(|s| s.as_str()).unwrap_or("");
                        
                        // Implementar lógica de asignación localmente para evitar préstamos mutuos
                        let extension = asset_name.split('.').last().unwrap_or("").to_lowercase();
                        if ["png", "jpg", "jpeg", "gif", "bmp", "webp"].contains(&extension.as_str()) {
                            let mut found_path: Option<std::path::PathBuf> = None;
                            for (folder_name, files) in &asset_browser.asset_tree {
                                if files.iter().any(|f| f.contains(asset_name)) {
                                    let mut path = asset_browser.assets_directory.clone();
                                    if folder_name != "root" {
                                        path.push(folder_name);
                                    }
                                    path.push(asset_name);
                                    if path.exists() {
                                        found_path = Some(path);
                                        break;
                                    }
                                }
                            }
                            
                            if let Some(path) = found_path {
                                app.selected_entity_sprite_path = Some(path.to_string_lossy().to_string());
                                app.console.add_message(LogLevel::Info, &format!("Sprite assigned to entity: {}", asset_name));
                            }
                        }
                    } else {
                        app.console.add_message(LogLevel::Warning, "No asset selected");
                    }
                }
            });
            
            ui.add_space(10.0);
            
            // Lista de assets
            ui.label("Assets:");
            
            // Zona de drop para assets
            if matches!(app.drag_operation, Some(DragOperation::Asset { .. })) {
                let asset_name = app.dragged_asset.as_ref().map(|a| a.path.clone()).unwrap_or_else(|| "unknown".to_string());
                ui.group(|ui| {
                    ui.label(format!("Drop {} here", asset_name));
                    ui.add_space(5.0);
                });
            }
            ui.add_space(5.0);
            
            // Obtener assets filtrados de manera owned para evitar préstamos concurrentes de app
            let filtered_assets: Vec<String> = app.asset_browser.get_filtered_assets()
                .iter()
                .map(|s| (*s).clone())
                .collect();
            
            if filtered_assets.is_empty() {
                ui.label("No assets found");
            } else {
                // Obtener paths de imágenes
                let image_paths: Vec<Option<PathBuf>> = filtered_assets.iter().map(|asset| {
                    app.asset_browser.get_image_path(asset.as_str())
                }).collect();
                
                let selected_asset_idx = app.asset_browser.selected_asset;

                // Renderizar assets
                for (index, (asset, image_path)) in filtered_assets.iter().zip(image_paths.iter()).enumerate() {
                    let is_selected = selected_asset_idx == Some(index);
                    
                    // Renderizar item
                    let _ = ui.group(|ui| {
                        ui.horizontal(|ui| {
                            // Preview de imagen con URL file://
                            if let Some(path) = image_path {
                                if path.exists() {
                                    ui.add_space(5.0);
                                    // Crear URL file:// para egui
                                    let file_url = format!("file://{}", path.to_string_lossy());
                                    let image = egui::Image::new(file_url);
                                    // Mostrar imagen con tamaño máximo 64x64px
                                    ui.scope(|ui| {
                                        ui.add_sized([64.0, 64.0], image);
                                    });
                                }
                            }
                            
                            // Nombre del asset
                            ui.label(format!("• {}", asset));
                            
                            // Indicador de selección
                            if is_selected {
                                ui.label(" (selected)");
                            }
                        });
                        
                        // Botón de selección
                        ui.add_space(10.0);
                        if ui.selectable_label(is_selected, "Select").clicked() {
                            app.asset_browser.selected_asset = Some(index);
                        }
                        
                        // Botón para agregar a escena
                        if ui.selectable_label(is_selected, "Add to Scene").clicked() {
                            app.console.add_message(
                                crate::debugger::LogLevel::Info,
                                &format!("Added asset '{}' to scene", asset),
                            );
                            app.add_asset_to_scene(asset);
                            app.selected_entity_sprite_path = Some(asset.to_string());
                        }
                        
                        // Iniciar drag automático al hacer click en el asset
                        let asset_ext = asset.split('.').last().unwrap_or("").to_lowercase();
                        let asset_type = crate::ui::AssetType::from_extension(&asset_ext);
                        let preview_type = AssetPreviewType::from_extension(&asset_ext);
                        
                        if ui.selectable_label(is_selected, asset).clicked() {
                            let asset_real = app.create_asset(asset);
                            app.dragged_asset = Some(asset_real);
                            app.drag_operation = Some(DragOperation::Asset {
                                asset_name: asset.to_string(),
                                asset_type: asset_type.clone(),
                            });
                            app.drop_target = Some("SceneTree".to_string());
                            
                            // Establecer preview del asset
                            app.asset_preview.set_preview_type(preview_type);
                            app.asset_preview.set_preview_path(asset.to_string());
                        }
                    });
                }
            }
        });
    }
}

