//! Integration tests for Timeline and Animation

use crate::timeline::TimelineManager;
use crate::components::AnimationComponent;

#[test]
fn test_timeline_manager_with_animation_component() {
    // Crear TimelineManager
    let mut manager = TimelineManager::new();

    // Crear AnimationComponent para una entidad
    let mut animation = AnimationComponent::new();
    manager.register_entity(1, animation);

    // Iniciar reproducción
    manager.play();

    // Actualizar con delta time
    manager.update(0.016);

    // Verificar que el frame avanzó
    assert_eq!(manager.timeline.current_frame, 1);

    // Detener reproducción
    manager.stop();
    assert!(!manager.is_playing);
}

#[test]
fn test_timeline_manager_frame_navigation() {
    let mut manager = TimelineManager::new();

    manager.set_frame(10);
    assert_eq!(manager.timeline.current_frame, 10);

    manager.next_frame();
    assert_eq!(manager.timeline.current_frame, 11);

    manager.prev_frame();
    assert_eq!(manager.timeline.current_frame, 10);
}

#[test]
fn test_timeline_manager_playback_speed() {
    let mut manager = TimelineManager::new();
    manager.play();

    // Velocidad normal
    manager.set_playback_speed(1.0);
    manager.update(0.016);
    assert_eq!(manager.timeline.current_frame, 1);

    // Velocidad doble
    manager.set_playback_speed(2.0);
    manager.update(0.016);
    assert_eq!(manager.timeline.current_frame, 3); // 1 + 2

    manager.stop();
}

#[test]
fn test_timeline_manager_multiple_entities() {
    let mut manager = TimelineManager::new();

    // Registrar múltiples entidades
    let mut anim1 = AnimationComponent::new();
    let mut anim2 = AnimationComponent::new();
    manager.register_entity(1, anim1);
    manager.register_entity(2, anim2);

    // Iniciar reproducción
    manager.play();

    // Actualizar
    manager.update(0.016);

    // Verificar que ambas animaciones se actualizaron
    assert!(manager.get_entity_animation(1).is_some());
    assert!(manager.get_entity_animation(2).is_some());

    manager.stop();
}
