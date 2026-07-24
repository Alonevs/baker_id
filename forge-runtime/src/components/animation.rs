//! Componente de Animación 2D para entidades

use std::collections::HashMap;
use uuid::Uuid;
use forge_animation::{Animation, AnimationState, LoopMode, InterpolationType, Keyframe, Transform};

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
    pub animations: HashMap<String, Animation>,
    pub state: AnimationState,
    pub loop_mode: LoopMode,
}

impl Default for AnimationComponent {
    fn default() -> Self {
        Self {
            animations: HashMap::new(),
            state: AnimationState::default(),
            loop_mode: LoopMode::Loop,
        }
    }
}

impl AnimationComponent {
    /// Crea una nueva instancia de AnimationComponent
    pub fn new() -> Self {
        Self::default()
    }

    /// Agrega una animación al componente
    pub fn add_animation(&mut self, name: &str, animation: Animation) {
        self.animations.insert(name.to_string(), animation);
    }

    /// Obtiene una animación por nombre
    pub fn get_animation(&self, name: &str) -> Option<&Animation> {
        self.animations.get(name)
    }

    /// Obtene una animación mutada por nombre
    pub fn get_animation_mut(&mut self, name: &str) -> Option<&mut Animation> {
        self.animations.get_mut(name)
    }

    /// Inicia reproducción de una animación
    pub fn play(&mut self, animation_name: &str) -> bool {
        if let Some(animation) = self.animations.get(animation_name) {
            self.state.current_animation = Some(animation.id);
            self.state.is_playing = true;
            self.state.current_time = 0.0;
            true
        } else {
            false
        }
    }

    /// Pausa la reproducción actual
    pub fn pause(&mut self) {
        self.state.is_playing = false;
    }

    /// Detiene la reproducción actual
    pub fn stop(&mut self) {
        self.state.is_playing = false;
        self.state.current_time = 0.0;
        self.state.current_animation = None;
    }

    /// Actualiza el estado de animación con delta time
    pub fn update(&mut self, dt: f32) {
        if !self.state.is_playing {
            return;
        }

        self.state.current_time += dt * self.state.playback_rate;

        // Aplicar loop
        if let Some(animation) = self.state.current_animation {
            if let Some(anim) = self.animations.get(animation) {
                if self.loop_mode == LoopMode::Loop {
                    self.state.current_time = self.state.current_time % anim.duration;
                } else if self.loop_mode == LoopMode::PingPong {
                    let cycles = (self.state.current_time / anim.duration) as i32;
                    let remainder = self.state.current_time % anim.duration;
                    if cycles % 2 == 1 {
                        self.state.current_time = anim.duration - remainder;
                    } else {
                        self.state.current_time = remainder;
                    }
                }
            }
        }

        // Calcular interpolación actual
        if let Some(animation) = self.state.current_animation {
            if let Some(anim) = self.animations.get(animation) {
                self.state.current_blend.clear();
                let blend = anim.interpolate(self.state.current_time);
                if blend.target != uuid::Uuid::nil() {
                    self.state.current_blend.insert(blend.target, blend.weight);
                }
            }
        }
    }

    /// Obtiene el transform interpolado en el tiempo actual
    pub fn get_current_transform(&self, target: Uuid) -> Transform {
        if let Some(animation) = self.state.current_animation {
            if let Some(anim) = self.animations.get(animation) {
                return anim.interpolate_transform(self.state.current_time, target);
            }
        }
        Transform::default()
    }

    /// Obtiene el valor interpolado de una propiedad
    pub fn interpolate_value(&self, property: &str, time: f32) -> f32 {
        if let Some(animation) = self.state.current_animation {
            if let Some(anim) = self.animations.get(animation) {
                // Buscar keyframe anterior y siguiente
                let keyframes: Vec<&Keyframe> = anim.keyframes.iter().collect();
                if keyframes.len() < 2 {
                    return 0.0;
                }

                let prev_keyframe = keyframes.iter().rfind(|k| k.time <= time);
                let next_keyframe = keyframes.iter().find(|k| k.time >= time);

                match (prev_keyframe, next_keyframe) {
                    (Some(prev), Some(next)) => {
                        if next.time == prev.time {
                            return prev.value;
                        }

                        let t = (time - prev.time) / (next.time - prev.time);
                        let interp_type = next.interpolation;
                        interp_type.interpolate(prev.value, next.value, t)
                    }
                    _ => 0.0,
                }
            }
        }
        0.0
    }

    /// Serializa el componente a JSON
    pub fn serialize(&self) -> String {
        serde_json::to_string_pretty(self).unwrap_or_default()
    }

    /// Deserializa desde JSON
    pub fn deserialize(data: &str) -> Self {
        serde_json::from_str(data).unwrap_or_else(|_| Self::default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_animation_component_play() {
        let mut component = AnimationComponent::new();
        let animation = Animation::new(
            Uuid::new_v4(),
            "test".to_string(),
            2.0,
            60.0,
        );
        component.add_animation("test", animation);

        assert!(component.play("test"));
        assert!(component.state.is_playing);
    }

    #[test]
    fn test_animation_component_update() {
        let mut component = AnimationComponent::new();
        let animation = Animation::new(
            Uuid::new_v4(),
            "test".to_string(),
            2.0,
            60.0,
        );
        component.add_animation("test", animation);
        component.play("test");

        component.update(0.016); // ~60 FPS
        assert!((component.state.current_time - 0.016).abs() < 0.001);
    }

    #[test]
    fn test_animation_component_loop() {
        let mut component = AnimationComponent::new();
        let animation = Animation::new(
            Uuid::new_v4(),
            "test".to_string(),
            2.0,
            60.0,
        );
        component.add_animation("test", animation);
        component.play("test");

        // Actualizar más de un ciclo
        component.update(5.0);
        assert!((component.state.current_time - 1.0).abs() < 0.001);
    }

    #[test]
    fn test_animation_component_serialize_deserialize() {
        let component = AnimationComponent::new();
        let serialized = component.serialize();
        let deserialized: AnimationComponent = serde_json::from_str(&serialized).unwrap();
        assert!(deserialized.animations.is_empty());
    }

    #[test]
    fn test_animation_component_get_animation() {
        let mut component = AnimationComponent::new();
        let animation = Animation::new(
            Uuid::new_v4(),
            "test".to_string(),
            2.0,
            60.0,
        );
        component.add_animation("test", animation);

        assert!(component.get_animation("test").is_some());
        assert!(component.get_animation("nonexistent").is_none());
    }
}
