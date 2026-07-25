//! # Event Node Editor UI
//! 
//! Editor visual para nodos de eventos con soporte para:
//! - Creación y edición de nodos
//! - Conexión de nodos con cables
//! - Ejecución de grafos de eventos
//! - Visualización de variables y flags

use crate::event_node_manager::EventNodeManager;
use crate::event_nodes::{EventNode, EventType, NodeData, Edge};
use egui::{Color32, Vec2, Response, Ui, Sense};

/// Editor de nodos de evento
pub struct EventNodeEditor {
    pub manager: EventNodeManager,
    selected_node: Option<String>,
    #[allow(dead_code)]
    show_properties: bool,
    #[allow(dead_code)]
    show_nodes_list: bool,
    #[allow(dead_code)]
    execution_count: u32,
    drag_connection: Option<(String, egui::Pos2)>,
    #[allow(dead_code)]
    drag_node_id: Option<String>,
    #[allow(dead_code)]
    drag_node_start_pos: Option<egui::Pos2>,
    connection_error: Option<String>,
}

impl EventNodeEditor {
    /// Crea un nuevo editor de nodos de evento
    pub fn new() -> Self {
        Self {
            manager: EventNodeManager::new(),
            selected_node: None,
            show_properties: true,
            show_nodes_list: true,
            execution_count: 0,
            drag_connection: None,
            drag_node_id: None,
            drag_node_start_pos: None,
            connection_error: None,
        }
    }

    pub fn get_graph(&self) -> crate::event_nodes::EventGraph {
        self.manager.get_graph()
    }

    pub fn load_graph(&mut self, graph: crate::event_nodes::EventGraph) {
        self.manager.load_graph(graph);
    }

    /// Renderiza el editor completo
    pub fn render(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            // Panel izquierdo: Lista de nodos y botones de añadir
            ui.allocate_ui_with_layout(
                Vec2::new(180.0, ui.available_height()),
                egui::Layout::top_down(egui::Align::Min),
                |ui| {
                    ui.group(|ui| {
                        ui.set_width(180.0);
                        ui.heading("⚡ Event Forge");
                        ui.separator();
                        
                        ui.label("Add Nodes:");
                        for event_type in [
                            EventType::OnStart, EventType::OnClick, EventType::OnTimer, 
                            EventType::ShowDialog, EventType::PlayAnimation, EventType::ChangeVariable, EventType::IfCondition
                        ] {
                            if ui.button(format!("➕ {:?}", event_type)).clicked() {
                                // Add at a visible position in the center of the canvas area
                                self.create_node(event_type, (200.0, 150.0));
                            }
                        }
                        
                        ui.add_space(10.0);
                        ui.separator();
                        
                        ui.label("Created Nodes:");
                        self.render_nodes_list(ui);
                    });
                }
            );
            
            // Tabs de visualización usando botones
            ui.horizontal(|ui| {
                let show_graph = ui.button("🎨 Graph").clicked();
                let show_nodes = ui.button("📋 Nodes").clicked();
                let show_execution = ui.button("🎮 Execution").clicked();
                let show_groups = ui.button("📁 Groups").clicked();
                
                if show_graph {
                    ui.allocate_ui_with_layout(
                        Vec2::new(ui.available_width() - 260.0, ui.available_height()),
                        egui::Layout::top_down(egui::Align::Min),
                        |ui| {
                            ui.group(|ui| {
                                self.render_graph_view(ui);
                            });
                        }
                    );
                    
                    ui.allocate_ui_with_layout(
                        Vec2::new(240.0, ui.available_height()),
                        egui::Layout::top_down(egui::Align::Min),
                        |ui| {
                            ui.group(|ui| {
                                self.render_properties_panel(ui);
                            });
                        }
                    );
                } else if show_nodes {
                    ui.allocate_ui_with_layout(
                        Vec2::new(ui.available_width() - 260.0, ui.available_height()),
                        egui::Layout::top_down(egui::Align::Min),
                        |ui| {
                            ui.group(|ui| {
                                self.render_nodes_list(ui);
                            });
                        }
                    );
                    
                    ui.allocate_ui_with_layout(
                        Vec2::new(240.0, ui.available_height()),
                        egui::Layout::top_down(egui::Align::Min),
                        |ui| {
                            ui.group(|ui| {
                                self.render_properties_panel(ui);
                            });
                        }
                    );
                } else if show_execution {
                    ui.allocate_ui_with_layout(
                        Vec2::new(ui.available_width(), ui.available_height()),
                        egui::Layout::top_down(egui::Align::Min),
                        |ui| {
                            ui.group(|ui| {
                                self.render_execution_panel(ui);
                            });
                        }
                    );
                } else if show_groups {
                    ui.allocate_ui_with_layout(
                        Vec2::new(ui.available_width(), ui.available_height()),
                        egui::Layout::top_down(egui::Align::Min),
                        |ui| {
                            ui.group(|ui| {
                                self.render_groups_panel(ui);
                            });
                        }
                    );
                }
            });
        });
    }

    /// Renderiza la lista de nodos
    fn render_nodes_list(&mut self, ui: &mut Ui) {
        let mut delete_id = None;
        let mut select_id = None;
        
        let nodes = self.manager.get_all_nodes();
        
        egui::ScrollArea::vertical()
            .show(ui, |ui| {
                for node in nodes {
                    let id = &node.id;
                    ui.horizontal(|ui| {
                        let is_selected = self.selected_node.as_ref() == Some(id);
                        let label_resp = ui.selectable_label(is_selected, format!("{:?} ({})", node.event_type, node.id));
                        if label_resp.clicked() {
                            select_id = Some(id.clone());
                        }
                        ui.add_space(5.0);
                        if ui.button("❌").clicked() {
                            delete_id = Some(id.clone());
                        }
                    });
                }
            });
        
        if let Some(id) = delete_id {
            if self.manager.delete_node(&id) {
                if self.selected_node == Some(id) {
                    self.selected_node = None;
                }
            }
        }
        
        if let Some(id) = select_id {
            self.selected_node = Some(id);
        }
    }

    /// Renderiza la vista del grafo
    fn render_graph_view(&mut self, ui: &mut Ui) {
        let area_size = ui.available_size();
        let (rect, response) = ui.allocate_exact_size(area_size, Sense::click_and_drag());
        
        let painter = ui.painter();
        
        // Fondo del Canvas oscuro
        painter.rect_filled(rect, 4.0, Color32::from_rgb(20, 20, 20));
        
        // Rejilla de fondo
        let grid_spacing = 25.0;
        let grid_stroke = egui::Stroke::new(1.0_f32, Color32::from_rgb(30, 30, 30));
        
        let mut x = rect.min.x + (grid_spacing - (rect.min.x % grid_spacing)) % grid_spacing;
        while x < rect.max.x {
            painter.line_segment([egui::Pos2::new(x, rect.min.y), egui::Pos2::new(x, rect.max.y)], grid_stroke);
            x += grid_spacing;
        }
        
        let mut y = rect.min.y + (grid_spacing - (rect.min.y % grid_spacing)) % grid_spacing;
        while y < rect.max.y {
            painter.line_segment([egui::Pos2::new(rect.min.x, y), egui::Pos2::new(rect.max.x, y)], grid_stroke);
            y += grid_spacing;
        }
        
        // Menú contextual con click derecho para añadir nodos
        response.context_menu(|ui| {
            ui.label("Add Node");
            for event_type in [
                EventType::OnStart, EventType::OnClick, EventType::OnTimer, 
                EventType::ShowDialog, EventType::PlayAnimation, EventType::ChangeVariable, EventType::IfCondition
            ] {
                if ui.button(format!("{:?}", event_type)).clicked() {
                    let mouse_pos = ui.input(|i| i.pointer.latest_pos().unwrap_or(egui::Pos2::new(100.0, 100.0)));
                    self.create_node(event_type, (mouse_pos.x, mouse_pos.y));
                    ui.close();
                }
            }
        });
        
        // Deseleccionar al pulsar en el fondo
        if response.clicked() {
            self.selected_node = None;
        }

        let _clip_painter = painter.with_clip_rect(rect);
        self.render_graph_nodes_and_edges(ui, &response, rect);
    }

    /// Renderiza nodos y conexiones del grafo
    fn render_graph_nodes_and_edges(&mut self, ui: &mut Ui, _canvas_response: &Response, _canvas_rect: egui::Rect) {
        let mut drag_to_apply = None;
        let mut select_to_apply = None;
        let mut start_connection = None;
        let mut end_connection = None;
        
        let mouse_pos = ui.input(|i| i.pointer.latest_pos());
        let mouse_released = ui.input(|i| i.pointer.any_released());
        
        let nodes = self.manager.get_all_nodes();
        let edges = self.manager.get_edges();
        
        // Guardar rect para mostrar mensaje de error
        let _rect = _canvas_rect;
        let _painter = ui.painter();
        
        // 1. Dibujar Cables Bézier de conexiones permanentes
        for edge in edges {
            let from_node = nodes.iter().find(|n| n.id == edge.from);
            let to_node = nodes.iter().find(|n| n.id == edge.to);
            if let (Some(from_node), Some(to_node)) = (from_node, to_node) {
                let start_pos = egui::Pos2::new(from_node.position.x + 150.0, from_node.position.y + 30.0);
                let end_pos = egui::Pos2::new(to_node.position.x, to_node.position.y + 30.0);
                self.draw_edge(
                    ui.painter(),
                    start_pos,
                    end_pos,
                    edge.condition.as_ref().map(|_| Color32::RED).unwrap_or(Color32::WHITE),
                );
            }
        }

        // 2. Dibujar cable temporal en arrastre activo
        if let Some((_, start_pos)) = self.drag_connection {
            if let Some(mp) = mouse_pos {
                self.draw_edge(
                    ui.painter(),
                    start_pos,
                    mp,
                    Color32::from_rgb(0, 120, 255),
                );
            }
        }

        // 3. Dibujar Tarjetas de Nodos y Sockets de puertos
        for node in nodes {
            let id = &node.id;
            let is_selected = self.selected_node.as_ref() == Some(id);
            let node_type_color = self.get_node_type_color(&node.event_type);
            
            let rect = egui::Rect::from_min_size(node.position, Vec2::new(150.0, 60.0));
            let painter = ui.painter();
            
            // Cuerpo de tarjeta de nodo
            let stroke_color = if is_selected { Color32::YELLOW } else { Color32::from_rgb(60, 60, 60) };
            let stroke_width = if is_selected { 2.0_f32 } else { 1.0_f32 };
            painter.rect(
                rect, 
                6.0, 
                Color32::from_rgb(30, 30, 30), 
                egui::Stroke::new(stroke_width, stroke_color), 
                egui::StrokeKind::Outside
            );
            
            // Encabezado pintado con el tipo de nodo
            let header_rect = egui::Rect::from_min_size(node.position, Vec2::new(150.0, 20.0));
            painter.rect_filled(header_rect, egui::CornerRadius { nw: 6, ne: 6, sw: 0, se: 0 }, node_type_color);
            
            // Texto del título (Tipo)
            let title_pos = node.position + Vec2::new(8.0, 4.0);
            painter.text(
                title_pos, 
                egui::Align2::LEFT_TOP, 
                format!("{:?}", node.event_type), 
                egui::FontId::proportional(11.0), 
                Color32::BLACK
            );
            
            // Identificador ID en el cuerpo
            let id_pos = node.position + Vec2::new(8.0, 26.0);
            painter.text(
                id_pos, 
                egui::Align2::LEFT_TOP, 
                id, 
                egui::FontId::monospace(10.0), 
                Color32::from_rgb(180, 180, 180)
            );
            
            // Contador de ejecuciones
            let count_pos = node.position + Vec2::new(8.0, 42.0);
            painter.text(
                count_pos, 
                egui::Align2::LEFT_TOP, 
                format!("Execs: {}", node.execution_count), 
                egui::FontId::proportional(9.0), 
                Color32::from_rgb(140, 140, 140)
            );
            
            // Puertos de Sockets
            let input_socket_pos = egui::Pos2::new(node.position.x, node.position.y + 30.0);
            let output_socket_pos = egui::Pos2::new(node.position.x + 150.0, node.position.y + 30.0);
            
            let is_input_hovered = mouse_pos.map_or(false, |mp| mp.distance(input_socket_pos) < 8.0);
            let is_output_hovered = mouse_pos.map_or(false, |mp| mp.distance(output_socket_pos) < 8.0);
            
            // Dibujar círculo puerto Entrada (Izquierda)
            painter.circle_filled(
                input_socket_pos, 
                if is_input_hovered { 6.0 } else { 4.0 }, 
                if is_input_hovered { Color32::WHITE } else { Color32::from_rgb(120, 120, 120) }
            );
            
            // Dibujar círculo puerto Salida (Derecha)
            painter.circle_filled(
                output_socket_pos, 
                if is_output_hovered { 6.0 } else { 4.0 }, 
                if is_output_hovered { Color32::from_rgb(100, 180, 255) } else { Color32::from_rgb(0, 120, 255) }
            );
            
            // Registrar inputs interactivos
            let node_response = ui.allocate_rect(rect, egui::Sense::click_and_drag());
            if node_response.clicked() {
                select_to_apply = Some(id.clone());
            }
            if node_response.dragged() {
                drag_to_apply = Some((id.clone(), node_response.drag_delta()));
            }
            
            // Iniciar arrastre de cable desde puerto output
            if is_output_hovered && ui.input(|i| i.pointer.any_pressed()) {
                start_connection = Some((id.clone(), output_socket_pos));
            }
            
            // Registrar destino de cable sobre puerto input
            if is_input_hovered && mouse_released {
                end_connection = Some(id.clone());
            }
        }
        
        // Aplicar selección
        if let Some(id) = select_to_apply {
            self.selected_node = Some(id);
        }
        
        // Aplicar arrastre de nodo
        if let Some((id, delta)) = drag_to_apply {
            if let Some(node) = self.manager.get_node_mut(&id) {
                node.position += delta;
            }
        }
        
        // Almacenar el cable temporal de origen al iniciar el click
        if let Some((from_id, pos)) = start_connection {
            self.drag_connection = Some((from_id, pos));
        }
        
        // Validar conexión antes de crearla
        let mut connection_valid = false;
        let end_connection_opt = end_connection.clone();
        
        // Validar fuera del bucle para evitar conflictos de préstamo
        if !connection_valid {
            if mouse_released {
                if let Some((from_id, _)) = self.drag_connection.take() {
                    if let Some(to_id) = end_connection_opt.clone() {
                        // Validación 1: No conectar un nodo consigo mismo
                        if from_id != to_id {
                            // Validación 2: Verificar que el nodo destino existe
                            if self.manager.get_node(&to_id).is_some() {
                                // Validación 3: Verificar que el nodo origen existe
                                if self.manager.get_node(&from_id).is_some() {
                                    // Validación 4: Verificar que no ya existe la conexión
                                    if !self.manager.edges.iter().any(|e| e.from == from_id && e.to == to_id) {
                                        // Validación 5: Verificar tipos compatibles (opcional - futuro)
                                        // Por ahora permitimos todas las conexiones
                                        connection_valid = true;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        
        // Crear conexión si es válida
        if connection_valid {
            if let Some((from_id, _)) = self.drag_connection.take() {
                if let Some(to_id) = end_connection_opt {
                    self.manager.edges.push(Edge {
                        from: from_id,
                        to: to_id,
                        condition: None,
                    });
                }
            }
            // Limpiar error de conexión si se creó exitosamente
            self.connection_error = None;
        }
    }

    /// Dibuja una conexión curvada de cable Bézier
    fn draw_edge(&self, painter: &egui::Painter, from: egui::Pos2, to: egui::Pos2, color: Color32) {
        let control_point_1 = from + Vec2::new(50.0, 0.0);
        let control_point_2 = to - Vec2::new(50.0, 0.0);
        let shape = egui::epaint::CubicBezierShape::from_points_stroke(
            [from, control_point_1, control_point_2, to],
            false,
            Color32::TRANSPARENT,
            egui::epaint::Stroke::new(2.0_f32, color),
        );
        painter.add(shape);
    }

    /// Renderiza el panel de grupos de nodos
    fn render_groups_panel(&mut self, ui: &mut Ui) {
        egui::ScrollArea::vertical().show(ui, |ui| {
            ui.heading("📁 Node Groups");
            ui.separator();
            
            // Botón para crear nuevo grupo
            ui.horizontal(|ui| {
                if ui.button("+ New Group").clicked() {
                    let mut group_name = String::from("New Group");
                    ui.text_edit_singleline(&mut group_name);
                    if !group_name.is_empty() {
                        let _ = self.manager.create_group(group_name.clone());
                    }
                }
                ui.add_space(10.0);
                if ui.button("📂 New Subgroup").clicked() {
                    let mut subgroup_name = String::from("Subgroup");
                    ui.text_edit_singleline(&mut subgroup_name);
                    if !subgroup_name.is_empty() {
                        let first_group_id = self.manager.groups.first().map(|g| g.id.clone());
                        if let Some(first_group_id) = first_group_id {
                            let _ = self.manager.create_child_group(&first_group_id, subgroup_name.clone());
                        }
                    }
                }
            });
            
            ui.add_space(10.0);
            
            // Renderizar grupos
            self.render_groups_list(ui);
        });
    }
    
    /// Renderiza la lista de grupos
    fn render_groups_list(&self, ui: &mut Ui) {
        if self.manager.groups.is_empty() {
            ui.label("No groups yet.");
            return;
        }
        
        for group in &self.manager.groups {
            let group_nodes = self.manager.get_nodes_in_group(&group.id);
            let node_count = group_nodes.len();
            let _group_id = group.id.clone();
            let group_name = group.name.clone();
            let is_collapsed = group.is_collapsed;
            
            ui.group(|ui| {
                ui.horizontal(|ui| {
                    ui.label(format!("{} Group", group_name));
                    
                    let collapsed_icon = if is_collapsed { "▶" } else { "▼" };
                    
                    ui.label(format!(" {} ({})", collapsed_icon, node_count));
                });
                
                ui.separator();
                ui.label(format!("Nodes in {} ({}):", group_name, node_count));
                
                if node_count == 0 {
                    ui.label("  (no nodes)");
                } else {
                    for node in &group_nodes {
                        ui.label(format!("  • {:?} [{}]", node.event_type, node.id));
                    }
                }
                
                if !group.children.is_empty() {
                    ui.separator();
                    ui.label(format!("Subgroups ({}):", group_name));
                    for child_id in &group.children {
                        if let Some(child_group) = self.manager.groups.iter().find(|g| g.id.as_str() == child_id.as_str()) {
                            let child_nodes = self.manager.get_nodes_in_group(child_id);
                            ui.label(format!("  └─ {} ({} nodes)", child_group.name, child_nodes.len()));
                        }
                    }
                }
            });
        }
    }

    /// Renderiza el panel de ejecución del grafo
    fn render_execution_panel(&mut self, ui: &mut Ui) {
        egui::ScrollArea::vertical().show(ui, |ui| {
            ui.heading("🎮 Graph Execution");
            ui.separator();
            
            // Botones de ejecución
            ui.horizontal(|ui| {
                if ui.button("▶ Execute Graph").clicked() {
                    self.manager.execute_all();
                    ui.label(format!("Executed {} nodes", self.manager.nodes.len()));
                }
                if ui.button("▶ Execute Selected").clicked() {
                    if let Some(ref node_id) = self.selected_node {
                        self.manager.execute_node(node_id.clone());
                    }
                }
                if ui.button("🔄 Reset Counts").clicked() {
                    for node in &mut self.manager.nodes {
                        node.execution_count = 0;
                        node.is_active = false;
                    }
                }
            });
            
            ui.add_space(10.0);
            
            // Verificar ciclos
            if self.manager.has_cycle() {
                ui.label("⚠️ Warning: Graph contains a cycle!");
            } else {
                ui.label("✓ Graph is acyclic (DAG)");
            }
            
            // Mostrar contadores de ejecución
            ui.separator();
            ui.label("Execution Counts:");
            
            let execution_counts = self.manager.get_execution_counts();
            if execution_counts.is_empty() {
                ui.label("No nodes executed yet");
            } else {
                egui::ScrollArea::horizontal().show(ui, |ui| {
                    for (node_id, count) in execution_counts {
                        let node = self.manager.get_node(&node_id);
                        let node_name = node.as_ref().map(|n| format!("{:?}", n.event_type)).unwrap_or_else(|| "Unknown".to_string());
                        ui.horizontal(|ui| {
                            ui.label(format!("{} ({})", node_name, node_id));
                            ui.label(format!("Exec: {}", count));
                        });
                    }
                });
            }
        });
    }
    
    /// Renderiza el panel de propiedades
    fn render_properties_panel(&mut self, ui: &mut Ui) {
        egui::ScrollArea::vertical().show(ui, |ui| {
            ui.heading("Node Properties");
            ui.separator();
            if let Some(node_id) = &self.selected_node {
                let mut node_found = None;
                if let Some(node) = self.manager.get_node_mut(node_id) {
                    Self::render_node_properties(ui, node);
                    node_found = Some(node.id.clone());
                }
                
                if let Some(id) = node_found {
                    ui.add_space(10.0);
                    ui.separator();
                    ui.horizontal(|ui| {
                        if ui.button("Execute").clicked() {
                            self.manager.execute_node(id.clone());
                        }
                        if ui.button("Execute All").clicked() {
                            self.manager.execute_all();
                        }
                        if ui.button("Duplicate").clicked() {
                            // Duplicar nodo manteniendo posición y conexiones
                            if let Some(node) = self.manager.get_node(&id) {
                                let node_name = id.clone();
                                let new_id = format!("{}_copy", node_name);
                                let mut new_node = node.clone();
                                new_node.id = new_id.clone();
                                // Mantener misma posición pero desplazada ligeramente
                                new_node.position = egui::Pos2::new(
                                    node.position.x + 20.0,
                                    node.position.y + 20.0
                                );
                                self.manager.nodes.push(new_node);
                                self.selected_node = Some(new_id);
                            }
                        }
                        if ui.button("Delete Node").clicked() {
                            let should_deselect = self.selected_node == Some(id.clone());
                            if self.manager.delete_node(&id) {
                                if should_deselect {
                                    self.selected_node = None;
                                }
                            }
                        }
                        if ui.button("Delete").clicked() {
                            self.manager.delete_node(&id);
                            self.selected_node = None;
                        }
                    });
                }
            } else {
                ui.label("Select a node to view properties");
            }
        });
    }

    /// Renderiza propiedades de un nodo específico
    fn render_node_properties(ui: &mut Ui, node: &mut EventNode) {
        // Información básica
        ui.label(format!("ID: {}", node.id));
        ui.label(format!("Type: {:?}", node.event_type));
        ui.label(format!("Position: ({}, {})", node.position.x, node.position.y));
        ui.label(format!("Executions: {}", node.execution_count));
        ui.label(format!("Active: {}", node.is_active));
        ui.add_space(10.0);

        // Configuración específica por tipo
        egui::CollapsingHeader::new("Configuration")
            .default_open(true)
            .show(ui, |ui| {
                match &node.event_type {
                    // Triggers
                    EventType::OnStart => Self::render_on_start_properties(ui, node),
                    EventType::OnClick => Self::render_on_click_properties(ui, node),
                    EventType::OnTimer => Self::render_on_timer_properties(ui, node),
                    EventType::OnEnter => Self::render_on_enter_properties(ui, node),
                    EventType::OnExit => Self::render_on_exit_properties(ui, node),
                    EventType::OnCollision => Self::render_on_collision_properties(ui, node),
                    EventType::OnTrigger => Self::render_on_trigger_properties(ui, node),
                    EventType::OnKeyPress => Self::render_on_key_press_properties(ui, node),
                    EventType::OnMouseOver => Self::render_on_mouse_over_properties(ui, node),
                    EventType::OnMouseClick => Self::render_on_mouse_click_properties(ui, node),
                    // Actions
                    EventType::ShowDialog => Self::render_show_dialog_properties(ui, node),
                    EventType::PlayAnimation => Self::render_play_animation_properties(ui, node),
                    EventType::StopAnimation => Self::render_stop_animation_properties(ui, node),
                    EventType::PauseAnimation => Self::render_pause_animation_properties(ui, node),
                    EventType::ResumeAnimation => Self::render_resume_animation_properties(ui, node),
                    EventType::ChangeVariable => Self::render_change_variable_properties(ui, node),
                    EventType::CallFunction => Self::render_call_function_properties(ui, node),
                    EventType::SetFlag => Self::render_set_flag_properties(ui, node),
                    EventType::ClearFlag => Self::render_clear_flag_properties(ui, node),
                    EventType::SpawnEntity => Self::render_spawn_entity_properties(ui, node),
                    EventType::RemoveEntity => Self::render_remove_entity_properties(ui, node),
                    EventType::PlaySound => Self::render_play_sound_properties(ui, node),
                    EventType::ChangeColor => Self::render_change_color_properties(ui, node),
                    EventType::ChangePosition => Self::render_change_position_properties(ui, node),
                    EventType::Rotate => Self::render_rotate_properties(ui, node),
                    EventType::Scale => Self::render_scale_properties(ui, node),
                    // Conditionals
                    EventType::IfCondition => Self::render_if_condition_properties(ui, node),
                    EventType::WhileLoop => Self::render_while_loop_properties(ui, node),
                    EventType::ForLoop => Self::render_for_loop_properties(ui, node),
                    EventType::Switch => Self::render_switch_properties(ui, node),
                    EventType::AndCondition => Self::render_and_condition_properties(ui, node),
                    EventType::OrCondition => Self::render_or_condition_properties(ui, node),
                    EventType::NotCondition => Self::render_not_condition_properties(ui, node),
                    // System
                    EventType::Empty => { ui.label("No configuration"); },
                    _ => { ui.label("No specific configuration"); }
                }
            });
    }

    /// Renderiza propiedades de nodo OnStart
    fn render_on_start_properties(ui: &mut Ui, node: &mut EventNode) {
        if let NodeData::OnStart { auto_execute } = &mut node.data {
            ui.checkbox(auto_execute, "Auto Execute on Start");
        }
    }

    /// Renderiza propiedades de nodo OnEnter
    fn render_on_enter_properties(ui: &mut Ui, node: &mut EventNode) {
        if let NodeData::OnEnter { target } = &mut node.data {
            ui.label("Target:");
            ui.text_edit_singleline(target);
        }
    }

    /// Renderiza propiedades de nodo OnExit
    fn render_on_exit_properties(ui: &mut Ui, node: &mut EventNode) {
        if let NodeData::OnExit { target } = &mut node.data {
            ui.label("Target:");
            ui.text_edit_singleline(target);
        }
    }

    /// Renderiza propiedades de nodo OnClick
    fn render_on_click_properties(ui: &mut Ui, node: &mut EventNode) {
        if let NodeData::OnClick { target, button } = &mut node.data {
            ui.label("Target:");
            ui.text_edit_singleline(target);
            ui.label("Button:");
            ui.add(egui::Slider::new(button, 0..=1).text("Button"));
        }
    }

    /// Renderiza propiedades de nodo OnTimer
    fn render_on_timer_properties(ui: &mut Ui, node: &mut EventNode) {
        if let NodeData::OnTimer { interval_ms, count } = &mut node.data {
            ui.label("Interval (ms):");
            ui.add(egui::Slider::new(interval_ms, 100..=10000).text("ms"));
            ui.label("Count:");
            ui.add(egui::Slider::new(count, 1..=1000).text("count"));
        }
    }

    /// Renderiza propiedades de nodo ShowDialog
    fn render_show_dialog_properties(ui: &mut Ui, node: &mut EventNode) {
        if let NodeData::ShowDialog { dialog_id, language } = &mut node.data {
            ui.label("Dialog ID:");
            ui.text_edit_singleline(dialog_id);
            ui.label("Language:");
            ui.text_edit_singleline(language);
        }
    }

    /// Renderiza propiedades de nodo PlayAnimation
    fn render_play_animation_properties(ui: &mut Ui, node: &mut EventNode) {
        if let NodeData::PlayAnimation { target, animation, duration_ms } = &mut node.data {
            ui.label("Target:");
            ui.text_edit_singleline(target);
            ui.label("Animation:");
            ui.text_edit_singleline(animation);
            ui.label("Duration (ms):");
            ui.add(egui::Slider::new(duration_ms, 100..=5000).text("ms"));
        }
    }

    /// Renderiza propiedades de nodo ChangeVariable
    fn render_change_variable_properties(ui: &mut Ui, node: &mut EventNode) {
        if let NodeData::ChangeVariable { variable, operation, value } = &mut node.data {
            ui.label("Variable:");
            ui.text_edit_singleline(variable);
            ui.label("Operation:");
            ui.text_edit_singleline(operation);
            ui.label("Value:");
            ui.text_edit_singleline(value);
        }
    }

    /// Renderiza propiedades de nodo IfCondition
    fn render_if_condition_properties(ui: &mut Ui, node: &mut EventNode) {
        if let NodeData::IfCondition { condition, then_branch, else_branch } = &mut node.data {
            ui.label("Condition:");
            ui.text_edit_singleline(condition);
            ui.label("Then Branch:");
            ui.text_edit_singleline(then_branch);
            ui.label("Else Branch:");
            ui.text_edit_singleline(else_branch);
        }
    }

    /// Renderiza propiedades de nodo StopAnimation
    fn render_stop_animation_properties(ui: &mut Ui, node: &mut EventNode) {
        if let NodeData::StopAnimation { target, animation } = &mut node.data {
            ui.label("Target:");
            ui.text_edit_singleline(target);
            ui.label("Animation:");
            ui.text_edit_singleline(animation);
        }
    }

    /// Renderiza propiedades de nodo PauseAnimation
    fn render_pause_animation_properties(ui: &mut Ui, node: &mut EventNode) {
        if let NodeData::PauseAnimation { target, animation } = &mut node.data {
            ui.label("Target:");
            ui.text_edit_singleline(target);
            ui.label("Animation:");
            ui.text_edit_singleline(animation);
        }
    }

    /// Renderiza propiedades de nodo ResumeAnimation
    fn render_resume_animation_properties(ui: &mut Ui, node: &mut EventNode) {
        if let NodeData::ResumeAnimation { target, animation } = &mut node.data {
            ui.label("Target:");
            ui.text_edit_singleline(target);
            ui.label("Animation:");
            ui.text_edit_singleline(animation);
        }
    }

    /// Renderiza propiedades de nodo CallFunction
    fn render_call_function_properties(ui: &mut Ui, node: &mut EventNode) {
        if let NodeData::CallFunction { function_name, args } = &mut node.data {
            ui.label("Function Name:");
            ui.text_edit_singleline(function_name);
            ui.label("Arguments:");
            for (i, arg) in args.iter_mut().enumerate() {
                ui.label(format!("Arg {}: ", i + 1));
                ui.text_edit_singleline(arg);
            }
        }
    }

    /// Renderiza propiedades de nodo SetFlag
    fn render_set_flag_properties(ui: &mut Ui, node: &mut EventNode) {
        if let NodeData::SetFlag { flag } = &mut node.data {
            ui.label("Flag:");
            ui.text_edit_singleline(flag);
        }
    }

    /// Renderiza propiedades de nodo ClearFlag
    fn render_clear_flag_properties(ui: &mut Ui, node: &mut EventNode) {
        if let NodeData::ClearFlag { flag } = &mut node.data {
            ui.label("Flag:");
            ui.text_edit_singleline(flag);
        }
    }

    /// Renderiza propiedades de nodo SpawnEntity
    fn render_spawn_entity_properties(ui: &mut Ui, node: &mut EventNode) {
        if let NodeData::SpawnEntity { entity_type, position } = &mut node.data {
            ui.label("Entity Type:");
            ui.text_edit_singleline(entity_type);
            ui.label("Position X:");
            ui.add(egui::Slider::new(&mut position[0], -1000.0..=1000.0).text("X"));
            ui.label("Position Y:");
            ui.add(egui::Slider::new(&mut position[1], -1000.0..=1000.0).text("Y"));
        }
    }

    /// Renderiza propiedades de nodo RemoveEntity
    fn render_remove_entity_properties(ui: &mut Ui, node: &mut EventNode) {
        if let NodeData::RemoveEntity { entity_id } = &mut node.data {
            ui.label("Entity ID:");
            ui.text_edit_singleline(entity_id);
        }
    }

    /// Renderiza propiedades de nodo PlaySound
    fn render_play_sound_properties(ui: &mut Ui, node: &mut EventNode) {
        if let NodeData::PlaySound { sound_name, volume } = &mut node.data {
            ui.label("Sound Name:");
            ui.text_edit_singleline(sound_name);
            ui.label("Volume:");
            ui.add(egui::Slider::new(volume, 0.0..=1.0).text("0.0 - 1.0"));
        }
    }

    /// Renderiza propiedades de nodo ChangeColor
    fn render_change_color_properties(ui: &mut Ui, node: &mut EventNode) {
        if let NodeData::ChangeColor { r, g, b, a } = &mut node.data {
            ui.label("Red:");
            ui.add(egui::Slider::new(r, 0.0..=1.0).text("R"));
            ui.label("Green:");
            ui.add(egui::Slider::new(g, 0.0..=1.0).text("G"));
            ui.label("Blue:");
            ui.add(egui::Slider::new(b, 0.0..=1.0).text("B"));
            ui.label("Alpha:");
            ui.add(egui::Slider::new(a, 0.0..=1.0).text("A"));
        }
    }

    /// Renderiza propiedades de nodo ChangePosition
    fn render_change_position_properties(ui: &mut Ui, node: &mut EventNode) {
        if let NodeData::ChangePosition { x, y } = &mut node.data {
            ui.label("X:");
            ui.add(egui::Slider::new(x, -1000.0..=1000.0).text("X"));
            ui.label("Y:");
            ui.add(egui::Slider::new(y, -1000.0..=1000.0).text("Y"));
        }
    }

    /// Renderiza propiedades de nodo Rotate
    fn render_rotate_properties(ui: &mut Ui, node: &mut EventNode) {
        if let NodeData::Rotate { angle_degrees } = &mut node.data {
            ui.label("Angle (degrees):");
            ui.add(egui::Slider::new(angle_degrees, -360.0..=360.0).text("°"));
        }
    }

    /// Renderiza propiedades de nodo Scale
    fn render_scale_properties(ui: &mut Ui, node: &mut EventNode) {
        if let NodeData::Scale { x, y } = &mut node.data {
            ui.label("Scale X:");
            ui.add(egui::Slider::new(x, 0.0..=2.0).text("X"));
            ui.label("Scale Y:");
            ui.add(egui::Slider::new(y, 0.0..=2.0).text("Y"));
        }
    }

    /// Renderiza propiedades de nodo WhileLoop
    fn render_while_loop_properties(ui: &mut Ui, node: &mut EventNode) {
        if let NodeData::WhileLoop { condition, body_branch } = &mut node.data {
            ui.label("Condition:");
            ui.text_edit_singleline(condition);
            ui.label("Body Branch:");
            ui.text_edit_singleline(body_branch);
        }
    }

    /// Renderiza propiedades de nodo ForLoop
    fn render_for_loop_properties(ui: &mut Ui, node: &mut EventNode) {
        if let NodeData::ForLoop { variable, start, end, body_branch } = &mut node.data {
            ui.label("Variable:");
            ui.text_edit_singleline(variable);
            ui.label("Start:");
            ui.text_edit_singleline(start);
            ui.label("End:");
            ui.text_edit_singleline(end);
            ui.label("Body Branch:");
            ui.text_edit_singleline(body_branch);
        }
    }

    /// Renderiza propiedades de nodo Switch
    fn render_switch_properties(ui: &mut Ui, node: &mut EventNode) {
        if let NodeData::Switch { value, cases } = &mut node.data {
            ui.label("Value:");
            ui.text_edit_singleline(value);
            ui.label("Cases:");
            for (i, (condition, branch)) in cases.iter_mut().enumerate() {
                ui.label(format!("Case {}: Condition:", i + 1));
                ui.text_edit_singleline(condition);
                ui.label(format!("Case {}: Branch:", i + 1));
                ui.text_edit_singleline(branch);
            }
        }
    }

    /// Renderiza propiedades de nodo AndCondition
    fn render_and_condition_properties(ui: &mut Ui, node: &mut EventNode) {
        if let NodeData::AndCondition { conditions } = &mut node.data {
            ui.label("Conditions:");
            for (i, condition) in conditions.iter_mut().enumerate() {
                ui.label(format!("Condition {}: ", i + 1));
                ui.text_edit_singleline(condition);
            }
        }
    }

    /// Renderiza propiedades de nodo OrCondition
    fn render_or_condition_properties(ui: &mut Ui, node: &mut EventNode) {
        if let NodeData::OrCondition { conditions } = &mut node.data {
            ui.label("Conditions:");
            for (i, condition) in conditions.iter_mut().enumerate() {
                ui.label(format!("Condition {}: ", i + 1));
                ui.text_edit_singleline(condition);
            }
        }
    }

    /// Renderiza propiedades de nodo NotCondition
    fn render_not_condition_properties(ui: &mut Ui, node: &mut EventNode) {
        if let NodeData::NotCondition { condition } = &mut node.data {
            ui.label("Condition:");
            ui.text_edit_singleline(condition);
        }
    }

    /// Renderiza propiedades de nodo OnCollision
    fn render_on_collision_properties(ui: &mut Ui, node: &mut EventNode) {
        if let NodeData::OnCollision { target, normal } = &mut node.data {
            ui.label("Target:");
            ui.text_edit_singleline(target);
            ui.label("Normal X:");
            ui.add(egui::Slider::new(&mut normal[0], -1.0..=1.0).text("X"));
            ui.label("Normal Y:");
            ui.add(egui::Slider::new(&mut normal[1], -1.0..=1.0).text("Y"));
        }
    }

    /// Renderiza propiedades de nodo OnTrigger
    fn render_on_trigger_properties(ui: &mut Ui, node: &mut EventNode) {
        if let NodeData::OnTrigger { target } = &mut node.data {
            ui.label("Target:");
            ui.text_edit_singleline(target);
        }
    }

    /// Renderiza propiedades de nodo OnKeyPress
    fn render_on_key_press_properties(ui: &mut Ui, node: &mut EventNode) {
        if let NodeData::OnKeyPress { key } = &mut node.data {
            ui.label("Key:");
            ui.text_edit_singleline(key);
        }
    }

    /// Renderiza propiedades de nodo OnMouseOver
    fn render_on_mouse_over_properties(ui: &mut Ui, node: &mut EventNode) {
        if let NodeData::OnMouseOver { target } = &mut node.data {
            ui.label("Target:");
            ui.text_edit_singleline(target);
        }
    }

    /// Renderiza propiedades de nodo OnMouseClick
    fn render_on_mouse_click_properties(ui: &mut Ui, node: &mut EventNode) {
        if let NodeData::OnMouseClick { target, button } = &mut node.data {
            ui.label("Target:");
            ui.text_edit_singleline(target);
            ui.label("Button:");
            ui.add(egui::Slider::new(button, 0..=2).text("0=Left, 1=Middle, 2=Right"));
        }
    }

    /// Renderiza panel de estadísticas
    #[allow(dead_code)]
    fn render_statistics(&mut self, ui: &mut Ui) {
        ui.separator();
        ui.label("Statistics");
        
        let counts = self.manager.get_execution_counts();
        let total_executions: u32 = counts.values().map(|&c| c as u32).sum();
        
        ui.label(format!("Total Executions: {}", total_executions));
        ui.label(format!("Total Nodes: {}", counts.len()));
        ui.label(format!("Total Connections: {}", self.manager.get_edges().len()));
    }

    /// Obtiene el color según el tipo de nodo
    fn get_node_type_color(&self, event_type: &EventType) -> Color32 {
        match event_type {
            // Triggers
            EventType::OnStart | EventType::OnClick | EventType::OnTimer | EventType::OnEnter | EventType::OnExit |
            EventType::OnCollision | EventType::OnTrigger | EventType::OnKeyPress |
            EventType::OnMouseOver | EventType::OnMouseClick => Color32::from_rgb(50, 200, 50),
            // Actions
            EventType::ShowDialog | EventType::PlayAnimation | EventType::StopAnimation | EventType::PauseAnimation | EventType::ResumeAnimation |
            EventType::ChangeVariable | EventType::CallFunction | EventType::SetFlag | EventType::ClearFlag |
            EventType::SpawnEntity | EventType::RemoveEntity | EventType::PlaySound |
            EventType::ChangeColor | EventType::ChangePosition | EventType::Rotate |
            EventType::Scale => Color32::from_rgb(50, 50, 200),
            // Conditionals
            EventType::IfCondition | EventType::WhileLoop | EventType::ForLoop | EventType::Switch |
            EventType::AndCondition | EventType::OrCondition | EventType::NotCondition => Color32::from_rgb(255, 200, 50),
            // System
            EventType::Empty | _ => Color32::from_rgb(100, 100, 100),
        }
    }

    /// Crea un nuevo nodo
    pub fn create_node(&mut self, event_type: EventType, position: (f32, f32)) -> String {
        let id = self.manager.create_node(event_type, position);
        self.selected_node = Some(id.clone());
        id
    }

    /// Selecciona un nodo
    pub fn select_node(&mut self, id: Option<String>) {
        self.selected_node = id;
    }

    /// Obtiene el manager de nodos
    pub fn get_manager(&mut self) -> &mut EventNodeManager {
        &mut self.manager
    }
}

impl Default for EventNodeEditor {
    fn default() -> Self {
        Self::new()
    }
}

