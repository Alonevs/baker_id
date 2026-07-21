use eframe::egui;
use crate::WizardStep;
use crate::WizardState;

pub fn render_template(ui: &mut egui::Ui, state: &mut WizardState) {
    ui.heading("Template");
    ui.label("Select a template");
    
    ui.horizontal(|ui| {
        if ui.selectable_value(&mut state.selected_template, Some("2d-platformer".to_string()), "2D Platformer").clicked() {}
        if ui.selectable_value(&mut state.selected_template, Some("2d-rpg".to_string()), "2D RPG").clicked() {}
        if ui.selectable_value(&mut state.selected_template, Some("2d-puzzle".to_string()), "2D Puzzle").clicked() {}
    });
    
    if ui.button("Next").clicked() {
        state.step = WizardStep::Complete;
    }
}
