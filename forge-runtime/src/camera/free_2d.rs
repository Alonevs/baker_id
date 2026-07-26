//! # Cámara Free-2D
//! 
//! Vista libre 2D para juegos de naves, shoot 'em up

use crate::camera::Camera;
use crate::math::Vec2;

/// Obtiene coordenadas de pantalla con zoom y desplazamiento
pub fn get_screen_position_with_zoom(entity_pos: Vec2, camera: &Camera) -> Vec2 {
    let viewport = camera.get_viewport();
    
    Vec2::new(
        (entity_pos.x - viewport.0) * camera.zoom,
        (entity_pos.y - viewport.1) * camera.zoom
    )
}

/// Obtiene el área visible corregido por zoom
pub fn get_visible_area_with_zoom(camera: &Camera) -> (f32, f32, f32, f32) {
    let viewport = camera.get_viewport();
    
    (
        viewport.0 / camera.zoom,
        viewport.1 / camera.zoom,
        viewport.2 / camera.zoom,
        viewport.3 / camera.zoom
    )
}

/// Obtiene el tile bajo el puntero del mouse
pub fn get_tile_at_screen_position(screen_pos: Vec2, camera: &Camera, tile_size: f32) -> Option<(i32, i32)> {
    let viewport = camera.get_viewport();
    let world_pos = Vec2::new(
        screen_pos.x / camera.zoom + viewport.0,
        screen_pos.y / camera.zoom + viewport.1
    );
    
    let tile_x = (world_pos.x / tile_size).floor() as i32;
    let tile_y = (world_pos.y / tile_size).floor() as i32;
    
    Some((tile_x, tile_y))
}

/// Obtiene la distancia desde el centro de pantalla
pub fn get_distance_from_center(entity_pos: Vec2, camera: &Camera) -> f32 {
    let viewport = camera.get_viewport();
    let center_x = viewport.0 + viewport.2 / 2.0;
    let center_y = viewport.1 + viewport.3 / 2.0;
    
    let dx = entity_pos.x - center_x;
    let dy = entity_pos.y - center_y;
    
    (dx * dx + dy * dy).sqrt()
}

/// Obtiene la dirección hacia el centro de pantalla
pub fn get_direction_to_center(entity_pos: Vec2, camera: &Camera) -> Vec2 {
    let viewport = camera.get_viewport();
    let center_x = viewport.0 + viewport.2 / 2.0;
    let center_y = viewport.1 + viewport.3 / 2.0;
    
    let dx = center_x - entity_pos.x;
    let dy = center_y - entity_pos.y;
    
    Vec2::new(dx, dy).normalize()
}

/// Test
#[cfg(test)]
mod tests {
    use super::*;
    use crate::math::Vec2;

    #[test]
    fn test_screen_position_with_zoom() {
        let mut camera = Camera::new();
        camera.config.screen_width = 800.0;
        camera.config.screen_height = 600.0;
        camera.zoom = 2.0;
        
        let entity_pos = Vec2::new(400.0, 300.0);
        let screen_pos = get_screen_position_with_zoom(entity_pos, &camera);
        
        // Con zoom 2x, la entidad debería aparecer más cerca del centro
        assert!(screen_pos.x > 0.0);
        assert!(screen_pos.y > 0.0);
    }

    #[test]
    fn test_distance_from_center() {
        let mut camera = Camera::new();
        camera.config.screen_width = 800.0;
        camera.config.screen_height = 600.0;
        
        let entity_pos = Vec2::new(400.0, 300.0);
        let distance = get_distance_from_center(entity_pos, &camera);
        
        // Debería ser 0 (está en el centro)
        assert!((distance - 0.0).abs() < 1.0);
    }
}
