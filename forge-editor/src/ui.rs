//! UI del editor

use crate::play_session::{PlaySession, Entity};
use crate::snapshot_manager::SnapshotManager;
use crate::timeline_integration::TimelineIntegration;
use crate::input_capture::UserInput;
use crate::hot_reload_panel::HotReloadPanel;
use crate::hot_reload_integration::SimpleFileWatcherIntegration;
use crate::explorer_panel_integration::ExplorerPanelIntegration;
use crate::toolbar::ToolbarWidget;
use crate::event_forge::EventForgeWidget;
use crate::property_panel::PropertyPanel;
use crate::export_import_panel::ExportImportPanel;
use crate::preview_panel::PreviewPanel;
use crate::drag_drop_integration::{DragDropSystem, DragDropData};
use eframe::egui;
use eframe::App;

/// App principal del editor
pub struct ForgeEditorApp {
    pub logs: Vec<String>,
    pub timeline_integration: Option<TimelineIntegration>,
    pub play_session: Option<PlaySession>,
    pub snapshot_manager: SnapshotManager,
    pub user_input: UserInput,
    pub is_playing: bool,
    pub hot_reload_panel: HotReloadPanel,
    pub hot_reload_manager: crate::hot_reload::HotReloadManager,
    pub file_watcher_integration: Option<SimpleFileWatcherIntegration>,
    pub explorer_panel_integration: ExplorerPanelIntegration,
    pub toolbar_widget: ToolbarWidget,
    pub event_forge_widget: EventForgeWidget,
    pub property_panel: PropertyPanel,
    pub export_import_panel: ExportImportPanel,
    pub preview_panel: PreviewPanel,
    pub drag_drop_system: DragDropSystem,
}

impl Default for ForgeEditorApp {
    fn default() -> Self {
        Self {
            logs: Vec::new(),
            timeline_integration: None,
            play_session: None,
            snapshot_manager: SnapshotManager::new(),
            user_input: UserInput::default(),
            is_playing: false,
            hot_reload_panel: HotReloadPanel::new(),
            hot_reload_manager: crate::hot_reload::HotReloadManager::new(),
            file_watcher_integration: None,
            explorer_panel_integration: ExplorerPanelIntegration::new(),
            toolbar_widget: ToolbarWidget::new(),
            event_forge_widget: EventForgeWidget::new(),
            property_panel: PropertyPanel::new(),
            export_import_panel: ExportImportPanel::new(),
            preview_panel: PreviewPanel::new(),
            drag_drop_system: DragDropSystem::new(),
        }
    }
}

impl ForgeEditorApp {
    pub fn new(_cc: &eframe::CreationContext) -> Self {
        let mut app = Self::default();
        
        // Inicializar FileWatcher Integration
        app.file_watcher_integration = Some(SimpleFileWatcherIntegration::new(
            app.hot_reload_manager.clone(),
            &std::env::temp_dir(),
            100
        ));
        app.file_watcher_integration.as_mut().unwrap().start();
        
        // Inicializar Toolbar
        app.toolbar_widget.toolbar.select();
        
        // Configurar Explorer Panel con proyecto actual
        app.explorer_panel_integration.explorer.panel.current_project_path = 
            Some(std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from(".")));
        
        // Configurar Export/Import Panel con ruta actual
        app.export_import_panel.export_path = std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from("project_export.json"));
        app.export_import_panel.import_path = std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from("project_import.json"));
        app.export_import_panel.update_summary();
        
        // Configurar Preview Panel con assets del directorio actual
        let current_dir = std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from("."));
        app.preview_panel.scan_directory(&current_dir);
        
        // Configurar Drag & Drop System
        app.drag_drop_system = DragDropSystem::new();
        
        app
    }
    
    /// Renderiza el widget de Event Forge
    pub fn render_event_forge(&mut self, ctx: &egui::Context) {
        self.event_forge_widget.render(ctx);
    }
    
    pub fn log(&mut self, message: String) {
        self.logs.push(message);
    }
    
    /// Inicia Play Mode
    pub fn start_play(&mut self, entities: &[Entity]) {
        self.play_session = Some(PlaySession::new(entities));
        self.is_playing = true;
        self.log("Play Mode iniciado".to_string());
    }
    
    /// Detiene Play Mode
    pub fn stop_play(&mut self) {
        if let Some(ref mut _session) = self.play_session {
            let _ = _session.stop(&mut []);
        }
        self.play_session = None;
        self.is_playing = false;
        self.log("Play Mode detenido".to_string());
    }
    
    /// Actualiza la aplicación
    pub fn update(&mut self, delta: f32) {
        if let Some(ref mut session) = self.play_session {
            session.update(delta, &self.user_input);
        }
        
        // Actualizar hot reload con file watcher
        self.hot_reload_manager.update(delta);
        
        // Verificar cambios periódicamente cada 100ms
        if let Some(ref mut integration) = self.file_watcher_integration {
            if delta > 0.1 {
                integration.check();
            }
        }
        
        // Actualizar Explorer Panel
        self.explorer_panel_integration.update();
        
        // Conectar Explorer con Preview
        self.update_preview_from_explorer();
        
        // Actualizar Toolbar
        self.toolbar_widget.toolbar.select();
        
        // Manejar Drag & Drop
        self.handle_drag_drop(delta);
    }
    
    /// Maneja Drag & Drop de archivos
    fn handle_drag_drop(&mut self, delta: f32) {
        // Inicializar sistema de Drag & Drop si es necesario
        if self.drag_drop_system.drag_data.is_none() {
            return;
        }
        
        // Verificar si el mouse fue soltado
        if egui::epaint::PointerButton::Middle.released() {
            // Procesar drop en la ruta del proyecto
            let viewport_path = self.get_viewport_drop_path();
            self.drag_drop_system.end_drag(viewport_path);
        }
    }
    
    /// Obtiene la ruta de drop en el viewport
    fn get_viewport_drop_path(&self) -> Option<std::path::PathBuf> {
        // Usar la ruta del proyecto actual como destino
        self.explorer_panel_integration.current_project_path.clone()
    }
    
    /// Conecta Explorer Panel con Preview Panel
    pub fn update_preview_from_explorer(&mut self) {
        if let Some(path) = self.explorer_panel_integration.get_selected_path() {
            // Agregar asset al preview panel
            let asset_type = crate::preview_panel::AssetInfo::detect_type(&path);
            let asset = crate::preview_panel::AssetInfo::new(
                path.file_name().and_then(|n| n.to_str()).unwrap_or("Unknown").to_string(),
                path,
                asset_type
            );
            self.preview_panel.add_asset(asset);
            
            // Seleccionar el último asset agregado
            if let Some(last_asset) = self.preview_panel.available_assets.last() {
                let index = self.preview_panel.available_assets.len() - 1;
                self.preview_panel.select_asset(index);
            }
            
            self.log(format!("📁 Preview set: {}", path.display()));
        }
    }
    
    /// Conecta Property Panel con Explorer Panel
    pub fn update_explorer_from_property(&mut self) {
        // Si hay entidad seleccionada en Property Panel, mostrar en Explorer
        if let Some(_) = self.property_panel.get_selected_entity() {
            // Actualizar Explorer para mostrar entidad
            self.explorer_panel_integration.update();
        }
    }
    
    /// Actualiza todos los panels integrados
    pub fn update_all_panels(&mut self, delta: f32) {
        // Actualizar Explorer Panel
        self.explorer_panel_integration.update();
        
        // Actualizar Toolbar
        self.toolbar_widget.toolbar.select();
        
        // Actualizar Preview desde Explorer
        self.update_preview_from_explorer();
        
        // Actualizar Explorer desde Property
        self.update_explorer_from_property();
    }
}

impl App for ForgeEditorApp {
    fn update(&mut self, ctx: &egui::Context, _ui: &mut eframe::Frame) {
        // Renderizar Toolbar superior
        self.toolbar_widget.render(ctx);
        
        // Renderizar Export/Import Panel (panel izquierdo)
        self.export_import_panel.ui(ctx, &mut egui::SidePanel::left("export_import_panel")
            .min_width(250.0).show(ctx, |ui| {
                ui.heading("Export / Import");
            }));
        
        // Renderizar Preview Panel (panel derecho)
        self.preview_panel.ui(ctx, &mut egui::SidePanel::right("preview_panel")
            .min_width(300.0).show(ctx, |ui| {
                ui.heading("Preview Panel");
            }));
        
        // Renderizar Event Forge (panel superior derecho)
        self.render_event_forge(ctx);
        
        // Explorer Panel (panel inferior izquierdo)
        self.explorer_panel_integration.ui(ctx, &mut egui::SidePanel::bottom("explorer_panel")
            .min_height(150.0).show(ctx, |ui| {
                ui.heading("Explorer Panel");
            }));
        
        egui::CentralPanel::default().show(ctx, |ui| {
            // Play Control superior
            ui.horizontal(|ui| {
                // Botón Play
                if ui.button("▶ Play").clicked() {
                    if let Some(entities) = self.get_entities() {
                        self.start_play(&entities);
                    }
                }
                
                // Botón Stop
                if ui.button("⏹ Stop").clicked() && self.is_playing {
                    self.stop_play();
                }
                
                // Estado
                ui.label(if self.is_playing { "▶ Playing" } else { "⏹ Stopped" });
            });
            
            ui.separator();
            
            // Viewport de renderizado
            ui.label("Viewport - Forge SDK Integration");
            ui.label(format!("Entidades: {}", self.get_entities().map(|e| e.len()).unwrap_or(0)));
            ui.label(format!("Estado: {}", if self.is_playing { "▶ Playing" } else { "⏹ Stopped" }));
            
            ui.separator();
            
            // Log
            egui::ScrollArea::vertical().show(ui, |ui| {
                for log in &self.logs {
                    ui.label(log.clone());
                }
            });
            
            ui.separator();
            
            // Hot Reload Panel
            self.hot_reload_panel.ui(ctx, &mut self.hot_reload_manager);
            
            // Property Panel (panel derecho)
            self.property_panel.ui(ctx, ui);
        });
    }
}

impl ForgeEditorApp {
    /// Obtiene las entidades actuales
    fn get_entities(&self) -> Option<Vec<Entity>> {
        if let Some(ref _session) = self.play_session {
            Some(vec![])
        } else {
            Some(vec![])
        }
    }
}
