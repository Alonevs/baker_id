//! # Cámara Top-Down
//! 
//! Vista de pájaro para RPG clásico, Zelda, Hotline Miami

use crate::camera::Camera;
use crate::math::Vec2;

/// Ordena entidades por Y (para Y-sorting)
pub fn sort_entities_by_y(entities: &mut [String]) {
    entities.sort_by(|a, b| {
        // Aquí deberías comparar las posiciones Y de las entidades
        // Por ahora usamos el orden alfabético como placeholder
        a.cmp(b)
    });
}

/// Obtiene el ángulo hacia un objetivo desde la posición de la cámara
pub fn get_angle_to_target(camera_pos: Vec2, target_pos: Vec2) -> f32 {
    let dx = target_pos.x - camera_pos.x;
    let dy = target_pos.y - camera_pos.y;
    
    dy.atan2(dx)
}

/// Obtiene la distancia entre dos posiciones
pub fn get_distance_to_target(camera_pos: Vec2, target_pos: Vec2) -> f32 {
    camera_pos.sub(target_pos).length()
}

/// Verifica si hay colisión entre dos entidades
pub fn check_collision(entity1_pos: Vec2, entity1_size: Vec2, entity2_pos: Vec2, entity2_size: Vec2) -> bool {
    let half_size1 = entity1_size.div(2.0);
    let half_size2 = entity2_size.div(2.0);
    
    let dx = (entity1_pos.x + half_size1.x) - (entity2_pos.x - half_size2.x);
    let dy = (entity1_pos.y + half_size1.y) - (entity2_pos.y - half_size2.y);
    
    dx.abs() < (entity1_size.x + entity2_size.x) / 2.0 &&
    dy.abs() < (entity1_size.y + entity2_size.y) / 2.0
}

/// Obtiene el tile bajo el puntero del mouse
pub fn get_tile_at_screen_position(screen_pos: Vec2, camera: &Camera, tile_size: f32) -> Option<(i32, i32)> {
    let viewport = camera.get_viewport();
    let world_pos = Vec2::new(
        screen_pos.x + viewport.0,
        screen_pos.y + viewport.1
    );
    
    let tile_x = (world_pos.x / tile_size).floor() as i32;
    let tile_y = (world_pos.y / tile_size).floor() as i32;
    
    Some((tile_x, tile_y))
}

/// Test
#[cfg(test)]
mod tests {
    use super::*;
    use crate::math::Vec2;

    #[test]
    fn test_angle_to_target() {
        let camera_pos = Vec2::new(0.0, 0.0);
        let target_pos = Vec2::new(10.0, 0.0);
        
        let angle = get_angle_to_target(camera_pos, target_pos);
        
        // Debería ser 0 (derecha)
        assert!((angle - 0.0).abs() < 0.1);
    }

    #[test]
    fn test_distance() {
        let pos1 = Vec2::new(0.0, 0.0);
        let pos2 = Vec2::new(3.0, 4.0);
        
        let distance = get_distance_to_target(pos1, pos2);
        
        assert!((distance - 5.0).abs() < 0.01);
    }

    #[test]
    fn test_collision() {
        let entity1_pos = Vec2::new(0.0, 0.0);
        let entity1_size = Vec2::new(10.0, 10.0);
        let entity2_pos = Vec2::new(5.0, 5.0);
        let entity2_size = Vec2::new(10.0, 10.0);
        
        let collision = check_collision(entity1_pos, entity1_size, entity2_pos, entity2_size);
        
        assert!(collision);
    }
}
