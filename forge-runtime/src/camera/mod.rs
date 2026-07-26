//! # Forge Camera System
//! 
//! Sistema de cámara unificado que soporta múltiples tipos de vista:
//! - Isometric (vista diagonal 45°)
//! - Side-Scroller (Metal Slug / Plataformas)
//! - Top-Down (RPG clásico / Hotline Miami)
//! - Free-2D (Naves / Shoot 'em up)
//! - Raycaster (Doom / Wolfenstein 3D)
//! - Static (Novela visual / Cinemáticas)

pub mod camera;
pub mod tile;
pub mod side_scroller;
pub mod top_down;
pub mod free_2d;
pub mod raycaster;
pub mod static_camera;

pub use camera::*;
pub use tile::*;
pub use side_scroller::*;
pub use top_down::*;
pub use free_2d::*;
pub use raycaster::*;
pub use static_camera::*;
