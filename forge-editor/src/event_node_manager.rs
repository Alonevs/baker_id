//! # Event Node Manager Stub
//! 
//! Módulo temporal para manager de nodos de evento.

use eframe::egui;
use crate::event_nodes::{EventNode, Edge as EventEdge};

/// Event Node Manager stub
#[derive(Default)]
pub struct EventNodeManager {
    pub nodes: Vec<EventNode>,
    pub edges: Vec<EventEdge>,
    pub selected_node: Option<String>,
}

impl EventNodeManager {
    pub fn new() -> Self {
        Self::default()
    }
    
    pub fn ui(&self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("Event Node Manager");
        });
    }

    pub fn get_all_nodes(&self) -> &[EventNode] {
        &self.nodes
    }

    pub fn get_edges(&self) -> &[EventEdge] {
        &self.edges
    }

    pub fn get_node(&self, id: &str) -> Option<&EventNode> {
        self.nodes.iter().find(|n| n.id == id)
    }

    pub fn get_node_mut(&mut self, id: &str) -> Option<&mut EventNode> {
        self.nodes.iter_mut().find(|n| n.id == id)
    }

    pub fn delete_node(&mut self, id: &str) -> bool {
        let len_before = self.nodes.len();
        self.nodes.retain(|n| n.id != id);
        self.edges.retain(|e| e.from != id && e.to != id);
        self.nodes.len() < len_before
    }

    pub fn execute_node(&mut self, _id: String) {
        // stub
    }

    pub fn execute_all(&mut self) {
        // stub
    }

    pub fn get_execution_counts(&self) -> std::collections::HashMap<String, usize> {
        std::collections::HashMap::new()
    }

    pub fn create_node(&mut self, event_type: crate::event_nodes::EventType, _position: (f32, f32)) -> String {
        let id = format!("node_{}", self.nodes.len() + 1);
        self.nodes.push(EventNode {
            id: id.clone(),
            event_type,
            position: egui::Pos2::new(_position.0, _position.1),
            execution_count: 0,
            is_active: false,
            data: crate::event_nodes::NodeData::Empty,
        });
        id
    }
}

