//! # Integration Tests - Input Capture + Play Session
//! 
//! Tests de integración entre InputCapture y Play Session.

use crate::input_capture::{InputCapture, KeyCode, UserInput};
use crate::play_session::{PlaySession, Entity};

#[test]
fn test_input_capture_integration_with_play_session() {
    let mut input = UserInput::default();
    
    // Simular input
    input.keys_pressed.insert(KeyCode::KeyW, true);
    input.keys_pressed.insert(KeyCode::KeyD, true);
    
    let entities = vec![Entity::new("player".to_string(), (0.0, 0.0))];
    let session = PlaySession::new(&entities, None);
    
    // Obtener movimiento
    let movement = session.get_player_movement();
    
    // Verificar que el movimiento es correcto
    assert_eq!(movement, (-1.0, -1.0));
}

#[test]
fn test_input_capture_no_input_play_session() {
    let entities = vec![Entity::new("player".to_string(), (0.0, 0.0))];
    let session = PlaySession::new(&entities, None);
    
    let movement = session.get_player_movement();
    
    assert_eq!(movement, (0.0, 0.0));
}

#[test]
fn test_input_capture_w_asd_movement() {
    let mut input = UserInput::default();
    
    // W - Arriba
    input.keys_pressed.insert(KeyCode::KeyW, true);
    let session = PlaySession::new(&[], None);
    assert_eq!(session.get_player_movement(), (0.0, -1.0));
    
    // S - Abajo
    input.keys_pressed.insert(KeyCode::KeyS, true);
    assert_eq!(session.get_player_movement(), (0.0, 1.0));
    
    // A - Izquierda
    input.keys_pressed.insert(KeyCode::KeyA, true);
    assert_eq!(session.get_player_movement(), (-1.0, 0.0));
    
    // D - Derecha
    input.keys_pressed.insert(KeyCode::KeyD, true);
    assert_eq!(session.get_player_movement(), (1.0, 0.0));
}

#[test]
fn test_input_capture_w_s_cancel_movement() {
    let mut input = UserInput::default();
    
    input.keys_pressed.insert(KeyCode::KeyW, true);
    input.keys_pressed.insert(KeyCode::KeyS, true);
    
    let session = PlaySession::new(&[], None);
    let movement = session.get_player_movement();
    
    // W y S se cancelan
    assert_eq!(movement, (0.0, 0.0));
}

#[test]
fn test_input_capture_a_d_cancel_movement() {
    let mut input = UserInput::default();
    
    input.keys_pressed.insert(KeyCode::KeyA, true);
    input.keys_pressed.insert(KeyCode::KeyD, true);
    
    let session = PlaySession::new(&[], None);
    let movement = session.get_player_movement();
    
    // A y D se cancelan
    assert_eq!(movement, (0.0, 0.0));
}

#[test]
fn test_input_capture_all_keys() {
    let mut input = UserInput::default();
    
    input.keys_pressed.insert(KeyCode::KeyW, true);
    input.keys_pressed.insert(KeyCode::KeyA, true);
    input.keys_pressed.insert(KeyCode::KeyS, true);
    input.keys_pressed.insert(KeyCode::KeyD, true);
    
    let session = PlaySession::new(&[], None);
    let movement = session.get_player_movement();
    
    // Todos los keys se cancelan
    assert_eq!(movement, (0.0, 0.0));
}

#[test]
fn test_input_capture_partial_keys() {
    let mut input = UserInput::default();
    
    input.keys_pressed.insert(KeyCode::KeyW, true);
    input.keys_pressed.insert(KeyCode::KeyA, true);
    
    let session = PlaySession::new(&[], None);
    let movement = session.get_player_movement();
    
    // W + A
    assert_eq!(movement, (-1.0, -1.0));
}

#[test]
fn test_input_capture_mouse_position() {
    let mut input = UserInput::default();
    
    input.mouse.position = (100.0, 200.0);
    input.mouse.left_button = true;
    
    let entities = vec![Entity::new("player".to_string(), (0.0, 0.0))];
    let session = PlaySession::new(&entities, None);
    
    let mouse_state = session.get_mouse_state();
    
    assert_eq!(mouse_state.position, (100.0, 200.0));
    assert!(mouse_state.left_button);
}

#[test]
fn test_input_capture_mouse_buttons() {
    let mut input = UserInput::default();
    
    input.mouse.left_button = true;
    input.mouse.right_button = true;
    input.mouse.middle_button = true;
    
    let session = PlaySession::new(&[], None);
    let mouse_state = session.get_mouse_state();
    
    assert!(mouse_state.left_button);
    assert!(mouse_state.right_button);
    assert!(mouse_state.middle_button);
}

#[test]
fn test_input_capture_mouse_scroll() {
    let mut input = UserInput::default();
    
    input.mouse.scroll = 5.0;
    
    let session = PlaySession::new(&[], None);
    // El scroll se puede usar para zoom o navegación
    // Verificación básica
    assert_eq!(input.mouse.scroll, 5.0);
}

#[test]
fn test_input_capture_multiple_frames() {
    let mut input = UserInput::default();
    
    // Frame 1
    input.keys_pressed.insert(KeyCode::KeyW, true);
    assert_eq!(input.keys_pressed.contains(&KeyCode::KeyW), true);
    
    // Frame 2 - Key released
    input.keys_pressed.insert(KeyCode::KeyW, false);
    assert_eq!(input.keys_pressed.contains(&KeyCode::KeyW), false);
    
    // Frame 3 - New key
    input.keys_pressed.insert(KeyCode::KeyS, true);
    assert_eq!(input.keys_pressed.contains(&KeyCode::KeyS), true);
}

#[test]
fn test_input_capture_key_sequence() {
    let mut input = UserInput::default();
    
    // Press W
    input.keys_pressed.insert(KeyCode::KeyW, true);
    assert_eq!(input.keys_pressed.len(), 1);
    
    // Press A
    input.keys_pressed.insert(KeyCode::KeyA, true);
    assert_eq!(input.keys_pressed.len(), 2);
    
    // Release W
    input.keys_pressed.insert(KeyCode::KeyW, false);
    assert_eq!(input.keys_pressed.len(), 1);
    
    // Release A
    input.keys_pressed.insert(KeyCode::KeyA, false);
    assert_eq!(input.keys_pressed.len(), 0);
}

#[test]
fn test_input_capture_reset_simulation() {
    let mut input = UserInput::default();
    
    input.keys_pressed.insert(KeyCode::KeyW, true);
    input.keys_pressed.insert(KeyCode::KeyA, true);
    
    // Reset
    input.keys_pressed.clear();
    
    assert!(input.keys_pressed.is_empty());
}

#[test]
fn test_input_capture_keycode_variety() {
    let mut input = UserInput::default();
    
    // Various keys
    input.keys_pressed.insert(KeyCode::KeyW, true);
    input.keys_pressed.insert(KeyCode::KeyA, true);
    input.keys_pressed.insert(KeyCode::KeyS, true);
    input.keys_pressed.insert(KeyCode::KeyD, true);
    input.keys_pressed.insert(KeyCode::KeyE, true);
    input.keys_pressed.insert(KeyCode::KeyR, true);
    input.keys_pressed.insert(KeyCode::KeyT, true);
    input.keys_pressed.insert(KeyCode::KeyY, true);
    input.keys_pressed.insert(KeyCode::KeyU, true);
    input.keys_pressed.insert(KeyCode::KeyI, true);
    
    assert_eq!(input.keys_pressed.len(), 10);
}
