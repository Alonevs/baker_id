//! # Timeline Editor UI

use crate::property_panel::PropertyPanel;

#[derive(Debug, Clone)]
pub struct AnimationTrack {
    pub name: String,
    pub entity_id: u64,
    pub clip_name: String,
    pub is_visible: bool,
    pub is_locked: bool,
    pub keyframes: Vec<Keyframe>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Keyframe {
    pub frame: u32,
    pub value: f32,
    pub interpolation: String,
}

#[derive(Debug, Clone)]
pub struct TimelineEditor {
    pub property_panel: PropertyPanel,
    pub current_frame: u32,
    pub frame_duration: f32,
    pub is_playing: bool,
    pub playback_speed: f32,
    pub tracks: Vec<AnimationTrack>,
    pub widgets: Vec<Widget>,
    pub total_frames: u32,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Widget {
    PlayPauseButton,
    StopButton,
    FrameCounter,
    TimelineTrack { track_index: usize },
    KeyframeEditor { track_index: usize, frame: u32 },
    TimelineScrollbar,
    TimelineCanvas,
}

impl Default for TimelineEditor {
    fn default() -> Self {
        Self::new(PropertyPanel::new())
    }
}

impl TimelineEditor {
    pub fn new(property_panel: PropertyPanel) -> Self {
        Self {
            property_panel,
            current_frame: 0,
            frame_duration: 16.67,
            is_playing: false,
            playback_speed: 60.0,
            tracks: Vec::new(),
            widgets: Vec::new(),
            total_frames: 0,
        }
    }

    pub fn create_widgets(&mut self) {
        self.widgets.clear();
        self.widgets.push(Widget::PlayPauseButton);
        self.widgets.push(Widget::StopButton);
        self.widgets.push(Widget::FrameCounter);
        for (i, _track) in self.tracks.iter().enumerate() {
            self.widgets.push(Widget::TimelineTrack { track_index: i });
        }
    }

    pub fn get_widgets(&self) -> &[Widget] {
        &self.widgets
    }

    pub fn add_track(&mut self, entity_id: u64, clip_name: String) {
        self.tracks.push(AnimationTrack {
            name: format!("Track_{}", self.tracks.len() + 1),
            entity_id,
            clip_name,
            is_visible: true,
            is_locked: false,
            keyframes: Vec::new(),
        });
        self.widgets.push(Widget::TimelineTrack {
            track_index: self.tracks.len() - 1,
        });
    }

    pub fn is_playing(&self) -> bool {
        self.is_playing
    }

    pub fn add_keyframe(&mut self, track_index: usize, frame: u32, value: f32) {
        if let Some(track) = self.tracks.get_mut(track_index) {
            track.keyframes.push(Keyframe {
                frame,
                value,
                interpolation: "linear".to_string(),
            });
        }
    }

    pub fn get_track_keyframes(&self, track_index: usize) -> Option<&Vec<Keyframe>> {
        self.tracks.get(track_index).map(|t| &t.keyframes)
    }

    pub fn set_total_frames(&mut self, total_frames: u32) {
        self.total_frames = total_frames;
    }

    pub fn start_playback(&mut self) {
        self.is_playing = true;
    }

    pub fn pause_playback(&mut self) {
        self.is_playing = false;
    }

    pub fn stop_playback(&mut self) {
        self.is_playing = false;
        self.current_frame = 0;
    }

    pub fn update(&mut self, delta_time: f32) {
        if self.is_playing {
            let frames_per_second = self.playback_speed;
            self.current_frame = self.current_frame.saturating_add((delta_time * frames_per_second) as u32);
            self.current_frame = self.current_frame.min(self.total_frames);
        }
    }

    pub fn set_frame(&mut self, frame: u32) {
        self.current_frame = frame.min(self.total_frames);
    }

    pub fn next_frame(&mut self) {
        self.current_frame = self.current_frame.saturating_add(1).min(self.total_frames);
    }

    pub fn prev_frame(&mut self) {
        self.current_frame = self.current_frame.saturating_sub(1).max(0);
    }

    pub fn get_current_time(&self) -> f32 {
        self.current_frame as f32 / self.playback_speed
    }

    pub fn get_remaining_time(&self) -> f32 {
        (self.total_frames - self.current_frame) as f32 / self.playback_speed
    }

    pub fn get_total_duration(&self) -> f32 {
        self.total_frames as f32 / self.playback_speed
    }

    pub fn serialize(&self) -> String {
        let mut map = serde_json::Map::new();
        map.insert("current_frame".to_string(), serde_json::json!(self.current_frame));
        map.insert("is_playing".to_string(), serde_json::json!(self.is_playing));
        map.insert("playback_speed".to_string(), serde_json::json!(self.playback_speed));
        map.insert("total_frames".to_string(), serde_json::json!(self.total_frames));
        let tracks_json: Vec<String> = self.tracks.iter().map(|t| {
            let mut track_map = serde_json::Map::new();
            track_map.insert("entity_id".to_string(), serde_json::json!(t.entity_id));
            track_map.insert("clip_name".to_string(), serde_json::json!(&t.clip_name));
            track_map.insert("keyframes".to_string(), serde_json::json!(&t.keyframes));
            serde_json::to_string(&track_map).unwrap()
        }).collect();
        map.insert("tracks".to_string(), serde_json::json!(tracks_json));
        serde_json::to_string(&map).unwrap()
    }

    pub fn deserialize(json: &str) -> Self {
        let map: serde_json::Map<String, serde_json::Value> = serde_json::from_str(json).unwrap();
        let current_frame = map.get("current_frame").and_then(|v| v.as_u64()).unwrap_or(0) as u32;
        let is_playing = map.get("is_playing").and_then(|v| v.as_bool()).unwrap_or(false);
        let playback_speed = map.get("playback_speed").and_then(|v| v.as_f64()).unwrap_or(60.0) as f32;
        let total_frames = map.get("total_frames").and_then(|v| v.as_u64()).unwrap_or(0) as u32;
        let tracks_vec: Vec<serde_json::Value> = map.get("tracks").and_then(|v| v.as_array()).unwrap_or(&Vec::<serde_json::Value>::new()).to_vec();
        let tracks: Vec<AnimationTrack> = tracks_vec.iter().map(|track_json| {
            let entity_id = track_json.get("entity_id").and_then(|v| v.as_u64()).unwrap_or(0);
            let clip_name = track_json.get("clip_name").and_then(|v| v.as_str()).unwrap_or("").to_string();
            let keyframes_vec: Vec<serde_json::Value> = track_json.get("keyframes").and_then(|v| v.as_array()).unwrap_or(&Vec::<serde_json::Value>::new()).to_vec();
            let keyframes: Vec<Keyframe> = keyframes_vec.iter().map(|kf_json| {
                let frame = kf_json.get("frame").and_then(|v| v.as_u64()).unwrap_or(0) as u32;
                let value = kf_json.get("value").and_then(|v| v.as_f64()).unwrap_or(0.0) as f32;
                let interpolation = kf_json.get("interpolation").and_then(|v| v.as_str()).unwrap_or("linear").to_string();
                Keyframe { frame, value, interpolation }
            }).collect();
            AnimationTrack {
                name: format!("Track_{}", keyframes_vec.len() + 1),
                entity_id,
                clip_name,
                is_visible: true,
                is_locked: false,
                keyframes,
            }
        }).collect();
        Self {
            property_panel: PropertyPanel::new(),
            current_frame,
            frame_duration: 16.67,
            is_playing,
            playback_speed,
            tracks,
            widgets: Vec::new(),
            total_frames,
        }
    }

    pub fn remove_track(&mut self, track_index: usize) {
        self.tracks.remove(track_index);
        self.widgets.retain(|w| !matches!(w, Widget::TimelineTrack { track_index: track_index }));
    }

    pub fn remove_keyframe(&mut self, track_index: usize, frame: u32) {
        if let Some(track) = self.tracks.get_mut(track_index) {
            track.keyframes.retain(|kf| kf.frame != frame);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_timeline_editor_new() {
        let editor = TimelineEditor::new(PropertyPanel::new());
        assert_eq!(editor.current_frame, 0);
        assert_eq!(editor.frame_duration, 16.67);
        assert!(!editor.is_playing);
        assert_eq!(editor.playback_speed, 60.0);
    }

    #[test]
    fn test_timeline_create_widgets() {
        let mut editor = TimelineEditor::new(PropertyPanel::new());
        editor.create_widgets();
        let widgets = editor.get_widgets();
        assert!(widgets.len() > 0);
        assert!(widgets.contains(&Widget::PlayPauseButton));
        assert!(widgets.contains(&Widget::StopButton));
        assert!(widgets.contains(&Widget::FrameCounter));
    }

    #[test]
    fn test_timeline_add_track() {
        let mut editor = TimelineEditor::new(PropertyPanel::new());
        editor.add_track(1, "test_clip".to_string());
        assert_eq!(editor.tracks.len(), 1);
        assert_eq!(editor.tracks[0].entity_id, 1);
        assert_eq!(editor.tracks[0].clip_name, "test_clip");
    }

    #[test]
    fn test_timeline_add_keyframe() {
        let mut editor = TimelineEditor::new(PropertyPanel::new());
        editor.add_track(1, "test".to_string());
        editor.add_keyframe(0, 10, 50.0);
        editor.add_keyframe(0, 20, 75.0);

        let keyframes = editor.get_track_keyframes(0).unwrap();
        assert_eq!(keyframes.len(), 2);
        assert_eq!(keyframes[0].frame, 10);
        assert_eq!(keyframes[0].value, 50.0);
    }

    #[test]
    fn test_timeline_playback() {
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

    #[test]
    fn test_timeline_frame_navigation() {
        let mut editor = TimelineEditor::new(PropertyPanel::new());
        editor.set_total_frames(100);

        editor.set_frame(50);
        assert_eq!(editor.current_frame, 50);

        editor.next_frame();
        assert_eq!(editor.current_frame, 51);

        editor.prev_frame();
        assert_eq!(editor.current_frame, 50);
    }

    #[test]
    fn test_timeline_time_calculation() {
        let mut editor = TimelineEditor::new(PropertyPanel::new());
        editor.set_total_frames(100);
        editor.playback_speed = 30.0;

        editor.set_frame(50);
        assert_eq!(editor.get_current_time(), 50.0 / 30.0);
        assert_eq!(editor.get_remaining_time(), 50.0 / 30.0);
        assert_eq!(editor.get_total_duration(), 100.0 / 30.0);
    }

    #[test]
    fn test_timeline_serialization() {
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
        assert_eq!(deserialized.tracks.len(), editor.tracks.len());
    }

    #[test]
    fn test_timeline_remove_track() {
        let mut editor = TimelineEditor::new(PropertyPanel::new());
        editor.add_track(1, "test1".to_string());
        editor.add_track(2, "test2".to_string());

        editor.remove_track(0);
        assert_eq!(editor.tracks.len(), 1);
        assert_eq!(editor.tracks[0].entity_id, 2);
    }

    #[test]
    fn test_timeline_remove_keyframe() {
        let mut editor = TimelineEditor::new(PropertyPanel::new());
        editor.add_track(1, "test".to_string());
        editor.add_keyframe(0, 10, 50.0);
        editor.add_keyframe(0, 20, 75.0);

        editor.remove_keyframe(0, 10);

        let keyframes = editor.get_track_keyframes(0).unwrap();
        assert_eq!(keyframes.len(), 1);
        assert_eq!(keyframes[0].frame, 20);
    }
}
