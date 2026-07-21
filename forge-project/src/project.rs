use std::path::PathBuf;
use std::fs;
use std::collections::HashMap;
use uuid::Uuid;
use serde::{Serialize, Deserialize};
use thiserror::Error;
use crate::input::{InputAction, InputAxis, InputButton};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub name: String,
    pub version: String,
    pub engine_version: String,
    pub settings: ProjectSettings,
    pub scenes: HashMap<String, Uuid>,
    pub assets: HashMap<String, Uuid>,
    pub scripts: HashMap<String, Uuid>,
    pub plugins: Vec<String>,
    pub export_presets: HashMap<String, ExportPreset>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectSettings {
    pub project_id: String,
    pub display_name: String,
    pub main_scene: Option<String>,
    pub default_icon: Option<String>,
    pub run_configuration: String,
    pub application: ApplicationSettings,
    pub physics: PhysicsSettings,
    pub rendering: RenderingSettings,
    pub audio: AudioSettings,
    pub input: InputSettings,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApplicationSettings {
    pub name: String,
    pub version: String,
    pub company: String,
    pub icon: Option<String>,
    pub build_templates: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhysicsSettings {
    pub gravity: [f32; 3],
    pub default_collision_layer: u32,
    pub default_collision_mask: u32,
    pub physics_ticks_per_second: f32,
    pub broadphase: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RenderingSettings {
    pub renderer: String,
    pub resolution: [u32; 2],
    pub render_distance: f32,
    pub shadow_quality: u32,
    pub antialiasing: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioSettings {
    pub samples: u32,
    pub buffer: u32,
    pub autoplay: bool,
    pub default_bus: String,
    pub music_bus: String,
    pub sfx_bus: String,
    pub voice_bus: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InputSettings {
    pub action_map: HashMap<String, Vec<InputAction>>,
    pub axes: HashMap<String, Vec<InputAxis>>,
    pub buttons: HashMap<String, Vec<InputButton>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InputEvent {
    Key { keycode: u32, key_label: String, pressed: bool },
    Button { button_index: u32, pressed: bool },
    Axis { axis: String, value: f32 },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportPreset {
    pub name: String,
    pub platform: String,
    pub options: HashMap<String, String>,
    pub include_scenes: Vec<String>,
    pub exclude_scenes: Vec<String>,
}

#[derive(Debug, Error)]
pub enum ProjectError {
    #[error("Project not found: {0}")]
    ProjectNotFound(String),
    #[error("Invalid project format")]
    InvalidFormat,
    #[error("Project locked: {0}")]
    ProjectLocked(String),
    #[error("Save failed: {0}")]
    SaveFailed(String),
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),
}

pub struct ProjectManager {
    pub current_project: Option<Project>,
    pub project_path: Option<PathBuf>,
    pub projects: HashMap<String, PathBuf>,
    pub settings: ProjectSettings,
}

impl ProjectManager {
    pub fn new() -> Self {
        Self {
            current_project: None,
            project_path: None,
            projects: HashMap::new(),
            settings: ProjectSettings::default(),
        }
    }

    pub fn create_project(&mut self, name: &str) -> Result<PathBuf, ProjectError> {
        let project_dir = PathBuf::from(format!("projects/{}", name.to_lowercase()));
        fs::create_dir_all(&project_dir)?;

        let project = Project {
            name: name.to_string(),
            version: "1.0.0".to_string(),
            engine_version: env!("CARGO_PKG_VERSION").to_string(),
            settings: ProjectSettings::default(),
            scenes: HashMap::new(),
            assets: HashMap::new(),
            scripts: HashMap::new(),
            plugins: Vec::new(),
            export_presets: HashMap::new(),
        };

        let project_file = project_dir.join("project.godot");
        self.save_project(&project, &project_file)?;

        self.current_project = Some(project);
        self.project_path = Some(project_dir.clone());

        Ok(project_dir)
    }

    pub fn load_project(&mut self, path: &PathBuf) -> Result<Project, ProjectError> {
        let project_file = path.join("project.godot");
        if !project_file.exists() {
            return Err(ProjectError::ProjectNotFound(path.display().to_string()));
        }

        let content = fs::read_to_string(&project_file)?;
        let project: Project = serde_json::from_str(&content)?;

        self.current_project = Some(project.clone());
        self.project_path = Some(path.clone());

        Ok(project)
    }

    pub fn save_project(&mut self, project: &Project, path: &PathBuf) -> Result<(), ProjectError> {
        let project_file = path.join("project.godot");
        let content = serde_json::to_string_pretty(project)?;
        fs::write(&project_file, content)?;
        Ok(())
    }

    pub fn open_project(&mut self, path: &PathBuf) -> Result<(), ProjectError> {
        let project = self.load_project(path)?;
        self.current_project = Some(project);
        self.project_path = Some(path.clone());
        Ok(())
    }

    pub fn save(&mut self) -> Result<(), ProjectError> {
        if let Some(project) = self.current_project.clone() {
            if let Some(path) = self.project_path.clone() {
                self.save_project(&project, &path)?;
            }
        }
        Ok(())
    }

    pub fn get_current_project(&self) -> Option<&Project> {
        self.current_project.as_ref()
    }

    pub fn get_current_project_mut(&mut self) -> Option<&mut Project> {
        self.current_project.as_mut()
    }

    pub fn get_project_path(&self) -> Option<&PathBuf> {
        self.project_path.as_ref()
    }

    pub fn register_scene(&mut self, name: &str) {
        if let Some(ref mut project) = self.current_project {
            project.scenes.insert(name.to_string(), Uuid::new_v4());
        }
    }

    pub fn register_asset(&mut self, name: &str) {
        if let Some(ref mut project) = self.current_project {
            project.assets.insert(name.to_string(), Uuid::new_v4());
        }
    }

    pub fn register_script(&mut self, name: &str) {
        if let Some(ref mut project) = self.current_project {
            project.scripts.insert(name.to_string(), Uuid::new_v4());
        }
    }

    pub fn open_project_dir(&mut self, path: &PathBuf) -> Result<(), ProjectError> {
        let project_file = path.join("project.godot");
        if project_file.exists() {
            self.open_project(path)?;
            Ok(())
        } else {
            Err(ProjectError::ProjectNotFound(path.display().to_string()))
        }
    }

    pub fn close_project(&mut self) {
        self.current_project = None;
        self.project_path = None;
    }

    pub fn list_projects(&self) -> Vec<(String, PathBuf)> {
        self.projects.iter().map(|(k, v)| (k.clone(), v.clone())).collect()
    }

    pub fn add_export_preset(&mut self, name: &str, platform: &str) {
        if let Some(ref mut project) = self.current_project {
            let preset = ExportPreset {
                name: name.to_string(),
                platform: platform.to_string(),
                options: HashMap::new(),
                include_scenes: Vec::new(),
                exclude_scenes: Vec::new(),
            };
            project.export_presets.insert(name.to_string(), preset);
        }
    }
}

impl Default for ProjectSettings {
    fn default() -> Self {
        Self {
            project_id: Uuid::new_v4().to_string(),
            display_name: "New Project".to_string(),
            main_scene: None,
            default_icon: None,
            run_configuration: "debug".to_string(),
            application: ApplicationSettings::default(),
            physics: PhysicsSettings::default(),
            rendering: RenderingSettings::default(),
            audio: AudioSettings::default(),
            input: InputSettings::default(),
        }
    }
}

impl Default for ApplicationSettings {
    fn default() -> Self {
        Self {
            name: "Forge Game".to_string(),
            version: "1.0".to_string(),
            company: "Forge".to_string(),
            icon: None,
            build_templates: HashMap::new(),
        }
    }
}

impl Default for PhysicsSettings {
    fn default() -> Self {
        Self {
            gravity: [-9.81, 0.0, 0.0],
            default_collision_layer: 1,
            default_collision_mask: 1,
            physics_ticks_per_second: 60.0,
            broadphase: "sweep_and_prune".to_string(),
        }
    }
}

impl Default for RenderingSettings {
    fn default() -> Self {
        Self {
            renderer: "gl_compatibility".to_string(),
            resolution: [1920, 1080],
            render_distance: 100.0,
            shadow_quality: 2,
            antialiasing: 2,
        }
    }
}

impl Default for AudioSettings {
    fn default() -> Self {
        Self {
            samples: 48000,
            buffer: 512,
            autoplay: false,
            default_bus: "master".to_string(),
            music_bus: "music".to_string(),
            sfx_bus: "sfx".to_string(),
            voice_bus: "voice".to_string(),
        }
    }
}

impl Default for InputSettings {
    fn default() -> Self {
        Self {
            action_map: HashMap::new(),
            axes: HashMap::new(),
            buttons: HashMap::new(),
        }
    }
}
