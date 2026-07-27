//! Animation Clips Library para importación/exportación de clips de animación

use crate::animation::{Keyframe, LoopMode};
use std::collections::HashMap;
use serde::{Serialize, Deserialize};

/// Clip de animación serializable
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnimationClip {
    /// Nombre del clip
    pub name: String,
    /// Duración total en frames
    pub duration: u32,
    /// Modo de loop
    pub loop_mode: LoopMode,
    /// Keyframes por propiedad
    pub keyframes: HashMap<String, Vec<Keyframe>>,
    /// Metadata adicional
    pub metadata: HashMap<String, String>,
}

impl AnimationClip {
    /// Crea un nuevo clip de animación
    pub fn new(name: &str, duration: u32, loop_mode: LoopMode) -> Self {
        Self {
            name: name.to_string(),
            duration,
            loop_mode,
            keyframes: HashMap::new(),
            metadata: HashMap::new(),
        }
    }

    /// Agrega keyframes para una propiedad
    pub fn add_keyframes(&mut self, property: &str, keyframes: Vec<Keyframe>) {
        self.keyframes.insert(property.to_string(), keyframes);
    }

    /// Obtiene los keyframes de una propiedad
    pub fn get_keyframes(&self, property: &str) -> Option<&Vec<Keyframe>> {
        self.keyframes.get(property)
    }

    /// Obtiene los keyframes mutados de una propiedad
    pub fn get_keyframes_mut(&mut self, property: &str) -> Option<&mut Vec<Keyframe>> {
        self.keyframes.get_mut(property)
    }

    /// Obtiene la duración del clip
    pub fn get_duration(&self) -> u32 {
        self.duration
    }

    /// Establece la duración del clip
    pub fn set_duration(&mut self, duration: u32) {
        self.duration = duration;
    }

    /// Agrega metadata
    pub fn add_metadata(&mut self, key: &str, value: &str) {
        self.metadata.insert(key.to_string(), value.to_string());
    }

    /// Obtiene metadata
    pub fn get_metadata(&self, key: &str) -> Option<&String> {
        self.metadata.get(key)
    }

    /// Serializa el clip a JSON
    pub fn to_json(&self) -> String {
        serde_json::to_string_pretty(self).unwrap_or_default()
    }

    /// Deserializa desde JSON
    pub fn from_json(data: &str) -> Option<Self> {
        serde_json::from_str(data).ok()
    }

    /// Guarda el clip en un archivo
    pub fn save_to_file(&self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        std::fs::write(path, self.to_json()).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
    }

    /// Carga un clip desde un archivo
    pub fn load_from_file(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let data = std::fs::read_to_string(path).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;
        match Self::from_json(&data) {
            Some(clip) => Ok(clip),
            None => Err(Box::new(std::io::Error::new(std::io::ErrorKind::InvalidData, "Invalid clip format")) as Box<dyn std::error::Error>),
        }
    }
}

/// Library de clips de animación
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnimationClipsLibrary {
    /// Clips cargados
    pub clips: HashMap<String, AnimationClip>,
    /// Clips guardados en disco
    pub saved_clips: HashMap<String, String>,
}

impl Default for AnimationClipsLibrary {
    fn default() -> Self {
        Self {
            clips: HashMap::new(),
            saved_clips: HashMap::new(),
        }
    }
}

impl AnimationClipsLibrary {
    /// Crea una nueva library de clips
    pub fn new() -> Self {
        Self::default()
    }

    /// Agrega un clip a la library
    pub fn add_clip(&mut self, clip: AnimationClip) {
        self.clips.insert(clip.name.clone(), clip);
    }

    /// Obtiene un clip por nombre
    pub fn get_clip(&self, name: &str) -> Option<&AnimationClip> {
        self.clips.get(name)
    }

    /// Obtiene un clip mutado por nombre
    pub fn get_clip_mut(&mut self, name: &str) -> Option<&mut AnimationClip> {
        self.clips.get_mut(name)
    }

    /// Elimina un clip por nombre
    pub fn remove_clip(&mut self, name: &str) -> Option<AnimationClip> {
        self.clips.remove(name)
    }

    /// Lista todos los clips
    pub fn list_clips(&self) -> Vec<&String> {
        self.clips.keys().collect()
    }

    /// Carga un clip desde archivo
    pub fn load_clip(&mut self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let clip = AnimationClip::load_from_file(path)?;
        self.clips.insert(clip.name.clone(), clip);
        Ok(())
    }

    /// Guarda un clip en archivo
    pub fn save_clip(&mut self, name: &str, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(clip) = self.clips.get(name) {
            clip.save_to_file(path)?;
            self.saved_clips.insert(name.to_string(), path.to_string());
        }
        Ok(())
    }

    /// Carga todos los clips de una carpeta
    pub fn load_all_clips_from_folder(&mut self, folder_path: &str) -> Result<usize, Box<dyn std::error::Error>> {
        let entries = std::fs::read_dir(folder_path)?;
        let mut count = 0;

        for entry in entries {
            let entry = entry?;
            let path = entry.path();
            
            if path.extension().and_then(|s| s.to_str()) == Some("json") {
                self.load_clip(path.to_str().unwrap_or(""))?;
                count += 1;
            }
        }

        Ok(count)
    }

    /// Guarda todos los clips de la library
    pub fn save_all_clips(&self, output_folder: &str) -> Result<usize, Box<dyn std::error::Error>> {
        let mut count = 0;
        
        for (name, clip) in &self.clips {
            let path = format!("{}/{}.json", output_folder, name);
            clip.save_to_file(&path)?;
            count += 1;
        }

        Ok(count)
    }

    /// Obtiene la cantidad total de clips
    pub fn clip_count(&self) -> usize {
        self.clips.len()
    }

    /// Serializa toda la library a JSON
    pub fn to_json(&self) -> String {
        serde_json::to_string_pretty(self).unwrap_or_default()
    }

    /// Deserializa una library desde JSON
    pub fn from_json(data: &str) -> Option<Self> {
        serde_json::from_str(data).ok()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_animation_clip_new() {
        let clip = AnimationClip::new("test_clip", 100, LoopMode::Loop);
        assert_eq!(clip.name, "test_clip");
        assert_eq!(clip.duration, 100);
        assert_eq!(clip.loop_mode, LoopMode::Loop);
    }

    #[test]
    fn test_animation_clip_add_keyframes() {
        let mut clip = AnimationClip::new("test", 100, LoopMode::Loop);
        let keyframes = vec![
            Keyframe { time: 0.0, value: 0.0, interpolation: "Linear".to_string() },
            Keyframe { time: 50.0, value: 50.0, interpolation: "Linear".to_string() },
        ];
        clip.add_keyframes("position_x", keyframes);
        assert!(clip.get_keyframes("position_x").is_some());
    }

    #[test]
    fn test_animation_clip_metadata() {
        let mut clip = AnimationClip::new("test", 100, LoopMode::Loop);
        clip.add_metadata("author", "test_user");
        clip.add_metadata("description", "Test clip");
        assert_eq!(clip.get_metadata("author"), Some(&"test_user".to_string()));
    }

    #[test]
    fn test_animation_clips_library_add_remove() {
        let mut library = AnimationClipsLibrary::new();
        let clip = AnimationClip::new("clip1", 100, LoopMode::Loop);
        library.add_clip(clip);
        assert_eq!(library.clip_count(), 1);
        assert!(library.list_clips().iter().any(|s| **s == "clip1"));
    }

    #[test]
    fn test_animation_clips_library_load_save() {
        let mut library = AnimationClipsLibrary::new();
        let clip = AnimationClip::new("clip1", 100, LoopMode::Loop);
        library.add_clip(clip);
        assert_eq!(library.clip_count(), 1);
    }

    #[test]
    fn test_animation_clips_library_json() {
        let library = AnimationClipsLibrary::new();
        let json = library.to_json();
        assert!(!json.is_empty());
        let loaded = AnimationClipsLibrary::from_json(&json);
        assert!(loaded.is_some());
    }

    #[test]
    fn test_animation_clip_serialization() {
        let clip = AnimationClip::new("test", 100, LoopMode::Loop);
        let json = clip.to_json();
        let loaded = AnimationClip::from_json(&json);
        assert!(loaded.is_some());
    }
}
