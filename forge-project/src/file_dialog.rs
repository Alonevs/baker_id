use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct FileDialog {
    pub current_dir: PathBuf,
    pub filters: Vec<String>,
}

impl FileDialog {
    pub fn new() -> Self {
        Self { current_dir: PathBuf::from("."), filters: Vec::new() }
    }

    pub fn set_directory(&mut self, dir: PathBuf) {
        self.current_dir = dir;
    }

    pub fn show_save(&self, default_name: &str) -> Option<PathBuf> {
        Some(self.current_dir.join(default_name))
    }

    pub fn show_open(&self) -> Option<PathBuf> {
        Some(self.current_dir.clone())
    }
}

impl Default for FileDialog {
    fn default() -> Self {
        Self::new()
    }
}