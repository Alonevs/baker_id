//! # Matemáticas Básicas
//! 
//! Vector2, Input, y otras utilidades matemáticas

use serde::{Deserialize, Serialize};

/// Vector 2D básico
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    /// Crea un nuevo vector
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    /// Vector cero
    pub fn zero() -> Self {
        Self { x: 0.0, y: 0.0 }
    }

    /// Obtiene la longitud
    pub fn length(&self) -> f32 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    /// Normaliza el vector
    pub fn normalize(&self) -> Vec2 {
        let len = self.length();
        if len > 0.0 {
            Vec2::new(self.x / len, self.y / len)
        } else {
            Vec2::zero()
        }
    }

    /// Obtiene la dirección
    pub fn direction(&self) -> f32 {
        self.x.atan2(self.y)
    }

    /// Suma con otro vector
    pub fn add(&self, other: Vec2) -> Vec2 {
        Vec2::new(self.x + other.x, self.y + other.y)
    }

    /// Resta con otro vector
    pub fn sub(&self, other: Vec2) -> Vec2 {
        Vec2::new(self.x - other.x, self.y - other.y)
    }

    /// Resta con otro vector (toma referencia)
    pub fn sub_ref(&self, other: &Vec2) -> Vec2 {
        Vec2::new(self.x - other.x, self.y - other.y)
    }

    /// Multiplica por un escalar
    pub fn mul(&self, scalar: f32) -> Vec2 {
        Vec2::new(self.x * scalar, self.y * scalar)
    }

    /// Divide por un escalar
    pub fn div(&self, scalar: f32) -> Vec2 {
        Vec2::new(self.x / scalar, self.y / scalar)
    }

    /// Obtiene el producto punto
    pub fn dot(&self, other: Vec2) -> f32 {
        self.x * other.x + self.y * other.y
    }

    /// Obtiene la distancia a otro vector
    pub fn distance_to(&self, other: Vec2) -> f32 {
        self.sub(other).length()
    }

    /// Clampear valores
    pub fn clamp(&self, min_x: f32, max_x: f32, min_y: f32, max_y: f32) -> Vec2 {
        Vec2::new(
            self.x.clamp(min_x, max_x),
            self.y.clamp(min_y, max_y)
        )
    }
}

impl Default for Vec2 {
    fn default() -> Self {
        Self::zero()
    }
}

/// Sistema de input básico
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Input {
    pub keys: Vec<KeyCode>,
    pub mouse_pos: Vec2,
    pub mouse_button: Option<MouseButton>,
}

impl Default for Input {
    fn default() -> Self {
        Self {
            keys: Vec::new(),
            mouse_pos: Vec2::zero(),
            mouse_button: None,
        }
    }
}

/// Código de tecla
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum KeyCode {
    A,
    S,
    D,
    W,
    Left,
    Right,
    Up,
    Down,
    Space,
    Escape,
    Enter,
}

/// Botón del mouse
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
}

/// Constantes matemáticas
pub mod constants {
    pub const PI: f32 = 3.14159265359;
    pub const TAU: f32 = 2.0 * PI;
    pub const DEG_TO_RAD: f32 = PI / 180.0;
    pub const RAD_TO_DEG: f32 = 180.0 / PI;
}

/// Utilidades matemáticas
pub fn random() -> f32 {
    let now = std::time::Instant::now().elapsed().as_nanos() as f32;
    now / 1_000_000_000.0
}

/// Implementa resta
impl std::ops::Sub for Vec2 {
    type Output = Vec2;
    fn sub(self, other: Vec2) -> Vec2 {
        Vec2::new(self.x - other.x, self.y - other.y)
    }
}

/// Implementa división por f32
impl std::ops::Div<f32> for Vec2 {
    type Output = Vec2;
    fn div(self, scalar: f32) -> Vec2 {
        Vec2::new(self.x / scalar, self.y / scalar)
    }
}

/// Test
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vec2_length() {
        let v = Vec2::new(3.0, 4.0);
        assert!((v.length() - 5.0).abs() < 0.01);
    }

    #[test]
    fn test_vec2_normalize() {
        let v = Vec2::new(3.0, 4.0);
        let normalized = v.normalize();
        assert!((normalized.length() - 1.0).abs() < 0.01);
    }

    #[test]
    fn test_vec2_add() {
        let v1 = Vec2::new(1.0, 2.0);
        let v2 = Vec2::new(3.0, 4.0);
        let result = v1.add(v2);
        assert_eq!(result.x, 4.0);
        assert_eq!(result.y, 6.0);
    }
}
