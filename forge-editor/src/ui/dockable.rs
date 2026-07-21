//! # Dockable API
//! 
//! Módulo para dockable panels.

use eframe::egui;

/// Dockable - panel dockable
pub trait Dockable {
    fn title(&self) -> &str;
    fn ui(&self, _ctx: &egui::Context) {}
    fn dock(&self) -> &str {
        "bottom"
    }
}

