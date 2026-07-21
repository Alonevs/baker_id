//! # Import Manager
//! 
//! Módulo para importar proyectos `.map`

use crate::export_manager::{ProjectData, EntityData, PhysicsData, ParticleData, AnimationData, DialogueData, EventData};
use std::fs::File;
use std::io::BufReader;


/// Importador de proyectos `.map`
#[derive(Debug, Clone)]
pub struct ImportManager {
    pub project: ProjectData,
}

impl ImportManager {
    /// Crea un nuevo importador
    pub fn new() -> Self {
        ImportManager {
            project: ProjectData {
                name: "Imported Project".to_string(),
                version: String::new(),
                entities: Vec::new(),
                physics: PhysicsData {
                    gravity: (0.0, -9.81, 0.0),
                    friction: 0.5,
                    restitution: 0.3,
                    bodies: Vec::new(),
                },
                particles: ParticleData {
                    particles: Vec::new(),
                    sprite_frames: Vec::new(),
                    fps: 60.0,
                },
                animations: AnimationData {
                    animations: Vec::new(),
                    keyframes: Vec::new(),
                },
                dialogues: Vec::new(),
                events: Vec::new(),
            },
        }
    }

    /// Importa un proyecto desde un archivo `.map`
    pub fn import(&mut self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let project: ProjectData = serde_json::from_reader(reader)?;
        
        self.project = project;
        Ok(())
    }

    /// Obtiene el proyecto importado
    pub fn get_project(&self) -> &ProjectData {
        &self.project
    }

    /// Obtiene el nombre del proyecto
    pub fn get_name(&self) -> &str {
        &self.project.name
    }

    /// Obtiene el número de entidades
    pub fn get_entity_count(&self) -> usize {
        self.project.entities.len()
    }

    /// Obtiene todas las entidades
    pub fn get_entities(&self) -> &[EntityData] {
        &self.project.entities
    }

    /// Obtiene el sistema de física
    pub fn get_physics(&self) -> &PhysicsData {
        &self.project.physics
    }

    /// Obtiene el sistema de partículas
    pub fn get_particles(&self) -> &ParticleData {
        &self.project.particles
    }

    /// Obtiene el sistema de animaciones
    pub fn get_animations(&self) -> &AnimationData {
        &self.project.animations
    }

    /// Obtiene los diálogos
    pub fn get_dialogues(&self) -> &[DialogueData] {
        &self.project.dialogues
    }

    /// Obtiene los eventos
    pub fn get_events(&self) -> &[EventData] {
        &self.project.events
    }

    /// Valida el proyecto importado
    pub fn validate(&self) -> Result<(), Box<dyn std::error::Error>> {
        // Verificar que el nombre no esté vacío
        if self.project.name.is_empty() {
            return Err("El nombre del proyecto no puede estar vacío".into());
        }

        // Verificar que el número de entidades no sea negativo
        if self.project.entities.len() == 0 {
            return Err("Número inválido de entidades".into());
        }

        // Verificar que los cuerpos físicos tengan posición válida
        for body in &self.project.physics.bodies {
            if body.position == (f32::INFINITY, f32::INFINITY, f32::INFINITY) {
                return Err("Posición inválida en cuerpo físico".into());
            }
        }

        Ok(())
    }

    /// Obtiene información resumida del proyecto
    pub fn get_summary(&self) -> String {
        format!(
            "Proyecto: {}\nVersión: {}\nEntidades: {}\nCuerpos físicos: {}\nPartículas: {}\nAnimaciones: {}\nDiálogos: {}\nEventos: {}",
            self.project.name,
            self.project.version,
            self.project.entities.len(),
            self.project.physics.bodies.len(),
            self.project.particles.particles.len(),
            self.project.animations.animations.len(),
            self.project.dialogues.len(),
            self.project.events.len()
        )
    }

    /// Importar asset desde disco
    pub fn import_asset(&mut self, path: &std::path::Path, name: &str) {
        // Verificar extensión soportada
        let extension = path.extension()
            .and_then(|e| e.to_str())
            .unwrap_or("")
            .to_lowercase();
        
        let supported_extensions: Vec<&str> = vec![
            "png", "jpg", "jpeg", "gif", "bmp", "webp",  // Sprites
            "mp3", "wav", "ogg", "flac", "aiff",         // Audio
            "csv", "json",                                // Dialogues
            "rs", "lua", "gdscript", "js", "ts",         // Scripts
            "mat", "mtl", "obj",                          // Materials
        ];
        
        if !supported_extensions.contains(&extension.as_str()) {
            println!("Extension no soportada: {}", extension);
            return;
        }
        
        // Registrar asset en el sistema de partículas (para sprites)
        if extension == "png" || extension == "jpg" || extension == "jpeg" || 
           extension == "gif" || extension == "bmp" || extension == "webp" {
            println!("Imported sprite: {}", name);
        } else {
            // Registrar como asset genérico
            println!("Asset imported: {} ({} - {})", name, path.display(), extension);
        }
    }
}

impl Default for ImportManager {
    fn default() -> Self {
        Self::new()
    }
}

