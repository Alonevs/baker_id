//! UI del editor

use crate::timeline_integration::TimelineIntegration;
use std::collections::HashMap;
use eframe::egui;
use eframe::App;

/// DragOperation para UI
#[derive(Debug, Clone, PartialEq)]
pub enum DragOperation {
    None,
    Move,
    Resize,
}

/// App principal del editor
pub struct ForgeEditorApp {
    pub logs: Vec<String>,
    pub timeline_integration: Option<TimelineIntegration>,
}

impl Default for ForgeEditorApp {
    fn default() -> Self {
        Self {
            logs: Vec::new(),
            timeline_integration: None,
        }
    }
}

impl ForgeEditorApp {
    pub fn new(cc: &eframe::CreationContext) -> Self {
        Self::default()
    }
    
    pub fn log(&mut self, message: String) {
        self.logs.push(message);
    }
}

impl App for ForgeEditorApp {
    fn update(&mut self, ctx: &egui::Context, _ui: &mut eframe::Frame) {
        // Update logic
    }
}
