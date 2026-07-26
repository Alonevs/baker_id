//! # Cámara Unificada
//! 
//! Estructura base que soporta múltiples tipos de cámara

use serde::{Deserialize, Serialize};
use crate::math::Vec2;

/// Tipos de cámara soportados
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CameraType {
    /// Vista isométrica (45° diagonal)
    Isometric,
    /// Side-scroller (Metal Slug, plataformas)
    SideScroller,
    /// Top-down (RPG clásico, Zelda, Hotline Miami)
    TopDown,
    /// Free-2D (Naves, Shoot 'em up)
    Free2D,
    /// Raycaster (Doom, Wolfenstein 3D)
    Raycaster,
    /// Estática (Novela visual, cinemáticas)
    Static,
}

impl Default for CameraType {
    fn default() -> Self {
        Self::SideScroller
    }
}

impl std::fmt::Display for CameraType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Isometric => write!(f, "Isometric"),
            Self::SideScroller => write!(f, "Side-Scroller"),
            Self::TopDown => write!(f, "Top-Down"),
            Self::Free2D => write!(f, "Free-2D"),
            Self::Raycaster => write!(f, "Raycaster"),
            Self::Static => write!(f, "Static"),
        }
    }
}

/// Configuración base de cámara
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CameraConfig {
    /// Tipo de cámara
    pub camera_type: CameraType,
    /// Posición actual de la cámara
    pub position: Vec2,
    /// Punto objetivo que la cámara sigue
    pub target: Vec2,
    /// Nivel de zoom (1.0 = normal)
    pub zoom: f32,
    /// Ancho de la pantalla en unidades del juego
    pub screen_width: f32,
    /// Alto de la pantalla en unidades del juego
    pub screen_height: f32,
    /// Suavizado del seguimiento (0.0 = instantáneo, 1.0 = muy suave)
    pub smooth_follow: bool,
    pub smooth_factor: f32,
    /// Velocidad de lerp de zoom
    pub zoom_lerp_speed: f32,
}

impl Default for CameraConfig {
    fn default() -> Self {
        Self {
            camera_type: CameraType::default(),
            position: Vec2::new(0.0, 0.0),
            target: Vec2::new(0.0, 0.0),
            zoom: 1.0,
            screen_width: 800.0,
            screen_height: 600.0,
            smooth_follow: true,
            smooth_factor: 0.1,
            zoom_lerp_speed: 2.0,
        }
    }
}

/// Configuración específica para cámara isométrica
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IsometricConfig {
    /// Escala X (horizontal)
    pub scale_x: f32,
    /// Escala Y (vertical)
    pub scale_y: f32,
    /// Rotación base (45° para isométrica)
    pub rotation: f32,
}

impl Default for IsometricConfig {
    fn default() -> Self {
        Self {
            scale_x: 0.7,
            scale_y: 0.5,
            rotation: 45.0,
        }
    }
}

/// Configuración específica para side-scroller
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SideScrollerConfig {
    /// Suavizado del seguimiento
    pub smooth_follow: bool,
    /// Factor de suavizado (0.01 - 0.5)
    pub smooth_factor: f32,
    /// Zona muerta (no sigue hasta que se mueva más que esto)
    pub dead_zone: f32,
    /// Clampar a límites de pantalla
    pub clamp_to_screen: bool,
    /// Límite lateral izquierdo
    pub left_limit: f32,
    /// Límite lateral derecho
    pub right_limit: f32,
}

impl Default for SideScrollerConfig {
    fn default() -> Self {
        Self {
            smooth_follow: true,
            smooth_factor: 0.1,
            dead_zone: 10.0,
            clamp_to_screen: true,
            left_limit: -1000.0,
            right_limit: 1000.0,
        }
    }
}

/// Configuración específica para top-down
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TopDownConfig {
    /// Permitir rotación libre de sprites
    pub rotation_enabled: bool,
    /// Velocidad de rotación
    pub rotation_speed: f32,
    /// Rotación base (0° = arriba)
    pub rotation_base: f32,
    /// Y-sorting (ordenar por profundidad)
    pub y_sorting: bool,
    /// Clampar al centro de pantalla
    pub clamp_to_center: bool,
}

impl Default for TopDownConfig {
    fn default() -> Self {
        Self {
            rotation_enabled: true,
            rotation_speed: 90.0,
            rotation_base: 0.0,
            y_sorting: true,
            clamp_to_center: true,
        }
    }
}

/// Configuración específica para free-2D
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Free2DConfig {
    /// Auto-scroll del escenario
    pub auto_scroll: bool,
    /// Velocidad de auto-scroll
    pub scroll_speed: f32,
    /// Zoom enabled
    pub zoom_enabled: bool,
    /// Zoom target
    pub zoom_target: f32,
    /// Zoom lerp speed
    pub zoom_lerp_speed: f32,
}

impl Default for Free2DConfig {
    fn default() -> Self {
        Self {
            auto_scroll: true,
            scroll_speed: 1.0,
            zoom_enabled: true,
            zoom_target: 1.0,
            zoom_lerp_speed: 2.0,
        }
    }
}

/// Configuración específica para raycaster
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RaycasterConfig {
    /// Campo de visión (FOV en grados)
    pub fov: f32,
    /// Resolución de renderizado
    pub resolution: Vec2,
    /// Tamaño de celda del mapa
    pub cell_size: f32,
    /// Altura del jugador
    pub player_height: f32,
    /// Escala de proyección
    pub projection_scale: f32,
}

impl Default for RaycasterConfig {
    fn default() -> Self {
        Self {
            fov: 90.0,
            resolution: Vec2::new(320.0, 200.0),
            cell_size: 64.0,
            player_height: 16.0,
            projection_scale: 1.0,
        }
    }
}

/// Configuración específica para cámara estática
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StaticConfig {
    /// Posición fija de la cámara
    pub fixed_position: Vec2,
    /// Duración de transición (segundos)
    pub transition_duration: f32,
    /// Estado de transición
    pub in_transition: bool,
    /// Fade in/out
    pub fade_enabled: bool,
    /// Fade speed
    pub fade_speed: f32,
}

impl Default for StaticConfig {
    fn default() -> Self {
        Self {
            fixed_position: Vec2::new(0.0, 0.0),
            transition_duration: 1.0,
            in_transition: false,
            fade_enabled: false,
            fade_speed: 2.0,
        }
    }
}

/// Estructura principal de cámara
#[derive(Debug, Clone)]
pub struct Camera {
    /// Tipo de cámara
    pub camera_type: CameraType,
    /// Posición actual
    pub position: Vec2,
    /// Objetivo
    pub target: Vec2,
    /// Rotación (para top-down)
    pub rotation: f32,
    /// Zoom actual
    pub zoom: f32,
    /// Configuración base
    pub config: CameraConfig,
    /// Configuración específica por tipo
    pub isometric: IsometricConfig,
    pub side_scroller: SideScrollerConfig,
    pub top_down: TopDownConfig,
    pub free_2d: Free2DConfig,
    pub raycaster: RaycasterConfig,
    pub static_config: StaticConfig,
    /// Entidad que sigue (opcional)
    pub follow_target: Option<String>,
    /// Entidades ordenadas por Y (para top-down)
    pub sorted_entities: Vec<String>,
}

impl Camera {
    /// Crea una nueva cámara con configuración por defecto
    pub fn new() -> Self {
        Self {
            camera_type: CameraType::default(),
            position: Vec2::new(0.0, 0.0),
            target: Vec2::new(0.0, 0.0),
            rotation: 0.0,
            zoom: 1.0,
            config: CameraConfig::default(),
            isometric: IsometricConfig::default(),
            side_scroller: SideScrollerConfig::default(),
            top_down: TopDownConfig::default(),
            free_2d: Free2DConfig::default(),
            raycaster: RaycasterConfig::default(),
            static_config: StaticConfig::default(),
            follow_target: None,
            sorted_entities: Vec::new(),
        }
    }

    /// Cambia el tipo de cámara
    pub fn set_camera_type(&mut self, camera_type: CameraType) {
        self.camera_type = camera_type;
    }

    /// Establece el objetivo que la cámara sigue
    pub fn set_follow_target(&mut self, target: Option<String>) {
        self.follow_target = target;
    }

    /// Actualiza la cámara según su tipo
    pub fn update(&mut self, dt: f32) {
        match self.camera_type {
            CameraType::Isometric => self.update_isometric(dt),
            CameraType::SideScroller => self.update_side_scroller(dt),
            CameraType::TopDown => self.update_top_down(dt),
            CameraType::Free2D => self.update_free_2d(dt),
            CameraType::Raycaster => self.update_raycaster(dt),
            CameraType::Static => self.update_static(dt),
        }
    }

    /// Actualiza cámara isométrica
    fn update_isometric(&mut self, dt: f32) {
        // Aplicar suavizado al objetivo
        if self.config.smooth_follow && self.follow_target.is_some() {
            self.target.x += (self.config.target.x - self.target.x) * self.config.smooth_factor * dt;
            self.target.y += (self.config.target.y - self.target.y) * self.config.smooth_factor * dt;
        }

        // Interpolación hacia el objetivo
        self.position.x += (self.target.x - self.position.x) * self.config.smooth_factor * dt;
        self.position.y += (self.target.y - self.position.y) * self.config.smooth_factor * dt;

        // Aplicar zoom
        self.zoom = (self.zoom + (self.config.zoom - self.zoom) * dt * self.config.zoom_lerp_speed).clamp(0.1, 5.0);
    }

    /// Actualiza cámara side-scroller
    fn update_side_scroller(&mut self, dt: f32) {
        // Aplicar suavizado
        if self.config.smooth_follow && self.follow_target.is_some() {
            let dx = self.config.target.x - self.target.x;
            let dy = self.config.target.y - self.target.y;
            
            if (dx.abs() > self.side_scroller.dead_zone) || (dy.abs() > self.side_scroller.dead_zone) {
                self.target.x += dx * self.config.smooth_factor * dt;
                self.target.y += dy * self.config.smooth_factor * dt;
            }
        }

        // Clampar a límites
        if self.side_scroller.clamp_to_screen {
            self.position.x = self.position.x.clamp(
                self.side_scroller.left_limit,
                self.side_scroller.right_limit
            );
        }
    }

    /// Actualiza cámara top-down
    fn update_top_down(&mut self, dt: f32) {
        // Rotación
        if self.top_down.rotation_enabled && self.follow_target.is_some() {
            let dx = self.config.target.x - self.position.x;
            let dy = self.config.target.y - self.position.y;
            
            // Calcular ángulo hacia el objetivo
            let angle = f32::atan2(dy, dx);
            self.rotation = angle;
        }

        // Suavizado
        if self.config.smooth_follow {
            self.position.x += (self.config.target.x - self.position.x) * self.config.smooth_factor * dt;
            self.position.y += (self.config.target.y - self.position.y) * self.config.smooth_factor * dt;
        }

        // Clampar al centro
        if self.top_down.clamp_to_center {
            self.position.x = self.position.x.clamp(
                self.config.screen_width / 2.0 - 1000.0,
                self.config.screen_width / 2.0 + 1000.0
            );
            self.position.y = self.position.y.clamp(
                self.config.screen_height / 2.0 - 1000.0,
                self.config.screen_height / 2.0 + 1000.0
            );
        }
    }

    /// Actualiza cámara free-2D
    fn update_free_2d(&mut self, dt: f32) {
        // Auto-scroll
        if self.free_2d.auto_scroll && self.follow_target.is_some() {
            let dx = self.config.target.x - self.position.x;
            let dy = self.config.target.y - self.position.y;
            
            self.position.x += dx * self.free_2d.scroll_speed * dt;
            self.position.y += dy * self.free_2d.scroll_speed * dt;
        }

        // Zoom
        if self.free_2d.zoom_enabled {
            self.zoom = (self.zoom + (self.free_2d.zoom_target - self.zoom) * dt * self.free_2d.zoom_lerp_speed).clamp(0.1, 5.0);
        }
    }

    /// Actualiza cámara raycaster
    fn update_raycaster(&mut self, dt: f32) {
        // Cámara estática en primera persona
        let pos = self.static_config.fixed_position.clone();
        self.position = pos.clone();
        self.target = pos.clone();
    }

    /// Actualiza cámara estática
    fn update_static(&mut self, dt: f32) {
        // Posición fija
        let pos = self.static_config.fixed_position.clone();
        self.position = pos.clone();
        self.target = pos.clone();

        // Fade in/out
        if self.static_config.fade_enabled {
            // Implementar lógica de fade
        }
    }

    /// Obtiene el viewport actual
    pub fn get_viewport(&self) -> (f32, f32, f32, f32) {
        (
            self.position.x - self.config.screen_width / 2.0 * self.zoom,
            self.position.y - self.config.screen_height / 2.0 * self.zoom,
            self.config.screen_width * self.zoom,
            self.config.screen_height * self.zoom,
        )
    }
}

impl Default for Camera {
    fn default() -> Self {
        Self::new()
    }
}
