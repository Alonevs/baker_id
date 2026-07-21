use egui_dock::{DockArea, DockState, TabViewer};

struct ExplorerDockable;

impl TabViewer for ExplorerDockable {
    type Tab = String;
    
    fn title(&mut self, _tab: &mut Self::Tab) -> egui::WidgetText {
        "Explorer".into()
    }
    
    fn ui(&mut self, _ui: &mut egui::Ui, _tab: &mut Self::Tab) {
        // implementation
    }
}
