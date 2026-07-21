use std::fs;
use std::path::PathBuf;

use crate::export_preset::{ExportPreset, Platform};

pub struct ExportManager {
    pub presets: Vec<ExportPreset>,
    pub current_preset: Option<String>,
}

impl ExportManager {
    pub fn new() -> Self {
        Self {
            presets: Vec::new(),
            current_preset: None,
        }
    }

    pub fn add_preset(&mut self, preset: ExportPreset) -> String {
        let id = preset.id.clone();
        self.presets.push(preset);
        id
    }

    pub fn set_current(&mut self, preset_id: String) {
        self.current_preset = Some(preset_id);
    }

    pub fn get_current(&self) -> Option<&ExportPreset> {
        if let Some(ref current) = self.current_preset {
            self.presets.iter().find(|p| p.id.as_str() == current)
        } else {
            None
        }
    }

    pub fn export(&self, project_path: &PathBuf) -> Result<String, ExportError> {
        let preset = self.get_current().ok_or(ExportError::NoPresetSelected)?;
        let output_dir = project_path.join(preset.get_output_path());
        
        if let Err(e) = fs::create_dir_all(&output_dir) {
            return Err(ExportError::CreateDir(e.to_string()));
        }
        
        match preset.platform {
            Platform::Windows => self.export_windows(&output_dir),
            Platform::Linux => self.export_linux(&output_dir),
            Platform::Macos => self.export_macos(&output_dir),
            Platform::Web => self.export_web(&output_dir),
            Platform::Android => self.export_android(&output_dir),
            Platform::Ios => self.export_ios(&output_dir),
        }
    }

    fn export_windows(&self, output_dir: &PathBuf) -> Result<String, ExportError> {
        let exe_path = output_dir.join("game.exe");
        if let Err(e) = fs::write(&exe_path, b"Placeholder for Windows executable") {
            return Err(ExportError::WriteFile(e.to_string()));
        }
        Ok(exe_path.to_str().unwrap().to_string())
    }

    fn export_linux(&self, output_dir: &PathBuf) -> Result<String, ExportError> {
        let exe_path = output_dir.join("game");
        if let Err(e) = fs::write(&exe_path, b"Placeholder for Linux executable") {
            return Err(ExportError::WriteFile(e.to_string()));
        }
        Ok(exe_path.to_str().unwrap().to_string())
    }

    fn export_macos(&self, output_dir: &PathBuf) -> Result<String, ExportError> {
        let app_path = output_dir.join("Game.app");
        if let Err(e) = fs::create_dir_all(&app_path) {
            return Err(ExportError::CreateDir(e.to_string()));
        }
        Ok(app_path.to_str().unwrap().to_string())
    }

    fn export_web(&self, output_dir: &PathBuf) -> Result<String, ExportError> {
        let dist_dir = output_dir.join("dist");
        if let Err(e) = fs::create_dir_all(&dist_dir) {
            return Err(ExportError::CreateDir(e.to_string()));
        }
        let index_path = dist_dir.join("index.html");
        if let Err(e) = fs::write(&index_path, b"<!DOCTYPE html><html><body>Game loaded</body></html>") {
            return Err(ExportError::WriteFile(e.to_string()));
        }
        Ok(dist_dir.to_str().unwrap().to_string())
    }

    fn export_android(&self, output_dir: &PathBuf) -> Result<String, ExportError> {
        let apk_path = output_dir.join("app-release.apk");
        if let Err(e) = fs::write(&apk_path, b"Placeholder for Android APK") {
            return Err(ExportError::WriteFile(e.to_string()));
        }
        Ok(apk_path.to_str().unwrap().to_string())
    }

    fn export_ios(&self, output_dir: &PathBuf) -> Result<String, ExportError> {
        let ipa_path = output_dir.join("Game.ipa");
        if let Err(e) = fs::write(&ipa_path, b"Placeholder for iOS IPA") {
            return Err(ExportError::WriteFile(e.to_string()));
        }
        Ok(ipa_path.to_str().unwrap().to_string())
    }
}

impl Default for ExportManager {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ExportError {
    #[error("No preset selected")]
    NoPresetSelected,
    #[error("Failed to create output directory: {0}")]
    CreateDir(String),
    #[error("Failed to write file: {0}")]
    WriteFile(String),
    #[error("Platform not supported: {0}")]
    PlatformNotSupported(String),
    #[error("Invalid project path")]
    InvalidProjectPath,
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}
