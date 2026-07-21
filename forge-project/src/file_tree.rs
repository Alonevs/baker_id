use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct FileTree {
    pub root: PathBuf,
    pub directories: HashMap<PathBuf, Vec<PathBuf>>,
}

impl FileTree {
    pub fn new(root: PathBuf) -> Self {
        Self { root, directories: HashMap::new() }
    }

    pub fn get_children(&self, path: &PathBuf) -> Vec<PathBuf> {
        self.directories.get(path).cloned().unwrap_or_default()
    }
}

impl Default for FileTree {
    fn default() -> Self {
        Self::new(PathBuf::from("."))
    }
}