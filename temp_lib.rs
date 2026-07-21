//! # Forge Editor
//! 
//! Editor de juego 2D con sistema de nodos, viewport, asset browser y preview de assets.

pub mod math;
use forge_scene::NodeData;

pub mod ui;
pub mod physics_2d;
pub mod particle_system;
pub mod animation_2d;
pub mod map_export;
pub mod viewport_2d;
pub mod viewport_integration;
pub mod property_panel;
pub mod property_types;
pub mod transform_editor;
pub mod component_editor;
pub mod timeline;
pub mod keyframe;
pub mod animation_track;
pub mod export_manager;
pub mod import_manager;

pub use math::*;
pub use physics_2d::{Physics2D, PhysicsBlock, ColliderType};
pub use particle_system::{ParticleSystem};
pub use animation_2d::{Animation2DManager};

pub use viewport_integration::ViewportIntegration;
pub use property_panel::{PropertyPanel, TransformProperties, ComponentProperties, ScriptProperties};
pub use property_panel::EntityId;
pub use transform_editor::TransformEditor;
pub use component_editor::ComponentEditor;
pub use keyframe::{Keyframe, KeyframeEditor};
pub use animation_track::{AnimationTrack, TrackData, InterpolationType};
pub use ui::timeline::TimelineEditor;
pub use export_manager::ExportManager;
pub use import_manager::ImportManager;
pub use map_export::ExportManager as MapExport;
pub use ui::console::LogLevel;
use serde::{Deserialize, Serialize};
use std::fs;
use std::collections::HashMap;
use std::path::Path;

pub use ui::*;
use ui::AssetType;

use egui_dock::{DockState, DockArea, Style};
use pollster::FutureExt;
use forge_scene;

/// Aplicación principal del Forge Editor
#[derive(Default)]
pub struct ForgeEditorApp {
    /// Escena actual
    pub scene: forge_scene::Scene,
    
    /// Assets cargados
    pub assets: HashMap<String, forge_scene::Asset>,
    
    /// Preview de asset seleccionado
    pub asset_preview: AssetPreview,
    
    /// Estado de la ventana
    pub window_pos: egui::Pos2,
    pub window_size: egui::Vec2,
    
    /// Dock layout
    pub dock_state: DockState,
    
    /// Timeline actual
    pub current_timeline: Option<String>,
    
    /// Zoom del viewport
    pub viewport_zoom: f32,
    
    /// Offset de cámara
    pub camera_offset: forge_scene::Vec2,
    
    /// Drag operation
    pub drag_operation: Option<DragOperation>,
    
    /// Timeline playing
    pub timeline_playing: bool,
    
    /// Timeline current time
    pub timeline_time: f32,
    
    /// Audio playing
    pub audio_playing: bool,
    
    /// Physics manager
    pub physics: Physics2D,
    
    /// Particle system
    pub particles: particle_system::ParticleSystem,
    
    /// Animation manager
    pub animation: animation_2d::Animation2DManager,
    
    /// Export manager
    pub export_mgr: ExportManager,
    
    /// Import manager
    pub import_mgr: ImportManager,
}

impl ForgeEditorApp {
    /// Crea una nueva instancia de la aplicación
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Carga una escena desde un archivo
    pub fn load_scene(&mut self, path: &str) -> Result<(), String> {
        let content = fs::read_to_string(path)
            .map_err(|e| format!("Error leyendo archivo: {}", e))?;
        
        let scene: forge_scene::Scene = serde_json::from_str(&content)
            .map_err(|e| format!("Error parseando JSON: {}", e))?;
        
        self.scene = scene;
        Ok(())
    }
    
    /// Guarda la escena actual
    pub fn save_scene(&self, path: &str) -> Result<(), String> {
        let content = serde_json::to_string_pretty(&self.scene)
            .map_err(|e| format!("Error serializando JSON: {}", e))?;
        
        fs::write(path, content)
            .map_err(|e| format!("Error escribiendo archivo: {}", e))?;
        
        Ok(())
    }
}
