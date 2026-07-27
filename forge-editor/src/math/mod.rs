pub mod vec2;
pub mod input;

pub use vec2::{Vector2, Vec2};
pub use input::{Input, KeyCode};

/// Utilidades matemáticas
pub struct Math;

impl Math {
    pub fn random() -> f32 {
        std::time::Instant::now().elapsed().as_nanos() as f32 / 1_000_000_000.0
    }
}
