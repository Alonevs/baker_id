//! # Diff View UI Module
//! 
//! Module for code difference visualization.

/// Code difference view
#[derive(Debug, Clone, Default)]
pub struct DiffView {
    pub left_file: String,
    pub right_file: String,
    pub differences: Vec<String>,
    pub show_line_numbers: bool,
}

impl DiffView {
    pub fn new(left_file: String, right_file: String) -> Self {
        Self {
            left_file,
            right_file,
            differences: Vec::new(),
            show_line_numbers: true,
        }
    }
    
    pub fn ui(&self, ui: &mut egui::Ui) {
        ui.heading("Diff View");
        ui.label(format!("Left: {}", self.left_file));
        ui.label(format!("Right: {}", self.right_file));
        for diff in &self.differences {
            ui.label(diff);
        }
    }
}

