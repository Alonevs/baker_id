use eframe::egui;
use crate::WizardStep;
use crate::WizardState;

pub fn render_project_name(ui: &mut egui::Ui, state: &mut WizardState) {
    ui.heading("Project Name");
    ui.label("Enter your project name (placeholder)");
    if ui.button("Next").clicked() {
        state.step = WizardStep::GameType;
    }
}
