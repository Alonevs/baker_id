//! # E2E Tests - Event Execution
//! 
//! Tests end-to-end para validar la ejecución de eventos.

use crate::event_play_integration::EventPlayIntegration;
use crate::event_node_manager::EventNodeManager;
use crate::play_session::{PlaySession, Entity};

#[test]
fn test_event_execution_e2e() {
    // 1. Crear EventNodeManager con nodos
    let mut event_manager = EventNodeManager::new();
    
    // Agregar nodo de inicio
    event_manager.add_node(
        "start_trigger",
        crate::event_nodes::EventType::OnStart,
        crate::event_nodes::NodeData::OnStart { auto_execute: true }
    );
    
    // 2. Crear PlaySession
    let entities = vec![Entity::new("player".to_string(), (0.0, 0.0))];
    let mut play_session = PlaySession::new(&entities, None);
    
    // 3. Inicializar integración
    play_session.init_event_integration(event_manager);
    
    // 4. Iniciar play
    let mut entities_copy = entities.clone();
    play_session.start(&mut entities_copy).unwrap();
    
    // 5. Actualizar (debería ejecutar eventos)
    play_session.update(0.016, &crate::input_capture::UserInput::default());
    
    // 6. Detener play
    play_session.stop(&mut entities_copy).unwrap();
}

#[test]
fn test_timer_event_e2e() {
    let mut integration = EventPlayIntegration::new();
    
    let entities = vec![Entity::new("player".to_string(), (0.0, 0.0))];
    let play_session = PlaySession::new(&entities, None);
    
    integration.init_with_play_session(play_session);
    integration.start();
    
    // Simular frames con temporizador
    for _ in 0..10 {
        integration.update(0.016);
    }
    
    integration.stop();
}

#[test]
fn test_variable_persistence_e2e() {
    let mut integration = EventPlayIntegration::new();
    
    let entities = vec![Entity::new("player".to_string(), (0.0, 0.0))];
    let play_session = PlaySession::new(&entities, None);
    
    integration.init_with_play_session(play_session);
    integration.start();
    
    // Modificar variables
    integration.set_variable("score", "100");
    integration.set_variable("lives", "3");
    
    // Verificar que persistieron
    assert_eq!(integration.get_variable("score"), Some("100".to_string()));
    assert_eq!(integration.get_variable("lives"), Some("3".to_string()));
    
    integration.stop();
}

#[test]
fn test_callback_registration_e2e() {
    let mut integration = EventPlayIntegration::new();
    
    let entities = vec![Entity::new("player".to_string(), (0.0, 0.0))];
    let play_session = PlaySession::new(&entities, None);
    
    integration.init_with_play_session(play_session);
    integration.start();
    
    // Registrar callbacks
    integration.register_callback("on_click", "click_handler");
    integration.register_callback("on_update", "update_handler");
    
    // Verificar que se registraron
    assert_eq!(integration.get_callbacks("on_click").len(), 1);
    assert_eq!(integration.get_callbacks("on_update").len(), 1);
    
    integration.stop();
}

#[test]
fn test_multiple_events_sequence_e2e() {
    let mut integration = EventPlayIntegration::new();
    
    let entities = vec![Entity::new("player".to_string(), (0.0, 0.0))];
    let play_session = PlaySession::new(&entities, None);
    
    integration.init_with_play_session(play_session);
    integration.start();
    
    // Ejecutar múltiples frames
    for _ in 0..20 {
        integration.update(0.016);
    }
    
    integration.stop();
}

#[test]
fn test_entity_control_e2e() {
    let mut integration = EventPlayIntegration::new();
    
    let entities = vec![Entity::new("player".to_string(), (0.0, 0.0))];
    let play_session = PlaySession::new(&entities, None);
    
    integration.init_with_play_session(play_session);
    integration.start();
    
    // Controlar entidad
    integration.set_entity_position("player", (100.0, 200.0));
    integration.set_entity_rotation("player", 45.0);
    integration.set_entity_scale("player", (2.0, 2.0));
    
    integration.stop();
}

#[test]
fn test_shared_variables_e2e() {
    let mut integration = EventPlayIntegration::new();
    
    let entities = vec![Entity::new("player".to_string(), (0.0, 0.0))];
    let play_session = PlaySession::new(&entities, None);
    
    integration.init_with_play_session(play_session);
    integration.start();
    
    // Crear variables compartidas
    integration.set_variable("player_health", "100");
    integration.set_variable("enemy_count", "5");
    integration.set_variable("game_state", "playing");
    
    // Verificar
    assert_eq!(integration.get_variable("player_health"), Some("100".to_string()));
    assert_eq!(integration.get_variable("enemy_count"), Some("5".to_string()));
    assert_eq!(integration.get_variable("game_state"), Some("playing".to_string()));
    
    integration.stop();
}

#[test]
fn test_event_stats_e2e() {
    let mut integration = EventPlayIntegration::new();
    
    let entities = vec![Entity::new("player".to_string(), (0.0, 0.0))];
    let play_session = PlaySession::new(&entities, None);
    
    integration.init_with_play_session(play_session);
    integration.start();
    
    // Ejecutar eventos
    for _ in 0..10 {
        integration.update(0.016);
    }
    
    let stats = integration.get_stats();
    
    // Verificar stats
    assert_eq!(stats.get("is_active").unwrap(), &"true");
    assert!(stats.get("events_executed").unwrap().parse::<u32>().unwrap() > 0);
    
    integration.stop();
}

#[test]
fn test_full_integration_flow_e2e() {
    let mut integration = EventPlayIntegration::new();
    
    let entities = vec![
        Entity::new("player".to_string(), (0.0, 0.0)),
        Entity::new("enemy".to_string(), (100.0, 100.0)),
    ];
    let play_session = PlaySession::new(&entities, None);
    
    integration.init_with_play_session(play_session);
    integration.start();
    
    // Setup inicial
    integration.set_variable("score", "0");
    integration.set_variable("level", "1");
    integration.register_callback("on_start", "start_callback");
    
    // Simular juego
    for frame in 0..50 {
        integration.update(0.016);
        
        // Actualizar variables
        if frame % 10 == 0 {
            integration.set_variable("score", frame.to_string());
        }
    }
    
    // Verificar resultados
    assert_eq!(integration.get_variable("score"), Some("40".to_string()));
    
    integration.stop();
}

#[test]
fn test_event_integration_with_input() {
    let mut integration = EventPlayIntegration::new();
    
    let entities = vec![Entity::new("player".to_string(), (0.0, 0.0))];
    let play_session = PlaySession::new(&entities, None);
    
    integration.init_with_play_session(play_session);
    integration.start();
    
    // Simular input
    let mut input = crate::input_capture::UserInput::default();
    for _ in 0..10 {
        input.keys_pressed.insert(crate::input_capture::KeyCode::KeyW, true);
        integration.update(0.016);
    }
    
    integration.stop();
}

#[test]
fn test_event_integration_cleanup() {
    let mut integration = EventPlayIntegration::new();
    
    let entities = vec![Entity::new("player".to_string(), (0.0, 0.0))];
    let play_session = PlaySession::new(&entities, None);
    
    integration.init_with_play_session(play_session);
    integration.start();
    
    // Ejecutar algo
    integration.update(0.016);
    
    integration.stop();
    
    // Verificar que está limpio
    assert!(!integration.is_active);
    assert_eq!(integration.get_stats().get("is_active").unwrap(), &"false");
}

#[test]
fn test_event_integration_multiple_start_stop() {
    let mut integration = EventPlayIntegration::new();
    
    let entities = vec![Entity::new("player".to_string(), (0.0, 0.0))];
    let play_session = PlaySession::new(&entities, None);
    
    integration.init_with_play_session(play_session);
    
    // Start/Stop múltiples veces
    integration.start();
    integration.stop();
    integration.start();
    integration.stop();
    
    // Debería estar inactivo
    assert!(!integration.is_active);
}
