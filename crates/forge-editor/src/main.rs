//! forge-editor - IDE visual con runtime de renderizado

use forge_types::{Project, GameType};
use serde::{Deserialize, Serialize};
use egui_dock::{DockState, egui};
use std::fs;
use forge_runtime::Timeline;

mod dockables;
pub mod ui;

pub use forge_types::dialogue;
pub use forge_types::event_system;
pub use forge_types::timeline;
pub use forge_types::gdd;
pub use forge_types::audio;

pub mod types {
    pub use forge_types::dialogue;
    pub use forge_types::event_system;
    pub use forge_types::timeline;
    pub use forge_types::gdd;
    pub use forge_types::audio;
}

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1280.0, 720.0])
            .with_min_inner_size([800.0, 600.0])
            .with_title("Forge Editor 2D"),
        ..Default::default()
    };

    eframe::run_native("Forge Editor 2D", options, Box::new(|_cc| Ok(Box::new(EditorApp::new()))))
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum WizardStep {
    Welcome,
    ProjectName,
    GameType,
    Template,
    Complete,
}

struct EditorApp {
    wizard: WizardState,
    has_wizard: bool,
    project_path: Option<String>,
    dock_state: DockState<String>,
    explorer: dockables::ExplorerDockable,
    inspector: dockables::InspectorDockable,
    viewport: dockables::ViewportDockable,
    sequencer: dockables::SequencerDockable,
    event_forge: dockables::EventForgeDockable,
    audio: dockables::AudioDockable,
    runtime: Runtime,
    timeline: Timeline,
    event_system: forge_runtime::EventSystem,
    dialogue_system: forge_runtime::DialogueSystem,
    audio_manager: forge_runtime::AudioManager,
    entities: Vec<forge_runtime::Entity>,
}

struct Runtime {
    enabled: bool,
    frame_count: u32,
    last_frame_time: std::time::Instant,
}

impl Default for Runtime {
    fn default() -> Self {
        Self {
            enabled: false,
            frame_count: 0,
            last_frame_time: std::time::Instant::now(),
        }
    }
}

pub struct WizardState {
    step: WizardStep,
    project_name: String,
    game_type: GameType,
    selected_template: Option<String>,
}

impl Default for WizardState {
    fn default() -> Self {
        Self {
            step: WizardStep::Welcome,
            project_name: "New Project".to_string(),
            game_type: GameType::Platformer,
            selected_template: None,
        }
    }
}

impl WizardState {
    fn next_step(&mut self) {
        match self.step {
            WizardStep::Welcome => self.step = WizardStep::ProjectName,
            WizardStep::ProjectName => {
                if self.project_name.trim().is_empty() { return; }
                self.step = WizardStep::GameType;
            }
            WizardStep::GameType => self.step = WizardStep::Template,
            WizardStep::Template => self.step = WizardStep::Complete,
            WizardStep::Complete => {}
        }
    }

    fn prev_step(&mut self) {
        match self.step {
            WizardStep::ProjectName => self.step = WizardStep::Welcome,
            WizardStep::GameType => self.step = WizardStep::ProjectName,
            WizardStep::Template => self.step = WizardStep::GameType,
            WizardStep::Complete => self.step = WizardStep::Template,
            WizardStep::Welcome => {}
        }
    }

    fn create_project(&self) -> Option<Project> {
        Some(Project::default())
    }

    fn save_project(&self, project: &Project) -> Result<(), Box<dyn std::error::Error>> {
        let project_path = format!("projects/{}", &self.project_name);
        let toml_path = format!("{}.toml", project_path);
        
        let content = toml::to_string(project)?;
        fs::write(&toml_path, content)?;
        
        Ok(())
    }

    fn create_new_project(&mut self) {
        let mut project = Project::default();
        project.name = self.project_name.clone();
        project.game_type = self.game_type;
        
        self.create_project();
        let _ = self.save_project(&project);
    }
}

impl EditorApp {
    fn new() -> Self {
        let (explorer, inspector, viewport, sequencer, event_forge, audio) = dockables::create_dockables();
        
        let dock_state = DockState::new(vec![
            "Explorer".to_string(),
            "Inspector".to_string(),
            "Viewport".to_string(),
            "Sequencer".to_string(),
            "Event Forge".to_string(),
            "Audio".to_string(),
        ]);

        let mut entities = Vec::new();
        // Crear entidad de ejemplo
        entities.push(forge_runtime::Entity {
            id: 1,
            position: forge_runtime::Position2D { x: 100.0, y: 100.0 },
            velocity: forge_runtime::Velocity2D { x: 0.0, y: 0.0 },
        });

        let timeline = Timeline::new();
        let event_system = forge_runtime::EventSystem::new();
        let dialogue_system = forge_runtime::DialogueSystem::new();
        let audio_manager = forge_runtime::AudioManager::default();

        Self {
            wizard: WizardState::default(),
            has_wizard: false,
            project_path: None,
            dock_state,
            explorer,
            inspector,
            viewport,
            sequencer,
            event_forge,
            audio,
            runtime: Runtime::default(),
            timeline,
            event_system,
            dialogue_system,
            audio_manager,
            entities,
        }
    }

    fn create_project(&mut self, project_name: String, _game_type: GameType) {
        let project_path = format!("projects/{}", project_name);
        
        self.explorer.set_project_path(project_path.clone());
    }

    fn show_wizard(&mut self, ctx: &egui::Context) {
        ui::show_wizard(ctx, &mut self.wizard);
    }

    fn run_game(&mut self) {
        self.runtime.enabled = true;
        self.timeline.reset();
        println!("Game runtime started!");
    }

    fn update_runtime(&mut self, delta_time: std::time::Duration) {
        if !self.runtime.enabled {
            return;
        }

        self.runtime.frame_count += 1;
        self.timeline.current_frame = self.runtime.frame_count;
        self.runtime.last_frame_time = std::time::Instant::now();

        // Ejecutar eventos de la timeline
        for event in self.timeline.get_event_at_frame(self.runtime.frame_count) {
            println!("Timeline event: {} at frame {} for entity {}", event.action, event.frame, event.entity_id);
        }

        // Actualizar física básica
        for entity in &mut self.entities {
            entity.velocity.x *= 0.99; // Fricción
            entity.position.x += entity.velocity.x * delta_time.as_secs_f32();
            entity.position.y += entity.velocity.y * delta_time.as_secs_f32();
        }
    }
}

impl eframe::App for EditorApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let now = std::time::Instant::now();
        let delta_time = now.duration_since(self.runtime.last_frame_time);
        self.runtime.last_frame_time = now;

        if self.has_wizard {
            ui::show_wizard(ctx, &mut self.wizard);
            return;
        }

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.label("Forge Editor 2D");
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if ui.button("Create New Project").clicked() {
                    self.has_wizard = true;
                }
                if ui.button("Run Game").clicked() {
                    self.run_game();
                }
                if ui.button("Pause").clicked() {
                    self.runtime.enabled = !self.runtime.enabled;
                }
            });
        });

        egui::SidePanel::left("left_panel")
            .min_width(200.0)
            .resizable(true)
            .show(ctx, |ui| {
                ui.heading("Explorer");
                ui.label(egui::RichText::new(&format!("Entities: {}", self.entities.len())));
                ui.separator();
                if let Some(dir) = &self.explorer.current_dir {
                    ui.label(&format!("Path: {:?}", dir.display()));
                }
                ui.separator();
                ui.label("Files:");
                for file in self.explorer.get_files().iter() {
                    ui.label(&format!("  - {}", file.display()));
                }
            });

        egui::SidePanel::right("right_panel")
            .min_width(250.0)
            .resizable(true)
            .show(ctx, |ui| {
                ui.heading("Inspector");
                ui.separator();
                ui.label("Entity Properties");
                ui.separator();
                if !self.entities.is_empty() {
                    for (i, entity) in self.entities.iter().enumerate() {
                        if i == 0 {
                            ui.label(&format!("ID: {}", entity.id));
                            ui.separator();
                            ui.label("Position:");
                            ui.horizontal(|ui| {
                                ui.label("X:");
                                ui.label(format!("{:.1}", entity.position.x));
                            });
                            ui.horizontal(|ui| {
                                ui.label("Y:");
                                ui.label(format!("{:.1}", entity.position.y));
                            });
                            ui.separator();
                            ui.label("Velocity:");
                            ui.horizontal(|ui| {
                                ui.label("X:");
                                ui.label(format!("{:.1}", entity.velocity.x));
                            });
                            ui.horizontal(|ui| {
                                ui.label("Y:");
                                ui.label(format!("{:.1}", entity.velocity.y));
                            });
                            ui.separator();
                            if ui.button("Delete Entity").clicked() {
                                self.entities.remove(i);
                            }
                            break;
                        }
                    }
                } else {
                    ui.label("No entity selected");
                }
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            if self.runtime.enabled {
                ui.heading(&format!("Game Preview - FPS: {}", self.runtime.frame_count));
                ui.horizontal(|ui| {
                    for entity in &self.entities {
                        let color = egui::Color32::from_rgb(
                            100,
                            150,
                            200,
                        );
                        ui.painter().line_segment(
                            [
                                egui::pos2(entity.position.x, entity.position.y),
                                egui::pos2(entity.position.x + 20.0, entity.position.y + 20.0),
                            ],
                            egui::Stroke::new(2.0, color),
                        );
                    }
                });
                ui.add(egui::Label::new(" ").sense(egui::Sense::click()));
            } else {
                ui.heading("Viewport");
                ui.label("Game Preview (wgpu)");
                ui.add(egui::Label::new(" ").sense(egui::Sense::click()));
            }
        });

        egui::TopBottomPanel::bottom("bottom_panels")
            .min_height(200.0)
            .resizable(true)
            .show(ctx, |ui| {
                ui.heading("Timeline & Audio & Event Forge");
                ui.separator();
                ui.horizontal(|ui| {
                    ui.label("Timeline Controls:");
                    if ui.button("Play").clicked() {
                        self.runtime.enabled = true;
                    }
                    if ui.button("Pause").clicked() {
                        self.runtime.enabled = false;
                    }
                    if ui.button("Stop").clicked() {
                        self.runtime.enabled = false;
                        self.runtime.frame_count = 0;
                    }
                    ui.add(egui::Label::new(format!("Frame: {}", self.runtime.frame_count)).sense(egui::Sense::click()));
                });
                ui.separator();
                ui.horizontal(|ui| {
                    ui.label("Audio:");
                    if ui.button("Play BGM").clicked() {
                        println!("Playing BGM - Audio Manager integrated");
                    }
                    if ui.button("Play SFX").clicked() {
                        println!("Playing SFX - Audio Manager integrated");
                    }
                    ui.label("Audio Manager integrated");
                });
                ui.separator();
                ui.horizontal(|ui| {
                    ui.label("Sequencer:");
                    if ui.button("Next Frame").clicked() {
                        self.sequencer.next_frame();
                    }
                    if ui.button("Prev Frame").clicked() {
                        self.sequencer.prev_frame();
                    }
                    ui.label(format!("Current: {}", self.sequencer.current_frame));
                });
                ui.separator();
                ui.horizontal(|ui| {
                    ui.label("Event Forge:");
                    if ui.button("Add Spawn Event").clicked() {
                        let id = self.event_system.add_node(forge_runtime::EventType::Spawn);
                        self.event_forge.selected_event = Some(id);
                    }
                    if ui.button("Add MoveTo Event").clicked() {
                        let id = self.event_system.add_node(forge_runtime::EventType::MoveTo);
                        self.event_forge.selected_event = Some(id);
                    }
                    if ui.button("Add Dialogue Event").clicked() {
                        let id = self.event_system.add_node(forge_runtime::EventType::TriggerDialogue);
                        self.event_forge.selected_event = Some(id);
                    }
                    if let Some(id) = self.event_forge.selected_event {
                        if let Some(node) = self.event_system.get_node(id) {
                            ui.label(format!("Selected: {}", node.event_type));
                        }
                    }
                });

                // Diálogo
                ui.separator();
                ui.label("Dialogue System");
                ui.horizontal(|ui| {
                    if ui.button("New Dialogue").clicked() {
                        let id = self.dialogue_system.dialogues.len() as u64;
                        self.dialogue_system.add_dialogue(id, format!("Dialogue {}", id));
                    }
                    if let Some(dialogue) = self.dialogue_system.get_dialogue(0) {
                        if dialogue.is_active {
                            ui.label(format!("Active: {}", dialogue.title));
                            if let Some(line) = dialogue.get_current_line() {
                                ui.label(&line.text);
                            }
                        }
                    }
                });
            });
    }
}
