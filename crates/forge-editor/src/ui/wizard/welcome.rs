use eframe::egui;
use crate::WizardStep;
use crate::WizardState;

pub fn render_welcome(ui: &mut egui::Ui, state: &mut WizardState) {
    ui.heading("Welcome");
    ui.label("Create a new Forge Editor project");
    if ui.button("Next").clicked() {
        state.step = WizardStep::ProjectName;
    }
}
