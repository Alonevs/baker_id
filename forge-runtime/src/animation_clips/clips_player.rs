//! Animation Clips Player - Reproductor de clips de animación

use crate::animation::{Keyframe, LoopMode};
use crate::animation_clips::AnimationClip;
use std::collections::HashMap;

/// Estado del reproductor de clips
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ClipsPlayerState {
    /// Nombre del clip actual
    pub current_clip: Option<String>,
    /// Tiempo actual en frames
    pub current_frame: u32,
    /// Duración total del clip
    pub duration: u32,
    /// Modo de loop
    pub loop_mode: LoopMode,
    /// Reproduciendo
    pub is_playing: bool,
    /// Velocidad de reproducción (frames por segundo)
    pub playback_speed: f32,
}

impl Default for ClipsPlayerState {
    fn default() -> Self {
        Self {
            current_clip: None,
            current_frame: 0,
            duration: 0,
            loop_mode: LoopMode::Once,
            is_playing: false,
            playback_speed: 30.0, // 30 FPS por defecto
        }
    }
}

/// Reproductor de clips de animación
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ClipsPlayer {
    /// Estado del reproductor
    pub state: ClipsPlayerState,
    /// Keyframes por propiedad
    pub keyframes: HashMap<String, Vec<Keyframe>>,
    /// Clips cargados
    pub loaded_clips: HashMap<String, AnimationClip>,
}

impl Default for ClipsPlayer {
    fn default() -> Self {
        Self {
            state: ClipsPlayerState::default(),
            keyframes: HashMap::new(),
            loaded_clips: HashMap::new(),
        }
    }
}

impl ClipsPlayer {
    /// Crea un nuevo reproductor de clips
    pub fn new() -> Self {
        Self::default()
    }

    /// Carga un clip de animación
    pub fn load_clip(&mut self, clip: AnimationClip) -> bool {
        if clip.duration == 0 {
            return false;
        }
        
        self.loaded_clips.insert(clip.name.clone(), clip.clone());
        self.state.current_clip = Some(clip.name.clone());
        self.state.duration = clip.duration;
        self.state.current_frame = 0;
        self.state.is_playing = false;
        self.state.loop_mode = clip.loop_mode;
        
        true
    }

    /// Obtiene el clip actual
    pub fn get_current_clip(&self) -> Option<&AnimationClip> {
        self.state.current_clip.as_ref().and_then(|name| self.loaded_clips.get(name))
    }

    /// Obtiene el clip mutado actual
    pub fn get_current_clip_mut(&mut self) -> Option<&mut AnimationClip> {
        self.state.current_clip.as_ref().and_then(|name| self.loaded_clips.get_mut(name))
    }

    /// Obtiene los keyframes para una propiedad
    pub fn get_keyframes(&self, property: &str) -> Option<&Vec<Keyframe>> {
        self.keyframes.get(property)
    }

    /// Obtiene los keyframes mutados para una propiedad
    pub fn get_keyframes_mut(&mut self, property: &str) -> Option<&mut Vec<Keyframe>> {
        self.keyframes.get_mut(property)
    }

    /// Agrega keyframes para una propiedad
    pub fn add_keyframes(&mut self, property: &str, keyframes: Vec<Keyframe>) {
        self.keyframes.insert(property.to_string(), keyframes);
    }

    /// Inicia la reproducción
    pub fn play(&mut self) {
        self.state.is_playing = true;
        self.state.current_frame = 0;
    }

    /// Pausa la reproducción
    pub fn pause(&mut self) {
        self.state.is_playing = false;
    }

    /// Detiene la reproducción y va al inicio
    pub fn stop(&mut self) {
        self.state.is_playing = false;
        self.state.current_frame = 0;
    }

    /// Actualiza el reproductor con delta time
    pub fn update(&mut self, dt: f32) {
        if !self.state.is_playing {
            return;
        }

        // Calcular avance de frames basado en velocidad y delta time
        let frames_per_second = self.state.playback_speed;
        let frame_delta = (dt * frames_per_second) as u32;
        
        eprintln!("UPDATE: dt={}, fps={}, delta={}, frame={}, duration={}, mode={:?}", 
            dt, frames_per_second, frame_delta, self.state.current_frame, self.state.duration, self.state.loop_mode);
        
        self.state.current_frame = self.state.current_frame.saturating_add(frame_delta);

        eprintln!("AFTER FRAME: frame={}, duration={}, mode={:?}", 
            self.state.current_frame, self.state.duration, self.state.loop_mode);

        // Manejo de loop mode
        if self.state.current_frame >= self.state.duration {
            eprintln!("LOOP HANDLING: frame={}, duration={}, mode={:?}", 
                self.state.current_frame, self.state.duration, self.state.loop_mode);
            match self.state.loop_mode {
                LoopMode::Once => {
                    eprintln!("  -> LoopMode::Once: stopping");
                    self.state.is_playing = false;
                    // Dejamos el frame en la última posición
                }
                LoopMode::Loop => {
                    eprintln!("  -> LoopMode::Loop: resetting to 0");
                    self.state.current_frame = 0;
                    // Continúa reproduciendo
                }
                LoopMode::PingPong => {
                    eprintln!("  -> LoopMode::PingPong: bouncing back to 0");
                    // En ping pong, cuando alcanzamos el final, rebotamos al inicio
                    // Esto permite que el clip se reproduzca de nuevo en sentido inverso
                    self.state.current_frame = 0;
                    self.state.is_playing = true;
                }
            }
        }
        eprintln!("END UPDATE: frame={}, mode={:?}", self.state.current_frame, self.state.loop_mode);
    }

    /// Obtiene el tiempo actual en segundos
    pub fn get_current_time(&self) -> f32 {
        self.state.current_frame as f32 / self.state.playback_speed
    }

    /// Obtiene el tiempo restante en segundos
    pub fn get_remaining_time(&self) -> f32 {
        (self.state.duration - self.state.current_frame) as f32 / self.state.playback_speed
    }

    /// Salta al frame especificado
    pub fn set_frame(&mut self, frame: u32) {
        if let Some(clip) = self.get_current_clip() {
            self.state.current_frame = frame.min(clip.duration);
        }
    }

    /// Siguiente frame
    pub fn next_frame(&mut self) {
        if let Some(clip) = self.get_current_clip() {
            self.state.current_frame = (self.state.current_frame + 1).min(clip.duration);
        }
    }

    /// Frame anterior
    pub fn prev_frame(&mut self) {
        self.state.current_frame = self.state.current_frame.saturating_sub(1);
    }

    /// Establece la velocidad de reproducción
    pub fn set_playback_speed(&mut self, speed: f32) {
        self.state.playback_speed = speed.max(1.0);
    }

    /// Obtiene la velocidad de reproducción actual
    pub fn get_playback_speed(&self) -> f32 {
        self.state.playback_speed
    }

    /// Obtiene el estado actual
    pub fn get_state(&self) -> &ClipsPlayerState {
        &self.state
    }

    /// Serializa el reproductor a JSON
    pub fn serialize(&self) -> String {
        use serde::Serialize;
        
        #[derive(Serialize)]
        struct SerializablePlayer {
            current_clip: Option<String>,
            current_frame: u32,
            duration: u32,
            loop_mode: String,
            is_playing: bool,
            playback_speed: f32,
            keyframes: HashMap<String, Vec<Keyframe>>,
            loaded_clips: HashMap<String, AnimationClip>,
        }
        
        let serializable = SerializablePlayer {
            current_clip: self.state.current_clip.clone(),
            current_frame: self.state.current_frame,
            duration: self.state.duration,
            loop_mode: format!("{:?}", self.state.loop_mode),
            is_playing: self.state.is_playing,
            playback_speed: self.state.playback_speed,
            keyframes: self.keyframes.clone(),
            loaded_clips: self.loaded_clips.clone(),
        };
        
        serde_json::to_string(&serializable).unwrap_or_default()
    }

    /// Deserializa el reproductor desde JSON
    pub fn deserialize(data: &str) -> Result<Self, serde_json::Error> {
        use serde::Deserialize;
        
        #[derive(Deserialize)]
        struct DeserializablePlayer {
            current_clip: Option<String>,
            current_frame: u32,
            duration: u32,
            loop_mode: String,
            is_playing: bool,
            playback_speed: f32,
            keyframes: HashMap<String, Vec<Keyframe>>,
            loaded_clips: HashMap<String, AnimationClip>,
        }
        
        let deserialized = serde_json::from_str::<DeserializablePlayer>(data)?;
        
        let mut player = Self::default();
        player.state.current_clip = player.state.current_clip.clone().or(deserialized.current_clip);
        player.state.current_frame = deserialized.current_frame;
        player.state.duration = deserialized.duration;
        player.state.loop_mode = deserialized.loop_mode.parse().unwrap_or(LoopMode::Once);
        player.state.is_playing = deserialized.is_playing;
        player.state.playback_speed = deserialized.playback_speed;
        player.keyframes = deserialized.keyframes;
        player.loaded_clips = deserialized.loaded_clips;
        
        Ok(player)
    }
}

#[cfg(test)]
mod tests {
    use crate::animation::LoopMode;
    use crate::animation_clips::{AnimationClip, ClipsPlayer, ClipsPlayerState};

    #[test]
    fn test_clips_player_new() {
        let player = ClipsPlayer::new();
        assert!(!player.state.is_playing);
        assert_eq!(player.state.current_frame, 0);
        assert_eq!(player.state.duration, 0);
    }

    #[test]
    fn test_clips_player_load_clip() {
        let mut player = ClipsPlayer::new();
        let clip = AnimationClip::new("test_clip", 100, LoopMode::Loop);
        let result = player.load_clip(clip);
        assert!(result);
        assert_eq!(player.state.current_clip, Some("test_clip".to_string()));
        assert_eq!(player.state.duration, 100);
    }

    #[test]
    fn test_clips_player_play_pause_stop() {
        let mut player = ClipsPlayer::new();
        let clip = AnimationClip::new("test", 100, LoopMode::Loop);
        player.load_clip(clip);
        
        player.play();
        assert!(player.state.is_playing);
        
        player.pause();
        assert!(!player.state.is_playing);
        
        player.stop();
        assert_eq!(player.state.current_frame, 0);
    }

    #[test]
    fn test_clips_player_update_loop() {
        let mut player = ClipsPlayer::new();
        let clip = AnimationClip::new("test", 10, LoopMode::Loop);
        player.load_clip(clip);
        
        // Avanzar más de la duración
        player.state.is_playing = true;
        player.state.playback_speed = 60.0; // 60 FPS
        for _ in 0..20 {
            player.update(1.0 / 60.0);
        }
        
        // Debería estar en el frame 0 del siguiente loop
        assert_eq!(player.state.current_frame, 0);
    }

    #[test]
    fn test_clips_player_once_mode() {
        let mut player = ClipsPlayer::new();
        let clip = AnimationClip::new("test", 10, LoopMode::Once);
        player.load_clip(clip);
        
        player.state.is_playing = true;
        player.state.playback_speed = 60.0;
        for _ in 0..20 {
            player.update(1.0 / 60.0);
        }
        
        // Debería estar detenido en el final
        assert!(!player.state.is_playing);
        assert_eq!(player.state.current_frame, 10);
    }

    #[test]
    fn test_clips_player_pingpong_mode() {
        let mut player = ClipsPlayer::new();
        let clip = AnimationClip::new("test", 10, LoopMode::PingPong);
        player.load_clip(clip.clone());
        
        player.state.is_playing = true;
        player.state.playback_speed = 60.0;
        
        // Avanzar hasta el final (frame 9)
        for i in 0..9 {
            player.update(1.0 / 60.0);
            eprintln!("Iter {}: frame={}, loop_mode={:?}", i, player.state.current_frame, player.state.loop_mode);
        }
        eprintln!("After 9 iterations: frame={}, loop_mode={:?}", player.state.current_frame, player.state.loop_mode);
        // En ping pong, el frame debería estar en 9 (último frame)
        assert_eq!(player.state.current_frame, 9);
        assert_eq!(player.state.loop_mode, LoopMode::PingPong);
        
        // Continuar - debería rebotar y continuar en sentido inverso
        // 1 update: rebote a frame 0
        // 9 más updates: frame 1-9
        // Total 10 updates: frame 9
        for _ in 0..10 {
            player.update(1.0 / 60.0);
        }
        // Después de 10 updates desde frame 9: rebote a 0, luego 1-9 = frame 9
        assert_eq!(player.state.current_frame, 9);
        assert_eq!(player.state.loop_mode, LoopMode::PingPong);
    }

    #[test]
    fn test_clips_player_set_frame() {
        let mut player = ClipsPlayer::new();
        let clip = AnimationClip::new("test", 100, LoopMode::Loop);
        player.load_clip(clip);
        
        player.set_frame(50);
        assert_eq!(player.state.current_frame, 50);
        
        player.set_frame(150); // Más allá de la duración
        assert_eq!(player.state.current_frame, 100);
    }

    #[test]
    fn test_clips_player_get_time() {
        let mut player = ClipsPlayer::new();
        let clip = AnimationClip::new("test", 100, LoopMode::Loop);
        player.load_clip(clip);
        
        player.state.current_frame = 50;
        player.state.playback_speed = 30.0;
        
        assert_eq!(player.get_current_time(), 50.0 / 30.0);
        assert_eq!(player.get_remaining_time(), 50.0 / 30.0);
    }

    #[test]
    fn test_clips_player_serialize_deserialize() {
        let mut player = ClipsPlayer::new();
        let clip = AnimationClip::new("test", 100, LoopMode::Loop);
        player.load_clip(clip);
        player.state.current_frame = 50;
        player.state.is_playing = true;
        
        let serialized = player.serialize();
        let deserialized = ClipsPlayer::deserialize(&serialized).unwrap();
        
        assert_eq!(deserialized.state.current_frame, player.state.current_frame);
        assert_eq!(deserialized.state.is_playing, player.state.is_playing);
    }
}
