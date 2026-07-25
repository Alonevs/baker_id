//! # Forge Runtime Main
//! 
//! Runtime principal con Toolbar y Play Mode.

use crate::toolbar::{Toolbar, ToolbarButtons};
use crate::play_mode::{PlayMode, PlayModeState, Entity};

fn main() {
    println!("Forge Runtime iniciado");
    
    // Crear Toolbar
    let mut toolbar = Toolbar::new();
    
    // Crear Play Mode
    let mut play_mode = PlayMode::new();
    
    // Crear algunas entidades de prueba
    let entities = vec![
        Entity::new("player_1".to_string(), (100.0, 200.0)),
        Entity::new("enemy_1".to_string(), (300.0, 200.0)),
        Entity::new("platform_1".to_string(), (150.0, 300.0)),
    ];
    
    println!("{} entidades creadas", entities.len());
    
    // Iniciar Play Mode
    play_mode.start();
    println!("Play Mode iniciado");
    
    // Simular update
    for i in 1..=10 {
        play_mode.update(0.016); // ~60 FPS
        println!("Frame {}: delta = {}", i, play_mode.get_delta());
    }
    
    // Detener Play Mode
    play_mode.stop();
    println!("Play Mode detenido");
    
    println!("Runtime finalizado");
}
