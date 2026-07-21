use std::path::PathBuf;
use std::io::{self, Read, Write, Seek, SeekFrom};
use std::collections::HashMap;
use thiserror::Error;

#[derive(Debug, Clone)]
pub struct FileInfo {
    pub path: PathBuf,
    pub name: String,
    pub extension: String,
    pub size: u64,
    pub modified: u64,
    pub created: u64,
    pub is_dir: bool,
    pub permissions: u32,
}

#[derive(Debug, Error)]
pub enum FileSystemError {
    #[error("IO error: {0}")]
    IoError(#[from] io::Error),
    #[error("File not found: {0}")]
    NotFound(String),
    #[error("Permission denied: {0}")]
    PermissionDenied(String),
    #[error("Invalid path: {0}")]
    InvalidPath(String),
}

pub struct FileSystem {
    pub current_dir: PathBuf,
    pub drives: Vec<PathBuf>,
    pub files: HashMap<PathBuf, FileInfo>,
    pub directories: HashMap<PathBuf, Vec<PathBuf>>,
}

impl FileSystem {
    pub fn new() -> Self {
        let mut fs = Self {
            current_dir: PathBuf::from("."),
            drives: Vec::new(),
            files: HashMap::new(),
            directories: HashMap::new(),
        };
        fs.refresh();
        fs
    }

    pub fn refresh(&mut self) {
        // Refrescar sistema de archivos
    }

    pub fn current_dir(&self) -> &Path {
        &self.current_dir
    }

    pub fn set_current_dir(&mut self, path: &Path) -> Result<(), FileSystemError> {
        if path.is_absolute() {
            self.current_dir = path.to_path_buf();
        } else {
            self.current_dir.push(path);
        }
        Ok(())
    }

    pub fn parent(&self) -> Option<PathBuf> {
        self.current_dir.parent().map(|p| p.to_path_buf())
    }

    pub fn go_up(&mut self) -> Result<(), FileSystemError> {
        if let Some(parent) = self.current_dir.parent() {
            self.current_dir = parent.to_path_buf();
        }
        Ok(())
    }

    pub fn list_files(&self) -> Vec<FileInfo> {
        let mut files = Vec::new();
        for (path, info) in &self.files {
            if path == &self.current_dir {
                continue;
            }
            files.push(info.clone());
        }
        files.sort_by(|a, b| a.name.cmp(&b.name));
        files
    }

    pub fn list_directories(&self) -> Vec<PathBuf> {
        self.directories
            .get(&self.current_dir)
            .cloned()
            .unwrap_or_default()
    }

    pub fn exists(&self, path: &Path) -> bool {
        let full_path = self.current_dir.join(path);
        self.files.contains_key(&full_path) || self.directories.contains_key(&full_path)
    }

    pub fn is_file(&self, path: &Path) -> bool {
        let full_path = self.current_dir.join(path);
        self.files.contains_key(&full_path)
    }

    pub fn is_directory(&self, path: &Path) -> bool {
        let full_path = self.current_dir.join(path);
        self.directories.contains_key(&full_path)
    }

    pub fn get_file(&self, path: &Path) -> Option<&FileInfo> {
        let full_path = self.current_dir.join(path);
        self.files.get(&full_path)
    }

    pub fn get_directory(&self, path: &Path) -> Option<&Vec<PathBuf>> {
        let full_path = self.current_dir.join(path);
        self.directories.get(&full_path)
    }

    pub fn create_file(&mut self, path: &Path, content: &[u8]) -> Result<(), FileSystemError> {
        let full_path = self.current_dir.join(path);
        let name = path.to_string_lossy().to_string();
        let extension = path
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("")
            .to_string();

        let info = FileInfo {
            path: full_path.clone(),
            name: name.clone(),
            extension: extension.clone(),
            size: content.len() as u64,
            modified: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_millis() as u64,
            created: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_millis() as u64,
            is_dir: false,
            permissions: 0o644,
        };

        self.files.insert(full_path, info.clone());
        Ok(())
    }

    pub fn read_file(&self, path: &Path) -> Result<Vec<u8>, FileSystemError> {
        let full_path = self.current_dir.join(path);
        if let Some(info) = self.files.get(&full_path) {
            let content = std::fs::read(&info.path)?;
            Ok(content)
        } else {
            Err(FileSystemError::NotFound(path.display().to_string()))
        }
    }

    pub fn write_file(&self, path: &Path, content: &[u8]) -> Result<(), FileSystemError> {
        let full_path = self.current_dir.join(path);
        std::fs::write(&full_path, content)?;
        Ok(())
    }

    pub fn delete_file(&mut self, path: &Path) -> Result<(), FileSystemError> {
        let full_path = self.current_dir.join(path);
        if let Some(_) = self.files.remove(&full_path) {
            Ok(())
        } else {
            Err(FileSystemError::NotFound(path.display().to_string()))
        }
    }

    pub fn create_directory(&mut self, path: &Path) -> Result<(), FileSystemError> {
        let full_path = self.current_dir.join(path);
        std::fs::create_dir_all(&full_path)?;
        self.directories.insert(full_path.clone(), Vec::new());
        Ok(())
    }

    pub fn delete_directory(&mut self, path: &Path) -> Result<(), FileSystemError> {
        let full_path = self.current_dir.join(path);
        if let Some(_) = self.directories.remove(&full_path) {
            Ok(())
        } else {
            Err(FileSystemError::NotFound(path.display().to_string()))
        }
    }

    pub fn rename(&mut self, old_path: &Path, new_path: &Path) -> Result<(), FileSystemError> {
        let old_full = self.current_dir.join(old_path);
        let new_full = self.current_dir.join(new_path);

        if let Some(info) = self.files.remove(&old_full) {
            self.files.insert(new_full.clone(), info);
            Ok(())
        } else if let Some(children) = self.directories.get_mut(&old_full) {
            for child in children {
                let child_old = old_full.join(child);
                let child_new = new_full.join(child);
                if let Some(info) = self.files.remove(&child_old) {
                    self.files.insert(child_new.clone(), info);
                }
            }
            Ok(())
        } else {
            Err(FileSystemError::NotFound(old_path.display().to_string()))
        }
    }

    pub fn get_size(&self) -> u64 {
        self.files.values().map(|f| f.size).sum()
    }

    pub fn get_file_count(&self) -> usize {
        self.files.len()
    }

    pub fn get_directory_count(&self) -> usize {
        self.directories.len()
    }

    pub fn get_absolute_path(&self, path: &Path) -> PathBuf {
        self.current_dir.join(path)
    }

    pub fn get_relative_path(&self, path: &Path) -> Option<PathBuf> {
        let full_path = self.get_absolute_path(path);
        if let Ok(relative) = std::path::absolute(&full_path)
            .parent()
            .and_then(|p| p.strip_prefix(&std::env::current_dir().ok()?))
        {
            Some(relative.to_path_buf())
        } else {
            None
        }
    }

    pub fn find_file(&self, pattern: &str) -> Option<PathBuf> {
        for (path, info) in &self.files {
            if path == &self.current_dir {
                continue;
            }
            if path.to_string_lossy().contains(pattern) {
                return Some(path.clone());
            }
        }
        None
    }

    pub fn copy_file(&mut self, src: &Path, dst: &Path) -> Result<(), FileSystemError> {
        let src_full = self.current_dir.join(src);
        let dst_full = self.current_dir.join(dst);
        
        if let Some(info) = self.files.get(&src_full) {
            let content = std::fs::read(&info.path)?;
            self.create_file(dst, &content)?;
            Ok(())
        } else {
            Err(FileSystemError::NotFound(src.display().to_string()))
        }
    }

    pub fn move_file(&mut self, src: &Path, dst: &Path) -> Result<(), FileSystemError> {
        self.rename(src, dst)
    }

    pub fn list_files_with_details(&self) -> Vec<(PathBuf, FileInfo)> {
        let mut files = Vec::new();
        for (path, info) in &self.files {
            if path == &self.current_dir {
                continue;
            }
            files.push((path.clone(), info.clone()));
        }
        files.sort_by(|a, b| a.cmp(&b.0));
        files
    }
}

impl Default for FileSystem {
    fn default() -> Self {
        Self::new()
    }
}

pub struct FileDialog {
    pub mode: DialogMode,
    pub filters: Vec<FileFilter>,
    pub current_path: PathBuf,
}

#[derive(Debug, Clone)]
pub enum DialogMode {
    Open,
    Save,
    Browse,
}

#[derive(Debug, Clone)]
pub struct FileFilter {
    pub name: String,
    pub extensions: Vec<String>,
}

impl FileDialog {
    pub fn new(mode: DialogMode) -> Self {
        Self {
            mode,
            filters: Vec::new(),
            current_path: PathBuf::from("."),
        }
    }

    pub fn set_filter(&mut self, filter: FileFilter) {
        self.filters.push(filter);
    }

    pub fn set_current_path(&mut self, path: &Path) {
        self.current_path = path.to_path_buf();
    }
}
