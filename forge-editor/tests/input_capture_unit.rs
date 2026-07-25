//! # Unit Tests - Input Capture
//! 
//! Tests unitarios para validar InputCapture y UserInput.

use crate::input_capture::{InputCapture, KeyCode, MouseState, UserInput};

#[test]
fn test_userinput_default() {
    let input = UserInput::default();
    
    assert!(input.keys_pressed.is_empty());
    assert!(input.mouse.position == (0.0, 0.0));
    assert!(!input.mouse.left_button);
    assert!(!input.mouse.right_button);
    assert!(!input.mouse.middle_button);
}

#[test]
fn test_userinput_keys_pressed() {
    let mut input = UserInput::default();
    
    input.keyboard.insert(KeyCode::KeyW, true);
    input.keyboard.insert(KeyCode::KeyA, true);
    input.keyboard.insert(KeyCode::KeyS, true);
    input.keyboard.insert(KeyCode::KeyD, true);
    
    assert!(input.keys_pressed.contains(&KeyCode::KeyW));
    assert!(input.keys_pressed.contains(&KeyCode::KeyA));
    assert!(input.keys_pressed.contains(&KeyCode::KeyS));
    assert!(input.keys_pressed.contains(&KeyCode::KeyD));
}

#[test]
fn test_userinput_keys_released() {
    let mut input = UserInput::default();
    
    input.keyboard.insert(KeyCode::KeyW, true);
    input.keyboard.insert(KeyCode::KeyW, false);
    
    assert!(!input.keys_pressed.contains(&KeyCode::KeyW));
}

#[test]
fn test_userinput_mouse_position() {
    let mut input = UserInput::default();
    
    input.mouse.position = (100.0, 200.0);
    
    assert_eq!(input.mouse.position, (100.0, 200.0));
}

#[test]
fn test_userinput_mouse_buttons() {
    let mut input = UserInput::default();
    
    input.mouse.left_button = true;
    input.mouse.right_button = true;
    input.mouse.middle_button = true;
    
    assert!(input.mouse.left_button);
    assert!(input.mouse.right_button);
    assert!(input.mouse.middle_button);
}

#[test]
fn test_userinput_scroll() {
    let mut input = UserInput::default();
    
    input.mouse.scroll = 5.0;
    
    assert_eq!(input.mouse.scroll, 5.0);
}

#[test]
fn test_inputcapture_default() {
    let capture = InputCapture::new();
    
    assert_eq!(capture.keys_pressed.len(), 0);
    assert_eq!(capture.mouse.position, (0.0, 0.0));
    assert!(!capture.mouse.left_button);
    assert_eq!(capture.mouse.scroll, 0.0);
}

#[test]
fn test_inputcapture_update_keys() {
    let mut capture = InputCapture::new();
    
    capture.update_key(KeyCode::KeyW, true);
    capture.update_key(KeyCode::KeyA, true);
    
    assert!(capture.keys_pressed.contains(&KeyCode::KeyW));
    assert!(capture.keys_pressed.contains(&KeyCode::KeyA));
}

#[test]
fn test_inputcapture_update_key_release() {
    let mut capture = InputCapture::new();
    
    capture.update_key(KeyCode::KeyW, true);
    capture.update_key(KeyCode::KeyW, false);
    
    assert!(!capture.keys_pressed.contains(&KeyCode::KeyW));
}

#[test]
fn test_inputcapture_update_mouse_position() {
    let mut capture = InputCapture::new();
    
    capture.update_mouse((100.0, 200.0));
    
    assert_eq!(capture.mouse.position, (100.0, 200.0));
}

#[test]
fn test_inputcapture_update_mouse_buttons() {
    let mut capture = InputCapture::new();
    
    capture.update_mouse_button(true, KeyCode::KeyLeft);
    capture.update_mouse_button(true, KeyCode::KeyRight);
    
    assert!(capture.mouse.left_button);
    assert!(capture.mouse.right_button);
}

#[test]
fn test_inputcapture_get_movement() {
    let mut capture = InputCapture::new();
    
    capture.update_key(KeyCode::KeyW, true);
    capture.update_key(KeyCode::KeyD, true);
    
    let movement = capture.get_movement();
    
    assert_eq!(movement, (-1.0, -1.0));
}

#[test]
fn test_inputcapture_no_movement() {
    let capture = InputCapture::new();
    
    let movement = capture.get_movement();
    
    assert_eq!(movement, (0.0, 0.0));
}

#[test]
fn test_inputcapture_all_directions() {
    let mut capture = InputCapture::new();
    
    // W - Up
    capture.update_key(KeyCode::KeyW, true);
    assert_eq!(capture.get_movement(), (0.0, -1.0));
    
    // S - Down
    capture.update_key(KeyCode::KeyS, true);
    assert_eq!(capture.get_movement(), (0.0, 1.0));
    
    // A - Left
    capture.update_key(KeyCode::KeyA, true);
    assert_eq!(capture.get_movement(), (-1.0, 0.0));
    
    // D - Right
    capture.update_key(KeyCode::KeyD, true);
    assert_eq!(capture.get_movement(), (1.0, 0.0));
    
    // All together
    capture.update_key(KeyCode::KeyW, true);
    capture.update_key(KeyCode::KeyA, true);
    capture.update_key(KeyCode::KeyS, true);
    capture.update_key(KeyCode::KeyD, true);
    assert_eq!(capture.get_movement(), (-1.0, 0.0)); // W + S cancel out
}

#[test]
fn test_inputcapture_mouse_over() {
    let mut capture = InputCapture::new();
    
    capture.update_mouse((100.0, 100.0));
    
    assert_eq!(capture.mouse.position, (100.0, 100.0));
}

#[test]
fn test_inputcapture_mouse_click_sequence() {
    let mut capture = InputCapture::new();
    
    // Click down
    capture.update_mouse_button(true, KeyCode::KeyLeft);
    assert!(capture.mouse.left_button);
    
    // Click up
    capture.update_mouse_button(false, KeyCode::KeyLeft);
    assert!(!capture.mouse.left_button);
}

#[test]
fn test_inputcapture_multiple_keys() {
    let mut capture = InputCapture::new();
    
    capture.update_key(KeyCode::KeyW, true);
    capture.update_key(KeyCode::KeyA, true);
    capture.update_key(KeyCode::KeyS, true);
    capture.update_key(KeyCode::KeyD, true);
    capture.update_key(KeyCode::KeyE, true);
    capture.update_key(KeyCode::KeyR, true);
    
    assert_eq!(capture.keys_pressed.len(), 6);
}

#[test]
fn test_inputcapture_reset() {
    let mut capture = InputCapture::new();
    
    capture.update_key(KeyCode::KeyW, true);
    capture.update_key(KeyCode::KeyA, true);
    
    // Reset simulation
    capture.keys_pressed.clear();
    
    assert!(capture.keys_pressed.is_empty());
}
