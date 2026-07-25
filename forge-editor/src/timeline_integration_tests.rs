//! Timeline Integration Tests
//! Tests que verifican la conexión entre TimelineEditor y TimelineSystem

#[test]
fn test_timeline_editor_new() {
    use crate::timeline::timeline_editor::TimelineEditor;
    use crate::property_panel::PropertyPanel;
    
    let editor = TimelineEditor::new(PropertyPanel::new());
    assert_eq!(editor.current_frame, 0);
    assert_eq!(editor.frame_duration, 16.67);
    assert!(!editor.is_playing);
    assert_eq!(editor.playback_speed, 60.0);
}

#[test]
fn test_timeline_serialization() {
    use crate::timeline::timeline_editor::TimelineEditor;
    use crate::property_panel::PropertyPanel;
    
    let mut editor = TimelineEditor::new(PropertyPanel::new());
    editor.add_track(1, "test_clip".to_string());
    editor.add_keyframe(0, 10, 50.0);
    editor.current_frame = 50;
    editor.is_playing = true;
    editor.playback_speed = 30.0;
    editor.total_frames = 100;
    
    let serialized = editor.serialize();
    let deserialized = TimelineEditor::deserialize(&serialized);
    
    assert_eq!(deserialized.current_frame, editor.current_frame);
    assert_eq!(deserialized.is_playing, editor.is_playing);
    assert_eq!(deserialized.playback_speed, editor.playback_speed);
    assert_eq!(deserialized.total_frames, editor.total_frames);
}

#[test]
fn test_timeline_playback() {
    use crate::timeline::timeline_editor::TimelineEditor;
    use crate::property_panel::PropertyPanel;
    
    let mut editor = TimelineEditor::new(PropertyPanel::new());
    editor.set_total_frames(100);
    
    editor.start_playback();
    assert!(editor.is_playing());
    
    editor.update(1.0 / 60.0);
    assert_eq!(editor.current_frame, 1);
    
    editor.pause_playback();
    assert!(!editor.is_playing());
    
    editor.stop_playback();
    assert_eq!(editor.current_frame, 0);
}
