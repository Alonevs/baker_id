use std::collections::HashMap;
use serde::{Serialize, Deserialize};

/// Tipo de interpolación
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum InterpolationType {
    Linear,
    EaseIn,
    EaseOut,
    EaseInOut,
}

/// Keyframe para una animación
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Keyframe<T> {
    pub time: f32,
    pub value: T,
}

/// Track de animación (ej: posición X, escala Y, etc.)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnimationTrack<T> {
    pub name: String,
    pub interpolation: InterpolationType,
    pub keyframes: Vec<Keyframe<T>>,
}

impl<T: Serialize + Deserialize<'static>> Default for AnimationTrack<T> {
    fn default() -> Self {
        Self {
            name: "default".to_string(),
            interpolation: InterpolationType::Linear,
            keyframes: vec![],
        }
    }
}

/// Animación 2D completa
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Animation2D {
    pub name: String,
    pub duration: f32, // en segundos
    pub loops: i32, // -1 = infinito
    pub tracks: Vec<AnimationTrack<f32>>,
}

impl Default for Animation2D {
    fn default() -> Self {
        Self {
            name: "default".to_string(),
            duration: 2.0,
            loops: -1,
            tracks: vec![
                AnimationTrack::default(), // Position X
                AnimationTrack::default(), // Position Y
                AnimationTrack::default(), // Scale X
                AnimationTrack::default(), // Scale Y
                AnimationTrack::default(), // Rotation
            ],
        }
    }
}

/// Motor de animaciones 2D (Simplificado - sin Mutex)
#[derive(Debug, Clone)]
pub struct Animation2DManager {
    pub animations: HashMap<String, Animation2D>,
    pub current_time: f32,
    pub paused: bool,
}

impl Default for Animation2DManager {
    fn default() -> Self {
        Self {
            animations: HashMap::new(),
            current_time: 0.0,
            paused: false,
        }
    }
}

impl Animation2DManager {
    /// Crear una nueva animación
    pub fn create_animation(&mut self, name: &str, duration: f32) -> Option<&mut Animation2D> {
        let animation = Animation2D {
            name: name.to_string(),
            duration,
            loops: -1,
            tracks: vec![
                AnimationTrack { name: "position_x".to_string(), interpolation: InterpolationType::Linear, keyframes: vec![] },
                AnimationTrack { name: "position_y".to_string(), interpolation: InterpolationType::Linear, keyframes: vec![] },
                AnimationTrack { name: "scale_x".to_string(), interpolation: InterpolationType::Linear, keyframes: vec![] },
                AnimationTrack { name: "scale_y".to_string(), interpolation: InterpolationType::Linear, keyframes: vec![] },
                AnimationTrack { name: "rotation".to_string(), interpolation: InterpolationType::Linear, keyframes: vec![] },
            ],
        };
        
        self.animations.insert(name.to_string(), animation);
        self.animations.get_mut(name)
    }

    /// Obtener referencia a una animación
    pub fn get_animation(&self, name: &str) -> Option<&Animation2D> {
        self.animations.get(name)
    }

    /// Obtener referencia mutada a una animación
    pub fn get_animation_mut(&mut self, name: &str) -> Option<&mut Animation2D> {
        self.animations.get_mut(name)
    }

    /// Añadir keyframe a una track
    pub fn add_keyframe(&mut self, animation_name: &str, track_name: &str, time: f32, value: f32) {
        if let Some(animation) = self.animations.get_mut(animation_name) {
            for track in animation.tracks.iter_mut() {
                if track.name == track_name {
                    track.keyframes.push(Keyframe { time, value });
                    break;
                }
            }
        }
    }

    /// Remover keyframe de una track
    pub fn remove_keyframe(&mut self, animation_name: &str, track_name: &str, time: f32) {
        if let Some(animation) = self.animations.get_mut(animation_name) {
            for track in animation.tracks.iter_mut() {
                if track.name == track_name {
                    track.keyframes.retain(|k| k.time != time);
                    break;
                }
            }
        }
    }

    /// Obtener valor interpolado en un tiempo dado
    pub fn interpolate(&self, animation_name: &str, track_name: &str, time: f32) -> Option<f32> {
        if let Some(animation) = self.animations.get(animation_name) {
            for track in animation.tracks.iter() {
                if track.name == track_name {
                    return self.interpolate_track(track, time);
                }
            }
        }
        None
    }

    /// Interpolación de una track
    fn interpolate_track(&self, track: &AnimationTrack<f32>, time: f32) -> Option<f32> {
        if track.keyframes.is_empty() {
            return Some(0.0);
        }

        // Buscar keyframes circundantes
        let keyframes: Vec<&Keyframe<f32>> = track.keyframes.iter().collect();
        if keyframes.len() < 2 {
            return Some(keyframes.first()?.value);
        }

        let prev_keyframe = keyframes.iter().rfind(|k| k.time <= time);
        let next_keyframe = keyframes.iter().find(|k| k.time >= time);

        match (prev_keyframe, next_keyframe) {
            (Some(prev), Some(key)) => {
                if key.time == prev.time {
                    return Some(key.value);
                }

                let t = (time - prev.time) / (key.time - prev.time);
                let value = match track.interpolation {
                    InterpolationType::Linear => {
                        prev.value + (key.value - prev.value) * t
                    }
                    InterpolationType::EaseIn => {
                        let ease = t * t * (3.0 - 2.0 * t);
                        prev.value + (key.value - prev.value) * ease
                    }
                    InterpolationType::EaseOut => {
                        let ease = 1.0 - ((1.0 - t) * (1.0 - t) * (3.0 - 2.0 * (1.0 - t)));
                        prev.value + (key.value - prev.value) * ease
                    }
                    InterpolationType::EaseInOut => {
                        let t_half = if t < 0.5 { 2.0 * t } else { 2.0 * t - 1.0 };
                        let ease = if t < 0.5 {
                            0.5 * t_half * t_half * (15.0 - 6.0 * t_half)
                        } else {
                            0.5 * (t_half * t_half * (-6.0 + 9.0 * t_half) - 7.0) + 1.0
                        };
                        prev.value + (key.value - prev.value) * ease
                    }
                };

                Some(value)
            }
            _ => {
                let last = track.keyframes.last()?;
                Some(last.value)
            }
        }
    }

    /// Actualizar tiempo de animación
    pub fn update(&mut self, dt: f32) {
        if self.paused {
            return;
        }

        self.current_time += dt;

        for animation in self.animations.values() {
            if animation.loops < 0 {
                // Loop infinito
                self.current_time = self.current_time % animation.duration;
            } else {
                // Loop limitado
                let elapsed = self.current_time % animation.duration;
                if elapsed >= animation.duration {
                    self.current_time = 0.0;
                }
            }
        }
    }

    /// Resetear tiempo de animación
    pub fn reset(&mut self) {
        self.current_time = 0.0;
    }

    /// Exportar a formato .map
    pub fn export_to_map(&self) -> String {
        let mut content = String::new();
        
        content.push_str("ANIMATIONS_DATA\n");
        content.push_str(format!("ANIMATION_COUNT: {}\n", self.animations.len()).as_str());
        
        for (name, animation) in self.animations.iter() {
            content.push_str(&format!("ANIMATION:{},{},{}\n", name, animation.duration, animation.loops));
            
            for track in animation.tracks.iter() {
                content.push_str(&format!("TRACK:{},{}\n", track.name, track.keyframes.len()));
                
                for keyframe in track.keyframes.iter() {
                    content.push_str(&format!("KEYFRAME:{},{}\n", keyframe.time, keyframe.value));
                }
            }
        }
        
        content
    }

    /// Importar desde formato .map (Guardando las animaciones en el mapa al final)
    pub fn import_from_map(&mut self, content: &str) {
        let lines: Vec<&str> = content.lines().collect();
        let mut current_animation: Option<Animation2D> = None;
        
        for line in lines {
            if line.starts_with("ANIMATION:") {
                // Guardar la animación anterior completada
                if let Some(anim) = current_animation.take() {
                    self.animations.insert(anim.name.clone(), anim);
                }
                
                let parts: Vec<&str> = line.split(',').collect();
                if parts.len() >= 3 {
                    let name = parts[1].trim().to_string();
                    let duration: f32 = parts[2].trim().parse().unwrap_or(2.0);
                    let loops: i32 = parts[3].trim().parse().unwrap_or(-1);
                    
                    current_animation = Some(Animation2D {
                        name,
                        duration,
                        loops,
                        tracks: vec![
                            AnimationTrack { name: "position_x".to_string(), interpolation: InterpolationType::Linear, keyframes: vec![] },
                            AnimationTrack { name: "position_y".to_string(), interpolation: InterpolationType::Linear, keyframes: vec![] },
                            AnimationTrack { name: "scale_x".to_string(), interpolation: InterpolationType::Linear, keyframes: vec![] },
                            AnimationTrack { name: "scale_y".to_string(), interpolation: InterpolationType::Linear, keyframes: vec![] },
                            AnimationTrack { name: "rotation".to_string(), interpolation: InterpolationType::Linear, keyframes: vec![] },
                        ],
                    });
                }
            } else if line.starts_with("TRACK:") {
                let parts: Vec<&str> = line.split(',').collect();
                if parts.len() >= 2 && current_animation.is_some() {
                    let track_name = parts[1].trim().to_string();
                    if let Some(animation) = current_animation.as_mut() {
                        for track in animation.tracks.iter_mut() {
                            if track.name == track_name {
                                break;
                            }
                        }
                    }
                }
            } else if line.starts_with("KEYFRAME:") {
                let parts: Vec<&str> = line.split(',').collect();
                if parts.len() >= 3 && current_animation.is_some() {
                    let time: f32 = parts[1].trim().parse().unwrap_or(0.0);
                    let value: f32 = parts[2].trim().parse().unwrap_or(0.0);
                    
                    if let Some(animation) = current_animation.as_mut() {
                        for track in animation.tracks.iter_mut() {
                            track.keyframes.push(Keyframe { time, value });
                        }
                    }
                }
            }
        }
        
        // Guardar la última animación leída
        if let Some(anim) = current_animation.take() {
            self.animations.insert(anim.name.clone(), anim);
        }
    }
}

/// Utilidades matemáticas
pub mod math {
    pub fn lerp(a: f32, b: f32, t: f32) -> f32 {
        a + (b - a) * t
    }
    
    pub fn ease_in(t: f32) -> f32 {
        t * t * (3.0 - 2.0 * t)
    }
    
    pub fn ease_out(t: f32) -> f32 {
        1.0 - ((1.0 - t) * (1.0 - t) * (3.0 - 2.0 * (1.0 - t)))
    }
    
    pub fn ease_in_out(t: f32) -> f32 {
        if t < 0.5 {
            2.0 * t * t * (15.0 - 6.0 * t) / 2.0
        } else {
            let t_half = t - 0.5;
            0.5 * (t_half * t_half * (-6.0 + 9.0 * t_half) - 7.0) + 1.0
        }
    }
}












