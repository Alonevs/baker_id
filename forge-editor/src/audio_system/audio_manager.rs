use std::collections::HashMap;
use std::fs;
use std::path::Path;

use crate::play_session::PlaySession;
use crate::scene_system::SceneManager;

/// Identificador único para efectos de sonido
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SfxId(pub u64);

impl SfxId {
    pub fn new(id: u64) -> Self {
        Self(id)
    }
}

/// Identificador único para pistas de música
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MusicTrackId(pub u64);

impl MusicTrackId {
    pub fn new(id: u64) -> Self {
        Self(id)
    }
}

/// Representación de una instancia de audio en tiempo de ejecución
#[derive(Debug, Clone)]
pub struct AudioInstance {
    pub id: SfxId,
    pub sound_name: String,
    pub volume: f32,
    pub pitch: f32,
    pub pan: f32,
    pub position: Vec3,
    pub is_playing: bool,
    pub r#loop: bool,
}

#[derive(Debug, Clone, Copy)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }
    
    pub fn zero() -> Self {
        Self { x: 0.0, y: 0.0, z: 0.0 }
    }
}

/// Manejador de efectos de sonido (SFX)
#[derive(Debug, Default)]
pub struct SfxManager {
    sfx_pool: Vec<AudioInstance>,
    sfx_cache: HashMap<String, AudioBuffer>,
    next_id: u64,
}

#[derive(Debug, Clone)]
pub struct AudioBuffer {
    pub duration: f32,
    pub sample_rate: u32,
    pub channels: u8,
}

impl SfxManager {
    pub fn new() -> Self {
        Self {
            sfx_pool: Vec::new(),
            sfx_cache: HashMap::new(),
            next_id: 1,
        }
    }
    
    /// Cargar un archivo de sonido en caché
    pub fn load_sound(&mut self, sound_name: &str) -> Result<AudioBuffer, String> {
        let file_path = format!("assets/audio/sfx/{}.wav", sound_name);
        
        if !Path::new(&file_path).exists() {
            return Err(format!("Sound file not found: {}", file_path));
        }
        
        // Simulación de carga (en producción usar librería real como rustaudio)
        let duration = 1.0; // Placeholder
        let sample_rate = 44100;
        let channels = 1;
        
        let buffer = AudioBuffer {
            duration,
            sample_rate,
            channels,
        };
        
        self.sfx_cache.insert(sound_name.to_string(), buffer);
        Ok(buffer)
    }
    
    /// Reproducir un efecto de sonido
    pub fn play_sfx(
        &mut self,
        sound_name: &str,
        position: Vec3,
        volume: f32,
        pitch: f32,
        pan: f32,
        loop_: bool,
    ) -> SfxId {
        // Cargar sonido si no está en caché
        if !self.sfx_cache.contains_key(sound_name) {
            let _ = self.load_sound(sound_name);
        }
        
        let id = SfxId::new(self.next_id);
        self.next_id += 1;
        
        let sound_name = sound_name.to_string();
        
        let instance = AudioInstance {
            id,
            sound_name,
            volume: volume.clamp(0.0, 1.0),
            pitch: pitch.clamp(0.5, 2.0),
            pan: pan.clamp(-1.0, 1.0),
            position,
            is_playing: true,
            r#loop: loop_,
        };
        
        self.sfx_pool.push(instance);
        id
    }
    
    /// Reproducir sonido en posición 0,0 (sin spatial audio)
    pub fn play_sfx_at(&mut self, sound_name: &str, volume: f32) -> SfxId {
        self.play_sfx(sound_name, Vec3::zero(), volume, 1.0, 0.0, false)
    }
    
    /// Reproducir sonido con spatial audio
    pub fn play_sfx_spatial(
        &mut self,
        sound_name: &str,
        position: Vec3,
        volume: f32,
    ) -> SfxId {
        self.play_sfx(sound_name, position, volume, 1.0, 0.0, false)
    }
    
    /// Detener un efecto de sonido específico
    pub fn stop_sfx(&mut self, sound_id: SfxId) {
        if let Some(index) = self.sfx_pool.iter().position(|sfx| sfx.id == sound_id) {
            self.sfx_pool[index].is_playing = false;
        }
    }
    
    /// Detener todos los efectos de sonido
    pub fn stop_all_sfx(&mut self) {
        for sfx in &mut self.sfx_pool {
            sfx.is_playing = false;
        }
    }
    
    /// Obtener volumen de un efecto específico
    pub fn get_sfx_volume(&self, sound_id: SfxId) -> Option<f32> {
        self.sfx_pool.iter().find(|sfx| sfx.id == sound_id).map(|sfx| sfx.volume)
    }
    
    /// Setear volumen de un efecto específico
    pub fn set_sfx_volume(&mut self, sound_id: SfxId, volume: f32) {
        if let Some(index) = self.sfx_pool.iter_mut().position(|sfx| sfx.id == sound_id) {
            self.sfx_pool[index].volume = volume.clamp(0.0, 1.0);
        }
    }
    
    /// Setear pitch de un efecto específico
    pub fn set_sfx_pitch(&mut self, sound_id: SfxId, pitch: f32) {
        if let Some(index) = self.sfx_pool.iter_mut().position(|sfx| sfx.id == sound_id) {
            self.sfx_pool[index].pitch = pitch.clamp(0.5, 2.0);
        }
    }
    
    /// Setear pan de un efecto específico
    pub fn set_sfx_pan(&mut self, sound_id: SfxId, pan: f32) {
        if let Some(index) = self.sfx_pool.iter_mut().position(|sfx| sfx.id == sound_id) {
            self.sfx_pool[index].pan = pan.clamp(-1.0, 1.0);
        }
    }
    
    /// Obtener número total de efectos activos
    pub fn active_sfx_count(&self) -> usize {
        self.sfx_pool.iter().filter(|sfx| sfx.is_playing).count()
    }
}

/// Sistema de música en el juego
#[derive(Debug, Default)]
pub struct MusicManager {
    music_tracks: HashMap<MusicTrackId, MusicTrack>,
    current_track: Option<MusicTrackId>,
    music_queue: Vec<MusicTrackId>,
    is_playing: bool,
    is_paused: bool,
    volume: f32,
}

#[derive(Debug, Clone)]
pub struct MusicTrack {
    pub id: MusicTrackId,
    pub name: String,
    pub volume: f32,
    pub r#loop: bool,
    pub fade_in: Option<f32>,
    pub fade_out: Option<f32>,
}

impl MusicManager {
    pub fn new() -> Self {
        Self {
            music_tracks: HashMap::new(),
            current_track: None,
            music_queue: Vec::new(),
            is_playing: false,
            is_paused: false,
            volume: 0.5,
        }
    }
    
    /// Cargar una pista de música
    pub fn load_music_track(&mut self, track_id: MusicTrackId, name: &str, loop_: bool) -> Result<(), String> {
        let track = MusicTrack {
            id: track_id,
            name: name.to_string(),
            volume: 0.5,
            r#loop: loop_,
            fade_in: None,
            fade_out: None,
        };
        
        self.music_tracks.insert(track_id, track);
        Ok(())
    }
    
    /// Reproducir una pista de música
    pub fn play_music(&mut self, track_id: MusicTrackId) -> Result<(), String> {
        if let Some(track) = self.music_tracks.get(&track_id) {
            self.current_track = Some(track_id);
            self.is_playing = true;
            self.is_paused = false;
            Ok(())
        } else {
            Err(format!("Music track not found: {:?}", track_id))
        }
    }
    
    /// Reproducir siguiente pista en la cola
    pub fn next_music_track(&mut self) {
        if let Some(track_id) = self.current_track {
            self.music_queue.push(track_id);
        }
    }
    
    /// Reproducir pista anterior
    pub fn previous_music_track(&mut self) {
        if let Some(track_id) = self.current_track {
            // Buscar pista anterior en la cola o reiniciar
            let _ = track_id;
        }
    }
    
    /// Detener música actual
    pub fn stop_music(&mut self) {
        self.current_track = None;
        self.is_playing = false;
    }
    
    /// Pausar música
    pub fn pause_music(&mut self) {
        self.is_paused = true;
    }
    
    /// Reanudar música
    pub fn resume_music(&mut self) {
        self.is_paused = false;
    }
    
    /// Setear volumen de música
    pub fn set_music_volume(&mut self, volume: f32) {
        self.volume = volume.clamp(0.0, 1.0);
        if let Some(track) = self.current_track.and_then(|id| self.music_tracks.get_mut(&id)) {
            track.volume = volume;
        }
    }
    
    /// Obtener volumen actual
    pub fn get_music_volume(&self) -> f32 {
        self.volume
    }
    
    /// Obtener pista actual
    pub fn current_track_id(&self) -> Option<MusicTrackId> {
        self.current_track
    }
    
    /// Obtener nombre de pista actual
    pub fn current_track_name(&self) -> Option<&str> {
        self.current_track.and_then(|id| self.music_tracks.get(&id).map(|t| t.name.as_str()))
    }
    
    /// Obtener estado de reproducción
    pub fn is_playing(&self) -> bool {
        self.is_playing && !self.is_paused
    }
}

/// Control de volumen y mezcla de audio
#[derive(Debug, Default)]
pub struct AudioMixer {
    master_volume: f32,
    sfx_volume: f32,
    music_volume: f32,
    dialogue_volume: f32,
    ambient_volume: f32,
}

impl AudioMixer {
    pub fn new() -> Self {
        Self {
            master_volume: 1.0,
            sfx_volume: 0.8,
            music_volume: 0.6,
            dialogue_volume: 1.0,
            ambient_volume: 0.5,
        }
    }
    
    /// Setear volumen maestro
    pub fn set_master_volume(&mut self, volume: f32) {
        self.master_volume = volume.clamp(0.0, 1.0);
    }
    
    /// Setear volumen de efectos
    pub fn set_sfx_volume(&mut self, volume: f32) {
        self.sfx_volume = volume.clamp(0.0, 1.0);
    }
    
    /// Setear volumen de música
    pub fn set_music_volume(&mut self, volume: f32) {
        self.music_volume = volume.clamp(0.0, 1.0);
    }
    
    /// Setear volumen de diálogo
    pub fn set_dialogue_volume(&mut self, volume: f32) {
        self.dialogue_volume = volume.clamp(0.0, 1.0);
    }
    
    /// Setear volumen de ambiente
    pub fn set_ambient_volume(&mut self, volume: f32) {
        self.ambient_volume = volume.clamp(0.0, 1.0);
    }
    
    /// Obtener volumen maestro
    pub fn get_master_volume(&self) -> f32 {
        self.master_volume
    }
    
    /// Obtener volumen de efectos
    pub fn get_sfx_volume(&self) -> f32 {
        self.sfx_volume
    }
    
    /// Obtener volumen de música
    pub fn get_music_volume(&self) -> f32 {
        self.music_volume
    }
    
    /// Obtener volumen de diálogo
    pub fn get_dialogue_volume(&self) -> f32 {
        self.dialogue_volume
    }
    
    /// Obtener volumen de ambiente
    pub fn get_ambient_volume(&self) -> f32 {
        self.ambient_volume
    }
}

/// Estado de audio para una escena específica
#[derive(Debug, Clone, Default)]
pub struct AudioState {
    sfx_volumes: HashMap<SfxId, f32>,
    music_track: Option<MusicTrackId>,
    music_volume: f32,
    is_music_paused: bool,
}

/// Manejador central de audio para todo el juego
#[derive(Debug)]
pub struct AudioManager {
    sfx_manager: SfxManager,
    music_manager: MusicManager,
    mixer: AudioMixer,
    scene_audio_states: HashMap<SceneId, AudioState>,
    current_scene: Option<SceneId>,
}

impl AudioManager {
    pub fn new() -> Self {
        Self {
            sfx_manager: SfxManager::new(),
            music_manager: MusicManager::new(),
            mixer: AudioMixer::new(),
            scene_audio_states: HashMap::new(),
            current_scene: None,
        }
    }
    
    /// Inicializar sistema de audio
    pub fn init_audio_system(&mut self) {
        println!("🎵 Audio System initialized");
    }
    
    /// Cerrar sistema de audio
    pub fn shutdown_audio_system(&mut self) {
        self.stop_all_audio();
        println!("🔇 Audio System shutdown");
    }
    
    /// Cargar archivos de audio desde ruta
    pub fn load_audio_files(&self, path: &str) -> Result<usize, String> {
        let entries = fs::read_dir(path).map_err(|e| e.to_string())?;
        let mut count = 0;
        
        for entry in entries {
            let entry = entry.ok().unwrap();
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some("wav") {
                let file_name = path.file_stem().and_then(|s| s.to_str()).unwrap_or("");
                let _ = self.sfx_manager.load_sound(file_name);
                count += 1;
            }
        }
        
        Ok(count)
    }
    
    /// Cargar música para escena actual
    pub fn load_scene_music(&mut self, scene_id: SceneId, music_id: MusicTrackId, loop_: bool) {
        if let Err(_) = self.music_manager.load_music_track(music_id, "background", loop_) {
            // Música no cargada, ignorar
        }
        self.scene_audio_states.insert(scene_id, AudioState {
            music_track: Some(music_id),
            music_volume: 0.5,
            is_music_paused: false,
            sfx_volumes: HashMap::new(),
        });
    }
    
    /// Aplicar estado de audio de escena
    pub fn apply_scene_audio(&mut self, scene_id: SceneId) {
        if let Some(state) = self.scene_audio_states.get(&scene_id) {
            if let Some(track_id) = state.music_track {
                let _ = self.music_manager.play_music(track_id);
            }
            if state.is_music_paused {
                self.music_manager.pause_music();
            } else {
                self.music_manager.resume_music();
            }
        }
    }
    
    /// Obtener estado de audio de escena actual
    pub fn get_current_scene_audio(&self) -> Option<&AudioState> {
        self.current_scene.and_then(|id| self.scene_audio_states.get(&id))
    }
    
    /// Setear volumen maestro
    pub fn set_master_volume(&mut self, volume: f32) {
        self.mixer.set_master_volume(volume);
    }
    
    /// Setear volumen de efectos
    pub fn set_sfx_volume(&mut self, volume: f32) {
        self.mixer.set_sfx_volume(volume);
        for sfx in &mut self.sfx_manager.sfx_pool {
            sfx.volume = volume * sfx.volume;
        }
    }
    
    /// Setear volumen de música
    pub fn set_music_volume(&mut self, volume: f32) {
        self.mixer.set_music_volume(volume);
        if let Some(track) = self.music_manager.current_track.and_then(|id| self.music_manager.music_tracks.get_mut(&id)) {
            track.volume = volume;
        }
    }
    
    /// Setear volumen de diálogo
    pub fn set_dialogue_volume(&mut self, volume: f32) {
        self.mixer.set_dialogue_volume(volume);
    }
    
    /// Reproducir efecto de sonido
    pub fn play_sfx(&mut self, sound_name: &str, position: Vec3, volume: f32) -> SfxId {
        let sfx_id = self.sfx_manager.play_sfx_spatial(sound_name, position, volume);
        sfx_id
    }
    
    /// Reproducir efecto de sonido sin spatial audio
    pub fn play_sfx_at(&mut self, sound_name: &str, volume: f32) -> SfxId {
        let sfx_id = self.sfx_manager.play_sfx_at(sound_name, volume);
        sfx_id
    }
    
    /// Detener efecto de sonido
    pub fn stop_sfx(&mut self, sound_id: SfxId) {
        self.sfx_manager.stop_sfx(sound_id);
    }
    
    /// Detener todos los efectos
    pub fn stop_all_sfx(&mut self) {
        self.sfx_manager.stop_all_sfx();
    }
    
    /// Detener toda la música
    pub fn stop_all_music(&mut self) {
        self.music_manager.stop_music();
    }
    
    /// Pausar música
    pub fn pause_music(&mut self) {
        self.music_manager.pause_music();
    }
    
    /// Reanudar música
    pub fn resume_music(&mut self) {
        self.music_manager.resume_music();
    }
    
    /// Reproducir siguiente pista
    pub fn next_music_track(&mut self) {
        self.music_manager.next_music_track();
    }
    
    /// Reproducir pista anterior
    pub fn previous_music_track(&mut self) {
        self.music_manager.previous_music_track();
    }
    
    /// Obtener número de efectos activos
    pub fn active_sfx_count(&self) -> usize {
        self.sfx_manager.active_sfx_count()
    }
    
    /// Obtener volumen maestro
    pub fn get_master_volume(&self) -> f32 {
        self.mixer.get_master_volume()
    }
    
    /// Obtener volumen de música
    pub fn get_music_volume(&self) -> f32 {
        self.mixer.get_music_volume()
    }
    
    /// Detener todo el audio
    pub fn stop_all_audio(&mut self) {
        self.stop_all_sfx();
        self.stop_all_music();
        self.music_manager = MusicManager::new();
    }
}

impl Default for AudioManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Extensiones para PlaySession con integración de audio
pub trait PlaySessionAudioExt {
    fn init_audio_system(&mut self, manager: &mut AudioManager);
    fn handle_audio_event(&mut self, event: AudioEvent);
}

impl PlaySessionAudioExt for PlaySession {
    fn init_audio_system(&mut self, manager: &mut AudioManager) {
        manager.init_audio_system();
    }
    
    fn handle_audio_event(&mut self, event: AudioEvent) {
        // Manejar eventos de audio del juego
        match event {
            AudioEvent::SfxPlayed { sound_id, .. } => {
                println!("🔊 SFX played: {:?}", sound_id);
            }
            AudioEvent::MusicChanged { track_id, .. } => {
                println!("🎵 Music changed: {:?}", track_id);
            }
            AudioEvent::VolumeChanged { volume, .. } => {
                println!("🔉 Volume changed to: {}", volume);
            }
        }
    }
}

/// Eventos de audio del juego
#[derive(Debug, Clone)]
pub enum AudioEvent {
    SfxPlayed { sound_id: SfxId, position: Vec3 },
    MusicChanged { track_id: MusicTrackId },
    VolumeChanged { volume: f32 },
}
