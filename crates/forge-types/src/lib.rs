//! forge-types - Datos compartidos entre todos los crates

pub mod project;
pub use project::{Project, GameType};
pub mod scene;
pub mod events;
pub mod dialogue;
pub mod event_system;
pub mod timeline;
pub mod gdd;
pub mod sequencer;
pub mod audio;
pub use sequencer::{
    Timeline, CommandSignal, CommandType,
    CameraEffect, CameraEffectType, ScreenVFX, VFXType, TextOverlay, TextEffect,
};
pub use event_system::{EventNode, NodePort, Connection, NodeMetadata, DecisionOption, Condition, NodeAction, DecisionNode, LogicTab, ExecutionState, ActionType, ConditionOperator};
pub use event_system::nodes::{ConditionExpression, EndType};
pub use event_system::nodes::{ActionNode, ConditionNode, CinematicNode, EndNode, DialogueStyle};
pub use dialogue::{*, NodeType};
pub use timeline::{TimelineTrack, TrackType, TimelineClip, ClipData, CommonClipData, ClipMetadata, TrackMetadata, TimeScale, PlayheadHotkey};
pub use gdd::{GDDNote, RoadmapTask, Roadmap, Milestone, CharacterSceneLog, CharacterSceneEntry, RevisionNote, GlobalScriptLog, GlobalScriptEntry, ScriptEntryMetadata, IndexerState, ChangePending, ChangeType, TaskStatus, TaskPriority, MapCoordinates, CharacterNote};
pub use audio::{
    AudioStream, AudioChannel, AudioChannelType, AudioSocket, AudioSocketType,
    AudioBehavior, AudioBehaviorType, AudioBehaviorState,
    PositionalAudio,
};
pub use audio::audio_socket::AudioSocketState;
pub use audio::audio_behavior::AudioBehaviorTrigger;
