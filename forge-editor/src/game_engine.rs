use crate::sprite_manager::SpriteManager;
use crate::animation_system::AnimationSystem;
use crate::physics::isometric_collision::IsometricCollisionSystem;
use crate::physics::physics::{PhysicsConfig, PhysicsBody};
use crate::player_controller::PlayerController;
use crate::level_manager::Level;
use crate::tile_system::TileSystem;
use crate::camera::isometric_camera::IsometricCamera;
use crate::dialogue_system::DialogueSystem;
use crate::dialogue_ui::dialogue_panel::DialogueUI;
use crate::dialogue_manager::DialogueManager;
use crate::input::Input;
use crate::math::Vector2;

pub struct GameEngine {
    sprite_manager: SpriteManager,
    animation_system: AnimationSystem,
    physics_system: IsometricCollisionSystem,
    player_controller: PlayerController,
    level: Level,
    tile_system: TileSystem,
    camera: IsometricCamera,
    dialogue_system: DialogueSystem,
    dialogue_ui: DialogueUI,
    dialogue_manager: DialogueManager,
    input: Input,
}

impl GameEngine {
    pub fn new() -> Self {
        // Create systems
        let sprite_manager = SpriteManager::new();
        let animation_system = AnimationSystem::new(sprite_manager.clone());
        let physics_system = IsometricCollisionSystem::new();
        let level = Level::new("level1".to_string(), &LevelData::load("assets/levels/level1.json").unwrap());
        let tile_system = TileSystem::new(level.clone(), sprite_manager.clone());
        let camera = IsometricCamera::new(75.0, 1.0);
        let dialogue_system = DialogueSystem::new();
        let dialogue_ui = DialogueUI::new(dialogue_system.clone());
        let dialogue_manager = DialogueManager::new();
        let player_controller = PlayerController::new(
            crate::sprite_system::sprite_renderer::SpriteRenderer::new(sprite_manager.clone()),
            animation_system.clone()
        );

        GameEngine {
            sprite_manager,
            animation_system,
            physics_system,
            player_controller,
            level,
            tile_system,
            camera,
            dialogue_system,
            dialogue_ui,
            dialogue_manager,
            input: Input::new(),
        }
    }

    pub fn run(&mut self) {
        let mut time: f32 = 0.0;
        
        loop {
            self.input.poll_events();
            let dt = self.input.get_delta_time();

            // Update camera to follow player
            self.camera.follow_player(&self.player_controller.position);
            self.camera.update(dt);

            // Update physics and check collisions
            let collisions = self.physics_system.check_collisions();
            self.player_controller.update(dt, &self.input, &collisions);

            // Update visible tiles
            let cam_x = self.camera.position.x;
            let cam_y = self.camera.position.y;
            self.tile_system.update_visible_tiles(cam_x, cam_y, 10.0);

            // Update dialogue manager
            self.dialogue_manager.handle_dialogue_input(self.input.get_key());

            // Render
            self.render();

            // Next frame
            time += dt;
        }
    }

    fn render(&mut self) {
        // Draw tiles
        self.tile_system.draw_tiles(self.camera.position.x, self.camera.position.y);

        // Draw player
        self.player_controller.draw(time);

        // Draw dialogue UI
        self.dialogue_ui.draw();
    }
}
