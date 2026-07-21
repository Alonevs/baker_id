//! Wizard for creating new projects

use eframe::egui;
use crate::WizardState;
use crate::WizardStep;

mod welcome;
mod project_name;
mod game_type;
mod template;
mod complete;

pub fn show_wizard(ctx: &egui::Context, state: &mut WizardState) {
    egui::CentralPanel::default().show(ctx, |ui| {
        ui.heading("New Project Wizard");
        ui.add_space(10.0);
        match state.step {
            WizardStep::Welcome => welcome::render_welcome(ui, state),
            WizardStep::ProjectName => project_name::render_project_name(ui, state),
            WizardStep::GameType => game_type::render_game_type(ui, state),
            WizardStep::Template => template::render_template(ui, state),
            WizardStep::Complete => complete::render_complete(ui, state),
        }
    });
}
