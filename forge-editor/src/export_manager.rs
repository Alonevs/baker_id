use serde::{Deserialize, Serialize};
use std::fs::File;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectData {
    pub name: String,
    pub version: String,
    pub entities: Vec<EntityData>,
    pub physics: PhysicsData,
    pub particles: ParticleData,
    pub animations: AnimationData,
    pub dialogues: Vec<DialogueData>,
    pub events: Vec<EventData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntityData {
    pub id: String,
    pub name: String,
    pub position: (f32, f32, f32),
    pub rotation: (f32, f32, f32),
    pub scale: (f32, f32, f32),
    pub visible: bool,
    pub components: Vec<String>,
    pub scripts: Vec<String>,
    pub sprite_path: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhysicsData {
    pub gravity: (f32, f32, f32),
    pub friction: f32,
    pub restitution: f32,
    pub bodies: Vec<BodyData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BodyData {
    pub id: String,
    pub position: (f32, f32, f32),
    pub rotation: (f32, f32, f32),
    pub scale: (f32, f32, f32),
    pub velocity: (f32, f32, f32),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParticleData {
    pub particles: Vec<ParticleDataItem>,
    pub sprite_frames: Vec<SpriteFrameData>,
    pub fps: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParticleDataItem {
    pub position: (f32, f32, f32),
    pub rotation: (f32, f32, f32),
    pub scale: (f32, f32, f32),
    pub velocity: (f32, f32, f32),
    pub lifetime: f32,
    pub max_lifetime: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpriteFrameData {
    pub texture: String,
    pub frame_rect: (f32, f32, f32, f32),
    pub frame_duration: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnimationData {
    pub animations: Vec<AnimationItem>,
    pub keyframes: Vec<KeyframeData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnimationItem {
    pub name: String,
    pub keyframes: Vec<KeyframeData>,
    pub interpolation: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyframeData {
    pub frame: u32,
    pub value: f32,
    pub interpolation: String,
}

impl From<&KeyframeData> for crate::keyframe::Keyframe {
    fn from(data: &KeyframeData) -> Self {
        crate::keyframe::Keyframe {
            time: data.frame as f32,
            value: data.value,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DialogueData {
    pub id: String,
    pub speaker: String,
    pub text: String,
    pub options: Vec<DialogueOption>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DialogueOption {
    pub text: String,
    pub action: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventData {
    pub id: String,
    pub name: String,
    pub trigger: String,
    pub action: String,
    pub parameters: Vec<String>,
}

pub struct ExportManager {
    project: ProjectData,
}

impl ExportManager {
    pub fn new() -> Self {
        ExportManager {
            project: ProjectData {
                name: "New Project".to_string(),
                version: "1.0.0".to_string(),
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

    pub fn add_entity(&mut self, id: &str, name: &str, position: (f32, f32, f32), rotation: (f32, f32, f32), scale: (f32, f32, f32), visible: bool, components: Vec<String>, scripts: Vec<String>) {
        self.project.entities.push(EntityData {
            id: id.to_string(),
            name: name.to_string(),
            position,
            rotation,
            scale,
            visible,
            components,
            scripts,
            sprite_path: None,
        });
    }

    pub fn add_body(&mut self, id: &str, position: (f32, f32, f32), rotation: (f32, f32, f32), scale: (f32, f32, f32), velocity: (f32, f32, f32)) {
        self.project.physics.bodies.push(BodyData {
            id: id.to_string(),
            position,
            rotation,
            scale,
            velocity,
        });
    }

    pub fn add_particle(&mut self, position: (f32, f32, f32), rotation: (f32, f32, f32), scale: (f32, f32, f32), velocity: (f32, f32, f32), lifetime: f32, max_lifetime: f32) {
        self.project.particles.particles.push(ParticleDataItem {
            position,
            rotation,
            scale,
            velocity,
            lifetime,
            max_lifetime,
        });
    }

    pub fn add_sprite_frame(&mut self, texture: &str, frame_rect: (f32, f32, f32, f32), frame_duration: f32) {
        self.project.particles.sprite_frames.push(SpriteFrameData {
            texture: texture.to_string(),
            frame_rect,
            frame_duration,
        });
    }

    pub fn add_animation(&mut self, name: &str, keyframes: Vec<(u32, f32, String)>) {
        let keyframe_data: Vec<KeyframeData> = keyframes.iter().map(|(frame, value, interpolation)| KeyframeData {
            frame: *frame,
            value: *value,
            interpolation: interpolation.clone(),
        }).collect();
        
        self.project.animations.animations.push(AnimationItem {
            name: name.to_string(),
            keyframes: keyframe_data,
            interpolation: "Linear".to_string(),
        });
    }

    pub fn add_dialogue(&mut self, id: &str, speaker: &str, text: &str, options: Vec<(String, String)>) {
        let options_data: Vec<DialogueOption> = options.iter().map(|(text, action)| DialogueOption {
            text: text.to_string(),
            action: action.to_string(),
        }).collect();
        
        self.project.dialogues.push(DialogueData {
            id: id.to_string(),
            speaker: speaker.to_string(),
            text: text.to_string(),
            options: options_data,
        });
    }

    pub fn add_event(&mut self, id: &str, name: &str, trigger: &str, action: &str, parameters: Vec<String>) {
        self.project.events.push(EventData {
            id: id.to_string(),
            name: name.to_string(),
            trigger: trigger.to_string(),
            action: action.to_string(),
            parameters,
        });
    }

    pub fn export(&self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let file = File::create(path)?;
        let mut writer = std::io::BufWriter::new(file);
        serde_json::to_writer(&mut writer, &self.project)?;
        Ok(())
    }

    pub fn import(path: &str) -> Result<ProjectData, Box<dyn std::error::Error>> {
        let file = File::open(path)?;
        let reader = std::io::BufReader::new(file);
        let project: ProjectData = serde_json::from_reader(reader)?;
        Ok(project)
    }

    pub fn get_project(&self) -> &ProjectData {
        &self.project
    }

    pub fn get_project_mut(&mut self) -> &mut ProjectData {
        &mut self.project
    }

    pub fn get_name(&self) -> &str {
        &self.project.name
    }

    pub fn set_name(&mut self, name: &str) {
        self.project.name = name.to_string();
    }
}

impl Default for ExportManager {
    fn default() -> Self {
        Self::new()
    }
}

