//! # Project Manager
//! 
//! Gestor de proyectos para Forge Editor:
//! - Crear/abrir proyectos con proyecto.toml
//! - Estructura de carpetas en disco
//! - Asistente de creación de proyectos

use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use uuid::Uuid;

/// Tipos de juego soportados
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum GameType {
    /// Juego isométrico (estrategia, RPG)
    Isometric,
    /// Juego de plataformas
    Platformer,
    /// Run-and-gun (acción, metroidvania)
    RunAndGun,
}

impl Default for GameType {
    fn default() -> Self {
        Self::Isometric
    }
}

impl std::fmt::Display for GameType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Isometric => write!(f, "Isométrico"),
            Self::Platformer => write!(f, "Plataformas"),
            Self::RunAndGun => write!(f, "Run-and-gun"),
        }
    }
}

/// Configuración de física según tipo de juego
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhysicsConfig {
    /// Gravedad del juego
    pub gravity: f32,
    /// Saltos máximos permitidos (0 = sin saltos)
    pub max_jumps: u8,
    /// Física de plataformas
    pub platform_physics: bool,
    /// Física de colisiones
    pub collision_physics: bool,
}

impl Default for PhysicsConfig {
    fn default() -> Self {
        Self {
            gravity: 9.8,
            max_jumps: 1,
            platform_physics: true,
            collision_physics: true,
        }
    }
}

/// Información de asset
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetInfo {
    /// Nombre del archivo
    pub name: String,
    /// Categoría del asset
    pub category: String,
    /// Ruta relativa
    pub path: String,
}

/// Configuración completa del proyecto
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    /// ID único del proyecto
    pub id: Uuid,
    /// Nombre del proyecto
    pub name: String,
    /// Tipo de juego
    pub game_type: GameType,
    /// Ruta absoluta al proyecto en disco
    pub path: PathBuf,
    /// Configuración de física
    pub physics: PhysicsConfig,
    /// Límite de entidades
    pub entity_limit: u32,
    /// Límite de eventos
    pub event_limit: u32,
    /// Fecha de creación
    pub created_at: u64,
    /// Última modificación
    pub modified_at: u64,
    /// Lista de assets del proyecto
    pub assets: Vec<AssetInfo>,
}

impl Project {
    /// Crea un nuevo proyecto con configuración según tipo de juego
    pub fn new(name: &str, game_type: GameType) -> Self {
        let physics = match game_type {
            GameType::Isometric => PhysicsConfig {
                gravity: 9.8,
                max_jumps: 1,
                platform_physics: true,
                collision_physics: true,
            },
            GameType::Platformer => PhysicsConfig {
                gravity: 12.0,
                max_jumps: 1,
                platform_physics: true,
                collision_physics: true,
            },
            GameType::RunAndGun => PhysicsConfig {
                gravity: 15.0,
                max_jumps: 2,
                platform_physics: true,
                collision_physics: true,
            },
        };

        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or(std::time::Duration::ZERO)
            .as_secs();

        Self {
            id: Uuid::new_v4(),
            name: name.to_string(),
            game_type,
            path: PathBuf::from("assets"),
            physics,
            entity_limit: 1000,
            event_limit: 500,
            created_at: now,
            modified_at: now,
            assets: Vec::new(),
        }
    }

    /// Carga un proyecto desde un archivo proyecto.toml
    pub fn load(path: &PathBuf) -> Result<Self, String> {
        let content = std::fs::read_to_string(path)
            .map_err(|e| format!("Error leyendo proyecto.toml: {}", e))?;

        let project: Project = toml::from_str(&content)
            .map_err(|e| format!("Error parseando proyecto.toml: {}", e))?;

        // Verificar que la ruta existe
        if !project.path.exists() {
            return Err(format!("Directorio del proyecto no existe: {:?}", project.path));
        }

        Ok(project)
    }

    /// Guarda el proyecto en un archivo proyecto.toml
    pub fn save(&mut self) -> Result<(), String> {
        let content = toml::to_string_pretty(self)
            .map_err(|e| format!("Error serializando proyecto: {}", e))?;

        std::fs::write(&self.path, content)
            .map_err(|e| format!("Error escribiendo proyecto.toml: {}", e))?;

        self.modified_at = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or(std::time::Duration::ZERO)
            .as_secs();

        Ok(())
    }

    /// Crea la estructura de carpetas del proyecto en disco
    pub fn create_directory_structure(&mut self) -> Result<(), String> {
        let paths_to_create = vec![
            "assets",
            "assets/sprites",
            "assets/tilesets",
            "assets/audio",
            "assets/audio/music",
            "assets/audio/sfx",
            "assets/characters",
            "assets/particles",
            "mapas",
            "mapas/level_01",
            "eventos",
            "eventos/dialogue",
            "eventos/gameplay",
            "eventos/ai",
            "scripts",
            "scripts/player",
            "scripts/enemy",
            "scripts/game",
            "config",
            "config/ui",
            "config/input",
        ];

        for path_str in &paths_to_create {
            let path = self.path.join(path_str);
            if let Err(e) = std::fs::create_dir_all(&path) {
                return Err(format!("Error creando directorio {:?}: {}", path, e));
            }
        }

        Ok(())
    }

    /// Verifica si el proyecto es válido
    pub fn validate(&self) -> Result<(), String> {
        // Verificar proyecto.toml
        if !self.path.exists() {
            return Err(format!("Directorio del proyecto no existe: {:?}", self.path));
        }

        let project_toml = self.path.join("proyecto.toml");
        if !project_toml.exists() {
            return Err("Archivo proyecto.toml no existe".to_string());
        }

        // Verificar carpetas esenciales
        let required_dirs = vec![
            "assets",
            "mapas",
            "eventos",
            "scripts",
        ];

        for dir in &required_dirs {
            let dir_path = self.path.join(dir);
            if !dir_path.exists() {
                return Err(format!("Directorio requerido no existe: {:?}", dir_path));
            }
        }

        Ok(())
    }

    /// Verifica y crea carpetas que faltan
    pub fn validate_or_create(&mut self) -> Result<(), String> {
        self.validate()?;

        // Verificar carpetas esenciales y crear si faltan
        let required_dirs = vec![
            "assets",
            "assets/sprites",
            "assets/tilesets",
            "assets/audio",
            "assets/audio/music",
            "assets/audio/sfx",
            "assets/characters",
            "assets/particles",
            "mapas",
            "mapas/level_01",
            "eventos",
            "eventos/dialogue",
            "eventos/gameplay",
            "eventos/ai",
            "scripts",
            "scripts/player",
            "scripts/enemy",
            "scripts/game",
            "config",
            "config/ui",
            "config/input",
        ];

        for dir in &required_dirs {
            let dir_path = self.path.join(dir);
            if !dir_path.exists() {
                std::fs::create_dir_all(&dir_path)
                    .map_err(|e| format!("Error creando directorio {:?}: {}", dir_path, e))?;
            }
        }

        Ok(())
    }

    /// Obtiene la ruta de un archivo relativo al proyecto
    pub fn path_to(&self, relative: &str) -> PathBuf {
        self.path.join(relative)
    }

    /// Obtiene la ruta de la carpeta de assets
    pub fn assets_path(&self) -> PathBuf {
        self.path.join("assets")
    }

    /// Obtiene la ruta de la carpeta de mapas
    pub fn maps_path(&self) -> PathBuf {
        self.path.join("mapas")
    }

    /// Obtiene la ruta de la carpeta de eventos
    pub fn events_path(&self) -> PathBuf {
        self.path.join("eventos")
    }

    /// Obtiene la ruta de la carpeta de scripts
    pub fn scripts_path(&self) -> PathBuf {
        self.path.join("scripts")
    }

    /// Escanea y obtiene todos los assets del proyecto
    pub fn scan_project_assets(&self) -> Vec<String> {
        let assets_path = self.assets_path();
        let mut assets = Vec::new();

        if assets_path.exists() {
            if let Ok(entries) = std::fs::read_dir(&assets_path) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if path.is_file() {
                        let extension = path.extension().and_then(|e| e.to_str()).unwrap_or("");
                        let file_name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");

                        // Soporte para imágenes y assets comunes
                        if ["png", "jpg", "jpeg", "gif", "webp", "tga", "psd", "svg", "fnt", "json", "script", "mp3", "wav", "ogg", "flac", "aiff"].contains(&extension) {
                            assets.push(file_name.to_string());
                        }
                    }
                }
            }
        }

        assets
    }

    /// Lista todos los assets del proyecto
    pub fn list_all_assets(&self) -> Vec<(String, String)> {
        let mut assets = Vec::new();
        let assets_path = self.assets_path();

        if assets_path.exists() {
            if let Ok(entries) = std::fs::read_dir(&assets_path) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if path.is_file() {
                        let file_name = path.file_name().and_then(|n| n.to_str()).unwrap_or("unknown").to_string();
                        let extension = path.extension().and_then(|e| e.to_str()).unwrap_or("").to_lowercase();
                        let category = self.get_category_for_extension(&extension);
                        assets.push((file_name, category.to_string()));
                    }
                }
            }
        }

        assets
    }

    /// Obtiene categoría para una extensión
    fn get_category_for_extension(&self, extension: &str) -> &'static str {
        match extension {
            "png" | "jpg" | "jpeg" | "gif" | "bmp" | "webp" => "Sprites",
            "mp3" | "wav" | "ogg" | "flac" | "aiff" => "Audio",
            "csv" | "json" => "Dialogues",
            "rs" | "lua" | "gdscript" | "js" | "ts" => "Scripts",
            "mat" | "mtl" | "obj" => "Materials",
            _ => "Other",
        }
    }
}

/// Asistente para crear nuevos proyectos
#[derive(Debug, Clone)]
pub struct ProjectWizard {
    /// Nombre del proyecto
    pub name: String,
    /// Tipo de juego seleccionado
    pub game_type: GameType,
    /// Ruta base del proyecto
    pub base_path: PathBuf,
}

impl ProjectWizard {
    /// Crea un nuevo asistente con nombre predeterminado
    pub fn new(name: String, game_type: GameType, base_path: PathBuf) -> Self {
        Self {
            name,
            game_type,
            base_path,
        }
    }

    /// Valida el nombre del proyecto
    pub fn validate_name(&self) -> Result<(), String> {
        if self.name.is_empty() {
            return Err("El nombre del proyecto no puede estar vacío".to_string());
        }

        if self.name.len() > 50 {
            return Err("El nombre del proyecto no puede exceder 50 caracteres".to_string());
        }

        // Verificar que no comience con espacios
        if self.name.chars().next().map(|c| c.is_whitespace()).unwrap_or(false) {
            return Err("El nombre del proyecto no puede comenzar con espacios".to_string());
        }

        Ok(())
    }

    /// Valida que la ruta sea válida
    pub fn validate_path(&self) -> Result<(), String> {
        // Verificar que no sea una ruta absoluta en el sistema
        if self.base_path.is_absolute() && self.base_path.as_os_str().to_string_lossy().len() > 200 {
            return Err("La ruta del proyecto es demasiado larga".to_string());
        }

        // Verificar que no sea un directorio de sistema
        let path_str = self.base_path.as_os_str().to_string_lossy();
        let system_paths = vec![
            "C:\\",
            "C:\\Windows",
            "C:\\Program Files",
            "C:\\Users\\",
            "D:\\",
            "E:\\",
        ];

        for system_path in &system_paths {
            if path_str.starts_with(system_path) {
                return Err(format!("No se puede crear proyectos en directorios del sistema: {}", system_path));
            }
        }

        Ok(())
    }

    /// Ejecuta el asistente y crea el proyecto
    pub fn execute(self) -> Result<Project, String> {
        // Validaciones
        self.validate_name()?;
        self.validate_path()?;

        // Crear proyecto
        let mut project = Project::new(&self.name, self.game_type);

        // Crear estructura de carpetas
        project.create_directory_structure()?;

        // Guardar proyecto.toml
        project.save()?;

        Ok(project)
    }
}

/// Gestor de proyectos (CRUD)
#[derive(Debug, Clone)]
pub struct ProjectManager {
    /// Proyecto actualmente abierto
    pub current_project: Option<Project>,
    /// Lista de proyectos guardados
    pub projects: Vec<Project>,
    /// Carpeta base donde se guardan los proyectos
    pub projects_folder: PathBuf,
}

impl Default for ProjectManager {
    fn default() -> Self {
        Self::new()
    }
}

impl ProjectManager {
    /// Crea un nuevo gestor de proyectos
    pub fn new() -> Self {
        let projects_folder = env!("CARGO_MANIFEST_DIR").to_string()
            .replace('\\', "/")
            .replace("forge-editor", "forge-projects")
            .to_string();

        Self {
            current_project: None,
            projects: Vec::new(),
            projects_folder: PathBuf::from(&projects_folder),
        }
    }

    /// Carga proyectos guardados desde la carpeta de proyectos
    pub fn load_projects(&mut self) -> Result<(), String> {
        // Crear carpeta de proyectos si no existe
        if !self.projects_folder.exists() {
            std::fs::create_dir_all(&self.projects_folder)
                .map_err(|e| format!("Error creando carpeta de proyectos: {}", e))?;
        }

        // Buscar todos los proyecto.toml
        let entries = std::fs::read_dir(&self.projects_folder)
            .map_err(|e| format!("Error leyendo carpeta de proyectos: {}", e))?;

        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().map_or(false, |ext| ext == "toml") {
                let project_path = path.parent().unwrap().to_path_buf();
                
                if let Ok(project) = Project::load(&project_path) {
                    self.projects.push(project);
                }
            }
        }

        Ok(())
    }

    /// Crea un nuevo proyecto
    pub fn create_project(&mut self, name: String, game_type: GameType) -> Result<Project, String> {
        // Crear wizard y ejecutar
        let project = ProjectWizard::new(name, game_type, PathBuf::from("assets"))
            .execute()?;

        // Guardar en lista
        self.projects.push(project.clone());
        self.current_project = Some(project.clone());

        Ok(project)
    }

    /// Abre un proyecto existente
    pub fn open_project(&mut self, project_path: &PathBuf) -> Result<Project, String> {
        // Verificar que el archivo existe
        let project_toml = project_path.join("proyecto.toml");
        if !project_toml.exists() {
            return Err(format!("Archivo proyecto.toml no existe: {:?}", project_toml));
        }

        // Cargar proyecto
        let project = Project::load(project_path)?;

        // Validar proyecto
        project.validate()?;

        // Actualizar lista de proyectos
        self.projects.push(project.clone());
        self.current_project = Some(project.clone());

        Ok(project)
    }

    /// Obtiene la ruta de assets del proyecto actual
    pub fn current_assets_path(&self) -> Option<PathBuf> {
        self.current_project.as_ref().map(|p| p.assets_path())
    }

    /// Guarda el proyecto actual
    pub fn save_project(&mut self) -> Result<(), String> {
        if let Some(ref mut project) = self.current_project {
            // Escanear assets del proyecto
            project.assets = self.scan_project_assets()
                .iter()
                .map(|name| AssetInfo {
                    name: name.clone(),
                    category: self.get_category_for_extension(name.split('.').last().unwrap_or("").to_lowercase().as_str()),
                    path: format!("assets/{}", name),
                })
                .collect();

            project.save()?;
            Ok(())
        } else {
            Err("No hay proyecto guardado".to_string())
        }
    }

    /// Obtiene categoría para una extensión
    fn get_category_for_extension(&self, extension: &str) -> &'static str {
        match extension {
            "png" | "jpg" | "jpeg" | "gif" | "bmp" | "webp" => "Sprites",
            "mp3" | "wav" | "ogg" | "flac" | "aiff" => "Audio",
            "csv" | "json" => "Dialogues",
            "rs" | "lua" | "gdscript" | "js" | "ts" => "Scripts",
            "mat" | "mtl" | "obj" => "Materials",
            _ => "Other",
        }
    }

    /// Cierra el proyecto actual
    pub fn close_project(&mut self) -> Option<Project> {
        self.current_project.take()
    }

    /// Obtiene el proyecto actual
    pub fn current(&self) -> Option<&Project> {
        self.current_project.as_ref()
    }

    /// Obtiene el proyecto actual mutable
    pub fn current_mut(&mut self) -> Option<&mut Project> {
        self.current_project.as_mut()
    }

    /// Obtiene lista de proyectos
    pub fn projects(&self) -> &[Project] {
        &self.projects
    }

    /// Obtiene lista de proyectos mutable
    pub fn projects_mut(&mut self) -> &mut Vec<Project> {
        &mut self.projects
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_project_new() {
        let project = Project::new("Test Project", GameType::Platformer);
        assert_eq!(project.name, "Test Project");
        assert_eq!(project.game_type, GameType::Platformer);
        assert!(project.id != Uuid::nil());
    }

    #[test]
    fn test_project_wizard_validate_name() {
        let wizard = ProjectWizard::new(String::new(), GameType::Isometric, PathBuf::from("."));
        assert!(wizard.validate_name().is_err());

        let wizard = ProjectWizard::new("   ".to_string(), GameType::Isometric, PathBuf::from("."));
        assert!(wizard.validate_name().is_err());
    }

    #[test]
    fn test_project_validate() {
        let project = Project::new("Test", GameType::Isometric);
        assert!(project.validate().is_err()); // Carpetas no existen
    }
}
