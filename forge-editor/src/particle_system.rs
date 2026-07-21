use std::sync::Arc;
use std::sync::RwLock;
use serde::{Serialize, Deserialize};
use crate::math::Math;

/// Animación de un sprite (secuencia de frames)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpriteAnimation {
    pub frames: Vec<String>,
    pub fps: f32,
    pub loop_count: i32,
    pub start_frame: usize,
    pub end_frame: usize,
}

impl Default for SpriteAnimation {
    fn default() -> Self {
        Self {
            frames: vec![],
            fps: 30.0,
            loop_count: -1, // -1 = infinito
            start_frame: 0,
            end_frame: 0,
        }
    }
}

/// Partícula con sprite animado
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnimatedParticle {
    pub id: String,
    pub position: Vector2,
    pub velocity: Vector2,
    pub size: Vector2,
    pub color: String,
    pub sprite_path: String,
    pub animation: SpriteAnimation,
    pub lifetime: f32,
    pub current_frame: usize,
    pub frame_time: f32,
    pub is_active: bool,
}

/// Vector 2D
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Vector2 {
    pub x: f32,
    pub y: f32,
}

impl Vector2 {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
    
    pub fn zero() -> Self {
        Self { x: 0.0, y: 0.0 }
    }
}

/// Sistema de partículas con sprites animados
#[derive(Debug, Clone)]
pub struct ParticleSystem {
    pub particles: Arc<RwLock<Vec<AnimatedParticle>>>,
    pub gravity: Vector2,
    pub dt: f32,
}

impl Default for ParticleSystem {
    fn default() -> Self {
        Self {
            particles: Arc::new(RwLock::new(Vec::new())),
            gravity: Vector2::new(0.0, 9.8),
            dt: 1.0 / 60.0,
        }
    }
}

impl ParticleSystem {
    /// Crear una nueva partícula con sprite animado
    pub fn create_particle(&mut self, config: ParticleConfig) -> String {
        let id = format!("particle_{}", std::process::id());
        
        let particle = AnimatedParticle {
            id: id.clone(),
            position: Vector2::new(config.position.x, config.position.y),
            velocity: Vector2::new(config.velocity.x, config.velocity.y),
            size: Vector2::new(config.size.x, config.size.y),
            color: config.color,
            sprite_path: config.sprite_path,
            animation: config.animation.clone(),
            lifetime: config.lifetime,
            current_frame: 0,
            frame_time: 1.0 / config.animation.fps,
            is_active: true,
        };
        
        self.particles.write().unwrap().push(particle);
        id
    }

    /// Obtener referencia a una partícula
    pub fn get_particle(&self, id: &str) -> Option<AnimatedParticle> {
        self.particles.read().unwrap().iter()
            .find(|p| p.id == id)
            .cloned()
    }

    /// Obtener referencia mutada a una partícula
    pub fn get_particle_mut(&mut self, id: &str) -> Option<AnimatedParticle> {
        self.particles.write().unwrap().iter_mut()
            .find(|p| p.id == id)
            .cloned()
    }

    /// Remover partícula
    pub fn remove_particle(&mut self, id: &str) -> bool {
        self.particles.write().unwrap().retain(|p| p.id != id);
        self.particles.write().unwrap().iter().any(|p| p.id == id)
    }

    /// Emitir partículas desde una posición
    pub fn emit(&mut self, config: ParticleEmitterConfig) {
        for _ in 0..config.count {
            let velocity = Vector2::new(
                (config.velocity_min.x + (config.velocity_max.x - config.velocity_min.x) * (Math::random() as f32))
                    .clamp(config.velocity_min.x, config.velocity_max.x),
                (config.velocity_min.y + (config.velocity_max.y - config.velocity_min.y) * (Math::random() as f32))
                    .clamp(config.velocity_min.y, config.velocity_max.y),
            );
            
            let id = format!("particle_{}_{}", std::process::id(), Math::random());
            let color = config.color.clone();
            let sprite_path = config.sprite_path.clone();
            let animation = config.animation.clone();
            let position = Vector2::new(config.position.x, config.position.y);
            let size = Vector2::new(config.size.x, config.size.y);
            let frame_time = 1.0 / animation.fps;
            
            let particle = AnimatedParticle {
                id,
                position,
                velocity,
                size,
                color,
                sprite_path,
                animation,
                lifetime: config.lifetime,
                current_frame: 0,
                frame_time,
                is_active: true,
            };
            
            self.particles.write().unwrap().push(particle);
        }
    }

    /// Actualizar sistema de partículas
    pub fn update(&mut self) {
        let mut particles = self.particles.write().unwrap();
        let _initial_count = particles.len();
        
        for particle in particles.iter_mut() {
            if !particle.is_active {
                continue;
            }
            
            // Actualizar posición
            particle.position.x += particle.velocity.x * self.dt;
            particle.position.y += particle.velocity.y * self.dt;
            
            // Aplicar gravedad
            particle.velocity.y += self.gravity.y * self.dt;
            
            // Actualizar animación
            if particle.animation.frames.is_empty() {
                continue;
            }
            
            // Avanzar frame
            particle.frame_time -= self.dt;
            if particle.frame_time <= 0.0 {
                if particle.animation.loop_count < 0 || particle.current_frame < particle.animation.end_frame {
                    particle.current_frame = (particle.current_frame + 1) % (particle.animation.end_frame - particle.animation.start_frame + 1);
                    particle.frame_time = 1.0 / particle.animation.fps;
                }
            }
            
            // Verificar vida
            particle.lifetime -= self.dt;
            if particle.lifetime <= 0.0 {
                particle.is_active = false;
            }
        }
        
        // Limpiar partículas inactivas
        particles.retain(|p| p.is_active);
    }

    /// Obtener número de partículas activas
    pub fn particle_count(&self) -> usize {
        self.particles.read().unwrap().iter().filter(|p| p.is_active).count()
    }

    /// Limpiar todas las partículas
    pub fn clear(&mut self) {
        self.particles.write().unwrap().clear();
    }

    /// Exportar a formato .map
    pub fn export_to_map(&self) -> String {
        let particles = self.particles.read().unwrap();
        let mut content = String::new();
        
        content.push_str("PARTICLES_DATA\n");
        content.push_str(format!("PARTICLE_COUNT: {}\n", particles.len()).as_str());
        
        for particle in particles.iter() {
            content.push_str(&format!(
                "PARTICLE:{},{},{},{},{},{},{},{},{},{},{},{}\n",
                particle.id,
                particle.position.x,
                particle.position.y,
                particle.size.x,
                particle.size.y,
                particle.velocity.x,
                particle.velocity.y,
                particle.lifetime,
                particle.current_frame,
                particle.frame_time,
                particle.animation.fps,
                particle.animation.loop_count
            ));
        }
        
        content
    }

    /// Importar desde formato .map
    pub fn import_from_map(&mut self, content: &str) {
        let lines: Vec<&str> = content.lines().collect();
        
        for line in lines {
            let parts: Vec<&str> = line.split(',').collect();
            if parts.len() >= 12 && line.starts_with("PARTICLE:") {
                let id = parts[1].trim().to_string();
                let x: f32 = parts[2].trim().parse().unwrap_or(0.0);
                let y: f32 = parts[3].trim().parse().unwrap_or(0.0);
                let width: f32 = parts[4].trim().parse().unwrap_or(10.0);
                let height: f32 = parts[5].trim().parse().unwrap_or(10.0);
                let vx: f32 = parts[6].trim().parse().unwrap_or(0.0);
                let vy: f32 = parts[7].trim().parse().unwrap_or(0.0);
                let lifetime: f32 = parts[8].trim().parse().unwrap_or(5.0);
                let frame: usize = parts[9].trim().parse().unwrap_or(0);
                let frame_time: f32 = parts[10].trim().parse().unwrap_or(0.033);
                let fps: f32 = parts[11].trim().parse().unwrap_or(30.0);
                let loop_count: i32 = parts[12].trim().parse().unwrap_or(-1);
                
                let particle = AnimatedParticle {
                    id,
                    position: Vector2::new(x, y),
                    velocity: Vector2::new(vx, vy),
                    size: Vector2::new(width, height),
                    color: "red".to_string(),
                    sprite_path: "default.png".to_string(),
                    animation: SpriteAnimation {
                        frames: vec![],
                        fps,
                        loop_count,
                        start_frame: 0,
                        end_frame: fps as usize - 1,
                    },
                    lifetime,
                    current_frame: frame,
                    frame_time,
                    is_active: true,
                };
                
                self.particles.write().unwrap().push(particle);
            }
        }
    }
}

/// Configuración para crear una partícula
#[derive(Debug, Clone)]
pub struct ParticleConfig {
    pub position: Vector2,
    pub velocity: Vector2,
    pub size: Vector2,
    pub color: String,
    pub sprite_path: String,
    pub animation: SpriteAnimation,
    pub lifetime: f32,
}

/// Configuración para emitir múltiples partículas
#[derive(Debug, Clone)]
pub struct ParticleEmitterConfig {
    pub position: Vector2,
    pub velocity_min: Vector2,
    pub velocity_max: Vector2,
    pub size: Vector2,
    pub color: String,
    pub sprite_path: String,
    pub animation: SpriteAnimation,
    pub lifetime: f32,
    pub count: usize,
}

/// Utilidades matemáticas
pub mod math {
    pub fn random() -> f32 {
        std::time::Instant::now().elapsed().as_nanos() as f32 / 1_000_000_000.0
    }
}

impl ParticleSystem {
    pub fn random() -> f32 {
        math::random()
    }
}

