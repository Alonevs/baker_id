use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub enum GameType {
    #[default]
    Platformer,
    TopDown,
    Puzzle,
    Adventure,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct AudioStream {
    pub id: String,
    pub source: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct AudioChannel {
    pub id: String,
    pub channel_type: AudioChannelType,
    pub volume: f32,
    pub state: AudioState,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct AudioSocket {
    pub id: String,
    pub channel: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct AudioBehavior {
    pub id: String,
    pub audio_socket: String,
    pub behavior_type: AudioBehaviorType,
    pub behavior_state: AudioBehaviorState,
    pub trigger: AudioBehaviorTrigger,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub enum AudioChannelType {
    #[default]
    Mono,
    Stereo,
    Surround,
    Bgm,
    Sfx,
    Voice,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub enum AudioSocketType {
    #[default]
    Input,
    Output,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub enum AudioBehaviorType {
    #[default]
    Loop,
    OneShot,
    FadeIn,
    FadeOut,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub enum AudioSocketState {
    #[default]
    Connected,
    Disconnected,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub enum AudioBehaviorState {
    #[default]
    Playing,
    Paused,
    Stopped,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub enum AudioBehaviorTrigger {
    #[default]
    Start,
    Stop,
    Pause,
    Resume,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct PositionalAudio {
    pub position: (f32, f32, f32),
    pub rotation: f32,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub enum AudioState {
    #[default]
    Playing,
    Paused,
    Stopped,
}
