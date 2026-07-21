use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct SceneExport {
    pub id: String,
    pub scene_path: PathBuf,
    pub format: SceneFormat,
    pub compressed: bool,
    pub include_dependencies: bool,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SceneFormat {
    Binary,
    JSON,
    XML,
    GDScene,
}

impl SceneExport {
    pub fn new(id: String, scene_path: PathBuf) -> Self {
        Self {
            id,
            scene_path,
            format: SceneFormat::Binary,
            compressed: false,
            include_dependencies: true,
        }
    }

    pub fn set_format(&mut self, format: SceneFormat) {
        self.format = format;
    }

    pub fn export(&self) -> Result<PathBuf, String> {
        let output_path = self.scene_path.with_extension(match self.format {
            SceneFormat::Binary => "bin",
            SceneFormat::JSON => "json",
            SceneFormat::XML => "xml",
            SceneFormat::GDScene => "gdscene",
        });
        
        // Placeholder for actual export logic
        Ok(output_path)
    }
}

impl Default for SceneExport {
    fn default() -> Self {
        Self::new("00000000-0000-0000-0000-000000000000".to_string(), PathBuf::new())
    }
}
