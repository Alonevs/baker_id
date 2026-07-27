//! # Cable System UI
//! 
//! Módulo para UI de conexión de cables entre nodos de eventos.

use crate::event_nodes::{Edge, EventNode};
use crate::event_node_manager::EventNodeManager;
use eframe::egui;

/// UI de cables para conexión entre nodos
pub struct CableSystemUI {
    pub manager: EventNodeManager,
    pub dragging: bool,
    pub start_node: Option<String>,
}

impl CableSystemUI {
    /// Crea un nuevo sistema de cables
    pub fn new() -> Self {
        Self {
            manager: EventNodeManager::new(),
            dragging: false,
            start_node: None,
        }
    }

    /// Inicia el arrastre de un cable desde un nodo
    pub fn start_drag(&mut self, node_id: &str) {
        self.dragging = true;
        self.start_node = Some(node_id.to_string());
    }

    /// Finaliza el arrastre de un cable
    pub fn end_drag(&mut self, target_node: &str) {
        if !self.dragging {
            return;
        }

        if self.start_node.as_deref() == Some(target_node) {
            return;
        }

        let edge = Edge {
            from: self.start_node.clone().unwrap_or_default(),
            to: target_node.to_string(),
            condition: None,
        };

        self.manager.edges.push(edge);

        self.dragging = false;
        self.start_node = None;
    }

    /// Elimina un cable
    pub fn remove_edge(&mut self, from: &str, to: &str) {
        self.manager.edges.retain(|e| e.from != from || e.to != to);
    }

    /// Renderiza el sistema de cables
    pub fn render(&mut self, ui: &mut egui::Ui, bounds: egui::Rect) {
        self.render_with_drag(ui, bounds);
    }

    /// Dibuja un cable entre dos nodos
    fn draw_edge(&self, ui: &mut egui::Ui, bounds: egui::Rect, edge: &Edge) {
        if let (Some(from_pos), Some(to_pos)) = (
            self.get_node_position(bounds, &edge.from),
            self.get_node_position(bounds, &edge.to)
        ) {
            let color = egui::Color32::from_rgb(100, 150, 255);
            let stroke = egui::Stroke::new(2.0_f32, color);
            
            ui.painter().line_segment([from_pos, to_pos], stroke);
        }
    }

    /// Obtiene la posición de un nodo
    fn get_node_position(&self, _bounds: egui::Rect, node_id: &str) -> Option<egui::Pos2> {
        if let Some(node) = self.manager.get_node(node_id) {
            Some(node.position)
        } else {
            None
        }
    }

    /// Obtiene el manager
    pub fn get_manager(&self) -> &EventNodeManager {
        &self.manager
    }

    /// Obtiene el manager mutado
    pub fn get_manager_mut(&mut self) -> &mut EventNodeManager {
        &mut self.manager
    }

    /// Maneja el inicio del drag desde un nodo
    pub fn on_drag_start(&mut self, item_id: &str, data: &str) -> Option<String> {
        if data.starts_with("node_") {
            let node_id = data.strip_prefix("node_").unwrap();
            self.start_drag(node_id);
            Some(format!("cable_{}", item_id))
        } else {
            None
        }
    }

    /// Maneja el drop en un nodo
    pub fn on_drop(&mut self, _data: &str, target_id: &str) -> bool {
        if self.dragging {
            self.end_drag(target_id);
            true
        } else {
            false
        }
    }

    /// Renderiza con drag & drop
    pub fn render_with_drag(&mut self, ui: &mut egui::Ui, bounds: egui::Rect) {
        let nodes = self.manager.get_all_nodes();

        for edge in &self.manager.edges {
            self.draw_edge(ui, bounds, edge);
        }

        for node in nodes {
            self.draw_node_with_connections(ui, bounds, node);
        }
    }

    /// Dibuja un nodo con conexiones
    fn draw_node_with_connections(&self, ui: &mut egui::Ui, bounds: egui::Rect, node: &EventNode) {
        let pos = self.get_node_position(bounds, &node.id).unwrap();
        let size = egui::Vec2::new(80.0, 40.0);

        // Crear nodo con salida para drag
        let response = ui.allocate_rect(
            egui::Rect::from_min_size(pos, size),
            egui::Sense::click_and_drag()
        );

        // Dibujar fondo del nodo
        if response.hovered() {
            ui.painter().rect_filled(
                response.rect.shrink(2.0),
                egui::CornerRadius::same(5),
                egui::Color32::from_rgb(200, 220, 255),
            );
        } else {
            ui.painter().rect_filled(
                response.rect.shrink(2.0),
                egui::CornerRadius::same(5),
                egui::Color32::from_rgb(240, 240, 240),
            );
        }

        ui.add_space(5.0);
        ui.label(&node.id);
        
        // Dibujar salida de cable desde el nodo
        let output_port = egui::vec2(pos.x + size.x - 10.0, pos.y + size.y / 2.0);
        
        // Obtener conexiones salientes de este nodo
        let outgoing_edges: Vec<&Edge> = self.manager.edges.iter()
            .filter(|e| e.from == node.id)
            .collect();
        
        // Dibujar líneas de salida
        for edge in &outgoing_edges {
            if let Some(target_pos) = self.get_node_position(bounds, &edge.to) {
                self.draw_cable_segment(ui, output_port, target_pos, &edge.condition);
            }
        }
        
        // Mostrar tooltip de drag
        if response.hovered() {
            ui.label("Drag from output to connect");
        }
    }
    
    /// Dibuja un segmento de cable
    fn draw_cable_segment(&self, ui: &mut egui::Ui, from: egui::Pos2, to: egui::Pos2, condition: &Option<String>) {
        let color = match condition {
            Some(_) => egui::Color32::from_rgb(255, 100, 100), // Red para condiciones
            None => egui::Color32::from_rgb(100, 150, 255), // Azul normal
        };
        let stroke = egui::Stroke::new(2.0_f32, color);
        
        ui.painter().line_segment([from, to], stroke);
    }
}

impl Default for CableSystemUI {
    fn default() -> Self {
        Self::new()
    }
}