//! Menú de Pausa y Opciones de Sistema

use serde::{Deserialize, Serialize};

/// Estado del menú de pausa
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PauseMenuState {
    pub is_paused: bool,
    pub last_state: AppState,
    pub savepoint_id: Option<String>,
    pub audio_settings: AudioSettings,
    pub video_settings: VideoSettings,
}

impl Default for PauseMenuState {
    fn default() -> Self {
        Self {
            is_paused: false,
            last_state: AppState::Exploration,
            savepoint_id: None,
            audio_settings: AudioSettings::default(),
            video_settings: VideoSettings::default(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioSettings {
    pub bgm_volume: f32,
    pub sfx_volume: f32,
    pub dialogue_volume: f32,
    pub master_volume: f32,
}

impl Default for AudioSettings {
    fn default() -> Self {
        Self {
            bgm_volume: 0.8,
            sfx_volume: 0.7,
            dialogue_volume: 1.0,
            master_volume: 1.0,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoSettings {
    pub brightness: f32,
    pub contrast: f32,
    pub saturation: f32,
    pub screen_shake_intensity: f32,
}

impl Default for VideoSettings {
    fn default() -> Self {
        Self {
            brightness: 1.0,
            contrast: 1.0,
            saturation: 1.0,
            screen_shake_intensity: 0.0,
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum AppState {
    /// Estado de exploración activa
    Exploration,
    /// Estado de cinemática
    Cinematic,
    /// Estado de menú de pausa
    PauseMenu,
    /// Estado de título
    Title,
    /// Estado de juego (tiempo activo)
    ActiveTime,
}

/// Acción del menú de pausa
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PauseAction {
    /// Reanudar juego
    Resume,
    /// Reiniciar desde checkpoint
    ReloadSavepoint,
    /// Abrir ajustes de audio
    OpenAudioSettings,
    /// Abrir ajustes de video
    OpenVideoSettings,
    /// Salir al título
    QuitToTitle,
}

/// Opciones personalizadas para ventanas UI
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomWindowConfig {
    pub window_id: String,
    pub call_button: Option<CallButton>,
    pub actions: Vec<WindowAction>,
    pub style: WindowStyle,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CallButton {
    pub button_name: String,
    pub action: WindowActionType,
    pub hotkey: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WindowActionType {
    ResumeGame,
    ReloadCheckpoint,
    OpenSettings,
    QuitToTitle,
    Custom(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WindowAction {
    Button { button_name: String, action: WindowActionType },
    WindowStyle { window_id: String, style: WindowStyle },
    Text { text: String, position: (f32, f32) },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WindowStyle {
    pub border_type: BorderType,
    pub font: Option<String>,
    pub text_color: [u8; 4],
    pub highlight_color: [u8; 4],
    pub line_spacing: f32,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum BorderType {
    NineSlice { top_left: String, top_right: String, bottom_left: String, bottom_right: String },
    Solid { color: [u8; 4] },
    Transparent,
}

/// Sistema de pausado de simulación
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PauseSystem {
    pub global_pause_enabled: bool,
    pub pause_key: Option<KeyCode>,
    pub active_time_menus: Vec<String>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum KeyCode {
    Start,
    Select,
    ButtonY,
    ButtonX,
    ButtonL,
    ButtonR,
}
