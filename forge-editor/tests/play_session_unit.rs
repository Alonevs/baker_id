//! # Unit Tests - Play Session
//! 
//! Tests unitarios para validar PlaySession y sus componentes.

use crate::play_session::{PlaySession, Entity, Vec2};
use crate::input_capture::{UserInput, KeyCode};

#[test]
fn test_play_session_creation() {
    let entities = vec![
        Entity::new("player1".to_string(), (0.0, 0.0)),
        Entity::new("enemy1".to_string(), (100.0, 100.0)),
    ];
    
    let session = PlaySession::new(&entities, None);
    
    // Verificar que se creó correctamente
    assert_eq!(session.entities.len(), 2);
    assert!(!session.is_running);
    assert_eq!(session.get_delta(), 0.0);
    assert_eq!(session.gravity, 9.8);
}

#[test]
fn test_entity_creation() {
    let entity = Entity::new("player".to_string(), (50.0, 50.0));
    
    assert_eq!(entity.id, "player");
    assert_eq!(entity.position, (50.0, 50.0));
    assert_eq!(entity.rotation, 0.0);
    assert_eq!(entity.scale, (1.0, 1.0));
    assert!(entity.is_active);
    assert_eq!(entity.name, "Unnamed");
}

#[test]
fn test_entity_with_name() {
    let entity = Entity::with_name("player".to_string(), (0.0, 0.0), "MyPlayer");
    
    assert_eq!(entity.id, "player");
    assert_eq!(entity.name, "MyPlayer");
}

#[test]
fn test_entity_physics_update() {
    let mut entity = Entity::new("player".to_string(), (0.0, 0.0));
    entity.velocity = (100.0, 0.0);
    
    // Actualizar física con delta de 0.016s (~60 FPS)
    entity.update_physics(0.016, 9.8);
    
    // Verificar que la posición cambió
    assert!((entity.position.0 - 1.6).abs() < 0.01);
}

#[test]
fn test_entity_set_active() {
    let mut entity = Entity::new("player".to_string(), (0.0, 0.0));
    
    entity.set_active(false);
    assert!(!entity.is_active);
    
    entity.set_active(true);
    assert!(entity.is_active);
}

#[test]
fn test_vec2_operations() {
    let v1 = Vec2::new(1.0, 2.0);
    let v2 = Vec2::new(3.0, 4.0);
    
    // Suma
    let sum = v1.add(v2);
    assert_eq!(sum, Vec2::new(4.0, 6.0));
    
    // Resta
    let diff = v1.sub(v2);
    assert_eq!(diff, Vec2::new(-2.0, -2.0));
    
    // Multiplicación por escalar
    let scaled = v1.mul(2.0);
    assert_eq!(scaled, Vec2::new(2.0, 4.0));
}

#[test]
fn test_vec2_zero() {
    let zero = Vec2::zero();
    assert_eq!(zero, Vec2::new(0.0, 0.0));
}

#[test]
fn test_input_capture_keys() {
    let mut input = UserInput::default();
    
    input.keys_pressed.insert(KeyCode::KeyW, true);
    input.keys_pressed.insert(KeyCode::KeyA, true);
    
    assert!(input.keys_pressed.contains(&KeyCode::KeyW));
    assert!(input.keys_pressed.contains(&KeyCode::KeyA));
    assert!(!input.keys_pressed.contains(&KeyCode::KeyS));
}

#[test]
fn test_input_capture_mouse() {
    let mut input = UserInput::default();
    
    input.mouse.position = Vec2::new(100.0, 200.0);
    input.mouse.left_button = true;
    
    assert_eq!(input.mouse.position, Vec2::new(100.0, 200.0));
    assert!(input.mouse.left_button);
}

#[test]
fn test_entity_velocity_y_gravity() {
    let mut entity = Entity::new("player".to_string(), (0.0, 0.0));
    entity.velocity_y = 10.0;
    
    // Aplicar gravedad
    entity.velocity_y += 9.8 * 0.016;
    
    // Verificar que la velocidad vertical aumentó
    assert!(entity.velocity_y > 10.0);
}

#[test]
fn test_entity_terminal_velocity() {
    let mut entity = Entity::new("player".to_string(), (0.0, 0.0));
    entity.velocity_y = 100.0; // Velocidad muy alta
    
    // Aplicar gravedad y límite
    entity.velocity_y += 9.8 * 0.016;
    
    // Verificar que no supera el terminal velocity
    assert!(entity.velocity_y <= 15.0);
}

#[test]
fn test_play_session_input_movement() {
    let entities = vec![Entity::new("player".to_string(), (0.0, 0.0))];
    let mut session = PlaySession::new(&entities, None);
    
    let mut input = UserInput::default();
    input.keys_pressed.insert(KeyCode::KeyW, true);
    input.keys_pressed.insert(KeyCode::KeyD, true);
    
    let movement = session.get_player_movement();
    
    assert_eq!(movement, (-1.0, -1.0));
}

#[test]
fn test_play_session_no_input() {
    let entities = vec![Entity::new("player".to_string(), (0.0, 0.0))];
    let session = PlaySession::new(&entities, None);
    
    let movement = session.get_player_movement();
    
    assert_eq!(movement, (0.0, 0.0));
}

#[test]
fn test_entity_id_uniqueness() {
    let entity1 = Entity::new("player1".to_string(), (0.0, 0.0));
    let entity2 = Entity::new("player1".to_string(), (10.0, 10.0));
    
    assert_eq!(entity1.id, entity2.id);
    assert_ne!(entity1.position, entity2.position);
}

#[test]
fn test_entity_clone() {
    let original = Entity::with_name("player".to_string(), (50.0, 50.0), "TestPlayer");
    let cloned = original.clone();
    
    assert_eq!(original.id, cloned.id);
    assert_eq!(original.position, cloned.position);
    assert_eq!(original.name, cloned.name);
}
