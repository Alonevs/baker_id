//! # E2E Tests - Basic Play Flow
//! 
//! Tests end-to-end para validar flujos completos del sistema.

use crate::play_session::{PlaySession, Entity};
use crate::input_capture::{UserInput, KeyCode};

#[test]
fn test_basic_play_flow() {
    // 1. Crear entidades
    let entities = vec![
        Entity::with_name("player".to_string(), (0.0, 0.0), "Player"),
        Entity::with_name("enemy".to_string(), (100.0, 100.0), "Enemy"),
    ];
    
    let mut session = PlaySession::new(&entities, None);
    let mut entities_copy = entities.clone();
    
    // 2. Iniciar play
    session.start(&mut entities_copy).unwrap();
    assert!(session.is_active());
    
    // 3. Simular frames
    let mut input = UserInput::default();
    for _ in 0..10 {
        input.keys_pressed.insert(KeyCode::KeyW, true);
        session.update(0.016, &input);
    }
    
    // 4. Detener play
    session.stop(&mut entities_copy).unwrap();
    assert!(!session.is_active());
}

#[test]
fn test_entity_movement_e2e() {
    let entities = vec![
        Entity::with_name("player".to_string(), (0.0, 0.0), "Player"),
    ];
    
    let mut session = PlaySession::new(&entities, None);
    let mut entities_copy = entities.clone();
    
    session.start(&mut entities_copy).unwrap();
    
    // Mover con WASD
    let mut input = UserInput::default();
    input.keys_pressed.insert(KeyCode::KeyW, true);
    input.keys_pressed.insert(KeyCode::KeyD, true);
    
    for _ in 0..5 {
        session.update(0.016, &input);
    }
    
    // Verificar que la posición cambió
    assert!(entities_copy[0].position.0 > 0.0);
    assert!(entities_copy[0].position.1 < 0.0);
    
    session.stop(&mut entities_copy).unwrap();
}

#[test]
fn test_entity_physics_e2e() {
    let entities = vec![
        Entity::with_name("player".to_string(), (0.0, 0.0), "Player"),
    ];
    
    let mut session = PlaySession::new(&entities, None);
    let mut entities_copy = entities.clone();
    
    session.start(&mut entities_copy).unwrap();
    
    // Aplicar velocidad inicial
    entities_copy[0].velocity_y = 50.0;
    
    // Simular caída con gravedad
    for _ in 0..20 {
        session.update(0.016, &UserInput::default());
    }
    
    // Verificar que la velocidad disminuyó por gravedad
    assert!(entities_copy[0].velocity_y < 50.0);
    
    session.stop(&mut entities_copy).unwrap();
}

#[test]
fn test_multiple_entities_e2e() {
    let entities = vec![
        Entity::with_name("player".to_string(), (0.0, 0.0), "Player"),
        Entity::with_name("enemy1".to_string(), (100.0, 100.0), "Enemy1"),
        Entity::with_name("enemy2".to_string(), (200.0, 200.0), "Enemy2"),
    ];
    
    let mut session = PlaySession::new(&entities, None);
    let mut entities_copy = entities.clone();
    
    session.start(&mut entities_copy).unwrap();
    
    // Mover a todas las entidades
    let mut input = UserInput::default();
    input.keys_pressed.insert(KeyCode::KeyW, true);
    
    for _ in 0..10 {
        session.update(0.016, &input);
    }
    
    // Verificar que todas las entidades se movieron
    assert!(entities_copy[0].position.1 < 0.0);
    assert!(entities_copy[1].position.1 < 100.0);
    assert!(entities_copy[2].position.1 < 200.0);
    
    session.stop(&mut entities_copy).unwrap();
}

#[test]
fn test_entity_inactive_e2e() {
    let entities = vec![
        Entity::with_name("player".to_string(), (0.0, 0.0), "Player"),
    ];
    
    let mut session = PlaySession::new(&entities, None);
    let mut entities_copy = entities.clone();
    
    session.start(&mut entities_copy).unwrap();
    
    // Desactivar entidad
    entities_copy[0].set_active(false);
    
    // Mover
    let mut input = UserInput::default();
    input.keys_pressed.insert(KeyCode::KeyW, true);
    
    for _ in 0..10 {
        session.update(0.016, &input);
    }
    
    // Verificar que la posición no cambió (entidad inactiva)
    assert_eq!(entities_copy[0].position.1, 0.0);
    
    session.stop(&mut entities_copy).unwrap();
}

#[test]
fn test_play_session_delta_tracking() {
    let entities = vec![Entity::new("player".to_string(), (0.0, 0.0))];
    let mut session = PlaySession::new(&entities, None);
    
    session.start(&mut entities).unwrap();
    
    for i in 0..5 {
        session.update(0.016 * (i + 1), &UserInput::default());
    }
    
    // Verificar que el delta se actualizó
    assert_eq!(session.get_delta(), 0.080);
    
    session.stop(&mut entities).unwrap();
}

#[test]
fn test_play_session_change_count() {
    let entities = vec![Entity::new("player".to_string(), (0.0, 0.0))];
    let mut session = PlaySession::new(&entities, None);
    
    session.start(&mut entities).unwrap();
    
    assert_eq!(session.change_count, 0);
    
    session.update(0.016, &UserInput::default());
    assert_eq!(session.change_count, 1);
    
    session.update(0.016, &UserInput::default());
    assert_eq!(session.change_count, 2);
    
    session.stop(&mut entities).unwrap();
}

#[test]
fn test_entity_snapshot_creation() {
    let entities = vec![
        Entity::with_name("player".to_string(), (50.0, 50.0), "Player"),
        Entity::with_name("enemy".to_string(), (100.0, 100.0), "Enemy"),
    ];
    
    let session = PlaySession::new(&entities, None);
    let snapshot = session.get_snapshot();
    
    assert_eq!(snapshot.entities.len(), 2);
    
    // Verificar que el snapshot tiene las posiciones correctas
    let player = snapshot.entities.get("player");
    let enemy = snapshot.entities.get("enemy");
    
    assert!(player.is_some());
    assert!(enemy.is_some());
    
    if let Some(p) = player {
        assert_eq!(p.position.x, 50.0);
        assert_eq!(p.position.y, 50.0);
    }
    
    if let Some(e) = enemy {
        assert_eq!(e.position.x, 100.0);
        assert_eq!(e.position.y, 100.0);
    }
}

#[test]
fn test_full_game_loop_simulation() {
    let entities = vec![
        Entity::with_name("player".to_string(), (0.0, 0.0), "Player"),
        Entity::with_name("enemy".to_string(), (200.0, 200.0), "Enemy"),
    ];
    
    let mut session = PlaySession::new(&entities, None);
    let mut entities_copy = entities.clone();
    
    session.start(&mut entities_copy).unwrap();
    
    // Simular 100 frames (aprox 1.6 segundos a 60 FPS)
    let mut input = UserInput::default();
    for frame in 0..100 {
        // Mover jugador con WASD
        input.keys_pressed.clear();
        if frame % 3 == 0 {
            input.keys_pressed.insert(KeyCode::KeyW, true);
        } else if frame % 3 == 1 {
            input.keys_pressed.insert(KeyCode::KeyD, true);
        } else if frame % 3 == 2 {
            input.keys_pressed.insert(KeyCode::KeyA, true);
        }
        
        session.update(0.016, &input);
        
        // Verificar que el juego sigue corriendo
        assert!(session.is_active());
    }
    
    // Verificar que el jugador se movió
    assert!(entities_copy[0].position.0 > 0.0 || entities_copy[0].position.1 < 0.0);
    
    session.stop(&mut entities_copy).unwrap();
}

#[test]
fn test_entity_velocity_terminal() {
    let entities = vec![
        Entity::with_name("player".to_string(), (0.0, 0.0), "Player"),
    ];
    
    let mut session = PlaySession::new(&entities, None);
    let mut entities_copy = entities.clone();
    
    session.start(&mut entities_copy).unwrap();
    
    // Aplicar velocidad muy alta
    entities_copy[0].velocity_y = 100.0;
    
    // Simular caída
    for _ in 0..50 {
        session.update(0.016, &UserInput::default());
    }
    
    // Verificar que la velocidad se limitó al terminal velocity
    assert!(entities_copy[0].velocity_y <= 15.0);
    
    session.stop(&mut entities_copy).unwrap();
}
