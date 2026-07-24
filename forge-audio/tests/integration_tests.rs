use forge_audio::{
    AudioManager,
    AudioBus,
    AudioBusSystem,
    AudioEffects,
    AudioEffect,
    AudioFormat,
    AudioMixer,
    AudioSample,
    AudioSource,
    SpatialAudio,
    SpatialAudioSource,
    AudioPosition,
    MixerChannel,
    BusType,
    EffectType,
    SourceState,
};
use std::path::PathBuf;

#[test]
fn test_audio_sample_creation() {
    let sample = AudioSample::new(
        "test_id".to_string(),
        "Test Sample".to_string(),
        PathBuf::from("test.wav"),
    );

    assert_eq!(sample.id, "test_id");
    assert_eq!(sample.name, "Test Sample");
    assert_eq!(sample.sample_rate, 44100);
}

#[test]
fn test_audio_format_display() {
    assert_eq!(format!("{}", AudioFormat::Wav), "WAV");
    assert_eq!(format!("{}", AudioFormat::Mp3), "MP3");
    assert_eq!(format!("{}", AudioFormat::Ogg), "OGG");
}

#[test]
fn test_audio_source_state() {
    let mut sample = AudioSample::new(
        "test".to_string(),
        "Test".to_string(),
        PathBuf::new(),
    );
    sample.set_duration(1.0);
    let mut source = AudioSource::new(sample);
    source.total_duration = 1.0;

    assert_eq!(source.state, SourceState::Stopped);
    assert!(!source.is_playing());

    source.play(None);
    assert_eq!(source.state, SourceState::Playing);
    assert!(source.is_playing());

    source.pause();
    assert_eq!(source.state, SourceState::Paused);

    source.stop();
    assert_eq!(source.state, SourceState::Stopped);
}

#[test]
fn test_audio_source_volume() {
    let sample = AudioSample::new(
        "test".to_string(),
        "Test".to_string(),
        PathBuf::new(),
    );
    let mut source = AudioSource::new(sample);

    source.set_volume(0.5);
    assert_eq!(source.get_volume(), 0.5);

    source.set_volume(1.5);
    assert_eq!(source.get_volume(), 1.0);

    source.set_volume(-0.5);
    assert_eq!(source.get_volume(), 0.0);
}

#[test]
fn test_audio_mixer_channels() {
    let mut mixer = AudioMixer::new(None);

    let channel = MixerChannel::new(
        "channel1".to_string(),
        "Test Channel".to_string(),
    );
    mixer.add_channel(channel);

    assert_eq!(mixer.get_channel_count(), 1);
    assert!(mixer.get_channel("channel1").is_some());
}

#[test]
fn test_audio_mixer_master_volume() {
    let mut mixer = AudioMixer::new(None);

    mixer.set_master_volume(0.5);
    assert_eq!(mixer.get_master_volume(), 0.5);
}

#[test]
fn test_audio_bus_creation() {
    let bus = AudioBus::new(
        "bus1".to_string(),
        "Test Bus".to_string(),
        BusType::Music,
    );

    assert_eq!(bus.id, "bus1");
    assert_eq!(bus.name, "Test Bus");
    assert_eq!(bus.bus_type, BusType::Music);
}

#[test]
fn test_audio_bus_system() {
    let bus_system = AudioBusSystem::new();
    assert_eq!(bus_system.get_bus_count(), 1);
    assert_eq!(bus_system.get_master_bus_id(), "master_bus");
}

#[test]
fn test_audio_effects_processing() {
    let mut effects = AudioEffects::new();

    let effect = AudioEffect::new(
        "effect1".to_string(),
        "Test Effect".to_string(),
        EffectType::Reverb,
    );
    effects.add_effect(effect);

    assert_eq!(effects.get_effect_count(), 1);

    let output = effects.process_sample(0.5);
    assert!(output.is_finite());
}

#[test]
fn test_spatial_audio_source() {
    let source = SpatialAudioSource::new(
        "source1".to_string(),
        "Test Source".to_string(),
    );
    assert_eq!(source.position.x, 0.0);
    assert_eq!(source.position.y, 0.0);
    assert_eq!(source.position.z, 0.0);
}

#[test]
fn test_spatial_audio_distance() {
    let mut source = SpatialAudioSource::new(
        "source1".to_string(),
        "Test".to_string(),
    );
    source.position = AudioPosition::new(10.0, 0.0, 0.0);
    source.min_distance = 1.0;
    source.max_distance = 100.0;

    let distance = source.get_distance();
    assert!(distance > 0.0);
    assert!(distance <= source.max_distance);
}

#[test]
fn test_spatial_audio_update() {
    let mut source = SpatialAudioSource::new(
        "source1".to_string(),
        "Test".to_string(),
    );
    source.velocity = AudioPosition::new(1.0, 0.0, 0.0);
    source.position = AudioPosition::new(0.0, 0.0, 0.0);

    source.update(1.0);
    assert_eq!(source.position.x, 1.0);

    source.update(1.0);
    assert_eq!(source.position.x, 2.0);
}

#[test]
fn test_audio_manager_creation() {
    let manager = AudioManager::new();
    assert_eq!(manager.get_source_count(), 0);
    assert_eq!(manager.get_bus_count(), 1);
}

#[test]
fn test_audio_manager_load_sample() {
    let mut manager = AudioManager::new();

    let sample = AudioSample::new(
        "test_sample".to_string(),
        "Test".to_string(),
        PathBuf::from("test.wav"),
    );
    let id = manager.load_sample(sample);

    assert_eq!(manager.get_source_count(), 1);
    assert!(manager.get_source(&id).is_some());
}

#[test]
fn test_audio_manager_play() {
    let mut manager = AudioManager::new();

    let sample = AudioSample::new(
        "test_sample".to_string(),
        "Test".to_string(),
        PathBuf::from("test.wav"),
    );
    manager.load_sample(sample);

    manager.play("test_sample", None);
    assert!(manager.get_source("test_sample").unwrap().is_playing());
}

#[test]
fn test_audio_manager_update() {
    let mut manager = AudioManager::new();

    let mut sample = AudioSample::new(
        "test_sample".to_string(),
        "Test".to_string(),
        PathBuf::from("test.wav"),
    );
    sample.set_duration(1.0);
    manager.load_sample(sample);

    manager.play("test_sample", None);
    manager.update(0.016);
    assert!(manager.get_source("test_sample").unwrap().get_position() > 0.0);
}

#[test]
fn test_audio_manager_master_volume() {
    let mut manager = AudioManager::new();

    manager.set_master_volume(0.5);
    assert_eq!(manager.mixer.get_master_volume(), 0.5);
}

#[test]
fn test_audio_manager_render() {
    let mut manager = AudioManager::new();

    let mut sample = AudioSample::new(
        "test_sample".to_string(),
        "Test".to_string(),
        PathBuf::from("test.wav"),
    );
    sample.set_duration(1.0);
    manager.load_sample(sample);

    manager.play("test_sample", None);
    manager.render();
    assert!(manager.get_source("test_sample").unwrap().get_position() > 0.0);
}
