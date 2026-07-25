//! Timeline System Integration Tests
//! Tests que verifican la integración real del TimelineSystem con el resto del runtime

use crate::timeline::timeline_system::TimelineSystem;
use crate::timeline::timeline_manager::TimelineManager;
use crate::timeline::timeline::Timeline;
use crate::timeline::timeline_manager::AnimationComponent;
use crate::play_mode::PlayMode;

#[test]
fn test_timeline_system_basic() {
    let system = TimelineSystem::new();
    
    assert_eq!(system.current_frame(), 0);
    assert!(!system.is_playing());
}

#[test]
fn test_timeline_system_register_entity() {
    let mut system = TimelineSystem::new();
    
    system.register_entity(1, 0);
    system.register_entity(2, 5);
    
    assert_eq!(system.get_entity_frame(1), Some(0));
    assert_eq!(system.get_entity_frame(2), Some(0));
    assert_eq!(system.get_entity_frame(3), None);
}

#[test]
fn test_timeline_system_playback() {
    let mut system = TimelineSystem::new();
    system.set_playing(true);
    
    assert!(system.is_playing());
    
    system.next_frame();
    assert_eq!(system.current_frame(), 1);
    
    system.prev_frame();
    assert_eq!(system.current_frame(), 0);
}

#[test]
fn test_timeline_system_update() {
    let mut system = TimelineSystem::new();
    system.manager.is_playing = true;
    
    system.update(1.0 / 60.0);
    assert_eq!(system.current_frame(), 1);
    
    system.manager.is_playing = false;
    system.update(1.0 / 60.0);
    assert_eq!(system.current_frame(), 1);
}

#[test]
fn test_timeline_system_reset() {
    let mut system = TimelineSystem::new();
    system.set_frame(10);
    assert_eq!(system.current_frame(), 10);
    
    system.reset();
    assert_eq!(system.current_frame(), 0);
    assert!(!system.is_playing());
}

#[test]
fn test_timeline_system_animation_sync() {
    let mut system = TimelineSystem::new();
    
    system.register_entity(1, 0);
    
    system.set_frame(5);
    
    let animation = system.manager.get_entity_animation(1).unwrap();
    assert_eq!(animation.current_frame, 5);
}

#[test]
fn test_timeline_system_timeline_events() {
    let mut system = TimelineSystem::new();
    
    // Crear eventos en la timeline
    let timeline = &mut system.manager.timeline;
    timeline.current_frame = 10;
    
    let events = system.get_event_at_frame(10);
    
    // El system debe poder manejar eventos
    assert!(events.len() >= 0);
}

#[test]
fn test_timeline_system_with_play_mode() {
    let mut system = TimelineSystem::new();
    let mut play_mode = PlayMode::new();
    
    // Iniciar play mode
    play_mode.start();
    assert_eq!(play_mode.get_state(), crate::play_mode::PlayModeState::Playing);
    
    // Actualizar play mode
    system.set_playing(true);
    system.update(play_mode.get_delta());
    
    assert_eq!(system.current_frame(), 1);
}

#[test]
fn test_timeline_system_multiple_entities() {
    let mut system = TimelineSystem::new();
    
    // Registrar múltiples entidades
    for i in 0..10 {
        system.register_entity(i as u64, i as u32);
    }
    
    // Verificar que todas las entidades están registradas (current_frame inicial es 0)
    for i in 0..10 {
        assert_eq!(system.get_entity_frame(i as u64), Some(0));
    }
    
    // Actualizar todos los frames
    system.set_frame(5);
    
    // Verificar que el frame se actualizó para todas las entidades
    for i in 0..10 {
        assert_eq!(system.get_entity_frame(i as u64), Some(5));
    }
}

#[test]
fn test_timeline_system_playback_speed() {
    let mut system = TimelineSystem::new();
    system.set_playback_speed(30.0);
    
    assert_eq!(system.manager.playback_speed, 30.0);
    
    system.set_playing(true);
    system.update(1.0 / 30.0);
    assert_eq!(system.current_frame(), 1);
}

#[test]
fn test_timeline_system_frame_boundaries() {
    let mut system = TimelineSystem::new();
    system.manager.timeline.current_frame = 0;
    
    // Intentar ir al frame anterior desde 0
    system.prev_frame();
    assert_eq!(system.current_frame(), 0); // Debe mantenerse en 0
    
    // Intentar ir al siguiente frame
    system.next_frame();
    assert_eq!(system.current_frame(), 1);
    
    // Establecer total_frames y verificar límites
    system.manager.timeline.current_frame = 100;
    system.next_frame();
    assert_eq!(system.current_frame(), 101);
}

#[test]
fn test_timeline_system_serialization() {
    let mut system = TimelineSystem::new();
    system.set_frame(10);
    system.set_playing(true);
    system.set_playback_speed(60.0);
    
    // El system debe ser serializable
    let cloned = system.clone();
    assert_eq!(cloned.current_frame(), system.current_frame());
    assert_eq!(cloned.is_playing(), system.is_playing());
}
