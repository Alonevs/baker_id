//! # Tile Projections
//! 
//! Proyecciones para tiles (cuadrados/paralelogramos) en diferentes tipos de vista

use crate::camera::Camera;
use crate::math::Vec2;

/// Proyecta coordenadas del mundo a coordenadas de pantalla (isométrica)
pub fn project_isometric(world_pos: Vec2, camera: &Camera) -> Vec2 {
    let screen_width = camera.config.screen_width;
    let screen_height = camera.config.screen_height;
    
    // Rotación de 45°
    let rotation_rad = camera.isometric.rotation * crate::math::constants::DEG_TO_RAD;
    
    // Rotar y escalar
    let x = world_pos.x * camera.isometric.scale_x - world_pos.y * camera.isometric.scale_y;
    let y = world_pos.x * camera.isometric.scale_x + world_pos.y * camera.isometric.scale_y;
    
    // Aplicar rotación
    let rotated_x = x * rotation_rad.cos() - y * rotation_rad.sin();
    let rotated_y = x * rotation_rad.sin() + y * rotation_rad.cos();
    
    // Proyectar a pantalla
    Vec2::new(
        screen_width / 2.0 - rotated_x * camera.zoom,
        screen_height / 2.0 - rotated_y * camera.zoom
    )
}

/// Proyecta coordenadas de pantalla a coordenadas del mundo (isométrica)
pub fn unproject_isometric(screen_pos: Vec2, camera: &Camera) -> Vec2 {
    let screen_width = camera.config.screen_width;
    let screen_height = camera.config.screen_height;
    
    // Invertir proyección
    let world_x = (screen_width / 2.0 - screen_pos.x) / (camera.isometric.scale_x * camera.zoom);
    let world_y = (screen_height / 2.0 - screen_pos.y) / (camera.isometric.scale_y * camera.zoom);
    
    // Invertir rotación de 45°
    let rotation_rad = camera.isometric.rotation * crate::math::constants::DEG_TO_RAD;
    
    Vec2::new(
        world_x * rotation_rad.cos() + world_y * rotation_rad.sin(),
        -world_x * rotation_rad.sin() + world_y * rotation_rad.cos()
    )
}

/// Obtiene las coordenadas de los 4 vértices de un tile
pub fn get_tile_vertices(tile_pos: Vec2, tile_size: f32, camera: &Camera) -> [Vec2; 4] {
    let center = project_isometric(tile_pos, camera);
    let half_size = tile_size * 0.5;
    
    [
        Vec2::new(center.x - half_size, center.y - half_size), // Top-left
        Vec2::new(center.x + half_size, center.y - half_size), // Top-right
        Vec2::new(center.x + half_size, center.y + half_size), // Bottom-right
        Vec2::new(center.x - half_size, center.y + half_size), // Bottom-left
    ]
}

/// Verifica si un punto de pantalla está dentro de un tile
pub fn is_point_in_tile(screen_pos: Vec2, tile_pos: Vec2, tile_size: f32, camera: &Camera) -> bool {
    let vertices = get_tile_vertices(tile_pos, tile_size, camera);
    
    // Verificar si el punto está dentro del tile
    let mut inside = true;
    
    for i in 0..4 {
        let p1 = vertices[i].clone();
        let p2 = vertices[(i + 1) % 4].clone();
        
        // Calcular vector del borde
        let edge = p2.sub_ref(&p1);
        let edge_len = edge.length();
        let edge_normalized = edge.div(edge_len);
        
        // Vector desde p1 hasta el punto
        let to_point = screen_pos.sub(p1);
        let edge_norm = edge_normalized.clone();
        let edge_norm_for_dot = edge_norm.clone();
        let dot_result = to_point.dot(edge_norm_for_dot);
        let edge_norm_for_proj = edge_norm.clone();
        let projection = to_point.sub(edge_norm_for_proj.mul(dot_result));
        
        // Verificar si está dentro del borde
        if projection.dot(edge_norm).abs() > tile_size * 0.5 {
            inside = false;
            break;
        }
    }
    
    inside
}

/// Obtiene el tile bajo el puntero del mouse
pub fn get_tile_at_screen_position(screen_pos: Vec2, camera: &Camera, map_width: i32, map_height: i32) -> Option<(i32, i32)> {
    // Invertir proyección para obtener posición del mundo
    let world_pos = unproject_isometric(screen_pos, camera);
    
    // Calcular tile
    let tile_x = (world_pos.x / 10.0).floor() as i32;
    let tile_y = (world_pos.y / 10.0).floor() as i32;
    
    if tile_x >= 0 && tile_x < map_width && tile_y >= 0 && tile_y < map_height {
        Some((tile_x, tile_y))
    } else {
        None
    }
}

/// Test
#[cfg(test)]
mod tests {
    use super::*;
    use crate::math::Vec2;

    #[test]
    fn test_tile_projection() {
        let mut camera = Camera::new();
        camera.config.screen_width = 800.0;
        camera.config.screen_height = 600.0;
        
        // Crear tile en el centro
        let tile_pos = Vec2::new(10.0, 10.0);
        let tile_size = 20.0;
        
        // Punto en el centro del tile (debería estar dentro)
        let vertices = get_tile_vertices(tile_pos.clone(), tile_size, &camera);
        let center_pos = vertices[0].clone();
        let center_pos = Vec2::new(center_pos.x + 5.0, center_pos.y + 5.0);
        
        let inside = is_point_in_tile(center_pos, tile_pos, tile_size, &camera);
        
        assert!(inside, "Punto en el centro debería estar dentro del tile");
    }

    #[test]
    fn test_isometric_projection() {
        let mut camera = Camera::new();
        camera.config.screen_width = 800.0;
        camera.config.screen_height = 600.0;
        
        let world_pos = Vec2::new(0.0, 0.0);
        let screen_pos = project_isometric(world_pos, &camera);
        
        assert!(screen_pos.x > 0.0);
        assert!(screen_pos.y > 0.0);
    }
}
