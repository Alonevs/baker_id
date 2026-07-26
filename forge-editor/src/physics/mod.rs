//! # Sistema de Física
//! 
//! Sistema de colisiones y movimiento genérico que funciona con cualquier tipo de cámara.
//! No está asociado a ningún tipo de visualización (isométrica, top-down, etc.).

pub mod collision;
pub mod physics;

pub use collision::CollisionSystem;
pub use physics::{PhysicsBody, PhysicsConfig};
