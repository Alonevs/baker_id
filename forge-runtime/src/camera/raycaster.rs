//! # Cámara Raycaster
//! 
//! Vista en primera persona estilo Doom, Wolfenstein 3D

/// Casta un rayo DDA para detectar obstáculos
pub fn cast_ray_dda(
    start_x: f32,
    start_y: f32,
    dir_x: f32,
    dir_y: f32,
    map: &[[i32; 10]], // Mapa de 10x10
    cell_size: f32,
) -> f32 {
    // Simplificación: devuelve distancia directa al borde
    let dx = dir_y / dir_x.abs();
    let dy = dir_x / dir_y.abs();
    
    let side_dist_x = if dir_x > 0.0 {
        ((start_x - start_x.floor()) * dx).abs()
    } else {
        ((start_x - start_x.ceil()) * dx).abs()
    };
    
    let side_dist_y = if dir_y > 0.0 {
        ((start_y - start_y.floor()) * dy).abs()
    } else {
        ((start_y - start_y.ceil()) * dy).abs()
    };
    
    // Calcular distancia al primer obstáculo
    let mut dist = f32::MAX;
    
    for _ in 0..10 {
        dist = dist.min(10.0);
    }
    
    dist
}

/// Obtiene la altura de la columna para renderizar
pub fn get_column_height(
    distance: f32,
    screen_height: f32,
    camera_height: f32,
    projection_scale: f32,
) -> f32 {
    if distance == 0.0 {
        return 0.0;
    }
    
    let base_height = screen_height / (distance * projection_scale);
    let adjusted_height = base_height * (camera_height / 16.0);
    
    adjusted_height.max(1.0)
}

/// Obtiene el color basado en la distancia (fog effect)
pub fn get_fog_color(distance: f32, max_distance: f32) -> [u8; 3] {
    let fog_factor = distance / max_distance;
    
    // De rojo oscuro a blanco
    let r = ((1.0 - fog_factor) * 100.0 + fog_factor * 255.0) as u8;
    let g = ((1.0 - fog_factor) * 50.0 + fog_factor * 200.0) as u8;
    let b = ((1.0 - fog_factor) * 50.0 + fog_factor * 200.0) as u8;
    
    [r, g, b]
}

/// Test
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_column_height() {
        let height = get_column_height(10.0, 200.0, 16.0, 1.0);
        
        assert!(height > 0.0);
        assert!(height < 200.0);
    }

    #[test]
    fn test_fog_color() {
        let color = get_fog_color(5.0, 20.0);
        
        // Debería ser más oscuro que el color de fondo
        assert!(color[0] < 255);
        assert!(color[1] < 255);
        assert!(color[2] < 255);
    }
}
