use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Posición de audio en 3D
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct AudioPosition {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl AudioPosition {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub fn origin() -> Self {
        Self::new(0.0, 0.0, 0.0)
    }
}

/// Configuración de audio espacial
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpatialAudioConfig {
    pub sample_rate: u32,
    pub distance_model: DistanceModel,
    pub rolloff_factor: f32,
    pub max_distance: f32,
    pub doppler_factor: f32,
}

impl Default for SpatialAudioConfig {
    fn default() -> Self {
        Self {
            sample_rate: 44100,
            distance_model: DistanceModel::InverseDistance,
            rolloff_factor: 1.0,
            max_distance: 1000.0,
            doppler_factor: 1.0,
        }
    }
}

/// Modelo de distancia para audio espacial
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum DistanceModel {
    /// Atenuación inversa a la distancia
    InverseDistance,
    /// Atenuación cuadrática
    Quadratic,
    /// Atenuación lineal
    Linear,
}

/// Fuente de audio espacial
#[derive(Debug, Clone)]
pub struct SpatialAudioSource {
    pub id: String,
    pub name: String,
    pub position: AudioPosition,
    pub velocity: AudioPosition,
    pub volume: f32,
    pub pitch: f32,
    pub pan: f32,
    pub listener: AudioPosition,
    pub enabled: bool,
    pub min_distance: f32,
    pub max_distance: f32,
    pub roll_off_factor: f32,
    pub cone_inner_angle: f32,
    pub cone_outer_angle: f32,
    pub cone_volume_factor: f32,
    pub cone_outer_volume_factor: f32,
}

impl SpatialAudioSource {
    pub fn new(id: String, name: String) -> Self {
        Self {
            id,
            name,
            position: AudioPosition::origin(),
            velocity: AudioPosition::origin(),
            volume: 1.0,
            pitch: 1.0,
            pan: 0.0,
            listener: AudioPosition::origin(),
            enabled: true,
            min_distance: 1.0,
            max_distance: 1000.0,
            roll_off_factor: 1.0,
            cone_inner_angle: 30.0,
            cone_outer_angle: 360.0,
            cone_volume_factor: 1.0,
            cone_outer_volume_factor: 0.0,
        }
    }

    pub fn set_position(&mut self, position: AudioPosition) {
        self.position = position;
    }

    pub fn set_velocity(&mut self, velocity: AudioPosition) {
        self.velocity = velocity;
    }

    pub fn set_listener(&mut self, listener: AudioPosition) {
        self.listener = listener;
    }

    pub fn update(&mut self, delta: f32) {
        // Actualizar posición
        self.position.x += self.velocity.x * delta;
        self.position.y += self.velocity.y * delta;
        self.position.z += self.velocity.z * delta;

        // Limitar velocidad
        let max_velocity = 100.0;
        self.velocity.x = self.velocity.x.clamp(-max_velocity, max_velocity);
        self.velocity.y = self.velocity.y.clamp(-max_velocity, max_velocity);
        self.velocity.z = self.velocity.z.clamp(-max_velocity, max_velocity);
    }

    pub fn get_distance(&self) -> f32 {
        let dx = self.position.x - self.listener.x;
        let dy = self.position.y - self.listener.y;
        let dz = self.position.z - self.listener.z;
        (dx * dx + dy * dy + dz * dz).sqrt()
    }

    pub fn get_distance_gain(&self) -> f32 {
        let distance = self.get_distance();
        let effective_distance = distance.max(self.min_distance);

        // Atenuación basada en distancia
        let gain = 1.0 / (1.0 + effective_distance / self.max_distance);
        gain.clamp(0.0, 1.0)
    }

    pub fn is_within_range(&self) -> bool {
        let distance = self.get_distance();
        distance <= self.max_distance
    }
}

/// Sistema de audio espacial
#[derive(Debug, Clone)]
pub struct SpatialAudio {
    pub config: SpatialAudioConfig,
    pub sources: HashMap<String, SpatialAudioSource>,
}

impl SpatialAudio {
    pub fn new(config: Option<SpatialAudioConfig>) -> Self {
        let config = config.unwrap_or_default();
        Self {
            config,
            sources: HashMap::new(),
        }
    }

    pub fn add_source(&mut self, source: SpatialAudioSource) {
        self.sources.insert(source.id.clone(), source);
    }

    pub fn get_source(&self, id: &str) -> Option<&SpatialAudioSource> {
        self.sources.get(id)
    }

    pub fn get_source_mut(&mut self, id: &str) -> Option<&mut SpatialAudioSource> {
        self.sources.get_mut(id)
    }

    pub fn remove_source(&mut self, id: &str) {
        self.sources.remove(id);
    }

    pub fn update(&mut self, delta: f32) {
        for source in self.sources.values_mut() {
            source.update(delta);
        }
    }

    pub fn get_source_count(&self) -> usize {
        self.sources.len()
    }

    pub fn get_listener_position(&self) -> AudioPosition {
        // Obtener la posición del listener (promedio de todas las fuentes)
        if self.sources.is_empty() {
            return AudioPosition::origin();
        }

        let mut total_x = 0.0;
        let mut total_y = 0.0;
        let mut total_z = 0.0;

        for source in self.sources.values() {
            total_x += source.listener.x;
            total_y += source.listener.y;
            total_z += source.listener.z;
        }

        let count = self.sources.len() as f32;
        AudioPosition::new(
            total_x / count,
            total_y / count,
            total_z / count,
        )
    }
}

impl Default for SpatialAudio {
    fn default() -> Self {
        Self::new(None)
    }
}
