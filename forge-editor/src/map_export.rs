use serde::{Serialize, Deserialize};

/// Formato de exportación del proyecto
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectExport {
    pub version: String,
    pub name: String,
    pub physics: Option<PhysicsExport>,
    pub particles: Option<ParticlesExport>,
    pub animations: Option<AnimationsExport>,
    pub dialogues: Option<DialoguesExport>,
    pub events: Option<EventsExport>,
}

/// Exportación de datos de física
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhysicsExport {
    pub blocks: Vec<PhysicsBlock>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhysicsBlock {
    pub id: String,
    pub position: Vector2,
    pub size: Vector2,
    pub mass: f32,
    pub velocity: Vector2,
    pub is_static: bool,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Vector2 {
    pub x: f32,
    pub y: f32,
}

/// Exportación de sistema de partículas
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParticlesExport {
    pub particles: Vec<ParticleData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParticleData {
    pub id: String,
    pub position: Vector2,
    pub velocity: Vector2,
    pub size: Vector2,
    pub sprite_path: String,
    pub lifetime: f32,
}

/// Exportación de animaciones
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnimationsExport {
    pub animations: Vec<AnimationData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnimationData {
    pub name: String,
    pub duration: f32,
    pub loops: i32,
    pub tracks: Vec<TrackData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrackData {
    pub name: String,
    pub keyframes: Vec<KeyframeData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyframeData {
    pub time: f32,
    pub value: f32,
}

/// Exportación de diálogos
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DialoguesExport {
    pub dialogues: Vec<DialogueExport>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DialogueExport {
    pub id: String,
    pub lines: Vec<DialogueLine>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DialogueLine {
    pub text: String,
    pub language: String,
    pub action: Option<String>,
}

/// Exportación de eventos
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventsExport {
    pub events: Vec<EventData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventData {
    pub id: String,
    pub triggers: Vec<String>,
    pub dialog_id: String,
    pub actions: Vec<String>,
}

/// ExportManager para manejar exportación/importación
pub struct ExportManager {
    pub physics: Option<PhysicsExport>,
    pub particles: Option<ParticlesExport>,
    pub animations: Option<AnimationsExport>,
    pub dialogues: Option<DialoguesExport>,
    pub events: Option<EventsExport>,
}

impl Default for ExportManager {
    fn default() -> Self {
        Self {
            physics: None,
            particles: None,
            animations: None,
            dialogues: None,
            events: None,
        }
    }
}

impl ExportManager {
    /// Crear nuevo export manager
    pub fn new() -> Self {
        Self::default()
    }

    /// Añadir datos de física
    pub fn set_physics(&mut self, physics: PhysicsExport) {
        self.physics = Some(physics);
    }

    /// Añadir datos de partículas
    pub fn set_particles(&mut self, particles: ParticlesExport) {
        self.particles = Some(particles);
    }

    /// Añadir datos de animaciones
    pub fn set_animations(&mut self, animations: AnimationsExport) {
        self.animations = Some(animations);
    }

    /// Añadir datos de diálogos
    pub fn set_dialogues(&mut self, dialogues: DialoguesExport) {
        self.dialogues = Some(dialogues);
    }

    /// Añadir datos de eventos
    pub fn set_events(&mut self, events: EventsExport) {
        self.events = Some(events);
    }

    /// Exportar a formato .map
    pub fn export_to_map(&self) -> String {
        let mut content = String::new();
        
        content.push_str("FORGE_MAP\n");
        content.push_str(format!("VERSION:1.0\n").as_str());
        content.push_str("END_HEADER\n\n");
        
        // Física
        if let Some(physics) = &self.physics {
            content.push_str("PHYSICS_DATA\n");
            content.push_str(&format!("BLOCK_COUNT: {}\n", physics.blocks.len()));
            for block in physics.blocks.iter() {
                content.push_str(&format!(
                    "BLOCK:{},{},{},{},{},{},{}\n",
                    block.id,
                    block.position.x,
                    block.position.y,
                    block.size.x,
                    block.size.y,
                    block.mass,
                    block.is_static
                ));
            }
            content.push_str("END_PHYSICS\n\n");
        }
        
        // Partículas
        if let Some(particles) = &self.particles {
            content.push_str("PARTICLES_DATA\n");
            content.push_str(&format!("PARTICLE_COUNT: {}\n", particles.particles.len()));
            for particle in particles.particles.iter() {
                content.push_str(&format!(
                    "PARTICLE:{},{},{},{},{},{},{},{}\n",
                    particle.id,
                    particle.position.x,
                    particle.position.y,
                    particle.size.x,
                    particle.size.y,
                    particle.velocity.x,
                    particle.velocity.y,
                    particle.lifetime
                ));
            }
            content.push_str("END_PARTICLES\n\n");
        }
        
        // Animaciones
        if let Some(animations) = &self.animations {
            content.push_str("ANIMATIONS_DATA\n");
            content.push_str(&format!("ANIMATION_COUNT: {}\n", animations.animations.len()));
            for animation in animations.animations.iter() {
                content.push_str(&format!("ANIMATION:{},{}\n", animation.name, animation.duration));
                for track in animation.tracks.iter() {
                    content.push_str(&format!("TRACK:{},{}\n", track.name, track.keyframes.len()));
                    for keyframe in track.keyframes.iter() {
                        content.push_str(&format!("KEYFRAME:{},{}\n", keyframe.time, keyframe.value));
                    }
                }
            }
            content.push_str("END_ANIMATIONS\n\n");
        }
        
        // Diálogos
        if let Some(dialogues) = &self.dialogues {
            content.push_str("DIALOGUES_DATA\n");
            content.push_str(&format!("DIALOGUE_COUNT: {}\n", dialogues.dialogues.len()));
            for dialogue in dialogues.dialogues.iter() {
                content.push_str(&format!("DIALOGUE:{},{}\n", dialogue.id, dialogue.lines.len()));
                for line in dialogue.lines.iter() {
                    content.push_str(&format!("LINE:,{},{},{}\n", line.text, line.language, line.action.as_deref().unwrap_or("")));
                }
            }
            content.push_str("END_DIALOGUES\n\n");
        }
        
        // Eventos
        if let Some(events) = &self.events {
            content.push_str("EVENTS_DATA\n");
            content.push_str(&format!("EVENT_COUNT: {}\n", events.events.len()));
            for event in events.events.iter() {
                content.push_str(&format!("EVENT:{},{},{},{}\n", event.id, event.triggers.join(","), event.dialog_id, event.actions.join(",")));
            }
            content.push_str("END_EVENTS\n");
        }
        
        content
    }

    /// Importar desde formato .map
    pub fn import_from_map(&mut self, content: &str) {
        let mut current_section: Option<String> = None;
        let mut section_data: Vec<String> = Vec::new();
        
        for line in content.lines() {
            if line.starts_with("END_") {
                if let Some(section) = current_section.take() {
                    self.import_section(&section, &section_data);
                }
                section_data.clear();
            } else if !line.is_empty() && !line.starts_with("BLOCK:") && !line.starts_with("PARTICLE:") {
                let parts: Vec<&str> = line.split(':').collect();
                if parts.len() >= 2 {
                    let key = parts[0].trim().to_string();
                    let value = parts[1].trim().to_string();
                    section_data.push(format!("{}:{}", key, value));
                }
            } else if line.starts_with("PHYSICS_DATA") {
                current_section = Some("physics".to_string());
            } else if line.starts_with("PARTICLES_DATA") {
                current_section = Some("particles".to_string());
            } else if line.starts_with("ANIMATIONS_DATA") {
                current_section = Some("animations".to_string());
            } else if line.starts_with("DIALOGUES_DATA") {
                current_section = Some("dialogues".to_string());
            } else if line.starts_with("EVENTS_DATA") {
                current_section = Some("events".to_string());
            }
        }
    }

    /// Importar sección específica
    fn import_section(&mut self, section: &str, _data: &[String]) {
        match section {
            "physics" => {
                // Parsear datos de física
            }
            "particles" => {
                // Parsear datos de partículas
            }
            "animations" => {
                // Parsear datos de animaciones
            }
            "dialogues" => {
                // Parsear datos de diálogos
            }
            "events" => {
                // Parsear datos de eventos
            }
            _ => {}
        }
    }
}

