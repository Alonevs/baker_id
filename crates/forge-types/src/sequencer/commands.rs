//! Tipos de pistas y comandos del secuenciador

use serde::{Deserialize, Serialize};

/// Señal de comando para el juego
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CommandSignal {
    /// Tipo de comando
    pub command_type: CommandType,
    /// Parámetros del comando
    pub parameters: serde_json::Value,
}

/// Tipos de comandos disponibles
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CommandType {
    /// Dar item al jugador
    GiveItem,
    /// Cargar mapa nuevo
    LoadMap,
    /// Teletransportar actor
    Teleport,
    /// Cambiar animación
    ChangeAnimation,
    /// Activar trigger
    ActivateTrigger,
    /// Reproducir diálogo
    PlayDialogue,
    /// Lanzar efecto de pantalla
    ScreenEffect,
    /// Cambiar cámara
    CameraEffect,
    /// Guardar partida
    SaveGame,
    /// Reiniciar checkpoint
    ReloadCheckpoint,
}

/// Efectos de cámara disponibles
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CameraEffectType {
    /// Movimiento Lerp suave
    Lerp,
    /// Zoom suave
    Zoom,
    /// Sacudida de pantalla
    ScreenShake,
    /// Fundido a negro
    FadeToBlack,
    /// Fundido a blanco
    FadeToWhite,
    /// Flash blanco
    Flash,
}

/// Efectos visuales de pantalla
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum VFXType {
    /// Fundido
    Fade,
    /// Filtro de color
    ColorFilter,
    /// Partículas de clima (lluvia, hojas)
    WeatherParticles,
    /// Efecto de desenfoque
    Blur,
    /// Efecto CRT retro
    CRT,
}

/// Texto superpuesto en pantalla
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextOverlay {
    /// Contenido del texto
    pub content: String,
    /// Posición en pantalla
    pub position: (f32, f32),
    /// Efecto tipográfico
    pub effect: TextEffect,
    /// Duración en segundos
    pub duration: f32,
}

/// Efectos tipográficos
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum TextEffect {
    /// Máquina de escribir
    Typewriter,
    /// Desvanecimiento
    FadeIn,
    /// Desvanecimiento
    FadeOut,
    /// Slide desde abajo
    SlideUp,
    /// Slide desde arriba
    SlideDown,
    /// Slide desde izquierda
    SlideLeft,
    /// Slide desde derecha
    SlideRight,
}

impl Default for TextEffect {
    fn default() -> Self {
        Self::FadeIn
    }
}
