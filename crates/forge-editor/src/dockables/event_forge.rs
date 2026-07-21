use eframe::egui;

pub struct EventForgeDockable {
    pub events_visible: bool,
    pub selected_event: Option<u64>,
}

impl Default for EventForgeDockable {
    fn default() -> Self {
        Self {
            events_visible: false,
            selected_event: None,
        }
    }
}

impl EventForgeDockable {
    pub fn select_event(&mut self, id: u64) {
        self.selected_event = Some(id);
    }
}

