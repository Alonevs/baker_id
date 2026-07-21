//! Entidades del juego 2D

use serde::{Deserialize, Serialize};

/// Componente base para entidades
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Component {
    pub id: u64,
    pub entity_id: u64,
    pub name: String,
}

/// Componente de posición 2D
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Position2D {
    pub x: f32,
    pub y: f32,
}

/// Componente de tamaño 2D
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Size2D {
    pub width: f32,
    pub height: f32,
}

/// Componente de color
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Color2D {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

/// Componente de velocidad
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Velocity2D {
    pub x: f32,
    pub y: f32,
}

/// Entity structure
#[derive(Clone, Debug)]
pub struct Entity {
    pub id: u64,
    pub position: Position2D,
    pub velocity: Velocity2D,
}

impl Default for Entity {
    fn default() -> Self {
        Self {
            id: 0,
            position: Position2D { x: 0.0, y: 0.0 },
            velocity: Velocity2D { x: 0.0, y: 0.0 },
        }
    }
}
