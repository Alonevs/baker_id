//! # Cámara Side-Scroller
//! 
//! Vista lateral para juegos tipo Metal Slug, plataformas

use crate::camera::Camera;
use crate::math::Vec2;

/// Obtiene las coordenadas de pantalla para una entidad side-scroller
pub fn get_screen_position(entity_pos: Vec2, camera: &Camera) -> Vec2 {
    let viewport = camera.get_viewport();
    
    Vec2::new(
        entity_pos.x - viewport.0,
        entity_pos.y - viewport.1
    )
}

/// Verifica si una entidad está visible en pantalla
pub fn is_entity_visible(entity_pos: Vec2, camera: &Camera) -> bool {
    let viewport = camera.get_viewport();
    
    entity_pos.x >= viewport.0 &&
    entity_pos.x <= viewport.2 &&
    entity_pos.y >= viewport.1 &&
    entity_pos.y <= viewport.3
}

/// Obtiene el área visible en unidades del juego
pub fn get_visible_area(camera: &Camera) -> (f32, f32, f32, f32) {
    camera.get_viewport()
}

/// Obtiene los límites de renderizado (para optimizar)
pub fn get_render_bounds(camera: &Camera, entity: &Vec2) -> (f32, f32, f32, f32) {
    let viewport = camera.get_viewport();
    
    // Expandir viewport para incluir entidad
    let left = entity.x - 50.0;
    let right = entity.x + 50.0;
    let top = entity.y - 50.0;
    let bottom = entity.y + 50.0;
    
    // Intersectar con viewport
    let left = left.max(viewport.0);
    let right = right.min(viewport.0 + viewport.2);
    let top = top.max(viewport.1);
    let bottom = bottom.min(viewport.1 + viewport.3);
    
    (left, top, right - left, bottom - top)
}

/// Test
#[cfg(test)]
mod tests {
    use super::*;
    use crate::math::Vec2;

    #[test]
    fn test_screen_position() {
        let mut camera = Camera::new();
        camera.config.screen_width = 800.0;
        camera.config.screen_height = 600.0;
        
        let entity_pos = Vec2::new(400.0, 300.0);
        let screen_pos = get_screen_position(entity_pos, &camera);
        
        // Entidad en el centro debería aparecer cerca del centro
        assert!(screen_pos.x > 0.0);
        assert!(screen_pos.y > 0.0);
    }
}
