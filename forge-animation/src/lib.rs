mod animation;
mod keyframe;
mod animation_clip;
mod animation_player;
mod blend_tree;

pub use animation::{Animation, AnimationState, LoopMode};
pub use keyframe::{Keyframe, Transform};
pub use animation_clip::{AnimationClip, AnimationLayer, AnimationEvent, BlendTree, BlendTreeType, Transition};
pub use animation_player::{AnimationPlayer, AnimationState as PlayerState, BlendWeight};

// Serialize/Deserialize
mod animation_save;
mod animation_load;
pub use animation_save::*;
pub use animation_load::*;
