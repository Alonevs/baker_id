//! # Scene System
//! 
//! Sistema completo de gestión de escenas.

pub mod scene_manager;
pub mod scene_graph;

// Re-exportar para uso externo
pub use scene_manager::{
    SceneManager, Scene, SceneData, SceneSettings, TransitionType, TransitionEffect,
    TransitionDuration, EaseFunction,
};
pub use scene_graph::{SceneGraph, SceneGraphNode, NodeType};
