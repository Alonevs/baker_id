pub mod sprite_manager;
pub mod animation_system;
pub mod sprite_system {
    pub mod sprite_renderer;
}
pub mod physics {
    pub mod physics;
    pub mod isometric_collision;
}
pub mod player_controller;
pub mod level_manager;
pub mod tile_system;
pub mod camera {
    pub mod isometric_camera;
}
pub mod dialogue_system;
pub mod dialogue_ui {
    pub mod dialogue_panel;
}
pub mod dialogue_manager;
pub mod game_engine;
pub mod math;
pub mod input;
pub mod optimization;

pub use sprite_manager::SpriteManager;
pub use animation_system::AnimationSystem;
pub use sprite_system::sprite_renderer::SpriteRenderer;
pub use physics::physics::{PhysicsConfig, PhysicsBody, CollisionShape, CollisionResult};
pub use physics::isometric_collision::IsometricCollisionSystem;
pub use player_controller::PlayerController;
pub use level_manager::{Level, LevelData, Tile, Platform, SpawnPoint};
pub use tile_system::TileSystem;
pub use camera::isometric_camera::IsometricCamera;
pub use dialogue_system::{Dialogue, DialogueOption, DialogueAction, DialogueSystem};
pub use dialogue_ui::dialogue_panel::DialogueUI;
pub use dialogue_manager::DialogueManager;
pub use game_engine::GameEngine;
pub use math::{Vector2, Input, KeyCode};
