//! Sistema de audio posicional y sockets de comportamiento

pub mod audio_stream;
pub mod audio_channel;
pub mod audio_socket;
pub mod audio_behavior;

pub use audio_stream::{AudioStream, AudioChannelType, AudioState};
pub use audio_socket::{AudioSocket, AudioSocketType, AudioSocketState};
pub use audio_behavior::{AudioBehavior, AudioBehaviorType, AudioBehaviorState, AudioBehaviorTrigger};
pub use audio_channel::{AudioChannel, PositionalAudio};
