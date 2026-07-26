use crate::sprite_manager::SpriteManager;
use crate::animation_system::AnimationSystem;
use crate::physics::isometric_collision::IsometricCollisionSystem;
use crate::physics::physics::{PhysicsConfig, PhysicsBody};
use crate::player_controller::PlayerController;
use crate::level_manager::Level;
use crate::tile_system::TileSystem;
use forge_runtime::camera::{Camera, CameraType};
use crate::dialogue_system::DialogueSystem;
use crate::dialogue_ui::dialogue_panel::DialogueUI;
use crate::dialogue_manager::DialogueManager;
use crate::input::Input;
use crate::math::Vector2;

/// Soporte para cambiar entre diferentes tipos de cámara
pub enum CameraMode {
    Isometric,        // Vista diagonal 45° (Downwell, Into the Breach)
    SideScroller,     // Vista lateral (Metal Slug, plataformas)
    TopDown,          // Vista de pájaro (Zelda, Hotline Miami)
    Free2D,           // Vista libre con zoom (Naves, Shoot 'em up)
    Raycaster,        // Vista primera persona (Doom, Wolfenstein 3D)
    Static,           // Plano fijo (Novela visual, cinemáticas)
}

impl Default for CameraMode {
    fn default() -> Self {
        Self::Isometric
    }
}

pub struct GameEngine {
    sprite_manager: SpriteManager,
    animation_system: AnimationSystem,
    physics_system: IsometricCollisionSystem,
    player_controller: PlayerController,
    level: Level,
    tile_system: TileSystem,
    camera: Camera,
    camera_mode: CameraMode,
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
        let camera = Camera::new();
        camera.set_camera_type(CameraType::Isometric);
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
            camera_mode: CameraMode::Isometric,
            dialogue_system,
            dialogue_ui,
            dialogue_manager,
            input: Input::new(),
        }
    }

    /// Cambia el tipo de cámara actual
    pub fn set_camera_mode(&mut self, mode: CameraMode) {
        self.camera_mode = mode;
        match mode {
            CameraMode::Isometric => {
                self.camera.set_camera_type(CameraType::Isometric);
            }
            CameraMode::SideScroller => {
                self.camera.set_camera_type(CameraType::SideScroller);
            }
            CameraMode::TopDown => {
                self.camera.set_camera_type(CameraType::TopDown);
            }
            CameraMode::Free2D => {
                self.camera.set_camera_type(CameraType::Free2D);
            }
            CameraMode::Raycaster => {
                self.camera.set_camera_type(CameraType::Raycaster);
            }
            CameraMode::Static => {
                self.camera.set_camera_type(CameraType::Static);
            }
        }
    }

    /// Obtiene el tipo de cámara actual como string
    pub fn get_camera_mode_name(&self) -> &'static str {
        match self.camera_mode {
            CameraMode::Isometric => "Isométrica",
            CameraMode::SideScroller => "Side-Scroller",
            CameraMode::TopDown => "Top-Down",
            CameraMode::Free2D => "Free-2D",
            CameraMode::Raycaster => "Raycaster",
            CameraMode::Static => "Estática",
        }
    }

    /// Cambia el tipo de cámara con tecla
    pub fn toggle_camera_mode(&mut self, key: &str) {
        match self.camera_mode {
            CameraMode::Isometric => self.set_camera_mode(CameraMode::SideScroller),
            CameraMode::SideScroller => self.set_camera_mode(CameraMode::TopDown),
            CameraMode::TopDown => self.set_camera_mode(CameraMode::Free2D),
            CameraMode::Free2D => self.set_camera_mode(CameraMode::Raycaster),
            CameraMode::Raycaster => self.set_camera_mode(CameraMode::Static),
            CameraMode::Static => self.set_camera_mode(CameraMode::Isometric),
        }
    }

    pub fn run(&mut self) {
        let mut time: f32 = 0.0;
        
        loop {
            self.input.poll_events();
            let dt = self.input.get_delta_time();

            // Cambiar tipo de cámara con tecla 'C'
            if self.input.is_key_pressed('C') {
                self.toggle_camera_mode("C");
                println!("📷 Cámara cambiada a: {}", self.get_camera_mode_name());
            }

            // Actualizar cámara según tipo
            match self.camera_mode {
                CameraMode::Isometric | CameraMode::SideScroller | CameraMode::TopDown | CameraMode::Free2D => {
                    // Seguir al jugador
                    self.camera.follow_player(&self.player_controller.position);
                    self.camera.update(dt);
                }
                CameraMode::Raycaster => {
                    // Cámara estática en primera persona
                    self.camera.update(dt);
                }
                CameraMode::Static => {
                    // Cámara fija
                    self.camera.update(dt);
                }
            }

            // Actualizar physics y check collisions
            let collisions = self.physics_system.check_collisions();
            self.player_controller.update(dt, &self.input, &collisions);

            // Actualizar visible tiles según tipo de cámara
            self.tile_system.update_visible_tiles(&self.camera);

            // Actualizar dialogue manager
            self.dialogue_manager.handle_dialogue_input(self.input.get_key());

            // Render
            self.render();

            // Next frame
            time += dt;
        }
    }

    fn render(&mut self) {
        // Render según tipo de cámara
        self.tile_system.draw_tiles(&self.camera);
        self.player_controller.draw(time);

        // Draw dialogue UI
        self.dialogue_ui.draw();
    }

    fn render_raycaster(&mut self) {
        // Render estilo Doom/Wolfenstein 3D
        // Implementación futura
        println!("👁️ Render Raycaster (modo 3D)");
    }
}
