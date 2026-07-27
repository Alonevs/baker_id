use forge_editor::ForgeEditorApp;
use egui;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1280.0, 720.0])
            .with_min_inner_size([800.0, 600.0])
            .with_title("Forge Editor"),
        ..Default::default()
    };
    
    eframe::run_native(
        "Forge Editor",
        options,
        Box::new(|cc| {
            // Registrar cargadores de imágenes nativos para egui 0.33.3
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Ok(Box::new(ForgeEditorApp::new(cc)))
        }),
    )
}

