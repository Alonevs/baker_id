use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::{PlayerState, BlendWeight, AnimationPlayer, Transition, BlendTree, AnimationEvent, AnimationLayer, AnimationClip, Transform, Keyframe, BlendTreeType};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnimationSaveData {
    pub animations: Vec<AnimationSaveData>,
    pub clips: Vec<AnimationClipSaveData>,
    pub players: Vec<AnimationPlayerSaveData>,
}

// Duplicate struct definition removed
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyframeSaveData {
    pub time: f32,
    pub target: String,
    pub transform: TransformSaveData,
    pub blend_weight: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransformSaveData {
    pub position: [f32; 3],
    pub rotation: [f32; 3],
    pub scale: [f32; 3],
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnimationClipSaveData {
    pub id: String,
    pub name: String,
    pub layers: Vec<AnimationLayerSaveData>,
    pub events: Vec<AnimationEventSaveData>,
    pub blend_tree: Option<BlendTreeSaveData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnimationLayerSaveData {
    pub name: String,
    pub animation: String,
    pub weight: f32,
    pub offset: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnimationEventSaveData {
    pub time: f32,
    pub event_id: String,
    pub parameters: Vec<(String, serde_json::Value)>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlendTreeSaveData {
    pub name: String,
    pub type_: BlendTreeType,
    pub root: String,
    pub transitions: Vec<TransitionSaveData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransitionSaveData {
    pub from: String,
    pub to: String,
    pub curve: Vec<(f32, f32)>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnimationPlayerSaveData {
    pub clips: Vec<String>,
    pub current_clip: Option<String>,
    pub state: PlayerStateSaveData,
    pub active: bool,
    pub auto_play: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerStateSaveData {
    pub current_time: f32,
    pub current_blend: BlendWeightSaveData,
    pub playing: bool,
    pub speed: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlendWeightSaveData {
    pub weight: f32,
    pub target: String,
}

impl KeyframeSaveData {
    pub fn to_keyframe(&self) -> Keyframe {
        Keyframe {
            time: self.time,
            target: Uuid::parse_str(&self.target).unwrap_or_else(|_| Uuid::new_v4()),
            transform: self.transform.to_transform(),
            blend_weight: self.blend_weight,
        }
    }
}

impl TransformSaveData {
    pub fn to_transform(&self) -> Transform {
        Transform {
            position: self.position,
            rotation: self.rotation,
            scale: self.scale,
        }
    }
}

impl AnimationClipSaveData {
    pub fn to_clip(&self) -> AnimationClip {
        AnimationClip {
            id: Uuid::parse_str(&self.id).unwrap_or_else(|_| Uuid::new_v4()),
            name: self.name.clone(),
            layers: self.layers.iter().map(|l| l.to_layer()).collect(),
            events: self.events.iter().map(|e| e.to_event()).collect(),
            blend_tree: self.blend_tree.as_ref().map(|b| b.to_blend_tree()),
        }
    }
}

impl AnimationLayerSaveData {
    pub fn to_layer(&self) -> AnimationLayer {
        AnimationLayer {
            name: self.name.clone(),
            animation: Uuid::parse_str(&self.animation).unwrap_or_else(|_| Uuid::new_v4()),
            weight: self.weight,
            offset: self.offset,
        }
    }
}

impl AnimationEventSaveData {
    pub fn to_event(&self) -> AnimationEvent {
        AnimationEvent {
            time: self.time,
            event_id: Uuid::parse_str(&self.event_id).unwrap_or_else(|_| Uuid::new_v4()),
            parameters: self.parameters.clone(),
        }
    }
}

impl BlendTreeSaveData {
    pub fn to_blend_tree(&self) -> BlendTree {
        BlendTree {
            name: self.name.clone(),
            type_: self.type_,
            root: Uuid::parse_str(&self.root).unwrap_or_else(|_| Uuid::new_v4()),
            transitions: self.transitions.iter().map(|t| t.to_transition()).collect(),
        }
    }
}

impl TransitionSaveData {
    pub fn to_transition(&self) -> Transition {
        Transition {
            from: Uuid::parse_str(&self.from).unwrap_or_else(|_| Uuid::new_v4()),
            to: Uuid::parse_str(&self.to).unwrap_or_else(|_| Uuid::new_v4()),
            curve: self.curve.clone(),
        }
    }
}

impl AnimationPlayerSaveData {
    pub fn to_player(&self) -> AnimationPlayer {
        let clips = self.clips.iter().map(|s| {
            let clip_id = Uuid::parse_str(s).unwrap_or_else(|_| Uuid::new_v4());
            AnimationClip {
                id: clip_id,
                name: String::new(),
                layers: Vec::new(),
                events: Vec::new(),
                blend_tree: None,
            }
        }).collect();
        
        AnimationPlayer {
            clips,
            current_clip: self.current_clip.as_ref().and_then(|s| Uuid::parse_str(s).ok()),
            state: self.state.to_state(),
            active: self.active,
            auto_play: self.auto_play,
        }
    }
}

impl PlayerStateSaveData {
    pub fn to_state(&self) -> PlayerState {
        PlayerState {
            current_time: self.current_time,
            current_blend: self.current_blend.to_blend(),
            playing: self.playing,
            speed: self.speed,
        }
    }
}

impl BlendWeightSaveData {
    pub fn to_blend(&self) -> BlendWeight {
        BlendWeight {
            weight: self.weight,
            target: Uuid::parse_str(&self.target).unwrap_or_else(|_| Uuid::new_v4()),
        }
    }
}