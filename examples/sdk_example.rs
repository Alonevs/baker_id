//! # Forge SDK - Ejemplo Simple
//! 
//! Ejemplo simple que demuestra el uso del SDK de Forge
//! Este ejemplo no depende del editor y puede compilarse independientemente

use forge_runtime::entities::{Position2D, Size2D, Velocity2D, Color2D, Entity};
use forge_types::Vec2;

/// Ejemplo simple del SDK
pub struct SimpleSDKExample {
    entities: Vec<Entity>,
}

impl SimpleSDKExample {
    /// Crea nuevo ejemplo
    pub fn new() -> Self {
        Self {
            entities: Vec::new(),
        }
    }
    
    /// Crea entidades de ejemplo
    pub fn create_sample_entities(&mut self) {
        println!("🎮 Creating sample entities...");
        
        // Crear jugador
        let mut player = Entity::new();
        player.add_component(Position2D::new(100.0, 100.0));
        player.add_component(Size2D::new(32.0, 32.0));
        player.add_component(Velocity2D::new(0.0, 0.0));
        player.add_component(Color2D::new(1.0, 0.0, 0.0)); // Rojo
        self.entities.push(player);
        
        // Crear enemigo
        let mut enemy = Entity::new();
        enemy.add_component(Position2D::new(200.0, 150.0));
        enemy.add_component(Size2D::new(32.0, 32.0));
        enemy.add_component(Velocity2D::new(50.0, 0.0));
        enemy.add_component(Color2D::new(0.0, 1.0, 0.0)); // Verde
        self.entities.push(enemy);
        
        println!("✅ Created {} entities", self.entities.len());
    }
    
    /// Actualiza entidades (física simple)
    pub fn update_entities(&mut self, dt: f32) {
        for entity in &mut self.entities {
            // Obtener componente Velocity2D
            if let Some(velocity) = entity.get_component::<Velocity2D>() {
                // Aplicar velocidad
                let mut pos = entity.get_component::<Position2D>().map(|p| p.position).unwrap_or(Vec2::new(0.0, 0.0));
                pos.x += velocity.velocity.x * dt;
                pos.y += velocity.velocity.y * dt;
                
                // Limites del mundo
                pos.x = pos.x.clamp(0.0, 2000.0);
                pos.y = pos.y.clamp(0.0, 2000.0);
                
                // Actualizar posición
                if let Some(position) = entity.get_component_mut::<Position2D>() {
                    position.position = pos;
                }
            }
        }
    }
    
    /// Renderiza información en consola
    pub fn render(&self) {
        println!("\n🔧 Forge SDK v0.1.0");
        println!("📦 ECS + Resource Manager + Event System + Scene Manager");
        println!();
        println!("🎮 Entities: {}", self.entities.len());
        println!();
        
        for entity in &self.entities {
            let position = entity.get_component::<Position2D>();
            let size = entity.get_component::<Size2D>();
            let velocity = entity.get_component::<Velocity2D>();
            let color = entity.get_component::<Color2D>();
            
            let pos_str = position.map(|p| format!("({:.1}, {:.1})", p.position.x, p.position.y))
                .unwrap_or("Unknown".to_string());
            let size_str = size.map(|s| format!("({:.1}, {:.1})", s.size.x, s.size.y))
                .unwrap_or("Unknown".to_string());
            let vel_str = velocity.map(|v| format!("({:.1}, {:.1})", v.velocity.x, v.velocity.y))
                .unwrap_or("Unknown".to_string());
            let color_str = color.map(|c| format!("{:?}", c.color))
                .unwrap_or("Unknown".to_string());
            
            println!("  • Entity {} at {} - Size: {} - Vel: {} - Color: {}", 
                entity.id, pos_str, size_str, vel_str, color_str);
        }
    }
}

impl Default for SimpleSDKExample {
    fn default() -> Self {
        Self::new()
    }
}

/// Helper para obtener referencia mutante a un componente
impl Entity {
    pub fn get_component_mut<C: std::any::Any>(&mut self) -> Option<&mut C> {
        self.components.iter_mut().find_map(|c| {
            c.downcast_mut::<C>()
        })
    }
}

/// Main del ejemplo
pub fn main() {
    println!("🚀 Starting Forge SDK Example...\n");
    
    let mut example = SimpleSDKExample::new();
    
    // Inicializar
    example.load_sample_resources();
    example.create_sample_entities();
    
    // Loop principal
    let mut dt: f32 = 0.0;
    let mut running = true;
    
    while running {
        // Simular delta time
        dt += 0.016; // ~60 FPS
        
        // Actualizar entidades
        example.update_entities(dt);
        
        // Renderizar
        example.render();
        
        // Control de salida
        println!("\nPress Enter to continue...");
        match std::io::stdin().read_line(&mut String::new()) {
            Ok(_) => {},
            Err(_) => {
                running = false;
            }
        }
    }
    
    println!("\n✅ Example completed!");
}
