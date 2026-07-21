use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct FileWatcher {
    pub paths: Vec<PathBuf>,
}

impl FileWatcher {
    pub fn new() -> Self {
        Self { paths: Vec::new() }
    }

    pub fn watch(&mut self, path: PathBuf) {
        self.paths.push(path);
    }

    pub fn get_changes(&self) -> Vec<String> {
        vec![]
    }
}

impl Default for FileWatcher {
    fn default() -> Self {
        Self::new()
    }
}