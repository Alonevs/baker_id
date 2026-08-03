//! UI del editor - Interfaz completa, responsiva e interactiva

use crate::play_session::{PlaySession, Entity};
use crate::snapshot_manager::SnapshotManager;
use crate::timeline_integration::TimelineIntegration;
use crate::input_capture::UserInput;
use crate::hot_reload_panel::HotReloadPanel;
use crate::hot_reload_integration::SimpleFileWatcherIntegration;
use crate::explorer_panel_integration::ExplorerPanelIntegration;
use crate::event_forge::EventForgeWidget;
use crate::property_panel::{PropertyPanel, EntityId};
use crate::export_import_panel::ExportImportPanel;
use crate::preview_panel::PreviewPanel;
use crate::toolbar::{ToolbarWidget, ToolType};
use crate::timeline::timeline_editor::TimelineEditor;
use eframe::egui;
use eframe::App;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LeftTab {
    Explorer,
    HotReload,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RightTab {
    Properties,
    Preview,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BottomTab {
    Timeline,
    EventForge,
    Bitacora,
    ExportImport,
}

/// App principal del editor
pub struct ForgeEditorApp {
    pub logs: Vec<String>,
    pub timeline_integration: Option<TimelineIntegration>,
    pub play_session: Option<PlaySession>,
    pub snapshot_manager: SnapshotManager,
    pub user_input: UserInput,
    pub is_playing: bool,
    pub is_paused: bool,
    pub hot_reload_panel: HotReloadPanel,
    pub hot_reload_manager: crate::hot_reload::HotReloadManager,
    pub file_watcher_integration: Option<SimpleFileWatcherIntegration>,
    pub explorer_panel_integration: ExplorerPanelIntegration,

    pub event_forge_widget: EventForgeWidget,
    pub property_panel: PropertyPanel,
    pub export_import_panel: ExportImportPanel,
    pub preview_panel: PreviewPanel,
    pub toolbar_widget: ToolbarWidget,
    pub timeline_editor: TimelineEditor,

    pub left_tab: LeftTab,
    pub right_tab: RightTab,
    pub bottom_tab: BottomTab,

    pub entities: Vec<Entity>,
    pub selected_entity_index: Option<usize>,
    pub dragged_entity_index: Option<usize>,
}

impl Default for ForgeEditorApp {
    fn default() -> Self {
        let property_panel = PropertyPanel::new();
        let timeline_editor = TimelineEditor::new(property_panel.clone());

        Self {
            logs: vec!["🚀 Forge Editor 2D iniciado correctamente.".to_string()],
            timeline_integration: None,
            play_session: None,
            snapshot_manager: SnapshotManager::new(),
            user_input: UserInput::default(),
            is_playing: false,
            is_paused: false,
            hot_reload_panel: HotReloadPanel::new(),
            hot_reload_manager: crate::hot_reload::HotReloadManager::new(),
            file_watcher_integration: None,
            explorer_panel_integration: ExplorerPanelIntegration::new(),

            event_forge_widget: EventForgeWidget::new(),
            property_panel,
            export_import_panel: ExportImportPanel::new(),
            preview_panel: PreviewPanel::new(),
            toolbar_widget: ToolbarWidget::new(),
            timeline_editor,

            left_tab: LeftTab::Explorer,
            right_tab: RightTab::Properties,
            bottom_tab: BottomTab::Timeline,

            entities: vec![
                Entity::new("Player_Hero".to_string(), (180.0, 140.0)),
                Entity::new("Enemy_Spike".to_string(), (380.0, 220.0)),
                Entity::new("Platform_Wood".to_string(), (280.0, 300.0)),
            ],
            selected_entity_index: Some(0),
            dragged_entity_index: None,
        }
    }
}

impl ForgeEditorApp {
    pub fn new(_cc: &eframe::CreationContext) -> Self {
        let mut app = Self::default();
        
        // Inicializar FileWatcher Integration
        app.file_watcher_integration = Some(SimpleFileWatcherIntegration::new(
            app.hot_reload_manager.clone(),
            &std::env::temp_dir(),
            100
        ));
        if let Some(ref mut watcher) = app.file_watcher_integration {
            watcher.start();
        }
        
        // Configurar Explorer Panel
        let current_dir = std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from("."));
        app.explorer_panel_integration.set_current_project(current_dir.clone());
        
        // Configurar Export/Import Panel
        app.export_import_panel.export_path = current_dir.join("project_export.json");
        app.export_import_panel.import_path = current_dir.join("project_import.json");
        app.export_import_panel.update_summary();
        
        // Configurar Preview Panel
        app.preview_panel.scan_directory(&current_dir);

        // Seleccionar entidad inicial en property panel
        if !app.entities.is_empty() {
            app.property_panel.set_entity(EntityId(1));
            app.sync_property_panel_from_entity(0);
        }
        
        app
    }
    
    pub fn log(&mut self, message: String) {
        self.logs.push(message);
    }

    /// Sincroniza el Property Panel desde la entidad seleccionada
    pub fn sync_property_panel_from_entity(&mut self, idx: usize) {
        if let Some(entity) = self.entities.get(idx) {
            self.property_panel.set_entity(EntityId((idx + 1) as u64));
            self.property_panel.transform_properties.position = entity.position;
            self.property_panel.transform_properties.rotation = entity.rotation;
            self.property_panel.transform_properties.scale = entity.scale.0;
        }
    }

    /// Sincroniza la entidad seleccionada desde los cambios en Property Panel
    pub fn sync_entity_from_property_panel(&mut self) {
        if let Some(idx) = self.selected_entity_index {
            if let Some(entity) = self.entities.get_mut(idx) {
                entity.position = self.property_panel.transform_properties.position;
                entity.rotation = self.property_panel.transform_properties.rotation;
                let s = self.property_panel.transform_properties.scale;
                entity.scale = (s, s);
            }
        }
    }
    
    /// Inicia Play Mode
    pub fn start_play(&mut self) {
        self.play_session = Some(PlaySession::new(&self.entities));
        self.is_playing = true;
        self.is_paused = false;
        self.log("▶ Play Mode iniciado".to_string());
    }
    
    /// Pausa/Reanuda Play Mode
    pub fn toggle_pause(&mut self) {
        if self.is_playing {
            self.is_paused = !self.is_paused;
            let status = if self.is_paused { "⏸ Pausado" } else { "▶ Reanudado" };
            self.log(format!("Play Mode {}", status));
        }
    }

    /// Detiene Play Mode
    pub fn stop_play(&mut self) {
        if let Some(ref mut _session) = self.play_session {
            let _ = _session.stop(&mut []);
        }
        self.play_session = None;
        self.is_playing = false;
        self.is_paused = false;
        self.log("⏹ Play Mode detenido".to_string());
    }
    
    /// Actualiza la lógica por frame
    pub fn update_logic(&mut self, delta: f32) {
        if self.is_playing && !self.is_paused {
            if let Some(ref mut session) = self.play_session {
                session.update(delta, &self.user_input);
            }
        }
        
        self.hot_reload_manager.update(delta);
        
        if let Some(ref mut integration) = self.file_watcher_integration {
            integration.check();
        }
        
        self.explorer_panel_integration.update();
    }
}

impl App for ForgeEditorApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.update_logic(0.016);

        // 1. Toolbar Superior (Persistente)
        self.toolbar_widget.render(ctx);

        // 2. Panel Izquierdo (Pestañas: Explorer | Hot Reload)
        egui::SidePanel::left("left_dock")
            .resizable(true)
            .default_width(260.0)
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.selectable_value(&mut self.left_tab, LeftTab::Explorer, "📁 Explorer");
                    ui.selectable_value(&mut self.left_tab, LeftTab::HotReload, "🔥 Hot Reload");
                });
                ui.separator();

                match self.left_tab {
                    LeftTab::Explorer => {
                        ui.heading("📁 Asset Explorer");
                        ui.separator();
                        egui::ScrollArea::vertical().show(ui, |ui| {
                            let root_node = self.explorer_panel_integration.explorer.panel.root.clone();
                            self.render_explorer_tree(ui, &root_node, 0);
                        });
                    }
                    LeftTab::HotReload => {
                        self.hot_reload_panel.ui(ctx, &mut self.hot_reload_manager);
                    }
                }
            });

        // 3. Panel Derecho (Pestañas: Properties | Preview)
        egui::SidePanel::right("right_dock")
            .resizable(true)
            .default_width(280.0)
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.selectable_value(&mut self.right_tab, RightTab::Properties, "📋 Properties");
                    ui.selectable_value(&mut self.right_tab, RightTab::Preview, "🖼️ Preview");
                });
                ui.separator();

                match self.right_tab {
                    RightTab::Properties => {
                        self.property_panel.render(ctx);
                        self.sync_entity_from_property_panel();
                    }
                    RightTab::Preview => {
                        self.preview_panel.ui(ctx, ui);
                    }
                }
            });

        // 4. Panel Inferior (Pestañas: Timeline | Event Graph | Bitácora | Export/Import)
        egui::TopBottomPanel::bottom("bottom_dock")
            .resizable(true)
            .default_height(200.0)
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.selectable_value(&mut self.bottom_tab, BottomTab::Timeline, "🎬 Timeline");
                    ui.selectable_value(&mut self.bottom_tab, BottomTab::EventForge, "⚡ Event Graph");
                    ui.selectable_value(&mut self.bottom_tab, BottomTab::Bitacora, "📝 Bitácora");
                    ui.selectable_value(&mut self.bottom_tab, BottomTab::ExportImport, "📦 Export/Import");
                });
                ui.separator();

                match self.bottom_tab {
                    BottomTab::Timeline => {
                        ui.horizontal(|ui| {
                            if ui.button(if self.timeline_editor.is_playing() { "⏸ Pause" } else { "▶ Play Timeline" }).clicked() {
                                if self.timeline_editor.is_playing() {
                                    self.timeline_editor.pause_playback();
                                } else {
                                    self.timeline_editor.start_playback();
                                }
                            }
                            if ui.button("⏹ Stop").clicked() {
                                self.timeline_editor.stop_playback();
                            }
                            if ui.button("⏮ Prev").clicked() {
                                self.timeline_editor.prev_frame();
                            }
                            if ui.button("⏭ Next").clicked() {
                                self.timeline_editor.next_frame();
                            }
                            ui.label(format!("Frame: {}/{}", self.timeline_editor.current_frame, self.timeline_editor.total_frames));
                            ui.add(egui::Slider::new(&mut self.timeline_editor.playback_speed, 1.0..=120.0).text("FPS"));
                        });
                        ui.separator();
                        ui.label("Animation Tracks:");
                        egui::ScrollArea::horizontal().show(ui, |ui| {
                            ui.horizontal(|ui| {
                                for (idx, track) in self.timeline_editor.tracks.iter().enumerate() {
                                    ui.button(format!("Track #{}: {} ({})", idx, track.name, track.clip_name));
                                }
                                if ui.button("➕ Add Track").clicked() {
                                    let id = (self.timeline_editor.tracks.len() + 1) as u64;
                                    self.timeline_editor.add_track(id, format!("Clip_{}", id));
                                    self.log(format!("🎬 Pista añadida: Track #{}", id));
                                }
                            });
                        });
                    }
                    BottomTab::EventForge => {
                        ui.heading("⚡ Event Forge - Visual Scripting");
                        ui.horizontal(|ui| {
                            if ui.button("➕ Add Event Node").clicked() {
                                self.event_forge_widget.add_node();
                                self.log("⚡ Nodo de evento añadido".to_string());
                            }
                            if ui.button("🗑️ Clear Graph").clicked() {
                                self.event_forge_widget.manager.graph.nodes.clear();
                                self.event_forge_widget.manager.graph.connections.clear();
                                self.log("⚡ Grafo de eventos limpiado".to_string());
                            }
                        });
                        ui.separator();
                        ui.label(format!("Nodos activos: {} | Conexiones: {}", 
                            self.event_forge_widget.manager.graph.nodes.len(),
                            self.event_forge_widget.manager.graph.connections.len()));
                    }
                    BottomTab::Bitacora => {
                        ui.horizontal(|ui| {
                            ui.heading("📝 Bitácora de Salida");
                            if ui.button("🗑️ Limpiar Logs").clicked() {
                                self.logs.clear();
                            }
                        });
                        ui.separator();
                        egui::ScrollArea::vertical().show(ui, |ui| {
                            for log_msg in &self.logs {
                                ui.label(log_msg);
                            }
                        });
                    }
                    BottomTab::ExportImport => {
                        self.export_import_panel.ui(ctx, ui);
                    }
                }
            });

        // 5. Central Panel (Viewport 2D Interactivo)
        egui::CentralPanel::default().show(ctx, |ui| {
            // Barra superior de controles del Viewport
            ui.horizontal(|ui| {
                if ui.button("▶ Play").clicked() {
                    self.start_play();
                }
                if ui.button("⏸ Pause").clicked() {
                    self.toggle_pause();
                }
                if ui.button("⏹ Stop").clicked() && self.is_playing {
                    self.stop_play();
                }
                
                ui.separator();
                let status_str = if self.is_playing {
                    if self.is_paused { "⏸ Paused" } else { "▶ Playing" }
                } else {
                    "⏹ Stopped"
                };
                ui.label(format!("Estado: {}", status_str));
                
                ui.separator();
                let current_tool = self.toolbar_widget.toolbar.get_current_tool();
                ui.label(format!("Tool: {:?}", current_tool));

                ui.separator();
                if ui.button("➕ Add Entity").clicked() {
                    let new_id = format!("Entity_{}", self.entities.len() + 1);
                    let pos = (200.0 + (self.entities.len() * 30) as f32, 180.0);
                    self.entities.push(Entity::new(new_id.clone(), pos));
                    self.selected_entity_index = Some(self.entities.len() - 1);
                    self.sync_property_panel_from_entity(self.entities.len() - 1);
                    self.log(format!("➕ Entidad creada: {}", new_id));
                }

                if ui.button("🗑️ Remove Selected").clicked() {
                    if let Some(idx) = self.selected_entity_index {
                        if idx < self.entities.len() {
                            let removed = self.entities.remove(idx);
                            self.selected_entity_index = if self.entities.is_empty() { None } else { Some(0) };
                            if let Some(new_idx) = self.selected_entity_index {
                                self.sync_property_panel_from_entity(new_idx);
                            }
                            self.log(format!("🗑️ Entidad eliminada: {}", removed.id));
                        }
                    }
                }
            });

            ui.separator();

            // Lienzo interactivo del Viewport 2D
            let (response, painter) = ui.allocate_painter(
                ui.available_size(),
                egui::Sense::click_and_drag()
            );

            let rect = response.rect;
            
            // Fondo del Viewport
            painter.rect_filled(rect, 0.0, egui::Color32::from_rgb(25, 27, 32));

            // Dibujar rejilla 2D
            let grid_size = 40.0;
            let stroke_grid = egui::Stroke::new(1.0, egui::Color32::from_rgb(40, 44, 52));
            
            let mut x = rect.min.x;
            while x < rect.max.x {
                painter.line_segment([egui::pos2(x, rect.min.y), egui::pos2(x, rect.max.y)], stroke_grid);
                x += grid_size;
            }
            let mut y = rect.min.y;
            while y < rect.max.y {
                painter.line_segment([egui::pos2(rect.min.x, y), egui::pos2(rect.max.x, y)], stroke_grid);
                y += grid_size;
            }

            // Manejo de interacción de ratón en el lienzo
            let current_tool = self.toolbar_widget.toolbar.get_current_tool();

            if response.clicked() {
                if let Some(pointer_pos) = response.interact_pointer_pos() {
                    let local_x = pointer_pos.x - rect.min.x;
                    let local_y = pointer_pos.y - rect.min.y;

                    match current_tool {
                        ToolType::Paint | ToolType::TileMap => {
                            let new_id = format!("Tile_{}", self.entities.len() + 1);
                            self.entities.push(Entity::new(new_id.clone(), (local_x, local_y)));
                            self.selected_entity_index = Some(self.entities.len() - 1);
                            self.sync_property_panel_from_entity(self.entities.len() - 1);
                            self.log(format!("🎨 Tile pintado en ({:.0}, {:.0})", local_x, local_y));
                        }
                        ToolType::PhysicsBrush => {
                            let new_id = format!("PhysicsBox_{}", self.entities.len() + 1);
                            self.entities.push(Entity::new(new_id.clone(), (local_x, local_y)));
                            self.selected_entity_index = Some(self.entities.len() - 1);
                            self.sync_property_panel_from_entity(self.entities.len() - 1);
                            self.log(format!("⚛️ Objeto de física generado en ({:.0}, {:.0})", local_x, local_y));
                        }
                        _ => {
                            // Seleccionar entidad haciendo clic sobre ella
                            let mut clicked_idx = None;
                            for (idx, entity) in self.entities.iter().enumerate() {
                                let ex = rect.min.x + entity.position.0;
                                let ey = rect.min.y + entity.position.1;
                                let entity_rect = egui::Rect::from_center_size(egui::pos2(ex, ey), egui::vec2(50.0, 50.0));
                                if entity_rect.contains(pointer_pos) {
                                    clicked_idx = Some(idx);
                                    break;
                                }
                            }
                            if let Some(idx) = clicked_idx {
                                self.selected_entity_index = Some(idx);
                                self.sync_property_panel_from_entity(idx);
                                self.log(format!("🎯 Entidad seleccionada: {}", self.entities[idx].id));
                            }
                        }
                    }
                }
            }

            if response.dragged() {
                if let Some(pointer_pos) = response.interact_pointer_pos() {
                    if let Some(idx) = self.selected_entity_index {
                        if idx < self.entities.len() {
                            let local_x = (pointer_pos.x - rect.min.x).max(0.0);
                            let local_y = (pointer_pos.y - rect.min.y).max(0.0);
                            self.entities[idx].position = (local_x, local_y);
                            self.sync_property_panel_from_entity(idx);
                        }
                    }
                }
            }

            // Renderizar entidades de la escena en el canvas
            for (idx, entity) in self.entities.iter().enumerate() {
                let ex = rect.min.x + entity.position.0;
                let ey = rect.min.y + entity.position.1;
                let is_selected = self.selected_entity_index == Some(idx);

                let color = if is_selected {
                    egui::Color32::from_rgb(0, 200, 255)
                } else {
                    egui::Color32::from_rgb(180, 100, 240)
                };

                let entity_rect = egui::Rect::from_center_size(egui::pos2(ex, ey), egui::vec2(48.0, 48.0));
                painter.rect_filled(entity_rect, 6.0, color);
                painter.rect_stroke(entity_rect, 6.0, egui::Stroke::new(2.0, egui::Color32::WHITE), egui::StrokeKind::Outside);

                // Nombre e ID
                painter.text(
                    egui::pos2(ex, ey - 30.0),
                    egui::Align2::CENTER_CENTER,
                    &entity.id,
                    egui::FontId::proportional(12.0),
                    egui::Color32::WHITE
                );
            }
        });
    }
}

impl ForgeEditorApp {
    /// Renderiza el árbol del Explorer Panel recursivamente
    fn render_explorer_tree(&mut self, ui: &mut egui::Ui, node: &crate::explorer_panel::ExplorerNode, depth: usize) {
        let is_dir = node.node_type == crate::explorer_panel::NodeType::Directory;
        let icon = if is_dir { "📁" } else { "📄" };
        
        ui.indent(depth, |ui| {
            if ui.button(format!("{} {}", icon, node.name)).clicked() {
                let path = &node.path;
                let asset_type = crate::preview_panel::AssetInfo::detect_type(path);
                let asset = crate::preview_panel::AssetInfo::new(
                    node.name.clone(),
                    path.clone(),
                    asset_type
                );
                self.preview_panel.add_asset(asset);
                self.right_tab = RightTab::Preview;
                self.log(format!("📁 Asset abierto en Preview: {}", node.name));
            }
            if is_dir {
                for child in &node.children {
                    self.render_explorer_tree(ui, child, depth + 1);
                }
            }
        });
    }
}
