//! Physics system for 2D game engine
//!
//! Implements physics profiles, movement mechanics, and collision detection.

pub mod movement;
pub mod collision;
pub mod physics_properties;

/// Movement profile configuration
#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct MovementProfile {
    pub gravity: f32,
    pub friction: f32,
    pub air_resistance: f32,
    pub terminal_velocity: f32,
    pub style: MovementStyle,
}

/// Movement style types
#[derive(Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum MovementStyle {
    Platformer,      // Mario-style with gravity
    TopDown,         // Zelda/RPG isometric
    Flight,          // Ship-style free movement
}

impl Default for MovementProfile {
    fn default() -> Self {
        Self {
            gravity: 9.8,
            friction: 0.8,
            air_resistance: 1.0,
            terminal_velocity: 200.0,
            style: MovementStyle::Platformer,
        }
    }
}

/// Physics properties for entities
#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct PhysicsProperties {
    pub physics_type: PhysicsType,
    pub mass: f32,
    pub is_trigger: bool,
    pub z_height: f32,
    pub static_override: Option<StaticOverride>,
}

#[derive(Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum PhysicsType {
    Static,         // Fixed, no gravity
    Kinematic,      // Controlled movement (platforms)
    Dynamic,        // Full physics simulation
    Trigger,        // Collision detection only
}

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct StaticOverride {
    pub fall_time: f32,
    pub fall_enabled: bool,
}

/// Collision box types
#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub enum CollisionShape {
    AABB {
        x: f32,
        y: f32,
        width: f32,
        height: f32,
    },
    Polygon {
        vertices: Vec<(f32, f32)>,
    },
}

/// Collision brush types for map painting
#[derive(Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum CollisionBrush {
    Solid,          // Red - total block
    OneWay,         // Orange - platform
    Ramp,           // Blue - slope
}

impl Default for PhysicsProperties {
    fn default() -> Self {
        Self {
            physics_type: PhysicsType::Dynamic,
            mass: 10.0,
            is_trigger: false,
            z_height: 0.0,
            static_override: None,
        }
    }
}
