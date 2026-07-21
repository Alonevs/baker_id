//! forge-runtime - Motor de renderizado 2D (placeholder)
//!
//! Este runtime renderiza juegos 2D en tiempo real para preview en vivo en el editor.

pub mod entities;
pub mod commands;
pub mod render;
pub mod physics;
pub mod audio;
pub mod timeline;
pub mod event_system;
pub mod dialogue;

pub use entities::*;
pub use physics::*;
pub use audio::{AudioSystem, AudioManager};
pub use timeline::*;
pub use event_system::*;
pub use dialogue::{Dialogue, DialogueSystem};
pub use forge_types::GameType;

/// Configuración del runtime
#[derive(Clone)]
pub struct RuntimeConfig {
    pub window_title: String,
    pub window_size: (u32, u32),
    pub game_type: GameType,
}

impl Default for RuntimeConfig {
    fn default() -> Self {
        Self {
            window_title: "Forge Runtime".to_string(),
            window_size: (800, 600),
            game_type: GameType::Platformer,
        }
    }
}

/// Plugin del runtime
pub struct RuntimePlugin;

impl RuntimePlugin {
    pub fn run(_config: RuntimeConfig) -> Result<(), Box<dyn std::error::Error>> {
        // Placeholder - implementación futura con wgpu/winit
        println!("Runtime iniciado: {}", _config.window_title);
        Ok(())
    }
}
