//! # Base Module
//! 
//! Módulo base con tipos compartidos entre main.rs y ui/
//! 
//! ## Arquitectura
//! 
//! ```
//! +------------------+     +------------------+     +------------------+
//! |    main.rs       | --> |    ui/mod.rs     | --> |  viewport.rs     |
//! +------------------+     +------------------+     +------------------+
//!         |                       |                           |
//!         | (App)                 | (App)                    | (UI)
//!         v                       v                           v
//! +--------+                 +--------+                   +--------+
//! | App    |                 | App    |                   | UI     |
//! +--------+                 +--------+                   +--------+
//! ```
//! 
//! ## Uso
//! 
//! ```rust
//! // En main.rs:
//! use base::App;
//! impl App for ForgeEditorApp { /* implementation */ }
//! 
//! // En ui/
//! use base::App;
//! struct Viewport<'a> { app: &'a mut dyn App }
//! ```

/// Trait que define la interfaz mínima de la aplicación
/// 
/// Permite desacoplar los módulos UI de la implementación concreta de la app.
pub trait App {
    /// Obtiene el estado de física
    fn physics(&self) -> &dyn Physics;
    
    /// Obtiene el sistema de partículas
    fn particles(&self) -> &dyn Particles;
    
    /// Obtiene el motor de animación
    fn animation(&self) -> &dyn Animation;
}

/// Trait para física
pub trait Physics {
    fn block_count(&self) -> usize;
}

/// Trait para partículas
pub trait Particles {
    fn particle_count(&self) -> usize;
}

/// Trait para animación
pub trait Animation {
    fn elapsed_time(&self) -> f32;
}

