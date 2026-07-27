//! # Forge SDK - Application Entry Point
//! 
//! This is the main entry point for the Forge SDK.
//! 
//! Run this to start the Forge game engine with:
//! ```bash
//! cargo run --bin forge
//! ```

use eframe::egui;
use eframe::NativeOptions;
use std::time::Instant;
use forge_runtime::resource::ResourceManager;
use forge_runtime::resource::ResourceType;
use forge_runtime::entities::Entity;
use forge_runtime::entities::Position2D;
use forge_runtime::entities::Velocity2D;
use forge_runtime::entities::Size2D;
use forge_runtime::entities::Color2D;

/// Configuración de la aplicación
#[derive(Debug, Clone)]
struct AppConfig {
    window_title: String,
    window_width: f32,
    window_height: f32,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            window_title: "Forge SDK - Game Engine".to_string(),
            window_width: 1280.0,
            window_height: 720.0,
        }
    }
}

/// Estado de la aplicación
struct ForgeApp {
    config: AppConfig,
    resources: ResourceManager,
    entities: Vec<Entity>,
    frame_count: u64,
    last_fps: Instant,
    fps: f32,
}

impl ForgeApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let config = AppConfig::default();
        
        // Crear Resource Manager
        let mut resources = ResourceManager::default();
        
        // Cargar ejemplo de sprite
        let sprite_json = r#"{
            "name": "hero",
            "width": 32,
            "height": 32,
            "texture_path": "sprites/hero.png",
            "frames": []
        }"#;
        
        if let Err(e) = resources.load_from_json("hero", sprite_json, ResourceType::Sprite) {
            println!("Warning: Failed to load hero sprite: {}", e);
        }
        
        // Crear entidades
        let mut entities = Vec::new();
        
        // Entidad 1: Jugador
        let mut player = Entity::new();
        player.add_component(Position2D::new(0.0, 0.0));
        player.add_component(Velocity2D::new(0.0, 0.0));
        player.add_component(Size2D::new(10.0, 10.0));
        player.add_component(Color2D::new(100.0, 100.0, 255.0));
        entities.push(player);
        
        // Entidad 2: Enemigo
        let mut enemy = Entity::new();
        enemy.add_component(Position2D::new(100.0, 50.0));
        enemy.add_component(Velocity2D::new(0.0, 0.0));
        enemy.add_component(Size2D::new(10.0, 10.0));
        enemy.add_component(Color2D::new(255.0, 100.0, 100.0));
        entities.push(enemy);
        
        Self {
            config,
            resources,
            entities,
            frame_count: 0,
            last_fps: Instant::now(),
            fps: 0.0,
        }
    }
    
    fn update(&mut self, _ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.frame_count += 1;
        
        // Calcular FPS
        if self.last_fps.elapsed().as_secs_f32() >= 1.0 {
            self.fps = self.frame_count as f32;
            self.frame_count = 0;
            self.last_fps = Instant::now();
        }
        
        // Actualizar entidades (simple physics)
        self.update_entities();
    }
    
    fn update_entities(&mut self) {
        // Simular movimiento
        for entity in &mut self.entities {
            if let Some(pos) = entity.get_component::<Position2D>() {
                if let Some(v) = entity.get_component::<Velocity2D>() {
                    let new_pos = forge_types::Vec2::add(&pos.position, v.velocity);
                    entity.set_component(Position2D::new(new_pos.x, new_pos.y));
                }
            }
        }
    }
    
    fn render(&self, ctx: &egui::Context, viewport: egui::Rect) {
        egui::CentralPanel::default().show(ctx, |ui| {
            self.render_background(ui, viewport);
            self.render_entities(ui, viewport);
            self.render_ui(ui);
        });
    }
    
    fn create_frame(&self) -> egui::Frame {
        egui::Frame::new()
            .fill(egui::Color32::TRANSPARENT)
            .stroke(egui::Stroke::new(1.0, egui::Color32::TRANSPARENT))
    }
    
    fn render_background(&self, ui: &mut egui::Ui, viewport: egui::Rect) {
        let rect = egui::Rect::from_min_size(egui::Pos2::new(0.0, 0.0), viewport.size());
        ui.painter().rect_filled(rect, 0.0, egui::Color32::from_rgb(30, 30, 30));
    }
    
    fn render_entities(&self, ui: &mut egui::Ui, viewport: egui::Rect) {
        let scale = viewport.width() / 800.0;
        
        for entity in &self.entities {
            if let Some(pos) = entity.get_component::<Position2D>() {
                if let Some(size) = entity.get_component::<Size2D>() {
                    if let Some(color) = entity.get_component::<Color2D>() {
                        let screen_pos = self.world_to_screen(pos.position, viewport);
                        let screen_size = egui::Vec2::new(
                            size.size.x * scale,
                            size.size.y * scale
                        );
                        
                        let rect = egui::Rect::from_center_size(
                            egui::Pos2::new(screen_pos.x, screen_pos.y),
                            screen_size,
                        );
                        
                        ui.painter().rect_filled(rect, 0.0, color.color);
                        ui.painter().rect_stroke(rect, 0.0, egui::Stroke::new(1.0, egui::Color32::WHITE), egui::StrokeKind::Inside);
                    }
                }
            }
        }
    }
    
    fn render_ui(&self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.label("Zoom:");
            if ui.button("Zoom In").clicked() {
                println!("Zoom In");
            }
            if ui.button("Zoom Out").clicked() {
                println!("Zoom Out");
            }
            
            ui.label(format!("FPS: {:.1}", self.fps));
            ui.label(format!("Entities: {}", self.entities.len()));
        });
        
        ui.separator();
        
        ui.label("Resources:");
        if ui.button("Load Sprite").clicked() {
            println!("Loading sprite...");
        }
        
        ui.label(format!("Cached Resources: {}", self.resources.len()));
    }
    
    fn world_to_screen(&self, world_pos: forge_types::Vec2, viewport: egui::Rect) -> egui::Vec2 {
        let center = viewport.center();
        egui::Vec2::new(
            center.x - world_pos.x * viewport.width() / 2.0,
            center.y - world_pos.y * viewport.height() / 2.0,
        )
    }
}

impl eframe::App for ForgeApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.render(ctx, egui::Rect::from_min_size(
            egui::Pos2::new(0.0, 0.0),
            egui::Vec2::new(self.config.window_width, self.config.window_height),
        ));
    }
}

fn main() -> eframe::Result<()> {
    let config = AppConfig::default();
    
    println!("Initializing Forge SDK...");
    println!("Window Title: {}", config.window_title);
    println!("Window Size: {}x{}", config.window_width, config.window_height);
    
    eframe::run_native(
        &config.window_title,
        NativeOptions {
            viewport: egui::ViewportBuilder::default()
                .with_inner_size([config.window_width, config.window_height])
                .with_resizable(true)
                .with_min_inner_size([800.0, 600.0]),
            ..NativeOptions::default()
        },
        Box::new(|cc| Ok(Box::new(ForgeApp::new(cc)) as Box<dyn eframe::App>)),
    )
}
