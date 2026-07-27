//! Runtime de audio

use std::collections::HashMap;

use forge_types::audio::{
    AudioStream, AudioChannel, AudioSocket, AudioBehavior, AudioChannelType,
    PositionalAudio, AudioState,
};

/// Sistema de audio del runtime
pub struct AudioSystem {
    /// Canales de audio
    channels: HashMap<String, AudioChannel>,
    /// Sockets de audio por entidad
    sockets: HashMap<String, Vec<AudioSocket>>,
    /// Comportamientos de audio
    behaviors: HashMap<String, Vec<AudioBehavior>>,
    /// Audio posicional
    positional: PositionalAudio,
}

impl Default for AudioSystem {
    fn default() -> Self {
        Self {
            channels: HashMap::new(),
            sockets: HashMap::new(),
            behaviors: HashMap::new(),
            positional: PositionalAudio::default(),
        }
    }
}

impl AudioSystem {
    /// Crear canal de audio
    pub fn create_channel(&mut self, channel: AudioChannel) {
        self.channels.insert(channel.id.clone(), channel);
    }

    /// Obtener canal por ID
    pub fn get_channel(&self, id: &str) -> Option<&AudioChannel> {
        self.channels.get(id)
    }

    /// Crear socket de audio para entidad
    pub fn create_socket(&mut self, socket: AudioSocket) {
        self.sockets
            .entry(socket.id.clone())
            .or_default()
            .push(socket);
    }

    /// Crear comportamiento de audio
    pub fn create_behavior(&mut self, behavior: AudioBehavior) {
        self.behaviors
            .entry(behavior.audio_socket.clone())
            .or_default()
            .push(behavior);
    }

    /// Reproducir stream en canal
    pub fn play_stream(&mut self, channel_type: AudioChannelType, stream: AudioStream) {
        if let Some(channel) = self.channels.values_mut().find(|c| {
            c.channel_type == channel_type
        }) {
            channel.state = AudioState::Playing;
        }
    }

    /// Detener stream en canal
    pub fn stop_stream(&mut self, channel_type: AudioChannelType) {
        if let Some(channel) = self.channels.values_mut().find(|c| {
            c.channel_type == channel_type
        }) {
            channel.state = AudioState::Stopped;
        }
    }

    /// Actualizar audio posicional
    pub fn update_positional(&mut self, position: (f32, f32, f32), rotation: (f32, f32)) {
        self.positional.position = position;
        self.positional.rotation = rotation.0;
    }

    /// Calcular volumen basado en distancia
    pub fn calculate_volume(&self, distance: f32, base_volume: f32, attenuation: f32) -> f32 {
        if distance <= 0.0 {
            return base_volume;
        }
        let attenuation_factor = 1.0 / (1.0 + attenuation * distance * distance);
        base_volume * attenuation_factor
    }
}

/// Manager de audio global
pub struct AudioManager {
    /// Sistema de audio
    audio_system: AudioSystem,
    /// Volumen maestro
    master_volume: f32,
}

impl Default for AudioManager {
    fn default() -> Self {
        Self {
            audio_system: AudioSystem::default(),
            master_volume: 1.0,
        }
    }
}

impl AudioManager {
    /// Reproducir BGM
    pub fn play_bgm(&mut self, stream: AudioStream) {
        self.audio_system.play_stream(AudioChannelType::Bgm, stream);
    }

    /// Reproducir SFX
    pub fn play_sfx(&mut self, stream: AudioStream) {
        self.audio_system.play_stream(AudioChannelType::Sfx, stream);
    }

    /// Reproducir voz
    pub fn play_voice(&mut self, stream: AudioStream) {
        self.audio_system.play_stream(AudioChannelType::Voice, stream);
    }

    /// Setear volumen maestro
    pub fn set_master_volume(&mut self, volume: f32) {
        self.master_volume = volume.clamp(0.0, 1.0);
    }

    /// Obtener volumen maestro
    pub fn get_master_volume(&self) -> f32 {
        self.master_volume
    }

    /// Actualizar sistema de audio
    pub fn update(&mut self) {
        // Actualizar streams activos
        for channel in self.audio_system.channels.values_mut() {
            if channel.state == AudioState::Playing {
                // LÃ³gica de reproducciÃ³n en loop o one-shot
            }
        }
    }
}

