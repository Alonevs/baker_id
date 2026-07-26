//! # Forge SDK - Ejemplo Plataformas
//! 
//! Ejemplo simple de juego de plataformas usando el SDK de Forge

use forge_runtime::entities::{Position2D, Size2D, Velocity2D, Color2D, Entity};

/// Entidad del jugador
pub struct Player {
    pub entity: Entity,
    pub is_jumping: bool,
}

impl Player {
    pub fn new() -> Self {
        let mut entity = Entity::new();
        entity.add_component(Position2D::new(100.0, 400.0));
        entity.add_component(Size2D::new(30.0, 50.0));
        entity.add_component(Velocity2D::new(0.0, 0.0));
        entity.add_component(Color2D::new(0.0, 1.0, 0.0)); // Verde
        Self { entity, is_jumping: false }
    }
}

/// Plataforma
pub struct Platform {
    pub entity: Entity,
    pub height: f32,
}

impl Platform {
    pub fn new(x: f32, y: f32, width: f32) -> Self {
        let mut entity = Entity::new();
        entity.add_component(Position2D::new(x, y));
        entity.add_component(Size2D::new(width, 20.0));
        entity.add_component(Color2D::new(0.8, 0.8, 0.8)); // Gris
        Self { entity, height: y }
    }
}

/// Sistema de Plataformas
pub struct PlatformerGame {
    pub player: Player,
    pub platforms: Vec<Platform>,
    pub running: bool,
}

impl PlatformerGame {
    pub fn new() -> Self {
        let mut platforms = Vec::new();
        
        // Plataforma inicial
        platforms.push(Platform::new(0.0, 400.0, 300.0));
        // Plataformas intermedias
        platforms.push(Platform::new(350.0, 300.0, 200.0));
        platforms.push(Platform::new(650.0, 200.0, 200.0));
        platforms.push(Platform::new(100.0, 100.0, 200.0));
        platforms.push(Platform::new(400.0, 150.0, 200.0));
        // Meta
        platforms.push(Platform::new(700.0, 50.0, 100.0));
        
        Self {
            player: Player::new(),
            platforms,
            running: true,
        }
    }
    
    /// Actualiza el juego
    pub fn update(&mut self, delta: f32, input: &Input) {
        // Obtener posición y velocidad del jugador
        let mut player_pos = self.player.entity.get_component::<Position2D>()
            .map(|p| p.position).unwrap_or(Position2D::new(100.0, 400.0).position);
        let mut player_vel = self.player.entity.get_component::<Velocity2D>()
            .map(|v| v.velocity).unwrap_or(Velocity2D::new(0.0, 0.0).velocity);
        
        // Gravedad
        player_vel.y += 980.0 * delta;
        
        // Movimiento horizontal
        if input.left {
            player_vel.x -= 400.0 * delta;
        }
        if input.right {
            player_vel.x += 400.0 * delta;
        }
        
        // Fricción
        player_vel.x *= 0.9;
        
        // Salto
        if input.jump && !self.player.is_jumping {
            player_vel.y = -600.0;
            self.player.is_jumping = true;
        }
        
        // Limites de velocidad
        player_vel.x = player_vel.x.clamp(-800.0, 800.0);
        
        // Actualizar posición X
        player_pos.x += player_vel.x * delta;
        
        // Colisiones X con plataformas
        for platform in &self.platforms {
            let plat_pos = platform.entity.get_component::<Position2D>()
                .map(|p| p.position).unwrap_or(Position2D::new(0.0, 0.0).position);
            let plat_size = platform.entity.get_component::<Size2D>()
                .map(|s| s.size).unwrap_or(Size2D::new(0.0, 0.0).size);
            
            if player_pos.x + 30.0 > plat_pos.x && player_pos.x < plat_pos.x + plat_size.x {
                if player_pos.y <= plat_pos.y + plat_size.y && player_pos.y + 50.0 >= plat_pos.y {
                    // Colisión X - rebote
                    player_vel.x = -player_vel.x * 0.5;
                    player_pos.x += player_vel.x * delta;
                }
            }
        }
        
        // Actualizar posición Y
        player_pos.y += player_vel.y * delta;
        
        // Colisiones Y con plataformas
        self.player.is_jumping = true; // Asumir que está en el aire
        for platform in &self.platforms {
            let plat_pos = platform.entity.get_component::<Position2D>()
                .map(|p| p.position).unwrap_or(Position2D::new(0.0, 0.0).position);
            let plat_size = platform.entity.get_component::<Size2D>()
                .map(|s| s.size).unwrap_or(Size2D::new(0.0, 0.0).size);
            
            if player_pos.x + 30.0 > plat_pos.x && player_pos.x < plat_pos.x + plat_size.x {
                if player_pos.y + 50.0 >= plat_pos.y && player_pos.y <= plat_pos.y + plat_size.y {
                    // Colisión Y - aterrizar
                    if player_vel.y > 0.0 && player_pos.y + 50.0 <= plat_pos.y + plat_size.y + 100.0f32 {
                        player_pos.y = plat_pos.y - 50.0;
                        player_vel.y = 0.0;
                        self.player.is_jumping = false;
                    }
                }
            }
        }
        
        // Limites del mundo
        player_pos.x = player_pos.x.clamp(0.0, 1200.0);
        player_pos.y = player_pos.y.clamp(0.0, 600.0);
        
        // Actualizar componentes
        if let Some(pos) = self.player.entity.get_component_mut::<Position2D>() {
            pos.position = player_pos;
        }
        if let Some(vel) = self.player.entity.get_component_mut::<Velocity2D>() {
            vel.velocity = player_vel;
        }
    }
    
    /// Renderiza el juego
    pub fn render(&self) {
        println!("\n🏃 Platformer - Forge SDK");
        println!("Player Position: ({:.1}, {:.1})", 
            self.player.entity.get_component::<Position2D>()
                .map(|p| p.position.x).unwrap_or(0.0),
            self.player.entity.get_component::<Position2D>()
                .map(|p| p.position.y).unwrap_or(0.0));
        println!("Controls: ← / → Move, ↑ Jump, q Quit");
    }
}

/// Input del juego
pub struct Input {
    pub left: bool,
    pub right: bool,
    pub jump: bool,
}

impl Default for Input {
    fn default() -> Self {
        Self {
            left: false,
            right: false,
            jump: false,
        }
    }
}

/// Main del ejemplo
pub fn main() {
    println!("🏃 Starting Platformer Example...\n");
    
    let mut game = PlatformerGame::new();
    let mut input = Input::default();
    
    println!("🎮 Platformer - Forge SDK Example");
    println!("==================================");
    println!();
    println!("📋 Controls:");
    println!("  ← / →  - Mover");
    println!("  ↑      - Saltar");
    println!("  q      - Salir");
    println!();
    println!("🏃 Platformer!");
    println!("==================================\n");
    
    loop {
        // Simular delta time
        let delta: f32 = 0.016; // ~60 FPS
        
        // Obtener input
        input = Input::default();
        let mut input_line = String::new();
        if std::io::stdin().read_line(&mut input_line).is_ok() {
            let input_str = input_line.trim().to_lowercase();
            
            if input_str == "q" {
                println!("\n👋 Goodbye!");
                break;
            } else if input_str == "left" || input_str == "←" {
                input.left = true;
            } else if input_str == "right" || input_str == "→" {
                input.right = true;
            } else if input_str == "jump" || input_str == "up" || input_str == "↑" {
                input.jump = true;
            }
        }
        
        // Actualizar juego
        game.update(delta, &input);
        
        // Renderizar
        game.render();
        
        // Control de salida
        println!("\nPress any key...");
        if let Ok(_) = std::io::stdin().read_line(&mut String::new()) {
            continue;
        }
        break;
    }
}
