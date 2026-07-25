//! Componente de Animación 2D para entidades

use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Estado de animación de una entidad
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnimationState {
    pub current_animation: Option<Uuid>,
    pub current_blend: HashMap<Uuid, f32>,
    pub playback_rate: f32,
    pub is_playing: bool,
    pub current_time: f32,
}

impl Default for AnimationState {
    fn default() -> Self {
        Self {
            current_animation: None,
            current_blend: HashMap::new(),
            playback_rate: 1.0,
            is_playing: false,
            current_time: 0.0,
        }
    }
}

/// Componente de animación para entidades
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnimationComponent {
    pub animations: HashMap<String, String>,
    pub state: AnimationState,
    pub loop_mode: String,
}

impl Default for AnimationComponent {
    fn default() -> Self {
        Self {
            animations: HashMap::new(),
            state: AnimationState::default(),
            loop_mode: String::new(),
        }
    }
}

impl AnimationComponent {
    /// Crea una nueva instancia de AnimationComponent
    pub fn new() -> Self {
        Self::default()
    }

    /// Agrega una animación al componente
    pub fn add_animation(&mut self, name: &str, animation: String) {
        self.animations.insert(name.to_string(), animation);
    }

    /// Obtiene una animación por nombre
    pub fn get_animation(&self, name: &str) -> Option<&String> {
        self.animations.get(name)
    }

    /// Obtiene una animación mutada por nombre
    pub fn get_animation_mut(&mut self, name: &str) -> Option<&mut String> {
        self.animations.get_mut(name)
    }

    /// Inicia reproducción de una animación
    pub fn play(&mut self, animation_name: &str) -> bool {
        if let Some(animation) = self.animations.get(animation_name) {
            self.state.current_animation = Some(Uuid::new_v4());
            true
        } else {
            false
        }
    }

    /// Actualiza el componente con delta time
    pub fn update(&mut self, dt: f32) {
        if self.state.is_playing {
            self.state.current_time += dt * self.state.playback_rate;
        }
    }

    /// Detiene la reproducción
    pub fn stop(&mut self) {
        self.state.is_playing = false;
    }

    /// Obtiene el transform actual (placeholder)
    pub fn get_current_transform(&self, _target: Uuid) -> String {
        String::new()
    }

    /// Obtiene los keyframes actuales (placeholder)
    pub fn get_current_keyframes(&self, _animation: &String) -> Vec<String> {
        Vec::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_animation_component_new() {
        let component = AnimationComponent::new();
        assert!(component.animations.is_empty());
        assert!(!component.state.is_playing);
    }

    #[test]
    fn test_animation_component_update() {
        let mut component = AnimationComponent::new();
        component.state.is_playing = true;
        component.state.playback_rate = 1.0;
        component.update(0.016);
        assert!((component.state.current_time - 0.016).abs() < 0.001);
    }

    #[test]
    fn test_animation_component_loop() {
        let mut component = AnimationComponent::new();
        component.state.is_playing = true;
        component.update(5.0);
        assert!((component.state.current_time - 5.0).abs() < 0.001);
    }

    #[test]
    fn test_animation_component_serialize_deserialize() {
        let component = AnimationComponent::new();
        let serialized = serde_json::to_string(&component).unwrap();
        let deserialized: AnimationComponent = serde_json::from_str(&serialized).unwrap();
        assert!(deserialized.animations.is_empty());
    }

    #[test]
    fn test_animation_component_get_animation() {
        let mut component = AnimationComponent::new();
        component.add_animation("test", "test".to_string());

        assert!(component.get_animation("test").is_some());
        assert!(component.get_animation("nonexistent").is_none());
    }
}

