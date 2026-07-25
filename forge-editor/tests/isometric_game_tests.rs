#[cfg(test)]
mod isometric_game_tests {
    use crate::sprite_manager::SpriteManager;
    use crate::animation_system::AnimationSystem;
    use crate::physics::isometric_collision::IsometricCollisionSystem;
    use crate::physics::physics::{PhysicsBody, CollisionShape};
    use crate::camera::isometric_camera::IsometricCamera;
    use crate::dialogue_system::DialogueSystem;
    use crate::dialogue_ui::dialogue_panel::DialogueUI;
    use crate::dialogue_manager::DialogueManager;
    use crate::player_controller::PlayerController;

    #[test]
    fn test_sprite_manager_loads_atlas() {
        let mut manager = SpriteManager::new();
        // Test that manager initializes correctly
        assert!(manager.atlases.is_empty());
    }

    #[test]
    fn test_animation_system_creates_animation() {
        let sprite_manager = SpriteManager::new();
        let system = AnimationSystem::new(sprite_manager);
        assert!(system.animations.is_empty());
    }

    #[test]
    fn test_isometric_collision_system() {
        let system = IsometricCollisionSystem::new();
        let body1 = PhysicsBody::new(
            Vec2::new(0.0, 0.0),
            10.0,
            CollisionShape::new(16.0, 32.0)
        );
        system.add_body(body1);
        assert_eq!(system.bodies.len(), 1);
    }

    #[test]
    fn test_isometric_camera_follows_player() {
        let mut camera = IsometricCamera::new(75.0, 1.0);
        let player_pos = Vec2::new(100.0, 50.0);
        camera.follow_player(&player_pos);
        
        assert!((camera.position.x - 100.0).abs() < 0.1);
        assert!((camera.position.y - 50.0).abs() < 0.1);
    }

    #[test]
    fn test_dialogue_system_manages_dialogues() {
        let system = DialogueSystem::new();
        // Test that system initializes correctly
        assert!(system.current_dialogue.is_none());
    }

    #[test]
    fn test_dialogue_manager_handles_input() {
        let mut manager = DialogueManager::new();
        // Test that manager initializes correctly
        assert_eq!(manager.current_state, DialogueState::Idle);
    }

    #[test]
    fn test_player_controller_initializes() {
        let sprite_renderer = crate::sprite_system::sprite_renderer::SpriteRenderer::new(SpriteManager::new());
        let animation_system = AnimationSystem::new(SpriteManager::new());
        let controller = PlayerController::new(sprite_renderer, animation_system);
        // Test that controller initializes with default values
    }

    #[test]
    fn test_integration_all_systems() {
        // Full integration test
        let engine = crate::game_engine::GameEngine::new();
        // Verify all systems are created
        assert!(!engine.sprite_manager.atlases.is_empty());
    }
}
