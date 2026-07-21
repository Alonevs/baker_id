use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct FileSystem {
    pub current_dir: PathBuf,
}

impl FileSystem {
    pub fn new() -> Self {
        Self { current_dir: PathBuf::from(".") }
    }

    pub fn change_dir(&mut self, dir: &str) {
        self.current_dir.push(dir);
    }

    pub fn list_files(&self) -> Vec<String> {
        vec![]
    }
}
