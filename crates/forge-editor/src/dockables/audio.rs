use eframe::egui;

pub struct AudioDockable {
    pub tracks_visible: bool,
    pub playing: bool,
}

impl Default for AudioDockable {
    fn default() -> Self {
        Self {
            tracks_visible: false,
            playing: false,
        }
    }
}

impl AudioDockable {
    pub fn toggle_play(&mut self) {
        self.playing = !self.playing;
    }
}

