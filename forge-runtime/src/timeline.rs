//! Timeline y TimelineEvent para gestión de eventos de animación

use std::collections::HashMap;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimelineEvent {
    pub frame: u32,
    pub event_type: String,
    pub data: HashMap<String, String>,
}

impl TimelineEvent {
    pub fn new(frame: u32, event_type: &str, data: HashMap<String, String>) -> Self {
        Self { frame, event_type: event_type.to_string(), data }
    }

    pub fn apply_frame_events(&self, frame: u32) {
        if self.frame == frame {
            // Aplicar eventos según el tipo
            match self.event_type.as_str() {
                "play" => {
                    println!("TimelineEvent: play at frame {}", frame);
                }
                "stop" => {
                    println!("TimelineEvent: stop at frame {}", frame);
                }
                "pause" => {
                    println!("TimelineEvent: pause at frame {}", frame);
                }
                "set_value" => {
                    if let Some(value) = self.data.get("value") {
                        println!("TimelineEvent: set_value to {} at frame {}", value, frame);
                    }
                }
                _ => {
                    println!("TimelineEvent: {} at frame {}", self.event_type, frame);
                }
            }
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Timeline {
    pub name: String,
    pub duration: u32,
    pub frame_rate: f32,
    pub events: HashMap<u32, Vec<TimelineEvent>>,
    pub metadata: HashMap<String, String>,
}

impl Timeline {
    pub fn new(name: &str, duration: u32, frame_rate: f32) -> Self {
        Self {
            name: name.to_string(),
            duration,
            frame_rate,
            events: HashMap::new(),
            metadata: HashMap::new(),
        }
    }

    pub fn add_event(&mut self, frame: u32, event_type: &str, data: HashMap<String, String>) {
        self.events.entry(frame).or_insert_with(Vec::new).push(
            TimelineEvent::new(frame, event_type, data)
        );
    }

    pub fn get_event(&self, frame: u32) -> Option<&TimelineEvent> {
        self.events.get(&frame).and_then(|events| events.first())
    }

    pub fn get_events(&self, frame: u32) -> Option<&Vec<TimelineEvent>> {
        self.events.get(&frame)
    }

    pub fn set_event(&mut self, frame: u32, event: TimelineEvent) {
        self.events.entry(frame).or_insert_with(Vec::new).push(event);
    }

    pub fn remove_event(&mut self, frame: u32, event_type: &str) -> Option<TimelineEvent> {
        if let Some(events) = self.events.get_mut(&frame) {
            if let Some(pos) = events.iter().position(|e| e.event_type == event_type) {
                return Some(events.remove(pos));
            }
        }
        None
    }

    pub fn get_duration(&self) -> u32 {
        self.duration
    }

    pub fn set_duration(&mut self, duration: u32) {
        self.duration = duration;
    }

    pub fn get_frame_rate(&self) -> f32 {
        self.frame_rate
    }

    pub fn set_frame_rate(&mut self, frame_rate: f32) {
        self.frame_rate = frame_rate;
    }

    pub fn add_metadata(&mut self, key: &str, value: &str) {
        self.metadata.insert(key.to_string(), value.to_string());
    }

    pub fn get_metadata(&self, key: &str) -> Option<&String> {
        self.metadata.get(key)
    }

    pub fn serialize(&self) -> String {
        serde_json::to_string_pretty(self).unwrap_or_default()
    }

    pub fn deserialize(data: &str) -> Option<Self> {
        serde_json::from_str(data).ok()
    }
}

impl Default for Timeline {
    fn default() -> Self {
        Self::new("default", 100, 60.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_timeline_new() {
        let timeline = Timeline::new("test", 100, 60.0);
        assert_eq!(timeline.name, "test");
        assert_eq!(timeline.duration, 100);
        assert_eq!(timeline.frame_rate, 60.0);
    }

    #[test]
    fn test_timeline_add_event() {
        let mut timeline = Timeline::new("test", 100, 60.0);
        let data = HashMap::from([("key".to_string(), "value".to_string())]);
        timeline.add_event(50, "play", data);
        assert!(timeline.get_events(50).is_some());
    }

    #[test]
    fn test_timeline_event_apply() {
        let mut timeline = Timeline::new("test", 100, 60.0);
        let data = HashMap::from([("value".to_string(), "100".to_string())]);
        timeline.add_event(10, "set_value", data);
        
        let event = timeline.get_event(10).unwrap();
        event.apply_frame_events(10);
    }

    #[test]
    fn test_timeline_metadata() {
        let mut timeline = Timeline::new("test", 100, 60.0);
        timeline.add_metadata("author", "test_user");
        assert_eq!(timeline.get_metadata("author"), Some(&"test_user".to_string()));
    }

    #[test]
    fn test_timeline_serialize_deserialize() {
        let timeline = Timeline::new("test", 100, 60.0);
        let json = timeline.serialize();
        let loaded = Timeline::deserialize(&json);
        assert!(loaded.is_some());
    }
}
