//! TimelineSystem para sincronización editor-runtime

#[derive(Debug, Clone)]
pub struct TimelineSyncData {
    pub frame: u32,
    pub events: Vec<String>,
    pub is_playing: bool,
    pub playback_speed: f32,
}

#[derive(Debug, Clone)]
pub struct TimelineSystem {
    pub frame: u32,
    pub is_playing: bool,
    pub playback_speed: f32,
    pub sync_data: TimelineSyncData,
    pub registered_entities: Vec<u64>,
}

impl TimelineSystem {
    pub fn new() -> Self {
        Self {
            frame: 0,
            is_playing: false,
            playback_speed: 1.0,
            sync_data: TimelineSyncData {
                frame: 0,
                events: Vec::new(),
                is_playing: false,
                playback_speed: 1.0,
            },
            registered_entities: Vec::new(),
        }
    }

    pub fn start(&mut self) {
        self.is_playing = true;
    }

    pub fn stop(&mut self) {
        self.is_playing = false;
    }

    pub fn set_frame(&mut self, frame: u32) {
        self.frame = frame;
        self.sync_data.frame = frame;
    }

    pub fn next_frame(&mut self) {
        self.frame += 1;
        self.sync_data.frame = self.frame;
    }

    pub fn prev_frame(&mut self) {
        if self.frame > 0 {
            self.frame -= 1;
            self.sync_data.frame = self.frame;
        }
    }

    pub fn update(&mut self, delta_time: f32) {
        if self.is_playing {
            self.frame += (delta_time * self.playback_speed) as u32;
            self.sync_data.frame = self.frame;
        }
    }

    pub fn register_entity(&mut self, entity_id: u64) {
        if !self.registered_entities.contains(&entity_id) {
            self.registered_entities.push(entity_id);
        }
    }

    pub fn get_sync_data(&self) -> &TimelineSyncData {
        &self.sync_data
    }

    pub fn get_frame(&self) -> u32 {
        self.frame
    }

    pub fn get_events(&self) -> &Vec<String> {
        &self.sync_data.events
    }

    pub fn set_events(&mut self, events: Vec<String>) {
        self.sync_data.events = events;
    }

    pub fn set_playback_speed(&mut self, speed: f32) {
        self.playback_speed = speed;
        self.sync_data.playback_speed = speed;
    }

    pub fn get_playback_speed(&self) -> f32 {
        self.playback_speed
    }
}

impl Default for TimelineSystem {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_timeline_system_new() {
        let system = TimelineSystem::new();
        assert_eq!(system.get_frame(), 0);
        assert!(!system.is_playing);
    }

    #[test]
    fn test_timeline_system_start_stop() {
        let mut system = TimelineSystem::new();
        system.start();
        assert!(system.is_playing);
        system.stop();
        assert!(!system.is_playing);
    }

    #[test]
    fn test_timeline_system_set_frame() {
        let mut system = TimelineSystem::new();
        system.set_frame(50);
        assert_eq!(system.get_frame(), 50);
        assert_eq!(system.get_sync_data().frame, 50);
    }

    #[test]
    fn test_timeline_system_next_prev_frame() {
        let mut system = TimelineSystem::new();
        system.set_frame(10);
        system.next_frame();
        assert_eq!(system.get_frame(), 11);
        system.prev_frame();
        assert_eq!(system.get_frame(), 10);
    }

    #[test]
    fn test_timeline_system_update() {
        let mut system = TimelineSystem::new();
        system.set_frame(0);
        system.start();
        system.set_playback_speed(1.0);
        system.update(1.0);
        assert_eq!(system.get_frame(), 1);
    }

    #[test]
    fn test_timeline_system_register_entity() {
        let mut system = TimelineSystem::new();
        system.register_entity(1);
        system.register_entity(2);
        assert!(system.registered_entities.contains(&1));
        assert!(system.registered_entities.contains(&2));
    }

    #[test]
    fn test_timeline_system_events() {
        let mut system = TimelineSystem::new();
        let events = vec![];
        system.set_events(events);
        assert!(system.get_events().is_empty());
    }

    #[test]
    fn test_timeline_system_playback_speed() {
        let mut system = TimelineSystem::new();
        system.set_playback_speed(2.0);
        assert_eq!(system.get_playback_speed(), 2.0);
    }
}
