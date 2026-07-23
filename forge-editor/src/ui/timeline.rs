//! # Timeline API
//! 
//! Módulo para el editor de timeline y gestión de keyframes.

use uuid::Uuid;
use eframe::egui;
use crate::animation_2d::{Animation2D, Animation2DManager};

/// Timeline Editor UI para gestión de keyframes y animación
/// 
/// Conecta con: animation_2d, keyframe
#[derive(Debug, Clone)]
pub struct TimelineEditor {
    pub current_frame: f64,
    pub duration: f64,
    pub fps: f32,
    pub events: Vec<TimelineEventData>,
    pub is_playing: bool,
    pub selected_events: Vec<Uuid>,
    pub current_track: Option<TrackId>,
    pub animation_manager: Animation2DManager,
}

impl Default for TimelineEditor {
    fn default() -> Self {
        Self {
            current_frame: 0.0,
            duration: 10.0,
            fps: 60.0,
            events: Vec::new(),
            is_playing: false,
            selected_events: Vec::new(),
            current_track: None,
            animation_manager: Animation2DManager::default(),
        }
    }
}

impl TimelineEditor {
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Crea una nueva animación
    pub fn create_animation(&mut self, name: &str, duration: f32) -> Option<&mut Animation2D> {
        self.animation_manager.create_animation(name, duration)
    }
    
    /// Renderiza el editor de timeline en la UI
    /// 
    /// Conecta con: animation_2d, keyframe
    pub fn render(ui: &mut egui::Ui, app: &mut crate::ForgeEditorApp) {
        ui.heading("Timeline");
        ui.add_space(10.0);
        ui.separator();
        ui.add_space(5.0);
        
        ui.group(|ui| {
            ui.label("Timeline");
            ui.add_space(5.0);
            ui.horizontal(|ui| {
                ui.label(format!("Frame: {:.0}", app.timeline.current_frame as u32));
                if ui.button("Play").clicked() {
                    app.timeline.is_playing = true;
                }
                if ui.button("Stop").clicked() {
                    app.timeline.is_playing = false;
                }
            });
            ui.add_space(5.0);
            ui.horizontal(|ui| {
                ui.label(format!("FPS: {}", app.timeline.fps));
                ui.label(format!("Duration: {}s", app.timeline.duration));
            });
            ui.add_space(5.0);
            ui.label(format!("Playing: {}", app.timeline.is_playing));
        });

        ui.add_space(10.0);
        ui.separator();
        ui.add_space(5.0);

        let mut keyframe_to_remove = None;
        ui.group(|ui| {
            ui.label("Animations");
            ui.add_space(5.0);

            for (name, animation) in app.animation.animations.iter() {
                ui.group(|ui| {
                    ui.label(format!("{} [{}s, loops: {}]", name, animation.duration, animation.loops));
                    ui.add_space(5.0);

                    // Renderizar tracks de esta animación
                    for track in animation.tracks.iter() {
                        ui.group(|ui| {
                            ui.label(&track.name);
                            ui.add_space(5.0);
                            ui.label(format!("Keyframes: {}", track.keyframes.len()));
                            ui.add_space(5.0);

                            // Renderizar keyframes en la UI
                            for keyframe in track.keyframes.iter() {
                                ui.horizontal(|ui| {
                                    ui.label(format!("Frame: {:.0}", keyframe.time as u32));
                                    ui.label(format!("Value: {:.2}", keyframe.value));
                                    if ui.button("Remove").clicked() {
                                        keyframe_to_remove = Some((name.clone(), track.name.clone(), keyframe.time));
                                    }
                                });
                            }
                        });
                    }
                });
            }
        });

        if let Some((anim_name, track_name, time)) = keyframe_to_remove {
            app.animation.remove_keyframe(&anim_name, &track_name, time);
        }
    }
}

/// Keyframe Editor UI para gestión de keyframes
/// 
/// Conecta con: animation_2d
#[derive(Debug, Clone)]
pub struct KeyframeEditor {
    pub keyframes: Vec<Keyframe>,
    pub current_keyframe: Option<usize>,
}

impl Default for KeyframeEditor {
    fn default() -> Self {
        Self {
            keyframes: Vec::new(),
            current_keyframe: None,
        }
    }
}

impl KeyframeEditor {
    pub fn new() -> Self {
        Self::default()
    }
    
    pub fn ui(&self, ui: &mut egui::Ui) {
        ui.heading("Keyframe Editor");
        ui.add_space(10.0);
        ui.separator();
        ui.add_space(5.0);
        
        ui.group(|ui| {
            ui.label("Keyframes");
            ui.add_space(5.0);
            ui.label("No keyframes selected");
        });
    }
}

#[derive(Debug, Clone)]
pub struct Keyframe {
    pub frame: f64,
    pub value: serde_json::Value,
}

#[derive(Debug, Clone)]
pub struct TimelineDockable {
    pub current_frame: f64,
    pub duration: f64,
    pub fps: f32,
    pub events: Vec<TimelineEventData>,
    pub is_playing: bool,
    pub selected_events: Vec<Uuid>,
    pub current_track: Option<TrackId>,
}

#[derive(Debug, Clone)]
pub enum TimelineEvent {
    Spawn { entity_id: Uuid },
    Destroy { entity_id: Uuid },
    MoveTo { entity_id: Uuid, position: [f32; 2] },
    ChangeProperty { entity_id: Uuid, property: String, value: serde_json::Value },
    PlaySound { sound_name: String },
    TriggerDialogue { dialogue_id: Uuid },
    ChangeScene { scene_id: Uuid },
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TrackId(pub u32);

#[derive(Debug, Clone)]
pub struct TimelineEventData {
    pub id: Uuid,
    pub event: TimelineEvent,
    pub frame: f64,
}

impl TimelineDockable {
    pub fn new() -> Self {
        Self {
            current_frame: 0.0,
            duration: 10.0,
            fps: 60.0,
            events: Vec::new(),
            is_playing: false,
            selected_events: Vec::new(),
            current_track: None,
        }
    }

    pub fn start(&mut self) {
        self.is_playing = true;
    }

    pub fn stop(&mut self) {
        self.is_playing = false;
    }

    pub fn toggle_play(&mut self) {
        self.is_playing = !self.is_playing;
    }

    pub fn add_event(&mut self, event: TimelineEvent, frame: f64) -> Uuid {
        let id = Uuid::new_v4();
        self.events.push(TimelineEventData { id, event, frame });
        id
    }

    pub fn remove_event(&mut self, event_id: &Uuid) {
        self.events.retain(|e| e.id != *event_id);
    }

    pub fn get_events(&self) -> &Vec<TimelineEventData> {
        &self.events
    }

    pub fn get_selected_events(&self) -> &Vec<Uuid> {
        &self.selected_events
    }

    pub fn update(&mut self, delta: f64) {
        if self.is_playing {
            self.current_frame += delta * (self.fps as f64);
        }
    }

    pub fn get_current_time(&self) -> f64 {
        self.current_frame
    }

    pub fn set_duration(&mut self, duration: f64) {
        self.duration = duration;
    }

    pub fn add_spawn_event(&mut self, entity_id: Uuid) -> Uuid {
        self.add_event(TimelineEvent::Spawn { entity_id }, self.current_frame)
    }

    pub fn add_destroy_event(&mut self, entity_id: Uuid) -> Uuid {
        self.add_event(TimelineEvent::Destroy { entity_id }, self.current_frame)
    }

    pub fn add_move_event(&mut self, entity_id: Uuid, position: [f32; 2]) -> Uuid {
        self.add_event(TimelineEvent::MoveTo { entity_id, position }, self.current_frame)
    }

    pub fn add_property_event(&mut self, entity_id: Uuid, property: String, value: serde_json::Value) -> Uuid {
        self.add_event(TimelineEvent::ChangeProperty { entity_id, property, value }, self.current_frame)
    }

    pub fn add_sound_event(&mut self, sound_name: String) -> Uuid {
        self.add_event(TimelineEvent::PlaySound { sound_name }, self.current_frame)
    }

    pub fn add_dialogue_event(&mut self, dialogue_id: Uuid) -> Uuid {
        self.add_event(TimelineEvent::TriggerDialogue { dialogue_id }, self.current_frame)
    }

    pub fn add_scene_event(&mut self, scene_id: Uuid) -> Uuid {
        self.add_event(TimelineEvent::ChangeScene { scene_id }, self.current_frame)
    }

    pub fn ui(&self, ui: &mut egui::Ui) {
        ui.heading("Timeline");
        ui.add_space(10.0);
        ui.separator();
        ui.add_space(5.0);
        
        ui.group(|ui| {
            ui.label("Timeline");
            ui.add_space(5.0);
            ui.label("Frame: 0");
            ui.label("FPS: 60");
            ui.label("Duration: 10.0s");
        });
    }
}

impl Default for TimelineDockable {
    fn default() -> Self {
        Self::new()
    }
}


