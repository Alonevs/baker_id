use eframe::egui;

pub struct SequencerDockable {
    pub timeline_visible: bool,
    pub current_frame: u32,
}

impl Default for SequencerDockable {
    fn default() -> Self {
        Self {
            timeline_visible: false,
            current_frame: 0,
        }
    }
}

impl SequencerDockable {
    pub fn next_frame(&mut self) {
        self.current_frame += 1;
    }

    pub fn prev_frame(&mut self) {
        self.current_frame = self.current_frame.saturating_sub(1);
    }
}

