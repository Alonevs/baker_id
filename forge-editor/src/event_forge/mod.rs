use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use eframe::egui;
use egui::epaint::Shape;

/// Resultado de ejecución de evento
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventResult {
    pub node_id: u32,
    pub success: bool,
    pub output_data: Option<String>,
}

/// Socket de conexión
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct SocketId {
    pub node_id: u32,
    pub socket_index: u32,
    pub is_output: bool,
}

impl SocketId {
    pub fn new(node_id: u32, socket_index: u32, is_output: bool) -> Self {
        Self {
            node_id,
            socket_index,
            is_output,
        }
    }

    pub fn output(node_id: u32, socket_index: u32) -> Self {
        Self::new(node_id, socket_index, true)
    }

    pub fn input(node_id: u32, socket_index: u32) -> Self {
        Self::new(node_id, socket_index, false)
    }
}

impl SocketId {
    pub fn from_id(socket_id: SocketId) -> Self {
        socket_id
    }
}

/// Nodo de evento
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Node {
    pub id: u32,
    pub node_type: NodeType,
    pub position: (f32, f32),
    pub properties: HashMap<String, String>,
    pub execution_count: u32,
}

impl Node {
    pub fn new(id: u32, node_type: NodeType) -> Self {
        Self {
            id,
            node_type,
            position: (0.0, 0.0),
            properties: HashMap::new(),
            execution_count: 0,
        }
    }
}

/// Tipo de nodo
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum NodeType {
    TriggerZone,
    Dialogue,
    Conditional,
    Cinematic,
    Action,
}

/// Conexión entre nodos
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Connection {
    pub from: SocketId,
    pub to: SocketId,
    pub bezier_path: Vec<(f32, f32)>,
}

/// Grafo de eventos
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventGraph {
    pub nodes: HashMap<u32, Node>,
    pub connections: Vec<Connection>,
}

impl EventGraph {
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            connections: Vec::new(),
        }
    }

    pub fn add_node(&mut self, node: Node) {
        self.nodes.insert(node.id, node);
    }

    pub fn get_node(&self, id: u32) -> Option<&Node> {
        self.nodes.get(&id)
    }

    pub fn get_node_mut(&mut self, id: u32) -> Option<&mut Node> {
        self.nodes.get_mut(&id)
    }

    pub fn remove_node(&mut self, id: u32) -> bool {
        let mut removed = false;
        
        // Eliminar conexiones salientes
        self.connections.retain(|conn| {
            if conn.from.node_id == id {
                removed = true;
                false
            } else {
                true
            }
        });

        // Eliminar conexiones entrantes
        self.connections.retain(|conn| {
            if conn.to.node_id == id {
                removed = true;
                false
            } else {
                true
            }
        });

        // Eliminar nodo
        self.nodes.remove(&id);
        removed
    }

    pub fn add_connection(&mut self, from: SocketId, to: SocketId) {
        let bezier_path = self.calculate_bezier(&from, &to);
        self.connections.push(Connection {
            from,
            to,
            bezier_path,
        });
    }

    fn calculate_bezier(&self, from: &SocketId, to: &SocketId) -> Vec<(f32, f32)> {
        // Simplificación: línea recta
        let node_from = self.nodes.get(&from.node_id);
        let node_to = self.nodes.get(&to.node_id);

        if let (Some(node_from), Some(node_to)) = (node_from, node_to) {
            let start = (node_from.position.0 + 100.0, node_from.position.1 + 50.0);
            let end = (node_to.position.0 - 100.0, node_to.position.1 + 50.0);

            // Curva de Bézier cuadrática simple
            let mid_x = (start.0 + end.0) / 2.0;
            let mid_y = (start.1 + end.1) / 2.0;

            vec![
                (start.0, start.1),
                (mid_x, mid_y),
                (end.0, end.1),
            ]
        } else {
            vec![]
        }
    }

    pub fn save_json(&self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let json = serde_json::to_string_pretty(self)?;
        std::fs::write(path, json)?;
        Ok(())
    }

    pub fn load_json(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let json = std::fs::read_to_string(path)?;
        let graph: Self = serde_json::from_str(&json)?;
        Ok(graph)
    }
}

/// Gestor de nodos de eventos
#[derive(Debug, Clone)]
pub struct EventNodeManager {
    pub graph: EventGraph,
    pub selected_node: Option<u32>,
    pub runtime_context: RuntimeContext,
}

impl Default for EventNodeManager {
    fn default() -> Self {
        Self::new()
    }
}

impl EventNodeManager {
    pub fn new() -> Self {
        Self {
            graph: EventGraph::new(),
            selected_node: None,
            runtime_context: RuntimeContext::new(),
        }
    }

    pub fn add_node(&mut self, node: Node) {
        self.graph.add_node(node);
    }

    pub fn get_node(&self, id: u32) -> Option<&Node> {
        self.graph.get_node(id)
    }

    pub fn get_node_mut(&mut self, id: u32) -> Option<&mut Node> {
        self.graph.get_node_mut(id)
    }

    pub fn select_node(&mut self, id: u32) {
        self.selected_node = Some(id);
    }

    pub fn deselect_node(&mut self) {
        self.selected_node = None;
    }

    pub fn create_connection(&mut self, from: SocketId, to: SocketId) {
        self.graph.add_connection(from, to);
    }

    pub fn remove_connection(&mut self, from: SocketId, to: SocketId) {
        self.graph.connections.retain(|conn| {
            !(conn.from == from && conn.to == to)
        });
    }

    pub fn execute(&mut self) -> Vec<EventResult> {
        let results = Vec::new();
        results
    }

    pub fn get_connections_for_node(&self, node_id: u32) -> Vec<&Connection> {
        self.graph.connections.iter().filter(|conn| {
            conn.from.node_id == node_id || conn.to.node_id == node_id
        }).collect()
    }

    pub fn save_json(&self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        self.graph.save_json(path)
    }

    pub fn load_json(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let graph = EventGraph::load_json(path)?;
        Ok(Self {
            graph,
            selected_node: None,
            runtime_context: RuntimeContext::new(),
        })
    }
}

/// Contexto de runtime para variables de estado
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuntimeContext {
    pub variables: HashMap<String, f32>,
    pub flags: HashMap<String, bool>,
    pub counters: HashMap<String, u32>,
}

impl RuntimeContext {
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
            flags: HashMap::new(),
            counters: HashMap::new(),
        }
    }

    pub fn set_variable(&mut self, name: &str, value: f32) {
        self.variables.insert(name.to_string(), value);
    }

    pub fn get_variable(&self, name: &str) -> Option<f32> {
        self.variables.get(name).copied()
    }

    pub fn set_flag(&mut self, name: &str, value: bool) {
        self.flags.insert(name.to_string(), value);
    }

    pub fn get_flag(&self, name: &str) -> Option<bool> {
        self.flags.get(name).copied()
    }

    pub fn increment_counter(&mut self, name: &str) -> u32 {
        let current = self.counters.get(name).copied().unwrap_or(0);
        let new_value = current + 1;
        self.counters.insert(name.to_string(), new_value);
        new_value
    }

    pub fn get_counter(&self, name: &str) -> u32 {
        *self.counters.get(name).unwrap_or(&0)
    }

    pub fn clear(&mut self) {
        self.variables.clear();
        self.flags.clear();
        self.counters.clear();
    }
}

/// Widget UI para EventForge
pub struct EventForgeWidget {
    pub manager: EventNodeManager,
    pub width: f32,
    pub height: f32,
    pub selected_node: Option<u32>,
    pub drag_start: Option<(f32, f32)>,
    pub is_dragging: bool,
}

impl Default for EventForgeWidget {
    fn default() -> Self {
        Self::new()
    }
}

impl EventForgeWidget {
    pub fn new() -> Self {
        Self {
            manager: EventNodeManager::new(),
            width: 800.0,
            height: 600.0,
            selected_node: None,
            drag_start: None,
            is_dragging: false,
        }
    }

    /// Renderiza el widget con egui
    pub fn render(&mut self, ctx: &egui::Context) {
        egui::TopBottomPanel::bottom("event_forge_panel").show(ctx, |ui| {
            ui.heading("Event Forge - Node Graph");
            
            // Botones de control
            ui.horizontal(|ui| {
                if ui.button("➕ Add Node").clicked() {
                    self.add_node();
                }
                
                if ui.button("🗑️ Clear All").clicked() {
                    self.manager.graph.nodes.clear();
                    self.manager.graph.connections.clear();
                    self.selected_node = None;
                }
                
                if ui.button("💾 Save Graph").clicked() {
                    if let Err(e) = self.manager.save_json("event_graph.json") {
                        ui.label(format!("Error: {}", e));
                    } else {
                        ui.label("✅ Saved");
                    }
                }
                
                if ui.button("📂 Load Graph").clicked() {
                    match EventGraph::load_json("event_graph.json") {
                        Ok(graph) => {
                            self.manager.graph = graph;
                            self.selected_node = None;
                        }
                        Err(e) => {
                            ui.label(format!("Error: {}", e));
                        }
                    }
                }
            });
            
            ui.separator();
            
            // Canvas para el grafo
            egui::ScrollArea::both().show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.vertical(|ui| {
                        ui.label(format!("Nodes: {}", self.manager.graph.nodes.len()));
                        ui.label(format!("Connections: {}", self.manager.graph.connections.len()));
                        
                        if let Some(node_id) = self.selected_node {
                            if let Some(node) = self.manager.get_node(node_id) {
                                ui.label(format!("Selected: {:?} (id: {})", node.node_type, node.id));
                                ui.label(format!("Position: ({}, {})", node.position.0, node.position.1));
                                ui.label(format!("Executions: {}", node.execution_count));
                            }
                        }
                    });
                    
                    // Canvas de renderizado
                    let mut response = ui.allocate_response(
                        egui::vec2(ui.available_width() - 200.0, ui.available_height()),
                        egui::Sense::click(),
                    );
                    
                    let rect = response.rect;
                    
                    // Dibujar conexiones primero (debajo de los nodos)
                    self.draw_connections(ui, rect);
                    
                    // Dibujar nodos
                    self.draw_nodes(ctx, rect, ui);
                    
                    // Manejar clics para seleccionar nodos
                    if let Some(pos) = ui.input(|i| i.pointer.hover_pos()) {
                        let (x, y) = (pos.x - rect.min.x, pos.y - rect.min.y);
                        
                        // Verificar si se hizo clic en un nodo
                        let mut clicked_node = false;
                        for (_, node) in &self.manager.graph.nodes {
                            let node_rect = egui::Rect::from_min_size(
                                egui::Pos2::new(node.position.0 - 50.0, node.position.1 - 50.0),
                                egui::Vec2::new(100.0, 100.0)
                            );
                            
                            if node_rect.contains(pos) {
                                self.selected_node = Some(node.id);
                                clicked_node = true;
                                break;
                            }
                        }
                        
                        // Si no se clickeó un nodo, deseleccionar
                        if !clicked_node {
                            self.selected_node = None;
                        }
                    }
                });
            });
        });
    }
    
    /// Dibuja las conexiones entre nodos
    fn draw_connections(&self, ui: &mut egui::Ui, rect: egui::Rect) {
        let scale = 0.8;
        
        for conn in &self.manager.graph.connections {
            let start = self.get_socket_position(&conn.from, scale);
            let end = self.get_socket_position(&conn.to, scale);
            
            // Dibujar línea de conexión con egui
            let line = egui::Shape::line_segment([
                egui::Pos2::new(start.x, start.y),
                egui::Pos2::new(end.x, end.y)
            ], egui::Stroke::new(2.0_f32, egui::Color32::WHITE));
            
            // Dibujar nodos de conexión
            let start_node = self.manager.graph.nodes.get(&conn.from.node_id);
            let end_node = self.manager.graph.nodes.get(&conn.to.node_id);
            
            if let (Some(start_node), Some(end_node)) = (start_node, end_node) {
                // Nodo de salida
                let start_pos = egui::Pos2::new(start_node.position.0, start_node.position.1);
                // let start_circle = egui::Shape::Circle(...);
                
                // Nodo de entrada
                let end_pos = egui::Pos2::new(end_node.position.0, end_node.position.1);
                // let end_circle = egui::Shape::Circle(...);
            }
            
            let painter = ui.painter();
            // Placeholder para círculos de conexión
            painter.add(egui::Shape::line_segment([
                egui::Pos2::new(start.x - 1.0, start.y),
                egui::Pos2::new(start.x + 1.0, start.y)
            ], egui::Stroke::new(1.0, egui::Color32::WHITE)));
            painter.add(egui::Shape::line_segment([
                egui::Pos2::new(end.x - 1.0, end.y),
                egui::Pos2::new(end.x + 1.0, end.y)
            ], egui::Stroke::new(1.0, egui::Color32::WHITE)));
        }
    }

    /// Dibuja los nodos
    fn draw_nodes(&self, ctx: &egui::Context, rect: egui::Rect, ui: &mut egui::Ui) {
        for (_, node) in &self.manager.graph.nodes {
            let x = node.position.0;
            let y = node.position.1;
            
            // Crear nodo UI
            let node_label = match node.node_type {
                NodeType::TriggerZone => "🎯 Trigger Zone",
                NodeType::Dialogue => "💬 Dialogue",
                NodeType::Conditional => "🔀 Conditional",
                NodeType::Cinematic => "🎬 Cinematic",
                NodeType::Action => "⚡ Action",
            };
            
            let mut node_ui = egui::Frame::NONE
                .fill(egui::Color32::from_rgb(50, 50, 80))
                .stroke(egui::Stroke::new(2.0_f32, egui::Color32::DARK_BLUE));
            
            if Some(node.id) == self.selected_node {
                node_ui = node_ui.fill(egui::Color32::from_rgb(80, 100, 150));
            }
            
            egui::Area::new(format!("node_{}", node.id).into())
                .fixed_pos(egui::Pos2::new(x - 60.0, y - 30.0))
                .anchor(egui::Align2::CENTER_CENTER, egui::Vec2::new(0.0, 0.0))
                .show(ctx, |ui| {
                    node_ui.show(ui, |ui| {
                        ui.add_space(5.0);
                        ui.label(node_label);
                        ui.add_space(5.0);
                        ui.label(format!("ID: {}", node.id));
                        ui.label(format!("Exec: {}", node.execution_count));
                    });
                });
        }
    }
    
    /// Obtiene la posición del socket
    fn get_socket_position(&self, socket: &SocketId, scale: f32) -> egui::Pos2 {
        let node = self.manager.graph.nodes.get(&socket.node_id);
        if let Some(node) = node {
            let x = node.position.0;
            let y = node.position.1;
            
            if socket.is_output {
                egui::Pos2::new(x + 50.0 * scale, y + 25.0 * scale)
            } else {
                egui::Pos2::new(x - 50.0 * scale, y + 25.0 * scale)
            }
        } else {
            egui::Pos2::new(0.0, 0.0)
        }
    }

    pub fn add_node(&mut self) {
        let node_type = match self.selected_node {
            Some(_) => {
                // Si hay nodo seleccionado, crear del mismo tipo
                if let Some(node) = self.manager.get_node(self.selected_node.unwrap()) {
                    node.node_type
                } else {
                    NodeType::TriggerZone
                }
            }
            None => NodeType::TriggerZone,
        };
        
        let mut node = Node::new(
            self.manager.graph.nodes.len() as u32 + 1,
            node_type,
        );
        // Posición aleatoria en el canvas
        node.position = (
            (self.width / 2.0 - 50.0) + ((self.manager.graph.nodes.len() as f32 * 100.0) % 200.0),
            (self.height / 2.0 - 50.0) + ((self.manager.graph.nodes.len() as f32 * 100.0) % 200.0)
        );
        self.manager.add_node(node);
        self.selected_node = None;
    }
}
