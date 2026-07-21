//! Comportamientos de audio

use serde::{Deserialize, Serialize};

/// Comportamiento de audio en una entidad
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioBehavior {
    /// ID único del comportamiento
    pub id: String,
    /// Nombre del comportamiento
    pub name: String,
    /// Tipo de comportamiento
    pub behavior_type: AudioBehaviorType,
    /// Socket de audio asociado
    pub audio_socket: String,
    /// Estado (active, inactive)
    pub state: AudioBehaviorState,
    /// Parámetros del comportamiento
    pub parameters: serde_json::Value,
    /// Trigger (on_enter, on_exit, on_attack, etc.)
    pub trigger: AudioBehaviorTrigger,
    /// Volumen base
    pub base_volume: f32,
    /// Volumen máximo
    pub max_volume: f32,
    /// Radio de escucha
    pub listen_radius: f32,
}

/// Tipo de comportamiento de audio
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum AudioBehaviorType {
    /// Reproducir en loop (Loop)
    Loop,
    /// Reproducir una vez (OneShot)
    OneShot,
    /// Reproducir al entrar (OnEnter)
    OnEnter,
    /// Reproducir al salir (OnExit)
    OnExit,
    /// Reproducir al atacar (OnAttack)
    OnAttack,
    /// Reproducir al recibir daño (OnDamage)
    OnDamage,
    /// Reproducir al recolectar (OnCollect)
    OnCollect,
    /// Reproducir al usar item (OnUse)
    OnUse,
    /// Reproducir al abrir (OnOpen)
    OnOpen,
    /// Reproducir al cerrar (OnClose)
    OnClose,
    /// Reproducir al completar acción (OnSuccess)
    OnSuccess,
    /// Reproducir al fallar (OnError)
    OnError,
    /// Reproducir habilidad (OnSkill)
    OnSkill,
    /// Reproducir muerte (OnDeath)
    OnDeath,
}

impl Default for AudioBehaviorType {
    fn default() -> Self {
        Self::Loop
    }
}

/// Estado del comportamiento de audio
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum AudioBehaviorState {
    /// Inactivo
    Inactive,
    /// Activo
    Active,
    /// Reproduciendo
    Playing,
    /// Detenido
    Stopped,
}

impl Default for AudioBehaviorState {
    fn default() -> Self {
        Self::Inactive
    }
}

/// Trigger de audio (cuándo reproducir)
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum AudioBehaviorTrigger {
    /// Al entrar en el radio
    OnEnter,
    /// Al salir del radio
    OnExit,
    /// Al atacar
    OnAttack,
    /// Al recibir daño
    OnDamage,
    /// Al recolectar
    OnCollect,
    /// Al usar item
    OnUse,
    /// Al abrir
    OnOpen,
    /// Al cerrar
    OnClose,
    /// Al completar
    OnSuccess,
    /// Al fallar
    OnError,
    /// Al usar habilidad
    OnSkill,
    /// Al morir
    OnDeath,
}

impl Default for AudioBehaviorTrigger {
    fn default() -> Self {
        Self::OnEnter
    }
}
