use eframe::egui;
use std::path::PathBuf;

pub struct ExplorerDockable {
    pub project_path: Option<String>,
    pub files: Vec<PathBuf>,
    pub current_dir: Option<PathBuf>,
}

impl Default for ExplorerDockable {
    fn default() -> Self {
        Self {
            project_path: None,
            files: Vec::new(),
            current_dir: None,
        }
    }
}

impl ExplorerDockable {
    pub fn set_project_path(&mut self, path: String) {
        self.project_path = Some(path.clone());
        self.current_dir = Some(PathBuf::from(&path));
        self.refresh_files();
    }

    fn refresh_files(&mut self) {
        if let Some(ref path) = self.current_dir {
            if let Ok(entries) = std::fs::read_dir(path) {
                self.files = entries
                    .filter_map(|e| e.ok())
                    .filter(|e| e.path().is_file())
                    .map(|e| e.path())
                    .collect();
            }
        }
    }

    pub fn get_files(&self) -> &[PathBuf] {
        &self.files
    }

    pub fn get_current_dir(&self) -> Option<&PathBuf> {
        self.current_dir.as_ref()
    }
}
