//! # Viewport API
//! 
//! Módulo para el viewport 2D y renderizado.

use eframe::egui;
use crate::{DragOperation, DropTarget};
use crate::forge_scene_stub;

/// Viewport 2D para renderizado
#[derive(Debug, Clone)]
pub struct Viewport {
    /// Zoom de la cámara
    pub camera_zoom: f32,
    /// Offset de la cámara
    pub camera_offset: (f32, f32),
    /// Mostrar grid
    pub show_grid: bool,
    /// Tamaño del grid
    pub grid_size: f32,
    /// Entidades seleccionadas
    pub selected_entities: Vec<usize>,
    /// Zoom in flag
    pub zoom_in: bool,
    /// Zoom out flag
    pub zoom_out: bool,
    /// Reset zoom flag
    pub zoom_reset: bool,
    /// Pan left flag
    pub pan_left: bool,
    /// Pan right flag
    pub pan_right: bool,
    /// Pan up flag
    pub pan_up: bool,
    /// Pan down flag
    pub pan_down: bool,
}

impl Default for Viewport {
    fn default() -> Self {
        Viewport::new()
    }
}

impl Viewport {
    /// Crea un nuevo viewport con valores por defecto
    pub fn new() -> Self {
        Self {
            camera_zoom: 1.0,
            camera_offset: (0.0, 0.0),
            show_grid: true,
            grid_size: 50.0,
            selected_entities: Vec::new(),
            zoom_in: false,
            zoom_out: false,
            zoom_reset: false,
            pan_left: false,
            pan_right: false,
            pan_up: false,
            pan_down: false,
        }
    }
    
    /// Renderiza el viewport en la UI
    /// 
    /// Conecta con: physics_2d, particle_system, animation_2d
    pub fn render(ui: &mut egui::Ui, app: &mut crate::ForgeEditorApp) {
        ui.heading("Viewport");
        ui.add_space(10.0);
        ui.separator();
        ui.add_space(5.0);
        
        ui.group(|ui| {
            ui.label("Viewport");
            ui.add_space(5.0);
            
            // Mostrar estadísticas de física - Conectadas con app.physics y app.particles
            let physics_blocks = app.physics.blocks.read().unwrap().len();
            let active_particles = app.particles.particle_count();
            let animations = app.animation.animations.len();
            
            ui.label(format!("Physics blocks: {}", physics_blocks));
            ui.label(format!("Active particles: {}", active_particles));
            ui.label(format!("Animations: {}", animations));
            
            ui.add_space(5.0);
            
            // Controles de física
            ui.group(|ui| {
                ui.label("Physics Controls");
                ui.horizontal(|ui| {
                    if ui.button("Add Block").clicked() {
                        let block = crate::physics_2d::PhysicsBlock {
                            id: "block_1".to_string(),
                            position: crate::physics_2d::Vector2::new(-100.0, -100.0),
                            size: crate::physics_2d::Vector2::new(200.0, 50.0),
                            mass: 10.0,
                            velocity: crate::physics_2d::Vector2::zero(),
                            friction: 0.5,
                            restitution: 0.7,
                            is_static: false,
                            collider_type: crate::physics_2d::ColliderType::Rectangle,
                        };
                        app.physics.add_block(block);
                    }
                });
                ui.add_space(5.0);
                ui.horizontal(|ui| {
                    if ui.button("Clear Blocks").clicked() {
                        let mut blocks = app.physics.blocks.write().unwrap();
                        blocks.clear();
                    }
                });
                ui.add_space(5.0);
                ui.horizontal(|ui| {
                    ui.label(format!("Gravity: {:.2}", app.physics.gravity.y));
                });
                ui.add_space(5.0);
                ui.horizontal(|ui| {
                    if ui.button("Toggle Gravity").clicked() {
                        app.physics.gravity.y = if app.physics.gravity.y == 0.0 { 9.8 } else { 0.0 };
                    }
                });
            });
            
            ui.add_space(5.0);
            
            // Controles de partículas
            ui.group(|ui| {
                ui.label("Particle System Controls");
                ui.horizontal(|ui| {
                    if ui.button("Emit Particles").clicked() {
                        let animation = crate::particle_system::SpriteAnimation {
                            frames: vec![],
                            fps: 30.0,
                            loop_count: -1,
                            start_frame: 0,
                            end_frame: 0,
                        };
                        let config = crate::particle_system::ParticleEmitterConfig {
                            position: crate::particle_system::Vector2::new(0.0, 0.0),
                            velocity_min: crate::particle_system::Vector2::new(-50.0, -100.0),
                            velocity_max: crate::particle_system::Vector2::new(50.0, 100.0),
                            size: crate::particle_system::Vector2::new(10.0, 10.0),
                            color: "red".to_string(),
                            sprite_path: "particle.png".to_string(),
                            animation,
                            lifetime: 3.0,
                            count: 20,
                        };
                        app.particles.emit(config);
                    }
                });
                ui.add_space(5.0);
                ui.horizontal(|ui| {
                    if ui.button("Clear Particles").clicked() {
                        app.particles.clear();
                    }
                });
                ui.add_space(5.0);
                ui.horizontal(|ui| {
                    ui.label(format!("FPS: {}", app.particles.dt * 60.0));
                });
            });
            
            ui.add_space(10.0);
            
            // Controles de zoom
            ui.horizontal(|ui| {
                if ui.button("Zoom In").clicked() {
                    app.viewport.zoom_in();
                }
                ui.add_space(5.0);
                if ui.button("Zoom Out").clicked() {
                    app.viewport.zoom_out();
                }
                ui.add_space(5.0);
                if ui.button("Reset Zoom").clicked() {
                    app.viewport.reset_zoom();
                }
            });
            
            ui.add_space(10.0);
            
            // Controles de pan
            ui.horizontal(|ui| {
                if ui.button("Pan Up").clicked() {
                    app.viewport.pan_up = !app.viewport.pan_up;
                }
                ui.add_space(5.0);
                if ui.button("Pan Down").clicked() {
                    app.viewport.pan_down = !app.viewport.pan_down;
                }
            });
            ui.horizontal(|ui| {
                if ui.button("Pan Left").clicked() {
                    app.viewport.pan_left = !app.viewport.pan_left;
                }
                ui.add_space(5.0);
                if ui.button("Pan Right").clicked() {
                    app.viewport.pan_right = !app.viewport.pan_right;
                }
            });
        });
    }
    
    /// Renderiza el viewport con soporte de drop
    pub fn ui_render(ui: &mut egui::Ui, app: &mut crate::ForgeEditorApp) {
        ui.heading("Viewport");
        ui.add_space(10.0);
        ui.separator();
        ui.add_space(5.0);
        
        ui.group(|ui| {
            ui.label("Viewport");
            ui.add_space(5.0);
            ui.label(format!("Zoom: x{}", app.viewport.camera_zoom));
            ui.label("Grid: enabled");
            
            // Zona de drop
            if matches!(app.drag_operation, Some(DragOperation::Asset { .. })) {
                let asset_name = app.dragged_asset.as_ref().map(|a| a.path.clone()).unwrap_or_else(|| "unknown".to_string());
                ui.label(format!("Drop {} here", asset_name));
                ui.add_space(5.0);
            }
        });
        
        // Actualizar viewport_rect antes de renderizar
        let rect = ui.available_rect_before_wrap();
        app.viewport_rect = rect;
        
        // Renderizar área de viewport
        let _response = ui.allocate_exact_size(egui::vec2(rect.width(), rect.height()), egui::Sense::click_and_drag());
        
        // Manejar drop en el viewport
        if app.drop_target == Some("Viewport".to_string()) {
            // Calcular posición del mouse relativa al viewport
            let viewport_rect = app.viewport_rect;
            if viewport_rect != egui::Rect::NOTHING {
                let mouse_pos = ui.input(|i| i.pointer.hover_pos());
                if let Some(pos) = mouse_pos {
                    // Calcular posición en coordenadas de la escena
                    let scene_x = (pos.x - viewport_rect.min.x - app.viewport.camera_offset.0) / app.viewport.camera_zoom;
                    let scene_y = (pos.y - viewport_rect.min.y - app.viewport.camera_offset.1) / app.viewport.camera_zoom;
                    
                    // Usar current_asset o dragged_asset
                    let asset = app.current_asset.clone().or_else(|| app.dragged_asset.clone());
                    if let Some(asset) = asset {
                        let sprite_name = asset.path.split('.').next().unwrap_or("Sprite").to_string();
                        
                        // Crear nodo Sprite en la posición especificada
                        let mut sprite_node = forge_scene_stub::NodeData {
                            id: uuid::Uuid::new_v4(),
                            name: sprite_name.clone(),
                            entity_type: forge_scene_stub::EntityType::Sprite,
                            parent_id: None,
                            transform: forge_scene_stub::Transform {
                                position: [scene_x, scene_y, 0.0],
                                rotation: 0.0,
                                scale: [1.0, 1.0, 0.0],
                            },
                            properties: std::collections::HashMap::new(),
                            components: vec![forge_scene_stub::ComponentData {
                                component_type: forge_scene_stub::ComponentType::Sprite,
                                data: serde_json::Value::Null,
                            }],
                        };
                        sprite_node.properties.insert("sprite_path".to_string(), asset.path.clone());
                        
                        app.scene_tree.tree.push(sprite_node.clone());
                        app.scene_tree.nodes.push(sprite_node);
                        
                        app.console.add_message(
                            crate::debugger::LogLevel::Info, 
                            &format!("Created Sprite node '{}' at ({}, {}) with texture '{}'", sprite_name, scene_x, scene_y, asset.path)
                        );
                    }
                }
            }
            
            app.drag_operation = None;
            app.dragged_asset = None;
            app.drop_target = None;
        }
    }

    /// Restablece el zoom a 1.0 y offset a (0, 0)
    pub fn reset_zoom(&mut self) {
        self.camera_zoom = 1.0;
        self.camera_offset = (0.0, 0.0);
    }
    
    /// Crear nodo Sprite en el viewport
    pub fn create_sprite_node(&mut self, app: &mut crate::ForgeEditorApp, x: f32, y: f32) {
        let asset = app.current_asset.clone().or_else(|| app.dragged_asset.clone());
        if let Some(asset) = asset {
            // Crear nodo Sprite en la escena
            self.create_sprite_at_position(x, y, asset.path.as_str(), app);
            
            // Limpiar drag operation
            app.drag_operation = None;
            app.dragged_asset = None;
            app.current_asset = None;
            app.drop_target = None;
        }
    }
    
    /// Crear nodo Sprite en la posición especificada
    fn create_sprite_at_position(&mut self, x: f32, y: f32, asset_name: &str, app: &mut crate::ForgeEditorApp) {
        // Crear nodo Sprite
        let sprite_name = asset_name.split('.').next().unwrap_or("Sprite").to_string();
        
        // Agregar nodo a la escena (esto requiere acceso a scene_tree)
        // Por ahora, solo registramos la acción
        app.console.add_message(crate::debugger::LogLevel::Info, &format!("Creating Sprite: {} at ({}, {})", sprite_name, x, y));
    }
    
    /// Manejar drop en el viewport
    pub fn handle_drop_in_viewport(&mut self, app: &mut crate::ForgeEditorApp) {
        if app.drop_target == Some("Viewport".to_string()) {
            self.create_sprite_node(app, self.camera_offset.0, self.camera_offset.1);
            app.drop_target = None;
        }
    }
    
    /// Aumenta el zoom
    pub fn zoom_in(&mut self) {
        self.camera_zoom *= 1.1;
    }

    /// Disminuye el zoom
    pub fn zoom_out(&mut self) {
        self.camera_zoom /= 1.1;
    }
}


