//! # Cámara Estática
//! 
//! Para novelas visuales, cinemáticas, planos fijos

use crate::camera::Camera;
use crate::math::Vec2;

/// Obtiene la posición fija de la cámara
pub fn get_static_position(camera: &Camera) -> Vec2 {
    camera.static_config.fixed_position.clone()
}

/// Obtiene el estado de fade
pub fn get_fade_state(camera: &Camera) -> f32 {
    if !camera.static_config.fade_enabled {
        return 1.0; // Sin fade
    }
    
    // Placeholder para lógica de fade
    1.0
}

/// Aplica fade in/out
pub fn apply_fade(camera: &mut Camera, dt: f32, fade_in: bool) {
    if !camera.static_config.fade_enabled {
        return;
    }
    
    // Placeholder para lógica de fade
}

/// Verifica si la cámara está en transición
pub fn is_in_transition(camera: &Camera) -> bool {
    camera.static_config.in_transition
}

/// Inicia una transición
pub fn start_transition(camera: &mut Camera) {
    camera.static_config.in_transition = true;
}

/// Finaliza una transición
pub fn end_transition(camera: &mut Camera) {
    camera.static_config.in_transition = false;
}

/// Obtiene los límites del plano estático
pub fn get_static_bounds(camera: &Camera) -> (f32, f32, f32, f32) {
    let pos = camera.static_config.fixed_position.clone();
    
    (
        pos.x - 1000.0,
        pos.y - 1000.0,
        pos.x + 1000.0,
        pos.y + 1000.0
    )
}

/// Test
#[cfg(test)]
mod tests {
    use super::*;
    use crate::math::Vec2;

    #[test]
    fn test_static_position() {
        let mut camera = Camera::new();
        camera.static_config.fixed_position = Vec2::new(100.0, 100.0);
        
        let pos = get_static_position(&camera);
        
        assert_eq!(pos.x, 100.0);
        assert_eq!(pos.y, 100.0);
    }

    #[test]
    fn test_static_bounds() {
        let mut camera = Camera::new();
        camera.static_config.fixed_position = Vec2::new(0.0, 0.0);
        
        let bounds = get_static_bounds(&camera);
        
        assert_eq!(bounds.0, -1000.0);
        assert_eq!(bounds.1, -1000.0);
        assert_eq!(bounds.2, 1000.0);
        assert_eq!(bounds.3, 1000.0);
    }
}
