//! # Viewport API
//! 
//! Módulo para el viewport 2D y renderizado.

use eframe::egui;
use serde_json;

/// Herramientas de edición en el Viewport
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum EditTool {
    Translate,
    Rotate,
    Scale,
}

/// Tipos de Gizmos activos de arrastre
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GizmoType {
    Translate,
    Rotate,
    Scale,
}

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
    /// Pan left flag
    pub pan_left: bool,
    /// Pan right flag
    pub pan_right: bool,
    /// Pan up flag
    pub pan_up: bool,
    /// Pan down flag
    pub pan_down: bool,
    /// ID del nodo que se está arrastrando en la escena
    pub drag_node_id: Option<uuid::Uuid>,
    /// Posición inicial al comenzar el arrastre
    pub drag_start_pos: Option<[f32; 3]>,
    /// Herramienta de edición activa
    pub active_tool: EditTool,
    /// Gizmo que se está arrastrando actualmente
    pub drag_gizmo: Option<GizmoType>,
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
            pan_left: false,
            pan_right: false,
            pan_up: false,
            pan_down: false,
            drag_node_id: None,
            drag_start_pos: None,
            active_tool: EditTool::Translate,
            drag_gizmo: None,
        }
    }

    pub fn zoom_in(&mut self) {
        self.camera_zoom = (self.camera_zoom * 1.1).clamp(0.1, 10.0);
    }
    
    pub fn zoom_out(&mut self) {
        self.camera_zoom = (self.camera_zoom / 1.1).clamp(0.1, 10.0);
    }
    
    pub fn reset_zoom(&mut self) {
        self.camera_zoom = 1.0;
        self.camera_offset = (0.0, 0.0);
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
    
    /// Renderiza el viewport con soporte de drop, rejilla, capas inteligentes y sprites interactivos
    pub fn ui_render(ui: &mut egui::Ui, app: &mut crate::ForgeEditorApp) {
        ui.heading("Viewport");
        ui.add_space(5.0);
        ui.separator();
        ui.add_space(5.0);
        
        ui.horizontal(|ui| {
            ui.label(format!("Zoom: x{:.2}", app.viewport.camera_zoom));
            ui.add_space(10.0);
            ui.checkbox(&mut app.viewport.show_grid, "Mostrar Rejilla");
            ui.add_space(10.0);
            ui.label("Grid Size:");
            ui.add(egui::DragValue::new(&mut app.viewport.grid_size).range(10.0..=200.0));
        });
        ui.add_space(5.0);

        // Selector visual de Capa Activa
        ui.horizontal(|ui| {
            ui.label("Capa Activa:");
            egui::ComboBox::from_id_salt("active_layer_select")
                .selected_text(match app.active_layer {
                    1 => "1: Fondo (Background)",
                    2 => "2: Suelo/Sólidos (Ground - Auto Collider)",
                    3 => "3: Entidades (Entities - Auto Collider+Behavior)",
                    4 => "4: Decoración (Foreground)",
                    _ => "Desconocida",
                })
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut app.active_layer, 1, "1: Fondo (Background)");
                    ui.selectable_value(&mut app.active_layer, 2, "2: Suelo/Sólidos (Ground - Auto Collider)");
                    ui.selectable_value(&mut app.active_layer, 3, "3: Entidades (Entities - Auto Collider+Behavior)");
                    ui.selectable_value(&mut app.active_layer, 4, "4: Decoración (Foreground)");
                });
        });
        ui.add_space(5.0);

        // Capturar atajos de teclado para herramientas de edición (W, E, R)
        if ui.input(|i| i.key_pressed(egui::Key::W)) {
            app.viewport.active_tool = EditTool::Translate;
            app.console.add_message(crate::debugger::LogLevel::Info, "Herramienta activa: Traslación (W)");
        }
        if ui.input(|i| i.key_pressed(egui::Key::E)) {
            app.viewport.active_tool = EditTool::Rotate;
            app.console.add_message(crate::debugger::LogLevel::Info, "Herramienta activa: Rotación (E)");
        }
        if ui.input(|i| i.key_pressed(egui::Key::R)) {
            app.viewport.active_tool = EditTool::Scale;
            app.console.add_message(crate::debugger::LogLevel::Info, "Herramienta activa: Escalado (R)");
        }

        // Selector visual de herramientas W/E/R
        ui.horizontal(|ui| {
            ui.label("Herramienta:");
            ui.selectable_value(&mut app.viewport.active_tool, EditTool::Translate, " W: ⬈ Mover");
            ui.selectable_value(&mut app.viewport.active_tool, EditTool::Rotate, " E: ⟲ Rotar");
            ui.selectable_value(&mut app.viewport.active_tool, EditTool::Scale, " R: ⤢ Escalar");
        });
        ui.add_space(5.0);

        let rect = ui.available_rect_before_wrap();
        app.viewport_rect = rect;
        
        let (response, painter) = ui.allocate_painter(
            egui::vec2(rect.width(), rect.height()),
            egui::Sense::click_and_drag(),
        );
        
        // 1. Panning & Zooming con el ratón
        if response.dragged_by(egui::PointerButton::Middle) || response.dragged_by(egui::PointerButton::Secondary) {
            let delta = response.drag_delta();
            app.viewport.camera_offset.0 += delta.x;
            app.viewport.camera_offset.1 += delta.y;
        }
        
        if response.hovered() {
            let scroll = ui.input(|i| i.raw_scroll_delta.y);
            if scroll != 0.0 {
                let zoom_factor = if scroll > 0.0 { 1.1 } else { 1.0 / 1.1 };
                app.viewport.camera_zoom = (app.viewport.camera_zoom * zoom_factor).clamp(0.1, 10.0);
            }
        }
        
        // 2. Definir límites de la pantalla virtual retro (960x540)
        let zoom = app.viewport.camera_zoom;
        let offset = app.viewport.camera_offset;
        let virtual_center = rect.center() + egui::vec2(offset.0, offset.1);
        let screen_size = egui::vec2(960.0 * zoom, 540.0 * zoom);
        let screen_rect = egui::Rect::from_center_size(virtual_center, screen_size);
        
        // Dibujar el fondo negro de la pantalla virtual
        painter.rect_filled(screen_rect, 0.0, egui::Color32::from_rgb(15, 15, 15));
        
        // Dibujar líneas de rejilla
        if app.viewport.show_grid {
            let grid_size = app.viewport.grid_size * zoom;
            let stroke = egui::Stroke::new(1.0_f32, egui::Color32::from_rgba_unmultiplied(45, 45, 45, 140));
            
            // Líneas verticales
            let mut x = virtual_center.x;
            while x < screen_rect.max.x {
                painter.line_segment([egui::pos2(x, screen_rect.min.y), egui::pos2(x, screen_rect.max.y)], stroke);
                x += grid_size;
            }
            let mut x = virtual_center.x - grid_size;
            while x > screen_rect.min.x {
                painter.line_segment([egui::pos2(x, screen_rect.min.y), egui::pos2(x, screen_rect.max.y)], stroke);
                x -= grid_size;
            }
            
            // Líneas horizontales
            let mut y = virtual_center.y;
            while y < screen_rect.max.y {
                painter.line_segment([egui::pos2(screen_rect.min.x, y), egui::pos2(screen_rect.max.x, y)], stroke);
                y += grid_size;
            }
            let mut y = virtual_center.y - grid_size;
            while y > screen_rect.min.y {
                painter.line_segment([egui::pos2(screen_rect.min.x, y), egui::pos2(screen_rect.max.x, y)], stroke);
                y -= grid_size;
            }
        }
        
        // Dibujar ejes cartesianos centrales
        let axis_stroke_x = egui::Stroke::new(1.5_f32, egui::Color32::from_rgba_unmultiplied(220, 60, 60, 160)); // Rojo para X
        let axis_stroke_y = egui::Stroke::new(1.5_f32, egui::Color32::from_rgba_unmultiplied(60, 220, 60, 160)); // Verde para Y
        if screen_rect.min.y <= virtual_center.y && virtual_center.y <= screen_rect.max.y {
            painter.line_segment([egui::pos2(screen_rect.min.x, virtual_center.y), egui::pos2(screen_rect.max.x, virtual_center.y)], axis_stroke_x);
        }
        if screen_rect.min.x <= virtual_center.x && virtual_center.x <= screen_rect.max.x {
            painter.line_segment([egui::pos2(virtual_center.x, screen_rect.min.y), egui::pos2(virtual_center.x, screen_rect.max.y)], axis_stroke_y);
        }
        
        // Dibujar borde de delimitación (rojo retro)
        painter.rect_stroke(screen_rect, 0.0, egui::Stroke::new(2.0_f32, egui::Color32::from_rgb(220, 50, 50)), egui::StrokeKind::Inside);
        
        // 3. Obtener y ordenar Sprites por Capa (Z-sorting)
        let mut sprites_to_draw = Vec::new();
        let nodes: Vec<_> = app.scene_tree.nodes.values().cloned().collect();
        for node in &nodes {
            if node.entity_type == ::forge_scene::EntityType::Sprite || node.properties.contains_key("sprite_path") {
                let sprite_path = node.properties.get("sprite_path")
                    .and_then(|v| v.as_str())
                    .map(|s| s.to_string());
                if let Some(path) = sprite_path {
                    let pos = node.transform.transform.position;
                    let scale = node.transform.transform.scale;
                    
                    let sprite_center = virtual_center + egui::vec2(pos[0] * zoom, pos[1] * zoom);
                    let base_size = 64.0;
                    let size = egui::vec2(base_size * scale[0] * zoom, base_size * scale[1] * zoom);
                    let sprite_rect = egui::Rect::from_center_size(sprite_center, size);
                    
                    let layer = node.properties.get("layer")
                        .and_then(|v| v.as_u64())
                        .unwrap_or(2) as u32;
                    
                    sprites_to_draw.push((layer, node.id, node.name.clone(), path, sprite_rect));
                }
            }
        }
        
        // Ordenar de forma que los de capas menores (ej. 1: Fondo) se pinten primero
        sprites_to_draw.sort_by_key(|(layer, _, _, _, _)| *layer);
        
        let mouse_clicked = response.clicked_by(egui::PointerButton::Primary);
        let pointer_pos = ui.input(|i| i.pointer.interact_pos());
        let mut clicked_node = None;
        let mut clicked_gizmo = None;
        
        // 1. Hit-test para Gizmos del nodo activo primero
        if let Some(active_id) = app.active_node_id {
            if let Some(node) = app.scene_tree.nodes.get(&active_id) {
                if let Some(mpos) = pointer_pos {
                    if mouse_clicked {
                        let pos = node.transform.transform.position;
                        let scale = node.transform.transform.scale;
                        let col_center = virtual_center + egui::vec2(pos[0] * zoom, pos[1] * zoom);
                        let col_size = egui::vec2(64.0 * scale[0] * zoom, 64.0 * scale[1] * zoom);
                        
                        match app.viewport.active_tool {
                            EditTool::Rotate => {
                                let rot_handle = col_center + egui::vec2(0.0, -col_size.y / 2.0 - 20.0 * zoom);
                                let rot_handle_rect = egui::Rect::from_center_size(rot_handle, egui::vec2(16.0 * zoom, 16.0 * zoom));
                                if rot_handle_rect.contains(mpos) {
                                    clicked_node = Some(active_id);
                                    clicked_gizmo = Some(GizmoType::Rotate);
                                }
                            }
                            EditTool::Scale => {
                                let scale_handle = col_center + egui::vec2(col_size.x / 2.0 + 10.0 * zoom, col_size.y / 2.0 + 10.0 * zoom);
                                let scale_handle_rect = egui::Rect::from_center_size(scale_handle, egui::vec2(16.0 * zoom, 16.0 * zoom));
                                if scale_handle_rect.contains(mpos) {
                                    clicked_node = Some(active_id);
                                    clicked_gizmo = Some(GizmoType::Scale);
                                }
                            }
                            _ => {}
                        }
                    }
                }
            }
        }

        // 2. Dibujar Sprites
        for (_layer, id, name, path, sprite_rect) in &sprites_to_draw {
            let uri = get_file_uri(path);
            ui.put(*sprite_rect, egui::Image::new(&uri));
            
            // Capturar click para selección si no se hizo click en un gizmo
            if clicked_gizmo.is_none() && mouse_clicked {
                if let Some(mpos) = pointer_pos {
                    if sprite_rect.contains(mpos) {
                        clicked_node = Some(*id);
                        clicked_gizmo = Some(GizmoType::Translate);
                    }
                }
            }
            
            // Dibujar contorno de selección activo (amarillo) y gizmos
            if app.active_node_id == Some(*id) {
                painter.rect_stroke(*sprite_rect, 0.0, egui::Stroke::new(2.0_f32, egui::Color32::from_rgb(255, 215, 0)), egui::StrokeKind::Inside);
                
                // Mostrar nombre del sprite
                let label_pos = sprite_rect.left_top() - egui::vec2(0.0, 14.0 * zoom);
                painter.text(
                    label_pos,
                    egui::Align2::LEFT_TOP,
                    name,
                    egui::FontId::proportional(12.0 * zoom),
                    egui::Color32::from_rgb(255, 215, 0)
                );

                // Dibujar Gizmos interactivos según la herramienta activa
                match app.viewport.active_tool {
                    EditTool::Translate => {
                        let center = sprite_rect.center();
                        painter.line_segment([center, center + egui::vec2(30.0 * zoom, 0.0)], egui::Stroke::new(2.0 * zoom, egui::Color32::from_rgb(220, 50, 50)));
                        painter.line_segment([center, center + egui::vec2(0.0, -30.0 * zoom)], egui::Stroke::new(2.0 * zoom, egui::Color32::from_rgb(50, 220, 50)));
                    }
                    EditTool::Rotate => {
                        let center = sprite_rect.center();
                        let size = sprite_rect.size();
                        let rot_handle = center + egui::vec2(0.0, -size.y / 2.0 - 20.0 * zoom);
                        
                        painter.line_segment([center, rot_handle], egui::Stroke::new(1.5 * zoom, egui::Color32::from_rgb(0, 255, 255)));
                        painter.circle_filled(rot_handle, 7.0 * zoom, egui::Color32::from_rgb(0, 255, 255));
                        painter.circle_stroke(rot_handle, 7.0 * zoom, egui::Stroke::new(1.0_f32, egui::Color32::WHITE));
                        painter.circle_stroke(center, size.x / 2.0 + 10.0 * zoom, egui::Stroke::new(1.0_f32, egui::Color32::from_rgba_unmultiplied(0, 255, 255, 100)));
                    }
                    EditTool::Scale => {
                        let center = sprite_rect.center();
                        let size = sprite_rect.size();
                        let scale_handle = center + egui::vec2(size.x / 2.0 + 10.0 * zoom, size.y / 2.0 + 10.0 * zoom);
                        
                        painter.line_segment([center + egui::vec2(size.x / 2.0, size.y / 2.0), scale_handle], egui::Stroke::new(1.5 * zoom, egui::Color32::from_rgb(255, 215, 0)));
                        let square_rect = egui::Rect::from_center_size(scale_handle, egui::vec2(12.0 * zoom, 12.0 * zoom));
                        painter.rect_filled(square_rect, 0.0, egui::Color32::from_rgb(255, 215, 0));
                        painter.rect_stroke(square_rect, 0.0, egui::Stroke::new(1.0_f32, egui::Color32::WHITE), egui::StrokeKind::Outside);
                    }
                }
            }
        }
        
        // 3.8. Dibujar contornos de colisiones (Gizmos de Físicas)
        for node in &nodes {
            let has_collider = node.components.iter().any(|c| matches!(c.component_type, ::forge_scene::ComponentType::Collider));
            let layer = node.properties.get("layer").and_then(|v| v.as_u64()).unwrap_or(2) as u32;
            
            if has_collider || layer == 2 {
                let pos = node.transform.transform.position;
                let scale = node.transform.transform.scale;
                
                let col_center = virtual_center + egui::vec2(pos[0] * zoom, pos[1] * zoom);
                let col_size = egui::vec2(64.0 * scale[0] * zoom, 64.0 * scale[1] * zoom);
                let col_rect = egui::Rect::from_center_size(col_center, col_size);
                
                let color = if app.active_node_id == Some(node.id) {
                    egui::Color32::from_rgba_unmultiplied(255, 215, 0, 200) // Amarillo (Seleccionado)
                } else if layer == 2 {
                    egui::Color32::from_rgba_unmultiplied(220, 50, 50, 160) // Rojo translúcido (Suelo Estático)
                } else {
                    egui::Color32::from_rgba_unmultiplied(50, 220, 50, 160) // Verde translúcido (Dinámico/Actor)
                };
                
                painter.rect_stroke(col_rect, 0.0, egui::Stroke::new(1.5_f32, color), egui::StrokeKind::Outside);
            }
        }
        
        // Manejar lógica de selección y arrastre
        if let Some(id) = clicked_node {
            app.active_node_id = Some(id);
            app.viewport.drag_node_id = Some(id);
            app.viewport.drag_gizmo = clicked_gizmo;
            if let Some(node) = app.scene_tree.nodes.get(&id) {
                app.viewport.drag_start_pos = Some(node.transform.transform.position);
            }
        } else if mouse_clicked {
            if let Some(mpos) = pointer_pos {
                if screen_rect.contains(mpos) {
                    let mut hit = false;
                    for (_, _, _, _, sprite_rect) in &sprites_to_draw {
                        if sprite_rect.contains(mpos) {
                            hit = true;
                            break;
                        }
                    }
                    if !hit {
                        app.active_node_id = None;
                        app.viewport.drag_node_id = None;
                        app.viewport.drag_start_pos = None;
                        app.viewport.drag_gizmo = None;
                    }
                }
            }
        }
        
        // Realizar traslación / rotación / escalado durante el arrastre
        if let Some(dragged_id) = app.viewport.drag_node_id {
            if response.dragged_by(egui::PointerButton::Primary) {
                if let Some(mpos) = pointer_pos {
                    if let Some(node) = app.scene_tree.nodes.get(&dragged_id) {
                        let mut updated_node = node.as_ref().clone();
                        let gizmo_type = app.viewport.drag_gizmo.unwrap_or(GizmoType::Translate);
                        
                        match gizmo_type {
                            GizmoType::Translate => {
                                let new_scene_x = (mpos.x - virtual_center.x) / zoom;
                                let new_scene_y = (mpos.y - virtual_center.y) / zoom;
                                updated_node.transform.transform.position = [new_scene_x, new_scene_y, 0.0];
                            }
                            GizmoType::Rotate => {
                                let pos = node.transform.transform.position;
                                let col_center = virtual_center + egui::vec2(pos[0] * zoom, pos[1] * zoom);
                                let diff = mpos - col_center;
                                // Calcular ángulo en radianes respecto a la vertical
                                let angle = diff.y.atan2(diff.x) + std::f32::consts::FRAC_PI_2;
                                updated_node.transform.transform.rotation = angle;
                            }
                            GizmoType::Scale => {
                                let pos = node.transform.transform.position;
                                let col_center = virtual_center + egui::vec2(pos[0] * zoom, pos[1] * zoom);
                                let base_dist = egui::vec2(64.0 / 2.0, 64.0 / 2.0).length();
                                let current_dist = (mpos - col_center).length();
                                let new_scale = (current_dist / (base_dist * zoom)).max(0.1);
                                updated_node.transform.transform.scale = [new_scale, new_scale, 1.0];
                            }
                        }
                        
                        let arc_node = std::sync::Arc::new(updated_node);
                        app.scene_tree.nodes.insert(dragged_id, arc_node.clone());
                        if app.scene_tree.root.as_ref().map(|n| n.id) == Some(dragged_id) {
                            app.scene_tree.root = Some(arc_node.clone());
                        }
                        if let Some(pos) = app.scene_tree.active_nodes.iter().position(|n| n.id == dragged_id) {
                            app.scene_tree.active_nodes[pos] = arc_node;
                        }
                    }
                }
            } else if response.drag_stopped() {
                app.viewport.drag_node_id = None;
                app.viewport.drag_start_pos = None;
                app.viewport.drag_gizmo = None;
            }
        }
        
        // 3.9. Controles de Teclado WASD / Impulsos Físicos en Play Mode
        if app.play_mode {
            if let Some(active_id) = app.active_node_id {
                let mut blocks = app.physics.blocks.write().unwrap();
                if let Some(block) = blocks.get_mut(&active_id.to_string()) {
                    let speed = 200.0;
                    
                    // Controles horizontales (A/D o Flechas Izquierda/Derecha)
                    if ui.input(|i| i.key_down(egui::Key::A) || i.key_down(egui::Key::ArrowLeft)) {
                        block.velocity.x = -speed;
                    } else if ui.input(|i| i.key_down(egui::Key::D) || i.key_down(egui::Key::ArrowRight)) {
                        block.velocity.x = speed;
                    } else {
                        block.velocity.x *= 0.8; // Fricción
                    }
                    
                    // Controles de Salto (W o Espacio o Flecha Arriba)
                    if ui.input(|i| i.key_pressed(egui::Key::W) || i.key_pressed(egui::Key::Space) || i.key_pressed(egui::Key::ArrowUp)) {
                        block.velocity.y = -300.0;
                    }
                }
            }
        }
        
        // 4. Recepción de drop (con soporte de Prefabs y Auto-Físicas)
        if app.drop_target == Some("Viewport".to_string()) {
            if let Some(mpos) = pointer_pos {
                let scene_x = (mpos.x - virtual_center.x) / zoom;
                let scene_y = (mpos.y - virtual_center.y) / zoom;
                
                let asset = app.current_asset.clone().or_else(|| app.dragged_asset.clone());
                if let Some(asset) = asset {
                    if asset.path.ends_with(".prefab") {
                        // Instanciación de Prefab
                        if let Ok(content) = std::fs::read_to_string(&asset.path) {
                            if let Ok(mut prefab_node) = serde_json::from_str::<::forge_scene::NodeData>(&content) {
                                prefab_node.id = uuid::Uuid::new_v4();
                                prefab_node.transform.transform.position = [scene_x, scene_y, 0.0];
                                
                                let new_id = app.scene_tree.add_node(std::sync::Arc::new(prefab_node));
                                app.active_node_id = Some(new_id);
                                
                                app.console.add_message(
                                    crate::debugger::LogLevel::Info,
                                    &format!("Instantiated Prefab from '{}' at ({:.1}, {:.1})", asset.path, scene_x, scene_y)
                                );
                            } else {
                                app.console.add_message(
                                    crate::debugger::LogLevel::Error,
                                    &format!("Failed to parse Prefab JSON from '{}'", asset.path)
                                );
                            }
                        } else {
                            app.console.add_message(
                                crate::debugger::LogLevel::Error,
                                &format!("Failed to read Prefab file at '{}'", asset.path)
                            );
                        }
                    } else {
                        // Creación normal de Sprite con físicas por Capa inteligente
                        let sprite_name = asset.path.split('.').next().unwrap_or("Sprite").to_string();
                        let mut sprite_node = ::forge_scene::NodeData::new(&sprite_name, ::forge_scene::EntityType::Sprite);
                        sprite_node.transform.transform.position = [scene_x, scene_y, 0.0];
                        sprite_node.properties.insert("sprite_path".to_string(), serde_json::Value::String(asset.path.clone()));
                        sprite_node.properties.insert("layer".to_string(), serde_json::Value::Number(serde_json::Number::from(app.active_layer)));
                        sprite_node.components.push(::forge_scene::ComponentData::new_sprite(asset.path.clone()));
                        
                        // Asignación de colisiones automáticas de acuerdo a la capa inteligente
                        if app.active_layer == 2 {
                            sprite_node.components.push(::forge_scene::ComponentData::new_collider());
                        } else if app.active_layer == 3 {
                            sprite_node.components.push(::forge_scene::ComponentData::new_collider());
                            sprite_node.components.push(::forge_scene::ComponentData::new_behavior());
                        }
                        
                        let new_id = app.scene_tree.add_node(std::sync::Arc::new(sprite_node));
                        app.active_node_id = Some(new_id);
                        
                        app.console.add_message(
                            crate::debugger::LogLevel::Info,
                            &format!("Created Sprite '{}' on Layer {} at ({:.1}, {:.1})", sprite_name, app.active_layer, scene_x, scene_y)
                        );
                    }
                }
            }
            app.drag_operation = None;
            app.dragged_asset = None;
            app.drop_target = None;
        }
    }
}

/// Helper para convertir ruta relativa en URI absoluta compatible con cargadores egui
fn get_file_uri(path_str: &str) -> String {
    let path = std::path::Path::new(path_str);
    if let Ok(abs_path) = std::fs::canonicalize(path) {
        let abs_str = abs_path.to_string_lossy().replace('\\', "/");
        let clean_abs = if abs_str.starts_with("//?/") {
            abs_str[4..].to_string()
        } else if abs_str.starts_with("\\\\?\\") {
            abs_str[4..].to_string()
        } else if abs_str.starts_with("?") {
            abs_str.trim_start_matches('?').to_string()
        } else {
            abs_str
        };
        
        if clean_abs.starts_with('/') {
            format!("file://{}", clean_abs)
        } else {
            format!("file:///{}", clean_abs)
        }
    } else {
        format!("file:///{}", path_str.replace('\\', "/"))
    }
}


