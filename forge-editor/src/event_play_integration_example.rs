//! # Event Forge - Play Session Integration - Ejemplo de Uso
//! 
//! Este módulo muestra cómo usar la integración de eventos en Play Session.

use crate::event_play_integration::EventPlayIntegration;
use crate::event_node_manager::EventNodeManager;
use crate::play_session::{PlaySession, Entity};

/// Ejemplo de uso de la integración de eventos
pub fn example_usage() {
    println!("[EVENT-PLAY INTEGRATION] Ejemplo de uso");
    
    // 1. Crear EventNodeManager con nodos de eventos
    let mut event_manager = EventNodeManager::new();
    
    // Agregar nodos de eventos
    event_manager.add_node("start_trigger", crate::event_nodes::EventType::OnStart, 
        crate::event_nodes::NodeData::OnStart { auto_execute: true });
    
    event_manager.add_node("move_player", crate::event_nodes::EventType::Action,
        crate::event_nodes::NodeData::ChangePosition { x: 100.0, y: 100.0 });
    
    // 2. Crear PlaySession
    let entities = vec![
        Entity::new("player1".to_string(), (0.0, 0.0)),
        Entity::new("enemy1".to_string(), (200.0, 200.0)),
    ];
    
    let mut play_session = PlaySession::new(&entities, None);
    
    // 3. Inicializar integración de eventos
    play_session.init_event_integration(event_manager);
    
    // 4. Iniciar play session
    let mut entities_copy = entities.clone();
    play_session.start(&mut entities_copy).unwrap();
    
    // 5. Actualizar cada frame
    let mut delta = 0.016; // ~60 FPS
    for _ in 0..100 {
        play_session.update(delta, &crate::input_capture::UserInput::default());
        delta = 0.016;
    }
    
    // 6. Detener play session
    play_session.stop(&mut entities_copy).unwrap();
    
    println!("[EVENT-PLAY INTEGRATION] Ejemplo completado");
}
