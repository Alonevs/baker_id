use eframe::egui;
use crate::bitacora_singleton::get_validator_singleton;

/// Panel de visualización de mensajes del linter
pub fn show_linter_panel(ctx: &egui::Context, ui: &mut egui::Ui) {
    let validator = get_validator_singleton();
    let has_errors = validator.has_errors();

    let panel = egui::Panel::new("Bitácora Linter")
        .frame(egui::Frame::none()
            .fill(if has_errors { egui::Color32::RED } else { egui::Color32::DARK_GRAY }))
        .show(ctx, ui);

    ui.horizontal(|ui| {
        ui.label("📋 Validación de Bitácora");

        ui.separator();

        let error_count = validator.get_errors().len();
        let warning_count = validator.get_warnings().len();
        let info_count = validator.get_infos().len();

        ui.label(format!("🔴 Errores: {}", error_count));
        ui.label(format!("🟡 Warnings: {}", warning_count));
        ui.label(format!("🔵 Infos: {}", info_count));
    });

    // Mostrar errores
    if !validator.get_errors().is_empty() {
        ui.separator();
        ui.label("❌ Errores:");
        for (i, error) in validator.get_errors().iter().enumerate() {
            ui.add(egui::Label::new(egui::RichText::new(format!("{}. {}", i + 1, error)).color(egui::Color32::RED)));
        }
    }

    // Mostrar warnings
    if !validator.get_warnings().is_empty() {
        ui.separator();
        ui.label("⚠️ Warnings:");
        for (i, warning) in validator.get_warnings().iter().enumerate() {
            ui.add(egui::Label::new(egui::RichText::new(format!("{}. {}", i + 1, warning)).color(egui::Color32::YELLOW)));
        }
    }

    // Mostrar infos
    if !validator.get_infos().is_empty() {
        ui.separator();
        ui.label("ℹ️ Notas:");
        for (i, info) in validator.get_infos().iter().enumerate() {
            ui.add(egui::Label::new(egui::RichText::new(format!("{}. {}", i + 1, info)).color(egui::Color32::BLUE)));
        }
    }

    ui.add_space(10.0);
}

