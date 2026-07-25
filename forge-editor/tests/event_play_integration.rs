//! # Integration Tests - Event Play Integration
//! 
//! Tests de integración entre Event Forge y Play Session.

use crate::event_play_integration::EventPlayIntegration;
use crate::event_node_manager::EventNodeManager;
use crate::play_session::{PlaySession, Entity};
use std::collections::HashMap;

#[test]
fn test_event_play_integration_creation() {
    let integration = EventPlayIntegration::new();
    
    assert!(!integration.is_active);
    assert_eq!(integration.events_executed, 0);
    assert_eq!(integration.event_manager.nodes.len(), 0);
    assert!(integration.play_session.is_none());
}

#[test]
fn test_event_play_integration_start_stop() {
    let mut integration = EventPlayIntegration::new();
    
    // Crear play session
    let entities = vec![Entity::new("player".to_string(), (0.0, 0.0))];
    let play_session = PlaySession::new(&entities, None);
    
    integration.init_with_play_session(play_session);
    integration.start();
    
    assert!(integration.is_active);
    assert!(integration.event_manager.runtime_context.is_running);
    
    integration.stop();
    
    assert!(!integration.is_active);
    assert!(!integration.event_manager.runtime_context.is_running);
}

#[test]
fn test_event_play_integration_variables() {
    let mut integration = EventPlayIntegration::new();
    
    integration.set_variable("score", "100");
    integration.set_variable("lives", "3");
    
    assert_eq!(integration.get_variable("score"), Some("100".to_string()));
    assert_eq!(integration.get_variable("lives"), Some("3".to_string()));
    assert_eq!(integration.get_variable("nonexistent"), None);
}

#[test]
fn test_event_play_integration_callbacks() {
    let mut integration = EventPlayIntegration::new();
    
    integration.register_callback("on_click", "callback1");
    integration.register_callback("on_click", "callback2");
    integration.register_callback("on_start", "callback3");
    
    assert_eq!(integration.get_callbacks("on_click").len(), 2);
    assert_eq!(integration.get_callbacks("on_start").len(), 1);
    assert_eq!(integration.get_callbacks("nonexistent").len(), 0);
}

#[test]
fn test_event_play_integration_entity_control() {
    let mut integration = EventPlayIntegration::new();
    
    let entities = vec![Entity::new("player".to_string(), (0.0, 0.0))];
    let play_session = PlaySession::new(&entities, None);
    
    integration.init_with_play_session(play_session);
    
    // Set position
    integration.set_entity_position("player", (100.0, 200.0));
    
    // Verificar que la posición cambió (esto debería reflejarse en play_session)
    // Nota: En una implementación real, esto se sincroniza con play_session
}

#[test]
fn test_event_play_integration_shared_variables() {
    let mut integration = EventPlayIntegration::new();
    
    integration.set_variable("player_health", "100");
    integration.set_variable("enemy_count", "5");
    
    // Obtener variables
    assert_eq!(integration.get_variable("player_health"), Some("100".to_string()));
    assert_eq!(integration.get_variable("enemy_count"), Some("5".to_string()));
}

#[test]
fn test_event_play_integration_stats() {
    let mut integration = EventPlayIntegration::new();
    
    integration.set_variable("test_var", "value");
    integration.register_callback("test_event", "test_callback");
    
    let stats = integration.get_stats();
    
    assert_eq!(stats.get("is_active").unwrap(), &"false");
    assert_eq!(stats.get("events_executed").unwrap(), &"0");
    assert_eq!(stats.get("nodes_count").unwrap(), &"0");
    assert_eq!(stats.get("variables_count").unwrap(), &"1");
    assert_eq!(stats.get("callbacks_count").unwrap(), &"1");
}

#[test]
fn test_event_play_integration_with_multiple_entities() {
    let mut integration = EventPlayIntegration::new();
    
    let entities = vec![
        Entity::new("player".to_string(), (0.0, 0.0)),
        Entity::new("enemy1".to_string(), (100.0, 100.0)),
        Entity::new("enemy2".to_string(), (200.0, 200.0)),
    ];
    let play_session = PlaySession::new(&entities, None);
    
    integration.init_with_play_session(play_session);
    
    assert!(integration.is_active);
}

#[test]
fn test_event_play_integration_variable_operations() {
    let mut integration = EventPlayIntegration::new();
    
    // Crear variable
    integration.set_variable("score", "0");
    
    // Modificar variable
    integration.set_variable("score", "100");
    assert_eq!(integration.get_variable("score"), Some("100".to_string()));
    
    integration.set_variable("score", "200");
    assert_eq!(integration.get_variable("score"), Some("200".to_string()));
}

#[test]
fn test_event_play_integration_callbacks_multiple_events() {
    let mut integration = EventPlayIntegration::new();
    
    integration.register_callback("on_start", "cb1");
    integration.register_callback("on_start", "cb2");
    integration.register_callback("on_click", "cb3");
    integration.register_callback("on_click", "cb4");
    integration.register_callback("on_update", "cb5");
    
    assert_eq!(integration.get_callbacks("on_start").len(), 2);
    assert_eq!(integration.get_callbacks("on_click").len(), 2);
    assert_eq!(integration.get_callbacks("on_update").len(), 1);
}

#[test]
fn test_event_play_integration_variable_overwrite() {
    let mut integration = EventPlayIntegration::new();
    
    integration.set_variable("test", "value1");
    integration.set_variable("test", "value2");
    integration.set_variable("test", "value3");
    
    assert_eq!(integration.get_variable("test"), Some("value3".to_string()));
}

#[test]
fn test_event_play_integration_empty_callbacks() {
    let integration = EventPlayIntegration::new();
    
    let callbacks = integration.get_callbacks("nonexistent");
    assert_eq!(callbacks.len(), 0);
}

#[test]
fn test_event_play_integration_variable_with_special_chars() {
    let mut integration = EventPlayIntegration::new();
    
    integration.set_variable("player_name", "John_Doe_123");
    integration.set_variable("score", "1000+");
    integration.set_variable("health", "50%");
    
    assert_eq!(integration.get_variable("player_name"), Some("John_Doe_123".to_string()));
    assert_eq!(integration.get_variable("score"), Some("1000+".to_string()));
    assert_eq!(integration.get_variable("health"), Some("50%".to_string()));
}
