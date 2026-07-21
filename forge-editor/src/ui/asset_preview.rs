//! # Asset Preview API
//! 
//! Módulo para preview de audio, scripts y otros assets.

use eframe::egui;
use std::path::Path;

/// Tipos de preview
#[derive(Debug, Clone, PartialEq)]
pub enum AssetPreviewType {
    None,
    Audio,
    Script,
    Texture,
    Dialogue,
}

impl AssetPreviewType {
    pub fn from_extension(extension: &str) -> Self {
        let ext = extension.to_lowercase();
        match ext.as_str() {
            "mp3" | "wav" | "ogg" | "flac" => AssetPreviewType::Audio,
            "cs" | "c#" | "cpp" | "py" | "js" | "ts" | "lua" | "gd" => AssetPreviewType::Script,
            "png" | "jpg" | "jpeg" | "gif" | "bmp" | "webp" | "tga" => AssetPreviewType::Texture,
            "json" | "dialogue" => AssetPreviewType::Dialogue,
            _ => AssetPreviewType::None,
        }
    }
}

/// Asset Preview - preview de assets (audio, scripts, texturas)
#[derive(Debug, Clone)]
pub struct AssetPreview {
    pub preview_type: AssetPreviewType,
    pub preview_path: Option<String>,
    pub preview_data: Option<String>,
    pub is_playing: bool,
    pub volume: f32,
}

impl Default for AssetPreview {
    fn default() -> Self {
        Self {
            preview_type: AssetPreviewType::None,
            preview_path: None,
            preview_data: None,
            is_playing: false,
            volume: 1.0,
        }
    }
}

impl AssetPreview {
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Renderiza el preview del asset seleccionado
    pub fn render(ui: &mut egui::Ui, app: &mut crate::ForgeEditorApp) {
        ui.heading("Asset Preview");
        ui.add_space(10.0);
        ui.separator();
        ui.add_space(5.0);
        
        ui.group(|ui| {
            // Mostrar tipo de preview
            ui.horizontal(|ui| {
                ui.label("Preview:");
                ui.separator();
                
                match app.asset_preview.preview_type {
                    AssetPreviewType::Audio => {
                        ui.label("🔊 Audio");
                    }
                    AssetPreviewType::Script => {
                        ui.label("📜 Script");
                    }
                    AssetPreviewType::Texture => {
                        ui.label("🖼️ Texture");
                    }
                    AssetPreviewType::Dialogue => {
                        ui.label("💬 Dialogue");
                    }
                    AssetPreviewType::None => {
                        ui.label("No preview available");
                    }
                }
            });
            
            ui.add_space(10.0);
            
            // Mostrar detalles según tipo
            match app.asset_preview.preview_type {
                AssetPreviewType::Audio => {
                    Self::render_audio_preview(ui, app);
                }
                AssetPreviewType::Script => {
                    Self::render_script_preview(ui, app);
                }
                AssetPreviewType::Texture => {
                    Self::render_texture_preview(ui, app);
                }
                AssetPreviewType::Dialogue => {
                    Self::render_dialogue_preview(ui, app);
                }
                AssetPreviewType::None => {
                    ui.label("Select an asset to preview");
                }
            }
        });
    }
    
    /// Preview de audio
    fn render_audio_preview(ui: &mut egui::Ui, app: &mut crate::ForgeEditorApp) {
        ui.group(|ui| {
            ui.label("Audio Preview");
            ui.add_space(5.0);
            
            if let Some(ref path) = app.asset_preview.preview_path {
                ui.label(format!("📁 {}", path));
                ui.add_space(3.0);
                
                // Controles de audio
                ui.horizontal(|ui| {
                    if ui.button("▶️ Play").clicked() {
                        app.asset_preview.is_playing = !app.asset_preview.is_playing;
                        app.console.add_message(
                            crate::debugger::LogLevel::Info,
                            &format!("Playing audio: {}", path)
                        );
                    }
                    
                    if ui.button("⏸️ Pause").clicked() {
                        app.asset_preview.is_playing = false;
                        app.console.add_message(
                            crate::debugger::LogLevel::Info,
                            "Audio paused"
                        );
                    }
                    
                    if ui.button("⏹️ Stop").clicked() {
                        app.asset_preview.is_playing = false;
                        app.console.add_message(
                            crate::debugger::LogLevel::Info,
                            "Audio stopped"
                        );
                    }
                    
                    ui.add_space(10.0);
                    
                    // Volumen
                    ui.horizontal(|ui| {
                        ui.label("Volume:");
                        ui.add(egui::Slider::new(&mut app.asset_preview.volume, 0.0..=1.0).text(""));
                    });
                });
                
                ui.add_space(10.0);
                
                let progress = if app.asset_preview.is_playing {
                    0.5
                } else {
                    0.0
                };
                ui.add(egui::ProgressBar::new(progress));
            } else {
                ui.label("No audio selected");
            }
        });
    }
    
    /// Preview de script
    fn render_script_preview(ui: &mut egui::Ui, app: &mut crate::ForgeEditorApp) {
        ui.group(|ui| {
            ui.label("Script Preview");
            ui.add_space(5.0);
            
            if let Some(ref path) = app.asset_preview.preview_path {
                ui.label(format!("📄 {}", path));
                ui.add_space(3.0);
                
                ui.horizontal(|ui| {
                    ui.label("Language:");
                    ui.label("C#");
                });
                
                ui.add_space(5.0);
                
                if ui.button("📄 Open in Editor").clicked() {
                    app.console.add_message(
                        crate::debugger::LogLevel::Info,
                        &format!("Opening script: {}", path)
                    );
                }
                
                ui.add_space(5.0);
                ui.label("// Script preview (code editor integration)");
            } else {
                ui.label("No script selected");
            }
        });
    }
    
    /// Preview de textura
    fn render_texture_preview(ui: &mut egui::Ui, app: &mut crate::ForgeEditorApp) {
        ui.group(|ui| {
            ui.label("Texture Preview");
            ui.add_space(5.0);
            
            if let Some(ref path) = app.asset_preview.preview_path {
                ui.label(format!("🖼️ {}", path));
                ui.add_space(3.0);
                
                let path_buf = Path::new(path).to_path_buf();
                if path_buf.exists() {
                        let file_url = format!("file://{}", path_buf.to_string_lossy());
                        let image = egui::Image::new(file_url);
                        
                        ui.scope(|ui| {
                            ui.add_sized([200.0, 200.0], image);
                        });
                        
                        ui.add_space(5.0);
                        
                        if ui.button("🔄 Refresh Preview").clicked() {
                            app.console.add_message(
                                crate::debugger::LogLevel::Info,
                                "Preview refreshed"
                            );
                        }
                    }
            } else {
                ui.label("No texture selected");
            }
        });
    }
    
    /// Preview de diálogo
    fn render_dialogue_preview(ui: &mut egui::Ui, app: &mut crate::ForgeEditorApp) {
        ui.group(|ui| {
            ui.label("Dialogue Preview");
            ui.add_space(5.0);
            
            if let Some(ref path) = app.asset_preview.preview_path {
                ui.label(format!("💬 {}", path));
                ui.add_space(3.0);
                
                ui.horizontal(|ui| {
                    ui.label("Status:");
                    ui.label("Inactive");
                });
                
                ui.add_space(5.0);
                
                if ui.button("📄 Open Dialogue").clicked() {
                    app.console.add_message(
                        crate::debugger::LogLevel::Info,
                        &format!("Opening dialogue: {}", path)
                    );
                }
                
                ui.add_space(5.0);
                ui.label("// Dialogue preview (dialogue editor integration)");
            } else {
                ui.label("No dialogue selected");
            }
        });
    }
    
    /// Establece el tipo de preview
    pub fn set_preview_type(&mut self, ty: AssetPreviewType) {
        self.preview_type = ty;
    }
    
    /// Establece la ruta del asset
    pub fn set_preview_path(&mut self, path: String) {
        self.preview_path = Some(path);
    }
}

