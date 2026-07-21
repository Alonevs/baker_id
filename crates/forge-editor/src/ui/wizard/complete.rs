use eframe::egui;
use crate::WizardStep;
use crate::WizardState;

pub fn render_complete(ui: &mut egui::Ui, state: &mut WizardState) {
    ui.heading("Project Created!");
    let template = &state.selected_template;
    ui.label(&format!("{} - {}", state.game_type, template.as_deref().unwrap_or("No template selected")));
    
    ui.add_space(20.0);
    
    ui.horizontal(|ui| {
        if ui.button("Edit Project").clicked() {}
        if ui.button("Close").clicked() {
            state.step = WizardStep::Welcome;
        }
    });
}
