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
        
        // Actualizar Explorer Panel y Toolbar
        self.explorer_panel_integration.update();
        self.toolbar_widget.toolbar.select();
    }
}

impl App for ForgeEditorApp {
    fn update(&mut self, ctx: &egui::Context, _ui: &mut eframe::Frame) {
        // Renderizar Toolbar superior
        self.toolbar_widget.render(ctx);
        
        // Renderizar Event Forge
        self.render_event_forge(ctx);
        
        egui::CentralPanel::default().show(ctx, |ui| {
            // Toolbar superior
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
            ui.label("Viewport - Renderizado en tiempo real");
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
            
            // Explorer Panel (debajo de todo)
            self.explorer_panel_integration.ui(ctx);
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
