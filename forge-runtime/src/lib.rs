//! forge-runtime - Motor de renderizado 2D

//! Este runtime renderiza juegos 2D en tiempo real para preview en vivo en el editor.

pub mod entities;
pub mod commands;
pub mod render;
pub mod physics;
pub mod audio;
pub mod timeline;
pub mod timeline_system;
pub mod timeline_integration_tests;
pub mod event_system;
pub mod dialogue;
pub mod play_mode;
pub mod components;
pub mod animation;
pub mod animation_clips;
pub mod viewport;
pub mod camera;
pub mod math;

pub use physics::*;
pub use timeline::timeline_manager::TimelineManager;
pub use event_system::*;
pub use dialogue::{Dialogue, DialogueSystem};
pub use components::AnimationComponent;
pub use forge_types::GameType;
pub use viewport::viewport::{Viewport, ViewportConfig};
pub use camera::*;
pub use math::*;
