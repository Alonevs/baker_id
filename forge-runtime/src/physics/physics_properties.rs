//! Physics properties configuration

use crate::physics::PhysicsType;
use crate::movement::StaticOverride;

/// Physics properties for entities
#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct PhysicsProperties {
    pub physics_type: PhysicsType,
    pub mass: f32,
    pub is_trigger: bool,
    pub z_height: f32,
    pub static_override: Option<StaticOverride>,
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

impl PhysicsProperties {
    /// Create static physics (fixed object)
    pub fn static_physics() -> Self {
        Self {
            physics_type: PhysicsType::Static,
            ..Default::default()
        }
    }
    
    /// Create kinematic physics (controlled movement)
    pub fn kinematic_physics() -> Self {
        Self {
            physics_type: PhysicsType::Kinematic,
            ..Default::default()
        }
    }
    
    /// Create trigger physics (collision detection only)
    pub fn trigger_physics() -> Self {
        Self {
            physics_type: PhysicsType::Trigger,
            is_trigger: true,
            ..Default::default()
        }
    }
    
    /// Set static override for falling platforms
    pub fn set_falling_platform(&mut self, fall_time: f32) {
        self.static_override = Some(StaticOverride {
            fall_time,
            fall_enabled: true,
        });
    }
    
    /// Set pushable object properties
    pub fn set_pushable(&mut self, mass: f32) {
        self.mass = mass;
        self.physics_type = PhysicsType::Dynamic;
    }
}

/// Global physics settings
#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct GlobalPhysics {
    pub gravity: f32,
    pub air_resistance: f32,
    pub terminal_velocity: f32,
    pub default_friction: f32,
}

impl Default for GlobalPhysics {
    fn default() -> Self {
        Self {
            gravity: 9.8,
            air_resistance: 1.0,
            terminal_velocity: 200.0,
            default_friction: 0.8,
        }
    }
}

/// Physics inspector panel data
#[derive(Clone)]
pub struct PhysicsInspector {
    pub global: GlobalPhysics,
    pub entity_physics: Option<PhysicsProperties>,
    pub preview_vector: (f32, f32),
}

impl Default for PhysicsInspector {
    fn default() -> Self {
        Self {
            global: GlobalPhysics::default(),
            entity_physics: None,
            preview_vector: (0.0, 0.0),
        }
    }
}
