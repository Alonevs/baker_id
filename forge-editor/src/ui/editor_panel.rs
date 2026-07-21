//! # Editor Panel Stub
//! 
//! Stub para EditorPanel.

/// Editor Panel
#[derive(Debug, Clone)]
pub struct EditorPanel {
    pub title: String,
    pub content: String,
}

impl Default for EditorPanel {
    fn default() -> Self {
        Self {
            title: String::new(),
            content: String::new(),
        }
    }
}

