//! UI del editor

use crate::play_session::{PlaySession, Entity};
use crate::snapshot_manager::SnapshotManager;
use crate::timeline_integration::TimelineIntegration;
use crate::input_capture::UserInput;
use eframe::egui;
use eframe::App;

/// DragOperation para UI
#[derive(Debug, Clone, PartialEq)]
pub enum DragOperation {
    None,
    Move,
    Resize,
}

/// App principal del editor
pub struct ForgeEditorApp {
    pub logs: Vec<String>,
    pub timeline_integration: Option<TimelineIntegration>,
    pub play_session: Option<PlaySession>,
    pub snapshot_manager: SnapshotManager,
    pub user_input: UserInput,
    pub is_playing: bool,
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
        }
    }
}

impl ForgeEditorApp {
    pub fn new(_cc: &eframe::CreationContext) -> Self {
        Self::default()
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
    }
}

impl App for ForgeEditorApp {
    fn update(&mut self, ctx: &egui::Context, _ui: &mut eframe::Frame) {
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
            
            // Contenido principal
            ui.label("Forge Editor - Content Area");
            
            // Log
            egui::ScrollArea::vertical().show(ui, |ui| {
                for log in &self.logs {
                    ui.label(log.clone());
                }
            });
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
