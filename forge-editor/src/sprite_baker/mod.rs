use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Configuración del Sprite Baker
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpriteBakerSettings {
    /// Estado del modelo
    pub model_path: Option<String>,
    pub model_scale: f32,
    
    /// Estado de animaciones
    pub available_animations: Vec<String>,
    pub selected_animation: Option<String>,
    pub selected_animations_to_bake: HashMap<String, bool>,
    pub is_playing: bool,
    pub current_time: f32,
    pub animation_speed: f32,
    pub selected_animation_duration: f32,
    
    /// Estado de la cámara (ángulos de Euler y límites)
    pub camera_yaw: f32,
    pub camera_pitch: f32,
    pub camera_distance: f32,
    pub camera_height: f32,
    pub camera_target_x: f32,
    pub camera_target_z: f32,
    pub use_orthographic: bool,
    pub ortho_scale: f32,
    
    /// Estado de exportación
    pub export_id_asset: String,
    pub export_is_ropero: bool,
    pub export_directions: usize,
    pub export_resolution: u32,
    pub export_frames_mode: FramesMode,
    pub export_frames_count: usize,
    pub export_fps: u32,
    pub export_project_path: String,
    
    /// Visor / Previewer 2D
    pub preview_frames: Vec<String>,
    pub preview_frame_idx: usize,
    pub preview_timer: f32,
    pub preview_is_playing: bool,
    pub preview_fps: f32,
    pub preview_library_path: String,
    
    /// Máquina de estados del Baker
    pub bake_state: BakeState,
    
    /// Sistema de iluminación (headlight effect)
    pub lighting_enabled: bool,
}

impl Default for SpriteBakerSettings {
    fn default() -> Self {
        Self {
            model_path: None,
            model_scale: 1.0,
            available_animations: Vec::new(),
            selected_animation: None,
            selected_animations_to_bake: HashMap::new(),
            is_playing: false,
            current_time: 0.0,
            animation_speed: 1.0,
            selected_animation_duration: 2.0,
            camera_yaw: 0.0,
            camera_pitch: 0.0,
            camera_distance: 5.0,
            camera_height: 0.0,
            camera_target_x: 0.0,
            camera_target_z: 0.0,
            use_orthographic: false,
            ortho_scale: 1.0,
            export_id_asset: String::new(),
            export_is_ropero: true,
            export_directions: 8,
            export_resolution: 256,
            export_frames_mode: FramesMode::All,
            export_frames_count: 1,
            export_fps: 30,
            export_project_path: String::new(),
            preview_frames: Vec::new(),
            preview_frame_idx: 0,
            preview_timer: 0.0,
            preview_is_playing: false,
            preview_fps: 30.0,
            preview_library_path: String::new(),
            bake_state: BakeState::Idle,
            lighting_enabled: false,
        }
    }
}

/// Máquina de Estados del Bake
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum BakeState {
    Idle,
    Preparing,
    Capturing {
        animation_idx: usize,
        direction_idx: usize,
        frame_idx: usize,
        wait_frames: u32,
        anim_name: String,
        duration: f32,
        total_frames: usize,
        active_bakes: Vec<String>,
        view_suffix: String,
    },
    Finished {
        message: String,
    },
}

/// Modo de exportación de fotogramas
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum FramesMode {
    All,
    Specific(usize),
}

/// Estado de carga del modelo
#[derive(Debug, Clone, PartialEq)]
pub enum ModelLoadState {
    Idle,
    Loading,
    Loaded,
    Failed(String),
}

