//! Resource Manager - Carga y gestión de recursos del juego

use std::collections::HashMap;
use std::path::Path;
use std::fs;
use serde::{Deserialize, Serialize};

// Importar tipos de recursos del mismo módulo
use super::sprite_resource::SpriteResource;
use super::audio_resource::AudioResource;
use super::script_resource::ScriptResource;
use super::level_resource::LevelResource;

/// Tipo de recurso
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ResourceType {
    Sprite,
    Audio,
    Script,
    Level,
    Custom(String),
}

impl ResourceType {
    pub fn to_string(&self) -> &str {
        match self {
            ResourceType::Sprite => "sprite",
            ResourceType::Audio => "audio",
            ResourceType::Script => "script",
            ResourceType::Level => "level",
            ResourceType::Custom(name) => name,
        }
    }
}

/// Configuración de caché de recursos
#[derive(Debug, Clone)]
pub struct CacheConfig {
    pub max_size: usize,
    pub max_per_type: HashMap<ResourceType, usize>,
}

impl Default for CacheConfig {
    fn default() -> Self {
        Self {
            max_size: 1000,
            max_per_type: HashMap::from([
                (ResourceType::Sprite, 500),
                (ResourceType::Audio, 100),
                (ResourceType::Script, 50),
                (ResourceType::Level, 100),
            ]),
        }
    }
}

impl CacheConfig {
    pub fn new(max_size: usize, max_per_type: HashMap<ResourceType, usize>) -> Self {
        Self {
            max_size,
            max_per_type,
        }
    }
}

/// Cache LRU para recursos
pub struct ResourceCache {
    cache: HashMap<String, Box<dyn Resource>>,
    access_order: Vec<String>,
    config: CacheConfig,
}

impl ResourceCache {
    pub fn new(config: CacheConfig) -> Self {
        Self {
            cache: HashMap::new(),
            access_order: Vec::new(),
            config: config,
        }
    }

    /// Carga un recurso desde el archivo
    pub fn load(&mut self, name: &str, file_path: &Path, resource_type: ResourceType) -> Result<(), String> {
        if self.cache.contains_key(name) {
            return Ok(());
        }

        let content = fs::read_to_string(file_path)
            .map_err(|e| format!("Failed to read {}: {}", file_path.display(), e))?;

        let data: serde_json::Value = serde_json::from_str(&content)
            .map_err(|e| format!("Failed to parse {}: {}", file_path.display(), e))?;

        let resource = create_resource_from_json(name, &data, resource_type)
            .map_err(|e| format!("Failed to create resource: {}", e))?;

        self.cache.insert(name.to_string(), resource);
        self.access_order.push(name.to_string());
        self.cleanup();

        Ok(())
    }

    /// Obtiene recurso por nombre
    pub fn get(&self, name: &str) -> Option<&dyn Resource> {
        self.cache.get(name).map(|r| r.as_ref())
    }

    /// Obtiene referencia mutable al Box
    pub fn get_mut_box(&mut self, name: &str) -> Option<&mut Box<dyn Resource>> {
        self.cache.get_mut(name)
    }

    /// Obtiene referencia mutable (con lifetime impl)
    pub fn get_mut(&mut self, name: &str) -> Option<&mut dyn Resource> {
        let box_ref = self.cache.get_mut(name)?;
        Some(box_ref.as_mut())
    }

    /// Marca recurso como usado (move al final de la lista)
    pub fn mark_used(&mut self, name: &str) {
        if let Some(pos) = self.access_order.iter().position(|n| n == name) {
            let item = self.access_order.remove(pos);
            self.access_order.push(item);
        }
    }

    /// Limpia cache si supera el límite
    pub fn cleanup(&mut self) {
        let mut removed = 0;
        
        while self.cache.len() > self.config.max_size || removed > 0 {
            if let Some(name) = self.access_order.first().cloned() {
                if let Some(resource) = self.cache.remove(&name) {
                    drop(resource);
                }
                self.access_order.remove(0);
                removed += 1;
            } else {
                break;
            }
        }
    }

    /// Inserta un recurso directamente (usado internamente)
    pub fn insert(&mut self, name: String, resource: Box<dyn Resource>) {
        self.cache.insert(name, resource);
    }

    /// Limpia recursos de un tipo específico
    pub fn cleanup_type(&mut self, resource_type: ResourceType) {
        let mut removed = 0;
        
        for name in self.access_order.clone() {
            if let Some(resource) = self.cache.get(&name) {
                if resource.resource_type() == &resource_type {
                    self.cache.remove(&name);
                    self.access_order.retain(|n| n != &name);
                    removed += 1;
                }
            }
        }
    }

    /// Retorna número de recursos en cache
    pub fn len(&self) -> usize {
        self.cache.len()
    }

    /// Verifica si cache está vacío
    pub fn is_empty(&self) -> bool {
        self.cache.is_empty()
    }

    /// Limpia todo el cache
    pub fn clear(&mut self) {
        self.cache.clear();
        self.access_order.clear();
    }

    /// Itera sobre todos los recursos (clave & recurso)
    pub fn iter(&self) -> impl Iterator<Item = (&str, &Box<dyn Resource>)> {
        self.cache.iter().map(|(k, v)| (k.as_ref(), v))
    }
}

impl Default for ResourceCache {
    fn default() -> Self {
        Self::new(CacheConfig::default())
    }
}

/// Trait para recursos
pub trait Resource: Send + Sync {
    fn name(&self) -> &str;
    fn resource_type(&self) -> &ResourceType;
    fn load(&mut self) -> Result<(), String>;
    fn unload(self) where Self: Sized;
}

/// Crea recurso desde JSON
fn create_resource_from_json(
    name: &str,
    data: &serde_json::Value,
    resource_type: ResourceType,
) -> Result<Box<dyn Resource>, String> {
    match resource_type {
        ResourceType::Sprite => {
            let sprite = SpriteResource::from_json(name, data)?;
            Ok(Box::new(sprite))
        }
        ResourceType::Audio => {
            let audio = AudioResource::from_json(name, data)?;
            Ok(Box::new(audio))
        }
        ResourceType::Script => {
            let script = ScriptResource::from_json(name, data)?;
            Ok(Box::new(script))
        }
        ResourceType::Level => {
            let level = LevelResource::from_json(name, data)?;
            Ok(Box::new(level))
        }
        ResourceType::Custom(type_name) => {
            Ok(Box::new(CustomResource::new(name, type_name, data)))
        }
    }
}

/// Resource Manager principal
pub struct ResourceManager {
    cache: ResourceCache,
    config: CacheConfig,
}

impl ResourceManager {
    /// Crea nuevo Resource Manager
    pub fn new(config: CacheConfig) -> Self {
        Self {
            cache: ResourceCache::new(config.clone()),
            config,
        }
    }

    /// Crea Resource Manager con configuración por defecto
    pub fn default() -> Self {
        Self::new(CacheConfig::default())
    }

    /// Carga recurso desde archivo
    pub fn load(&mut self, name: &str, file_path: &Path, resource_type: ResourceType) -> Result<(), String> {
        self.cache.load(name, file_path, resource_type)
    }

    /// Carga recurso desde string JSON
    pub fn load_from_json(&mut self, name: &str, json: &str, resource_type: ResourceType) -> Result<(), String> {
        let data: serde_json::Value = serde_json::from_str(json)
            .map_err(|e| format!("Failed to parse JSON: {}", e))?;

        let resource = create_resource_from_json(name, &data, resource_type)
            .map_err(|e| format!("Failed to create resource: {}", e))?;

        self.cache.insert(name.to_string(), resource);
        self.cache.access_order.push(name.to_string());
        self.cache.cleanup();

        Ok(())
    }

    /// Obtiene recurso por nombre
    pub fn get(&self, name: &str) -> Option<&dyn Resource> {
        self.cache.get(name)
    }

    /// Obtiene referencia mutable
    pub fn get_mut(&mut self, name: &str) -> Option<&mut dyn Resource> {
        self.cache.get_mut(name)
    }

    /// Obtiene número de recursos en caché
    pub fn len(&self) -> usize {
        self.cache.len()
    }

    /// Verifica si la caché está vacía
    pub fn is_empty(&self) -> bool {
        self.cache.is_empty()
    }

    /// Itera sobre todos los recursos (clave & recurso)
    pub fn iter(&self) -> impl Iterator<Item = (&str, &Box<dyn Resource>)> {
        self.cache.iter().map(|(k, v)| (k.as_ref(), v))
    }

    /// Marca recurso como usado
    pub fn mark_used(&mut self, name: &str) {
        self.cache.mark_used(name);
    }

    /// Limpia cache si es necesario
    pub fn cleanup(&mut self) {
        self.cache.cleanup();
    }

    /// Limpia recursos de un tipo
    pub fn cleanup_type(&mut self, resource_type: ResourceType) {
        self.cache.cleanup_type(resource_type);
    }

    /// Limpia todo
    pub fn clear(&mut self) {
        self.cache.clear();
    }
}

impl Default for ResourceManager {
    fn default() -> Self {
        Self::new(CacheConfig::default())
    }
}

/// Custom resource para recursos personalizados
#[derive(Debug)]
pub struct CustomResource {
    name: String,
    resource_type: ResourceType,
    data: serde_json::Value,
}

impl CustomResource {
    pub fn new(name: &str, resource_type: String, data: &serde_json::Value) -> Self {
        Self {
            name: name.to_string(),
            resource_type: ResourceType::Custom(resource_type),
            data: data.clone(),
        }
    }
}

impl Default for CustomResource {
    fn default() -> Self {
        Self::new("", String::new(), &serde_json::Value::Null)
    }
}

impl Resource for CustomResource {
    fn name(&self) -> &str { &self.name }
    fn resource_type(&self) -> &ResourceType { &self.resource_type }
    fn load(&mut self) -> Result<(), String> { Ok(()) }
    fn unload(self) where Self: Sized {}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resource_manager_new() {
        let manager = ResourceManager::default();
        assert!(manager.is_empty());
    }

    #[test]
    fn test_resource_cache_new() {
        let cache = ResourceCache::default();
        assert!(cache.is_empty());
    }

    #[test]
    fn test_resource_type() {
        assert_eq!(ResourceType::Sprite.to_string(), "sprite");
        assert_eq!(ResourceType::Audio.to_string(), "audio");
        assert_eq!(ResourceType::Script.to_string(), "script");
        assert_eq!(ResourceType::Level.to_string(), "level");
        assert_eq!(ResourceType::Custom("custom".to_string()).to_string(), "custom");
    }

    #[test]
    fn test_resource_manager_load_sprite() {
        let mut manager = ResourceManager::default();
        let json = r#"{
            "name": "test_sprite",
            "width": 32,
            "height": 32,
            "texture_path": "sprites/test.png",
            "frames": []
        }"#;

        assert!(manager.load_from_json("test", json, ResourceType::Sprite).is_ok());
        assert_eq!(manager.len(), 1);
        assert!(manager.get("test").is_some());
    }

    #[test]
    fn test_resource_manager_load_audio() {
        let mut manager = ResourceManager::default();
        let json = r#"{
            "name": "test_audio",
            "duration": 1.0,
            "path": "audio/test.wav",
            "channels": 2,
            "sample_rate": 44100,
            "format": "stereo"
        }"#;

        assert!(manager.load_from_json("test", json, ResourceType::Audio).is_ok());
        assert_eq!(manager.len(), 1);
    }

    #[test]
    fn test_resource_manager_load_script() {
        let mut manager = ResourceManager::default();
        let json = r#"{
            "name": "test_script",
            "language": "lua",
            "code": "print('hello')",
            "is_compiled": false,
            "dependencies": []
        }"#;

        assert!(manager.load_from_json("test", json, ResourceType::Script).is_ok());
        assert_eq!(manager.len(), 1);
    }

    #[test]
    fn test_resource_manager_load_level() {
        let mut manager = ResourceManager::default();
        let json = r#"{
            "name": "test_level",
            "width": 10,
            "height": 10,
            "tiles": [],
            "entities": []
        }"#;

        assert!(manager.load_from_json("test", json, ResourceType::Level).is_ok());
        assert_eq!(manager.len(), 1);
    }

    #[test]
    fn test_resource_manager_cache_lru() {
        let mut manager = ResourceManager::default();
        let sprite_json = r#"{
            "name": "sprite1",
            "width": 32,
            "height": 32,
            "texture_path": "sprites/test.png",
            "frames": []
        }"#;

        manager.load_from_json("sprite1", sprite_json, ResourceType::Sprite).unwrap();
        manager.load_from_json("sprite2", sprite_json, ResourceType::Sprite).unwrap();
        manager.load_from_json("sprite3", sprite_json, ResourceType::Sprite).unwrap();

        assert_eq!(manager.len(), 3);

        // Mark sprite1 as used
        manager.mark_used("sprite1");

        // Add more to trigger cleanup
        manager.load_from_json("sprite4", sprite_json, ResourceType::Sprite).unwrap();
        manager.load_from_json("sprite5", sprite_json, ResourceType::Sprite).unwrap();

        // sprite1 should still be in cache (marked as used)
        assert!(manager.get("sprite1").is_some());
    }

    #[test]
    fn test_resource_manager_cleanup_type() {
        let mut manager = ResourceManager::default();
        let sprite_json = r#"{
            "name": "sprite1",
            "width": 32,
            "height": 32,
            "texture_path": "sprites/test.png",
            "frames": []
        }"#;
        let audio_json = r#"{
            "name": "audio1",
            "duration": 1.0,
            "path": "audio/test.wav",
            "channels": 2,
            "sample_rate": 44100,
            "format": "stereo"
        }"#;

        manager.load_from_json("sprite1", sprite_json, ResourceType::Sprite).unwrap();
        manager.load_from_json("audio1", audio_json, ResourceType::Audio).unwrap();
        manager.load_from_json("sprite2", sprite_json, ResourceType::Sprite).unwrap();

        assert_eq!(manager.len(), 3);

        // Cleanup sprite type
        manager.cleanup_type(ResourceType::Sprite);

        assert_eq!(manager.len(), 1);
        assert!(manager.get("audio1").is_some());
        assert!(manager.get("sprite1").is_none());
        assert!(manager.get("sprite2").is_none());
    }

    #[test]
    fn test_resource_manager_clear() {
        let mut manager = ResourceManager::default();
        let sprite_json = r#"{
            "name": "sprite1",
            "width": 32,
            "height": 32,
            "texture_path": "sprites/test.png",
            "frames": []
        }"#;

        manager.load_from_json("sprite1", sprite_json, ResourceType::Sprite).unwrap();
        assert_eq!(manager.len(), 1);

        manager.clear();
        assert_eq!(manager.len(), 0);
        assert!(manager.is_empty());
    }

    #[test]
    fn test_custom_resource() {
        let data = serde_json::json!({
            "name": "custom",
            "data": "test"
        });

        let resource = CustomResource::new("test", "custom".to_string(), &data);
        assert_eq!(resource.name(), "test");
        assert_eq!(resource.resource_type(), &ResourceType::Custom("custom".to_string()));
    }

    #[test]
    fn test_resource_manager_default() {
        let manager = ResourceManager::default();
        assert_eq!(manager.cache.len(), 0);
        assert_eq!(manager.config.max_size, 1000);
    }
}
