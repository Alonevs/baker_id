//! Timeline del editor

/// Timeline del editor
#[derive(Clone, Debug, Default)]
pub struct Timeline {
    pub events: Vec<TimelineEvent>,
    pub current_frame: u32,
    pub is_playing: bool,
    pub playback_speed: f32,
}

/// Evento en la timeline
#[derive(Clone, Debug, Default)]
pub struct TimelineEvent {
    pub id: u64,
    pub frame: u32,
    pub entity_id: u64,
    pub action: String,
    pub value: f32,
}

impl Timeline {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_event(&mut self, entity_id: u64, action: String, value: f32) -> u64 {
        let id = self.events.len() as u64;
        let event = TimelineEvent {
            id,
            frame: self.current_frame,
            entity_id,
            action,
            value,
        };
        self.events.push(event);
        id
    }

    pub fn get_event_at_frame(&self, frame: u32) -> Vec<&TimelineEvent> {
        self.events
            .iter()
            .filter(|e| e.frame == frame)
            .collect()
    }

    pub fn next_frame(&mut self) {
        self.current_frame += 1;
    }

    pub fn prev_frame(&mut self) {
        self.current_frame = self.current_frame.saturating_sub(1);
    }

    pub fn reset(&mut self) {
        self.current_frame = 0;
        self.is_playing = false;
    }

    pub fn set_playback_speed(&mut self, speed: f32) {
        self.playback_speed = speed;
    }

    pub fn set_frame(&mut self, frame: u32) {
        self.current_frame = frame;
    }
}
