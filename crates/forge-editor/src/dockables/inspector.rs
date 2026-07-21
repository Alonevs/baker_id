use eframe::egui;

pub struct InspectorDockable {
    pub selected_entity: Option<u64>,
}

impl Default for InspectorDockable {
    fn default() -> Self {
        Self { selected_entity: None }
    }
}

impl InspectorDockable {
    pub fn select_entity(&mut self, id: u64) {
        self.selected_entity = Some(id);
    }
}

