//! Sprite Resource - Gestión de sprites y texturas

use serde::{Deserialize, Serialize};
use std::path::Path;

// Importar trait Resource del módulo padre
use super::resource_manager::{Resource, ResourceType};

/// Sprite Resource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpriteResource {
    pub name: String,
    pub width: u32,
    pub height: u32,
    pub texture_path: Option<String>,
    pub frames: Option<Vec<SpriteFrame>>,
    pub atlas: Option<String>,
    pub metadata: Option<serde_json::Value>,
}

impl SpriteResource {
    /// Crea nuevo sprite
    pub fn new(name: &str, width: u32, height: u32) -> Self {
        Self {
            name: name.to_string(),
            width,
            height,
            texture_path: None,
            frames: None,
            atlas: None,
            metadata: None,
        }
    }

    /// Crea sprite desde JSON
    pub fn from_json(name: &str, data: &serde_json::Value) -> Result<Self, String> {
        let width = data["width"]
            .as_u64()
            .map(|w| w as u32)
            .ok_or_else(|| format!("Missing width in sprite: {}", name))?;

        let height = data["height"]
            .as_u64()
            .map(|h| h as u32)
            .ok_or_else(|| format!("Missing height in sprite: {}", name))?;

        Ok(Self {
            name: name.to_string(),
            width,
            height,
            texture_path: data["texture_path"].as_str().map(|s| s.to_string()),
            frames: data["frames"].as_array().map(|f| {
                f.iter()
                    .filter_map(|frame| SpriteFrame::from_json(frame).ok())
                    .collect()
            }),
            atlas: data["atlas"].as_str().map(|s| s.to_string()),
            metadata: data.get("metadata").cloned(),
        })
    }

    /// Obtiene tamaño del sprite
    pub fn size(&self) -> (u32, u32) {
        (self.width, self.height)
    }

    /// Verifica si tiene frames (animación)
    pub fn is_animated(&self) -> bool {
        self.frames.is_some() && !self.frames.as_ref().unwrap().is_empty()
    }

    /// Obtiene número de frames
    pub fn frame_count(&self) -> usize {
        self.frames.as_ref().map(|f| f.len()).unwrap_or(0)
    }
}

impl Default for SpriteResource {
    fn default() -> Self {
        Self::new("default", 64, 64)
    }
}

/// Frame de animación
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpriteFrame {
    pub name: String,
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub duration: f32,
    pub metadata: Option<serde_json::Value>,
}

impl SpriteFrame {
    /// Crea nuevo frame
    pub fn new(name: &str, x: f32, y: f32, width: f32, height: f32, duration: f32) -> Self {
        Self {
            name: name.to_string(),
            x,
            y,
            width,
            height,
            duration,
            metadata: None,
        }
    }

    /// Crea frame desde JSON
    pub fn from_json(data: &serde_json::Value) -> Result<Self, String> {
        let name = data["name"]
            .as_str()
            .ok_or_else(|| "Missing frame name".to_string())?
            .to_string();

        let x = data["x"]
            .as_f64()
            .ok_or_else(|| "Missing frame x".to_string())?
            as f32;

        let y = data["y"]
            .as_f64()
            .ok_or_else(|| "Missing frame y".to_string())?
            as f32;

        let width = data["width"]
            .as_f64()
            .ok_or_else(|| "Missing frame width".to_string())?
            as f32;

        let height = data["height"]
            .as_f64()
            .ok_or_else(|| "Missing frame height".to_string())?
            as f32;

        let duration = data["duration"]
            .as_f64()
            .ok_or_else(|| "Missing frame duration".to_string())?
            as f32;

        Ok(Self {
            name,
            x,
            y,
            width,
            height,
            duration,
            metadata: data.get("metadata").cloned(),
        })
    }

    /// Obtiene duración en milisegundos
    pub fn duration_ms(&self) -> f32 {
        self.duration * 1000.0
    }
}

// Implementación del trait Resource
impl Resource for SpriteResource {
    fn name(&self) -> &str { &self.name }
    fn resource_type(&self) -> &ResourceType { &ResourceType::Sprite }
    fn load(&mut self) -> Result<(), String> { Ok(()) }
    fn unload(self) where Self: Sized {}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sprite_resource_new() {
        let sprite = SpriteResource::new("test", 64, 64);
        assert_eq!(sprite.width, 64);
        assert_eq!(sprite.height, 64);
    }

    #[test]
    fn test_sprite_resource_default() {
        let sprite = SpriteResource::default();
        assert_eq!(sprite.width, 64);
        assert_eq!(sprite.height, 64);
    }

    #[test]
    fn test_sprite_frame_new() {
        let frame = SpriteFrame::new("frame1", 0.0, 0.0, 64.0, 64.0, 0.1);
        assert_eq!(frame.name, "frame1");
        assert_eq!(frame.duration, 0.1);
    }
}
