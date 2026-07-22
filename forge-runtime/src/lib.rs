pub mod audio;
pub mod commands;
pub mod dialogue;
pub mod entities;
pub mod event_system;

pub use audio::AudioManager;
pub use commands::CommandManager;
pub use dialogue::DialogueManager;
pub use entities::EntityManager;
pub use event_system::EventManager;

pub struct Runtime;

impl Runtime {
    pub fn new() -> Self {
        Self
    }
}
