//! # Resource Manager
//! 
//! Sistema universal para cargar y gestionar recursos del juego:
//! - Sprites y texturas
//! - Audio (sonidos, música)
//! - Scripts de usuario
//! - Niveles y escenas
//! - Cualquier otro recurso personalizado

pub mod sprite_resource;
pub mod audio_resource;
pub mod script_resource;
pub mod level_resource;
pub mod resource_manager;

pub use resource_manager::{ResourceManager, ResourceType};
pub use sprite_resource::SpriteResource;
pub use audio_resource::AudioResource;
pub use script_resource::ScriptResource;
pub use level_resource::LevelResource;
