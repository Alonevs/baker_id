pub mod animations;
pub mod dialogue_manager;
pub mod dialogue_ui;
pub mod options_system;
pub mod variables;

pub use animations::{
    DialogueAnimationSystem,
    DialogueAnimationType,
    AnimationState,
    EaseFunction,
    DialogueAnimationExt,
};

pub use dialogue_manager::{
    DialogueManager,
    DialogueId,
    DialogueLine as DialogueLineType,
    DialogueChoice,
    DialogueVariable,
    DialogueValue,
    DialogueContext,
    DialogueContextManager,
};

pub use dialogue_ui::{
    DialogueUI,
    DialogueState,
    DialogueStyle,
    PlaySessionDialogueExt,
};

pub use options_system::{
    OptionsManager,
    GraphicsOptions,
    AudioOptions,
    ControlOptions,
    AccessibilityOptions,
    Resolution,
    GraphicsQuality,
    FullscreenMode,
    AntiAliasing,
    KeyboardLayout,
    SubtitleSize,
    ColorBlindMode,
};

pub use variables::{
    DialogueVariables,
    DialogueContext as DialogueContext2,
    DialogueContextManager as DialogueContextManager2,
};
