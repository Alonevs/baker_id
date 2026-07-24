use crate::{
    audio_bus::{AudioBusSystem, BusType},
    audio_effects::{AudioEffect, AudioEffects},
    audio_mixer::AudioMixer,
    audio_sample::AudioSample,
    audio_source::AudioSource,
    spatial_audio::{SpatialAudio, SpatialAudioSource},
};
use std::collections::HashMap;

/// AudioManager - Gestión central de audio
#[derive(Debug)]
pub struct AudioManager {
    pub mixer: AudioMixer,
    pub bus_system: AudioBusSystem,
    pub effects: AudioEffects,
    pub spatial_audio: SpatialAudio,
    pub sources: HashMap<String, AudioSource>,
    pub spatial_sources: HashMap<String, SpatialAudioSource>,
    pub master_bus_id: String,
    pub sample_rate: u32,
}

impl AudioManager {
    pub fn new() -> Self {
        let mixer = AudioMixer::new(None);
        let bus_system = AudioBusSystem::new();
        let master_bus_id = bus_system.get_master_bus_id().to_string();

        Self {
            mixer,
            bus_system,
            effects: AudioEffects::new(),
            spatial_audio: SpatialAudio::new(None),
            sources: HashMap::new(),
            spatial_sources: HashMap::new(),
            master_bus_id,
            sample_rate: 44100,
        }
    }

    /// Cargar una muestra de audio
    pub fn load_sample(&mut self, sample: AudioSample) -> String {
        let id = sample.id.clone();
        self.sources.insert(id.clone(), AudioSource::new(sample));
        id
    }

    /// Obtener una fuente de audio
    pub fn get_source(&self, id: &str) -> Option<&AudioSource> {
        self.sources.get(id)
    }

    /// Obtener una fuente de audio mutante
    pub fn get_source_mut(&mut self, id: &str) -> Option<&mut AudioSource> {
        self.sources.get_mut(id)
    }

    /// Reproducir una fuente de audio
    pub fn play(&mut self, id: &str, callback: Option<Box<dyn Fn() + Send + Sync>>) {
        if let Some(source) = self.sources.get_mut(id) {
            source.play(callback);
        }
    }

    /// Pausar una fuente de audio
    pub fn pause(&mut self, id: &str) {
        if let Some(source) = self.sources.get_mut(id) {
            source.pause();
        }
    }

    /// Detener una fuente de audio
    pub fn stop(&mut self, id: &str) {
        if let Some(source) = self.sources.get_mut(id) {
            source.stop();
        }
    }

    /// Actualizar todas las fuentes de audio
    pub fn update(&mut self, delta: f32) {
        // Actualizar fuentes de audio
        for source in self.sources.values_mut() {
            source.update(delta as f32, self.sample_rate);
        }

        // Actualizar fuentes espaciales
        self.spatial_audio.update(delta);

        // Mezclar audio
        self.mixer.mix(delta);
    }

    /// Configurar volumen master
    pub fn set_master_volume(&mut self, volume: f32) {
        self.mixer.set_master_volume(volume);
    }

    /// Configurar bus de audio
    pub fn configure_bus(&mut self, bus_id: &str, config: &BusType) {
        if let Some(bus) = self.bus_system.get_bus_mut(bus_id) {
            bus.bus_type = *config;
        }
    }

    /// Agregar efecto a un bus
    pub fn add_effect_to_bus(&mut self, bus_id: &str, effect: AudioEffect) {
        if let Some(bus) = self.bus_system.get_bus_mut(bus_id) {
            bus.add_effect(effect.id.clone());
            self.effects.add_effect(effect);
        }
    }

    /// Agregar fuente espacial
    pub fn add_spatial_source(&mut self, source: SpatialAudioSource) {
        self.spatial_sources.insert(source.id.clone(), source);
    }

    /// Obtener número de fuentes
    pub fn get_source_count(&self) -> usize {
        self.sources.len()
    }

    /// Obtener número de buses
    pub fn get_bus_count(&self) -> usize {
        self.bus_system.get_bus_count()
    }

    /// Obtener número de canales
    pub fn get_channel_count(&self) -> usize {
        self.mixer.get_channel_count()
    }

    /// Obtener master bus ID
    pub fn get_master_bus_id(&self) -> &str {
        &self.master_bus_id
    }

    /// Obtener sample rate
    pub fn get_sample_rate(&self) -> u32 {
        self.sample_rate
    }

    /// Renderizar audio (mezclar todo)
    pub fn render(&mut self) {
        self.update(1.0 / self.sample_rate as f32);
    }
}

impl Default for AudioManager {
    fn default() -> Self {
        Self::new()
    }
}
