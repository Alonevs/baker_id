//! # Preview Panel - Vista previa de assets
//! 
//! Panel para preview de:
//! - Imágenes (PNG, JPG, GIF, BMP, WEBP)
//! - Audio (MP3, WAV, OGG, FLAC)
//! - Scripts (.bf)
//! - Textos y JSON

use eframe::egui;
use std::path::PathBuf;

/// Tipos de assets soportados
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AssetType {
    Image,
    Audio,
    Script,
    Json,
    Text,
}

impl std::fmt::Display for AssetType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AssetType::Image => write!(f, "🖼️ Image"),
            AssetType::Audio => write!(f, "🎵 Audio"),
            AssetType::Script => write!(f, "💻 Script"),
            AssetType::Json => write!(f, "📄 JSON"),
            AssetType::Text => write!(f, "📝 Text"),
        }
    }
}

/// Info de un asset
#[derive(Debug, Clone)]
pub struct AssetInfo {
    pub name: String,
    pub path: PathBuf,
    pub asset_type: AssetType,
    pub size: u64,
    pub preview_data: Option<String>,
}

impl AssetInfo {
    pub fn new(name: String, path: PathBuf, asset_type: AssetType) -> Self {
        let metadata = std::fs::metadata(&path).unwrap_or_else(|_| std::fs::Metadata::from_bytes(&[]));
        Self {
            name,
            path,
            asset_type,
            size: metadata.len(),
            preview_data: None,
        }
    }
    
    pub fn detect_type(path: &PathBuf) -> AssetType {
        let extension = path.extension()
            .and_then(|e| e.to_str())
            .unwrap_or("")
            .to_lowercase();
        
        match extension.as_str() {
            "png" | "jpg" | "jpeg" | "gif" | "bmp" | "webp" => AssetType::Image,
            "mp3" | "wav" | "ogg" | "flac" | "aiff" => AssetType::Audio,
            "bf" | "rs" | "lua" | "gdscript" | "js" | "ts" => AssetType::Script,
            "json" | "toml" | "yaml" | "yml" => AssetType::Json,
            _ => AssetType::Text,
        }
    }
}

/// Preview Panel para assets
pub struct PreviewPanel {
    pub selected_asset: Option<AssetInfo>,
    pub available_assets: Vec<AssetInfo>,
    pub zoom_level: f32,
    pub playback_speed: f32,
    pub current_frame: usize,
    pub frame_count: usize,
    pub scroll_position: i32,
    pub show_line_numbers: bool,
    pub syntax_highlighted: bool,
}

impl Default for PreviewPanel {
    fn default() -> Self {
        Self::new()
    }
}

impl PreviewPanel {
    pub fn new() -> Self {
        Self {
            selected_asset: None,
            available_assets: Vec::new(),
            zoom_level: 1.0,
            playback_speed: 1.0,
            current_frame: 0,
            frame_count: 0,
            scroll_position: 0,
            show_line_numbers: false,
            syntax_highlighted: true,
        }
    }
    
    /// Agrega asset al panel
    pub fn add_asset(&mut self, asset: AssetInfo) {
        self.available_assets.push(asset);
    }
    
    /// Selecciona asset
    pub fn select_asset(&mut self, index: usize) {
        if let Some(asset) = self.available_assets.get(index) {
            self.selected_asset = Some(asset.clone());
            self.current_frame = 0;
            self.frame_count = self.available_assets.len();
        }
    }
    
    /// Obtiene asset seleccionado
    pub fn get_selected_asset(&self) -> Option<&AssetInfo> {
        self.selected_asset.as_ref()
    }
    
    /// Preview de imagen
    fn render_image_preview(&self, ui: &mut egui::Ui, asset: &AssetInfo) {
        ui.label(format!("🖼️ Image Preview: {}", asset.name));
        ui.label(format!("Size: {} bytes", asset.size));
        
        if let Some(preview_data) = &asset.preview_data {
            ui.label(format!("Preview: {}", preview_data.chars().take(50).collect::<String>()));
        }
        
        // Placeholder para renderizado de imagen con egui_extras
        ui.horizontal(|ui| {
            if ui.button("📸 Full Screen").clicked() {
                ui.add(egui::Label::new("🖼️ Full screen preview placeholder"));
            }
            if ui.button("🔄 Refresh").clicked() {
                ui.add(egui::Label::new("🔄 Refresh placeholder"));
            }
        });
    }
    
    /// Preview de audio
    fn render_audio_preview(&self, ui: &mut egui::Ui, asset: &AssetInfo) {
        ui.label(format!("🎵 Audio Preview: {}", asset.name));
        ui.label(format!("Size: {} bytes", asset.size));
        
        ui.horizontal(|ui| {
            if ui.button("⏮️ Prev").clicked() {
                self.current_frame = if self.current_frame > 0 { self.current_frame - 1 } else { 0 };
            }
            
            if ui.button("▶ Play").clicked() {
                ui.add(egui::Label::new(format!("▶ Playing: {}", asset.name)));
            }
            
            if ui.button("⏸️ Pause").clicked() {
                ui.add(egui::Label::new("⏸️ Pause placeholder"));
            }
            
            if ui.button("⏭️ Next").clicked() {
                self.current_frame = if self.current_frame < self.frame_count - 1 { self.current_frame + 1 } else { self.frame_count - 1 };
            }
        });
        
        ui.add_space(10.0);
        
        // Waveform placeholder
        ui.label("📊 Waveform: [Placeholder]");
    }
    
    /// Preview de script
    fn render_script_preview(&self, ui: &mut egui::Ui, asset: &AssetInfo) {
        ui.label(format!("💻 Script Preview: {}", asset.name));
        ui.label(format!("Size: {} bytes", asset.size));
        
        // Controles de script
        ui.horizontal(|ui| {
            if ui.button("🔍 Minimap").clicked() {
                ui.add(egui::Label::new("🔍 Minimap placeholder"));
            }
            if ui.button("⚙️ Settings").clicked() {
                ui.add(egui::Label::new("⚙️ Settings placeholder"));
            }
        });
        
        ui.add_space(10.0);
        
        // Editor de script placeholder
        ui.label("📝 Script Editor: [Placeholder]");
    }
    
    /// Preview de JSON
    fn render_json_preview(&self, ui: &mut egui::Ui, asset: &AssetInfo) {
        ui.label(format!("📄 JSON Preview: {}", asset.name));
        ui.label(format!("Size: {} bytes", asset.size));
        
        ui.horizontal(|ui| {
            if ui.button("🔍 Validate").clicked() {
                ui.add(egui::Label::new("✅ Validated placeholder"));
            }
            if ui.button("📊 Format").clicked() {
                ui.add(egui::Label::new("📊 Formatted placeholder"));
            }
        });
        
        ui.add_space(10.0);
        
        ui.label("📋 JSON Content: [Placeholder]");
    }
    
    /// Preview de texto
    fn render_text_preview(&self, ui: &mut egui::Ui, asset: &AssetInfo) {
        ui.label(format!("📝 Text Preview: {}", asset.name));
        ui.label(format!("Size: {} bytes", asset.size));
        
        ui.horizontal(|ui| {
            if ui.button("🔍 Search").clicked() {
                ui.add(egui::Label::new("🔍 Search placeholder"));
            }
            if ui.button("📊 Word Count").clicked() {
                ui.add(egui::Label::new("📊 Word count placeholder"));
            }
        });
        
        ui.add_space(10.0);
        
        ui.label("📄 Text Content: [Placeholder]");
    }
    
    /// UI completa del Preview Panel
    pub fn ui(&mut self, ctx: &egui::Context, ui: &mut egui::Ui) {
        egui::SidePanel::right("preview_panel")
            .resizable(true)
            .min_width(300.0)
            .show(ctx, |ui| {
                ui.heading("🖼️ Preview Panel");
                
                // Selector de asset
                ui.label("Select Asset:");
                egui::ScrollArea::vertical()
                    .max_height(ui.available_height() - 200.0)
                    .show(ui, |ui| {
                        for (index, asset) in self.available_assets.iter().enumerate() {
                            ui.horizontal(|ui| {
                                if ui.selectable_label(
                                    self.selected_asset.as_ref().map(|a| a.path == asset.path).unwrap_or(false),
                                    &asset.name
                                ).clicked() {
                                    self.select_asset(index);
                                }
                                
                                ui.label(format!("({})", asset.size));
                            });
                        }
                    });
                
                ui.separator();
                
                // Preview actual
                if let Some(asset) = &self.selected_asset {
                    ui.label(format!("Preview: {}", asset.name));
                    ui.separator();
                    
                    match asset.asset_type {
                        AssetType::Image => self.render_image_preview(ui, asset),
                        AssetType::Audio => self.render_audio_preview(ui, asset),
                        AssetType::Script => self.render_script_preview(ui, asset),
                        AssetType::Json => self.render_json_preview(ui, asset),
                        AssetType::Text => self.render_text_preview(ui, asset),
                    }
                } else {
                    ui.label("No asset selected");
                    ui.label("Select an asset from the list above");
                }
                
                ui.separator();
                
                // Controles generales
                ui.label("Controls:");
                ui.horizontal(|ui| {
                    if ui.button("🔄 Refresh All").clicked() {
                        ui.add(egui::Label::new("🔄 Refresh all assets placeholder"));
                    }
                    if ui.button("📁 Import Assets").clicked() {
                        ui.add(egui::Label::new("📁 Import assets dialog placeholder"));
                    }
                    if ui.button("🗑️ Clear Selection").clicked() {
                        self.selected_asset = None;
                        ui.add(egui::Label::new("🗑️ Cleared selection"));
                    }
                });
                
                ui.separator();
                
                // Información
                ui.label("Info:");
                ui.label(format!("Total Assets: {}", self.available_assets.len()));
                ui.label(format!("Zoom: {}x", self.zoom_level));
                ui.label(format!("Playback Speed: {}x", self.playback_speed));
            });
    }
    
    /// Agrega assets desde disco
    pub fn scan_directory(&mut self, path: &std::path::Path) {
        if let Ok(entries) = std::fs::read_dir(path) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_file() {
                    let name = path.file_name()
                        .and_then(|n| n.to_str())
                        .unwrap_or("")
                        .to_string();
                    
                    let asset_type = AssetInfo::detect_type(&path);
                    let asset = AssetInfo::new(name, path, asset_type);
                    
                    self.add_asset(asset);
                }
            }
        }
    }
}