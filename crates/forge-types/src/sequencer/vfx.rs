//! Efectos visuales de pantalla (Screen VFX)

use serde::{Deserialize, Serialize};

/// Efecto visual de pantalla
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScreenVFX {
    /// Tipo de efecto
    pub vfx_type: VFXType,
    /// Parámetros del efecto
    pub parameters: serde_json::Value,
    /// Duración en segundos
    pub duration: f32,
}

/// Tipos de efectos visuales
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum VFXType {
    /// Fundido a negro
    FadeToBlack,
    /// Fundido a blanco
    FadeToWhite,
    /// Filtro sepia
    Sepia,
    /// Filtro blanco y negro
    BlackAndWhite,
    /// Filtro de contraste
    HighContrast,
    /// Lluvia
    Rain,
    /// Hojas cayendo
    FallingLeaves,
    /// Niebla
    Fog,
    /// Desenfoque
    Blur,
    /// Efecto CRT retro
    CRT,
}

impl Default for VFXType {
    fn default() -> Self {
        Self::FadeToBlack
    }
}

/// Efectos de cámara
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CameraEffect {
    /// Tipo de efecto
    pub effect_type: CameraEffectType,
    /// Intensidad
    pub intensity: f32,
    /// Duración
    pub duration: f32,
}

/// Tipos de efectos de cámara
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum CameraEffectType {
    /// Zoom in
    ZoomIn,
    /// Zoom out
    ZoomOut,
    /// Shake
    Shake,
    /// Vignette
    Vignette,
    /// Blur
    Blur,
    /// Focus
    Focus,
}

impl Default for CameraEffectType {
    fn default() -> Self {
        Self::ZoomIn
    }
}

/// Overlay de texto en pantalla
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextOverlay {
    /// Texto
    pub text: String,
    /// Posición
    pub position: (f32, f32),
    /// Color
    pub color: (f32, f32, f32, f32),
    /// Duración
    pub duration: f32,
}

/// Efectos de texto
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum TextEffect {
    /// Fade in
    FadeIn,
    /// Fade out
    FadeOut,
    /// Typewriter
    Typewriter,
    /// Glitch
    Glitch,
}

impl Default for TextEffect {
    fn default() -> Self {
        Self::FadeIn
    }
}
