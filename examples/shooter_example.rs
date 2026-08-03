//! # Forge SDK - Ejemplo Shooter
//! 
//! Ejemplo simple de juego de disparos usando el SDK de Forge

use forge_runtime::entities::{Position2D, Size2D, Velocity2D, Color2D, Entity};

/// Entidad del jugador
pub struct Player {
    pub entity: Entity,
    pub health: i32,
    pub score: i32,
}

impl Player {
    pub fn new() -> Self {
        let mut entity = Entity::new();
        entity.add_component(Position2D::new(400.0, 300.0));
        entity.add_component(Size2D::new(40.0, 40.0));
        entity.add_component(Velocity2D::new(0.0, 0.0));
        entity.add_component(Color2D::new(0.0, 0.0, 1.0)); // Azul
        Self { entity, health: 100, score: 0 }
    }
}

/// Enemigo
pub struct Enemy {
    pub entity: Entity,
    pub health: i32,
    pub score_value: i32,
}

impl Enemy {
    pub fn new(x: f32, y: f32) -> Self {
        let mut entity = Entity::new();
        entity.add_component(Position2D::new(x, y));
        entity.add_component(Size2D::new(30.0, 30.0));
        entity.add_component(Velocity2D::new(0.0, 0.0));
        entity.add_component(Color2D::new(1.0, 0.0, 0.0)); // Rojo
        Self { entity, health: 3, score_value: 10 }
    }
    
    pub fn take_damage(&mut self, damage: i32) {
        self.health -= damage;
        if self.health <= 0 {
            self.health = 0;
            println!("🎯 Enemy destroyed! Score: +{}", self.score_value);
        }
    }
}

/// Proyectil
pub struct Bullet {
    pub entity: Entity,
    pub damage: i32,
}

impl Bullet {
    pub fn new(x: f32, y: f32, angle: f32) -> Self {
        let mut entity = Entity::new();
        entity.add_component(Position2D::new(x, y));
        entity.add_component(Size2D::new(5.0, 5.0));
        entity.add_component(Velocity2D::new(0.0, 0.0));
        entity.add_component(Color2D::new(1.0, 1.0, 0.0)); // Amarillo
        Self { entity, damage: 1 }
    }
    
    pub fn set_velocity(&mut self, angle: f32, speed: f32) {
        let vel_x = speed * angle.cos();
        let vel_y = speed * angle.sin();
        if let Some(vel) = self.entity.get_component_mut::<Velocity2D>() {
            vel.velocity = Velocity2D::new(vel_x, vel_y).velocity;
        }
    }
}

/// Sistema de Disparos
pub struct ShooterGame {
    pub player: Player,
    pub enemies: Vec<Enemy>,
    pub bullets: Vec<Bullet>,
    
    pub running: bool,
    pub enemy_spawn_timer: f32,
}

impl ShooterGame {
    pub fn new() -> Self {
        let mut enemies = Vec::new();
        let mut bullets = Vec::new();
        
        // Crear enemigos iniciales
        for i in 0..5 {
            let x = 100.0 + (i as f32 * 120.0);
            let y = 100.0 + (i as f32 * 80.0);
            enemies.push(Enemy::new(x, y));
        }
        
        Self {
            player: Player::new(),
            enemies,
            bullets,
            running: true,
            enemy_spawn_timer: 0.0,
        }
    }
    
    /// Actualiza el juego
    pub fn update(&mut self, delta: f32, input: &Input) {
        // Mover jugador
        let mut player_pos = self.player.entity.get_component::<Position2D>()
            .map(|p| p.position).unwrap_or(Position2D::new(400.0, 300.0).position);
        
        if input.left {
            player_pos.x -= 300.0 * delta;
        }
        if input.right {
            player_pos.x += 300.0 * delta;
        }
        if input.up {
            player_pos.y -= 300.0 * delta;
        }
        if input.down {
            player_pos.y += 300.0 * delta;
        }
        
        // Limites del mundo
        player_pos.x = player_pos.x.clamp(0.0, 800.0);
        player_pos.y = player_pos.y.clamp(0.0, 600.0);
        
        // Disparar
        if input.space {
            let angle = input.mouse_angle.unwrap_or(0.0);
            let bullet = Bullet::new(
                player_pos.x + 20.0,
                player_pos.y + 20.0,
                angle
            );
            bullet.set_velocity(angle, 800.0);
            self.bullets.push(bullet);
            println!("🔫 Shot!");
        }
        
        // Actualizar proyectiles
        for bullet in &mut self.bullets {
            let mut bullet_pos = bullet.entity.get_component::<Position2D>()
                .map(|p| p.position).unwrap_or(forge_types::Vec2::new(0.0, 0.0));
            let mut bullet_vel = bullet.entity.get_component::<Velocity2D>()
                .map(|v| v.velocity).unwrap_or(forge_types::Vec2::new(0.0, 0.0));
            
            bullet_pos.x += bullet_vel.x * delta;
            bullet_pos.y += bullet_vel.y * delta;
            
            // Limites
            bullet_pos.x = bullet_pos.x.clamp(0.0, 800.0);
            bullet_pos.y = bullet_pos.y.clamp(0.0, 600.0);
            
            // Colisiones con enemigos
            for enemy in &mut self.enemies {
                let enemy_pos = enemy.entity.get_component::<Position2D>()
                    .map(|p| p.position).unwrap_or(forge_types::Vec2::new(0.0, 0.0));
                
                // Colisión simple
                let dist = ((bullet_pos.x - enemy_pos.x).powi(2) + (bullet_pos.y - enemy_pos.y).powi(2)).sqrt();
                if dist < 30.0 {
                    enemy.take_damage(self.bullet.damage);
                    bullet.entity.add_component(Position2D::new(-1.0, -1.0)); // Marcar para eliminar
                }
            }
            
            // Actualizar componente
            if let Some(pos) = bullet.entity.get_component_mut::<Position2D>() {
                pos.position = bullet_pos;
            }
            if let Some(vel) = bullet.entity.get_component_mut::<Velocity2D>() {
                vel.velocity = bullet_vel;
            }
        }
        
        // Eliminar enemigos muertos
        self.enemies.retain(|enemy| {
            enemy.entity.get_component::<Position2D>()
                .map(|p| p.position.x) != Some(-1.0)
        });
        
        // Eliminar proyectiles fuera de pantalla
        self.bullets.retain(|bullet| {
            let pos = bullet.entity.get_component::<Position2D>()
                .map(|p| p.position).unwrap_or(forge_types::Vec2::new(0.0, 0.0));
            pos.x >= 0.0 && pos.x <= 800.0 && pos.y >= 0.0 && pos.y <= 600.0
        });
        
        // Spawnear enemigos
        self.enemy_spawn_timer += delta;
        if self.enemy_spawn_timer > 2.0 && self.enemies.len() < 15 {
            let x = (100.0 + (self.enemies.len() as f32 * 100.0)).clamp(0.0, 700.0);
            let y = 100.0 + (self.enemies.len() as f32 * 50.0);
            self.enemies.push(Enemy::new(x, y));
            self.enemy_spawn_timer = 0.0;
        }
        
        // Actualizar jugador
        if let Some(pos) = self.player.entity.get_component_mut::<Position2D>() {
            pos.position = player_pos;
        }
    }
    
    /// Renderiza el juego
    pub fn render(&self) {
        println!("\n🔫 Shooter - Forge SDK");
        println!("Player: Health={}/100, Score={}", 
            self.player.health, self.player.score);
        println!("Enemies: {}", self.enemies.len());
        println!("Bullets: {}", self.bullets.len());
        println!("Controls: WASD Move, Space Shoot, q Quit");
    }
}

/// Input del juego
pub struct Input {
    pub left: bool,
    pub right: bool,
    pub up: bool,
    pub down: bool,
    pub space: bool,
    pub mouse_angle: Option<f32>,
}

impl Default for Input {
    fn default() -> Self {
        Self {
            left: false,
            right: false,
            up: false,
            down: false,
            space: false,
            mouse_angle: None,
        }
    }
}

/// Main del ejemplo
pub fn main() {
    println!("🔫 Starting Shooter Example...\n");
    
    let mut game = ShooterGame::new();
    let mut input = Input::default();
    
    println!("🎮 Shooter - Forge SDK Example");
    println!("==============================");
    println!();
    println!("📋 Controls:");
    println!("  W/A/S/D - Mover");
    println!("  Space   - Disparar");
    println!("  q       - Salir");
    println!();
    println!("🔫 Shooter!");
    println!("==============================\n");
    
    loop {
        // Obtener input
        if let Ok(ref mut stdin) = std::io::stdin() {
            if stdin.read_line(&mut String::new()).is_ok() {
                let input_str = String::from_utf8_lossy(stdin.lock().as_mut_slice()).trim().to_lowercase();
                input = Input::default();
                
                if input_str == "q" {
                    println!("\n👋 Goodbye!");
                    break;
                } else if input_str == "left" || input_str == "a" {
                    input.left = true;
                } else if input_str == "right" || input_str == "d" {
                    input.right = true;
                } else if input_str == "up" || input_str == "w" {
                    input.up = true;
                } else if input_str == "down" || input_str == "s" {
                    input.down = true;
                } else if input_str == "space" || input_str == " " {
                    input.space = true;
                }
            }
        }
        
        // Simular delta time
        let delta: f32 = 0.016; // ~60 FPS
        
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
