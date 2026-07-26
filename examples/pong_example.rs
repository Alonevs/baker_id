//! # Forge SDK - Ejemplo Pong
//! 
//! Ejemplo simple del juego Pong usando el SDK de Forge

use forge_runtime::entities::{Position2D, Size2D, Velocity2D, Color2D, Entity};

/// Entidad del jugador (paleta izquierda)
pub struct Player {
    pub entity: Entity,
    pub score: i32,
}

impl Player {
    pub fn new(x: f32) -> Self {
        let mut entity = Entity::new();
        entity.add_component(Position2D::new(x, 100.0));
        entity.add_component(Size2D::new(10.0, 80.0));
        entity.add_component(Velocity2D::new(0.0, 0.0));
        entity.add_component(Color2D::new(0.0, 0.5, 1.0)); // Azul
        Self { entity, score: 0 }
    }
}

/// Entidad de la IA (paleta derecha)
pub struct Ai {
    pub entity: Entity,
}

impl Ai {
    pub fn new(x: f32) -> Self {
        let mut entity = Entity::new();
        entity.add_component(Position2D::new(x, 100.0));
        entity.add_component(Size2D::new(10.0, 80.0));
        entity.add_component(Velocity2D::new(0.0, 0.0));
        entity.add_component(Color2D::new(1.0, 1.0, 1.0)); // Blanco
        Self { entity }
    }
}

/// Entidad de la pelota
pub struct Ball {
    pub entity: Entity,
    pub score: i32,
}

impl Ball {
    pub fn new() -> Self {
        let mut entity = Entity::new();
        entity.add_component(Position2D::new(400.0, 300.0));
        entity.add_component(Size2D::new(10.0, 10.0));
        entity.add_component(Velocity2D::new(100.0, 50.0));
        entity.add_component(Color2D::new(1.0, 1.0, 0.0)); // Amarillo
        Self { entity, score: 0 }
    }
    
    pub fn reset(&mut self) {
        self.entity.add_component(Position2D::new(400.0, 300.0));
        self.entity.add_component(Velocity2D::new(100.0, 50.0));
    }
}

/// Sistema de Pong
pub struct PongGame {
    pub player: Player,
    pub ai: Ai,
    pub ball: Ball,
    pub running: bool,
}

impl PongGame {
    pub fn new() -> Self {
        Self {
            player: Player::new(100.0),
            ai: Ai::new(700.0),
            ball: Ball::new(),
            running: true,
        }
    }
    
    /// Actualiza el juego
    pub fn update(&mut self, delta: f32, input: &Input) {
        // Mover pelota
        let mut ball_pos = self.ball.entity.get_component::<Position2D>()
            .map(|p| p.position).unwrap_or(Position2D::new(400.0, 300.0).position);
        let mut ball_vel = self.ball.entity.get_component::<Velocity2D>()
            .map(|v| v.velocity).unwrap_or(Velocity2D::new(100.0, 50.0).velocity);
        
        ball_pos.x += ball_vel.x * delta;
        ball_pos.y += ball_vel.y * delta;
        
        // Rebote en techo y suelo
        if ball_pos.y < 0.0 || ball_pos.y > 600.0 {
            ball_vel.y = -ball_vel.y;
        }
        
        // Rebote en paletas
        // Paleta izquierda (jugador)
        if ball_pos.x <= 110.0 && ball_pos.y >= self.player.entity.get_component::<Position2D>()
            .map(|p| p.position.y).unwrap_or(100.0) - 40.0 && ball_pos.y <= self.player.entity.get_component::<Position2D>()
            .map(|p| p.position.y).unwrap_or(100.0) + 40.0 {
            ball_vel.x = -ball_vel.x * 1.1;
            ball_vel.y += (ball_pos.y - (self.player.entity.get_component::<Position2D>()
                .map(|p| p.position.y).unwrap_or(100.0))) * 0.1;
        }
        
        // Paleta derecha (IA)
        if ball_pos.x >= 690.0 && ball_pos.y >= self.ai.entity.get_component::<Position2D>()
            .map(|p| p.position.y).unwrap_or(100.0) - 40.0 && ball_pos.y <= self.ai.entity.get_component::<Position2D>()
            .map(|p| p.position.y).unwrap_or(100.0) + 40.0 {
            ball_vel.x = -ball_vel.x * 1.1;
        }
        
        // Puntuación
        if ball_pos.x < 0.0 {
            self.player.score += 1;
            self.ball.reset();
        } else if ball_pos.x > 800.0 {
            self.player.score += 1;
            self.ball.reset();
        }
        
        // Actualizar componentes
        if let Some(pos) = self.ball.entity.get_component_mut::<Position2D>() {
            pos.position = ball_pos;
        }
        if let Some(vel) = self.ball.entity.get_component_mut::<Velocity2D>() {
            vel.velocity = ball_vel;
        }
        
        // Mover paletas
        if input.up {
            let mut player_pos = self.player.entity.get_component::<Position2D>()
                .map(|p| p.position).unwrap_or(Position2D::new(100.0, 100.0).position);
            player_pos.y -= 200.0 * delta;
            player_pos.y = player_pos.y.clamp(0.0, 600.0 - 80.0);
            if let Some(pos) = self.player.entity.get_component_mut::<Position2D>() {
                pos.position = player_pos;
            }
        }
        
        if input.down {
            let mut player_pos = self.player.entity.get_component::<Position2D>()
                .map(|p| p.position).unwrap_or(Position2D::new(100.0, 100.0).position);
            player_pos.y += 200.0 * delta;
            player_pos.y = player_pos.y.clamp(0.0, 600.0 - 80.0);
            if let Some(pos) = self.player.entity.get_component_mut::<Position2D>() {
                pos.position = player_pos;
            }
        }
        
        // IA simple
        let ai_pos = self.ai.entity.get_component::<Position2D>()
            .map(|p| p.position).unwrap_or(Position2D::new(700.0, 100.0).position);
        let mut ai_pos = ai_pos;
        ai_pos.y += (ball_pos.y - ai_pos.y) * 300.0 * delta;
        ai_pos.y = ai_pos.y.clamp(0.0, 600.0 - 80.0);
        if let Some(pos) = self.ai.entity.get_component_mut::<Position2D>() {
            pos.position = ai_pos;
        }
    }
    
    /// Renderiza el juego
    pub fn render(&self) {
        println!("\n🏓 Pong - Forge SDK");
        println!("Score: {}", self.player.score);
        println!("Controls: ↑ / ↓ para mover paleta");
        println!("Press 'q' to quit");
    }
}

/// Input del juego
pub struct Input {
    pub up: bool,
    pub down: bool,
}

impl Default for Input {
    fn default() -> Self {
        Self {
            up: false,
            down: false,
        }
    }
}

/// Main del ejemplo
pub fn main() {
    println!("🏓 Starting Pong Example...\n");
    
    let mut game = PongGame::new();
    let mut input = Input::default();
    
    println!("🎮 Pong - Forge SDK Example");
    println!("================================");
    println!();
    
    println!("📋 Controls:");
    println!("  ↑ / ↓  - Mover paleta");
    println!("  q     - Salir");
    println!();
    println!("🏓 Pong!");
    println!("================================\n");
    
    loop {
        // Simular delta time
        let delta: f32 = 0.016; // ~60 FPS
        
        // Actualizar juego
        game.update(delta, &input);
        
        // Renderizar
        game.render();
        
        // Control de salida
        println!("\nPress any key...");
        let mut input_line = String::new();
        if std::io::stdin().read_line(&mut input_line).is_ok() {
            let trimmed = input_line.trim();
            if trimmed == "q" {
                println!("\n👋 Goodbye!");
                break;
            }
        }
    }
}
