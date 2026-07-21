

/// Toolbar con botones de acción
#[derive(Debug, Clone)]
pub struct Toolbar {
    pub show_toolbar: bool,
    pub buttons: Vec<String>,
    pub selected_button: usize,
    pub current_mode: ToolbarMode,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ToolbarMode {
    Select,
    Translate,
    Rotate,
    Scale,
    Pan,
    Zoom,
}

impl Default for Toolbar {
    fn default() -> Self {
        Self {
            show_toolbar: true,
            buttons: vec![
                "Select".to_string(),
                "Translate".to_string(),
                "Rotate".to_string(),
                "Scale".to_string(),
                "Pan".to_string(),
                "Zoom".to_string(),
            ],
            selected_button: 0,
            current_mode: ToolbarMode::Select,
        }
    }
}

impl Toolbar {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_mode(&mut self, mode: ToolbarMode) {
        self.current_mode = mode;
        self.selected_button = self.buttons.iter().position(|b| {
            match mode {
                ToolbarMode::Select => b == "Select",
                ToolbarMode::Translate => b == "Translate",
                ToolbarMode::Rotate => b == "Rotate",
                ToolbarMode::Scale => b == "Scale",
                ToolbarMode::Pan => b == "Pan",
                ToolbarMode::Zoom => b == "Zoom",
            }
        }).unwrap_or(0);
    }
}

