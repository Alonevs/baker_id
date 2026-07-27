//! Toolbar del editor

pub struct ToolbarWidget {
    pub toolbar: Vec<String>,
}

impl ToolbarWidget {
    pub fn new() -> Self {
        Self {
            toolbar: vec![
                "File".to_string(),
                "Edit".to_string(),
                "View".to_string(),
                "Insert".to_string(),
                "Help".to_string(),
            ],
        }
    }

    pub fn select(&mut self) {
        // Implementación vacía
    }
}
