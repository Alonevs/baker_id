use eframe::egui;
use crate::GameType;
use crate::WizardStep;
use crate::WizardState;

pub fn render_game_type(ui: &mut egui::Ui, state: &mut WizardState) {
    ui.heading("Game Type");
    ui.label("Select game type");
    
    ui.horizontal(|ui| {
        if ui.selectable_value(&mut state.game_type, GameType::Platformer, "Platformer").clicked() {}
        if ui.selectable_value(&mut state.game_type, GameType::Shooter, "Shooter").clicked() {}
        if ui.selectable_value(&mut state.game_type, GameType::Adventure, "Adventure").clicked() {}
        if ui.selectable_value(&mut state.game_type, GameType::Puzzle, "Puzzle").clicked() {}
    });
    
    if ui.button("Next").clicked() {
        state.step = WizardStep::Template;
    }
}
