//! Movement mechanics and physics profiles

use crate::physics::{MovementProfile, MovementStyle, CollisionBrush};

/// Movement state for entities
#[derive(Clone)]
pub struct MovementState {
    pub velocity_x: f32,
    pub velocity_y: f32,
    pub is_moving: bool,
    pub is_airborne: bool,
    pub is_on_ground: bool,
    pub facing_direction: f32, // 0-360 degrees
}

impl Default for MovementState {
    fn default() -> Self {
        Self {
            velocity_x: 0.0,
            velocity_y: 0.0,
            is_moving: false,
            is_airborne: false,
            is_on_ground: false,
            facing_direction: 0.0,
        }
    }
}

/// Apply physics to entity based on profile
pub fn apply_physics(
    state: &mut MovementState,
    profile: &MovementProfile,
    input: &PlayerInput,
    dt: f32,
) {
    match profile.style {
        MovementStyle::Platformer => apply_platformer_physics(state, profile, input, dt),
        MovementStyle::TopDown => apply_topdown_physics(state, profile, input, dt),
        MovementStyle::Flight => apply_flight_physics(state, profile, input, dt),
    }
}

fn apply_platformer_physics(
    state: &mut MovementState,
    profile: &MovementProfile,
    input: &PlayerInput,
    _dt: f32,
) {
    // Horizontal movement
    let mut accel_x = 0.0;
    if input.left { accel_x -= 500.0; }
    if input.right { accel_x += 500.0; }
    
    state.velocity_x += accel_x * _dt;
    state.velocity_x *= profile.friction;
    
    // Gravity
    if !state.is_airborne {
        state.velocity_y += profile.gravity * _dt;
        state.is_on_ground = true;
    } else {
        state.velocity_y += profile.gravity * _dt * profile.air_resistance;
        state.is_on_ground = false;
    }
    
    // Terminal velocity
    state.velocity_y = state.velocity_y.clamp(-profile.terminal_velocity, profile.terminal_velocity);
    
    // Jump
    if input.jump && state.is_on_ground {
        state.velocity_y = -profile.gravity * 0.5;
        state.is_airborne = true;
    }
}

fn apply_topdown_physics(
    state: &mut MovementState,
    profile: &MovementProfile,
    input: &PlayerInput,
    _dt: f32,
) {
    // 8-directional movement without gravity
    let move_speed = 300.0;
    
    if input.up { state.velocity_y -= move_speed; }
    if input.down { state.velocity_y += move_speed; }
    if input.left { state.velocity_x -= move_speed; }
    if input.right { state.velocity_x += move_speed; }
    
    state.velocity_x *= profile.friction * _dt;
    state.velocity_y *= profile.friction * _dt;
    
    state.is_airborne = false;
    state.is_on_ground = true;
}

fn apply_flight_physics(
    state: &mut MovementState,
    profile: &MovementProfile,
    input: &PlayerInput,
    _dt: f32,
) {
    // Free 360° movement with acceleration
    let thrust = 800.0;
    
    if input.up { state.velocity_y -= thrust * _dt; }
    if input.down { state.velocity_y += thrust * _dt; }
    if input.left { state.velocity_x -= thrust * _dt; }
    if input.right { state.velocity_x += thrust * _dt; }
    
    // Drag
    state.velocity_x *= profile.air_resistance;
    state.velocity_y *= profile.air_resistance;
    
    // Auto-rotate to face movement
    if state.velocity_x.abs() > 1.0 || state.velocity_y.abs() > 1.0 {
        let angle = (state.velocity_x * 180.0 / std::f32::consts::PI + state.velocity_y * 180.0 / std::f32::consts::PI).atan() * 180.0 / std::f32::consts::PI;
        state.facing_direction = angle;
    }
}

/// Player input state
#[derive(Clone)]
pub struct PlayerInput {
    pub left: bool,
    pub right: bool,
    pub up: bool,
    pub down: bool,
    pub jump: bool,
    pub attack: bool,
    pub sprint: bool,
}

impl Default for PlayerInput {
    fn default() -> Self {
        Self {
            left: false,
            right: false,
            up: false,
            down: false,
            jump: false,
            attack: false,
            sprint: false,
        }
    }
}

/// Static override for dynamic objects
#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct StaticOverride {
    pub fall_time: f32,
    pub fall_enabled: bool,
}

impl Default for StaticOverride {
    fn default() -> Self {
        Self {
            fall_time: 0.8,
            fall_enabled: false,
        }
    }
}

/// Collision brush painter
pub struct BrushPainter {
    pub brush: CollisionBrush,
    pub brush_size: f32,
}

impl BrushPainter {
    pub fn new(brush: CollisionBrush, size: f32) -> Self {
        Self { brush, brush_size: size }
    }
    
    pub fn paint(&self, _x: f32, _y: f32) -> CollisionBrush {
        self.brush
    }
}
