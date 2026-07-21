use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::animation_player::BlendWeight;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct BlendTree {
    pub name: String,
    pub type_: BlendTreeType,
    pub root: Uuid,
    pub transitions: Vec<Transition>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[allow(dead_code)]
pub enum BlendTreeType {
    Linear,
    Radial,
    Tree,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct Transition {
    pub from: Uuid,
    pub to: Uuid,
    pub curve: Vec<(f32, f32)>,
}

#[allow(dead_code)]
impl BlendTree {
    pub fn new(name: String, type_: BlendTreeType, root: Uuid) -> Self {
        Self {
            name,
            type_,
            root,
            transitions: Vec::new(),
        }
    }

    pub fn add_transition(&mut self, transition: Transition) {
        self.transitions.push(transition);
    }

    pub fn get_transition(&self, from: &Uuid) -> Option<&Transition> {
        self.transitions.iter().find(|t| t.from == *from)
    }

    pub fn interpolate(&self, time: f32) -> BlendWeight {
        let t = time % 1.0;
        
        // Simple linear interpolation between first transitions
        if let Some(first) = self.transitions.first() {
            if !first.curve.is_empty() {
                let start = first.curve.first().unwrap().0;
                let end = if first.curve.len() > 1 {
                    first.curve.last().unwrap().0
                } else {
                    start
                };
                
                if end > start {
                    let progress = (t - start) / (end - start);
                    return BlendWeight {
                        weight: progress.clamp(0.0, 1.0),
                        target: first.to,
                    };
                }
            }
        }
        
        BlendWeight {
            weight: 0.0,
            target: Uuid::nil(),
        }
    }
}

impl Default for BlendTree {
    fn default() -> Self {
        Self::new(String::new(), BlendTreeType::Linear, Uuid::nil())
    }
}
