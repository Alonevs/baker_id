//! # Event Node Editor UI
//! 
//! Editor visual para nodos de eventos con soporte para:
//! - Creación y edición de nodos
//! - Conexión de nodos con cables
//! - Ejecución de grafos de eventos
//! - Visualización de variables y flags

use crate::event_node_manager::EventNodeManager;
use crate::event_nodes::{EventNode, EventType, NodeData, Edge, EventGraph, VariableOperation};
use crate::cable_system::CableSystem;
use egui::{Color32, RichText, Vec2, Response, Ui, Sense, Frame, Margin, Widget};
use egui::epaint;

/// Editor de nodos de evento
pub struct EventNodeEditor {
    manager: EventNodeManager,
    selected_node: Option<String>,
    show_properties: bool,
    show_nodes_list: bool,
    execution_count: u32,
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
        }
    }

    /// Renderiza el editor completo
    pub fn render(&mut self, ui: &mut Ui, app: &mut crate::ForgeEditorApp) {
        ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
            // Panel izquierdo: Lista de nodos
            if self.show_nodes_list {
                self.render_nodes_list(ui);
            }

            // Panel central: Visualización del grafo
            self.render_graph_view(ui);

            // Panel derecho: Propiedades del nodo seleccionado
            if self.show_properties {
                self.render_properties_panel(ui);
            }

            // Panel inferior: Estadísticas
            self.render_statistics(ui);
        });
    }

    /// Renderiza la lista de nodos
    fn render_nodes_list(&mut self, ui: &mut Ui) {
        let collapsed = ui.ctx().data_mut(|d| {
            let val = d.get_temp_mut_or_default::<bool>(egui::Id::new("nodes_list_collapsed"));
            *val = !*val;
            *val
        });

        egui::CollapsingHeader::new("Event Nodes")
            .default_open(!collapsed)
            .show(ui, |ui| {
                let mut delete_id = None;
                let mut select_id = None;
                
                let nodes = self.manager.get_all_nodes();
                
                egui::ScrollArea::vertical()
                    .show(ui, |ui| {
                        for node in nodes {
                            let id = &node.id;
                            let is_selected = self.selected_node.as_ref() == Some(id);
                            let node_type_color = self.get_node_type_color(&node.event_type);
                            
                            let frame = egui::Frame::none()
                                .fill(node_type_color)
                                .stroke(egui::Stroke { 
                                    color: if is_selected { Color32::YELLOW } else { Color32::from_rgb(50, 50, 50) }, 
                                    width: 1.0 
                                })
                                .inner_margin(egui::Margin::same(5));
                            
                            let response = frame.show(ui, |ui| {
                                ui.horizontal(|ui| {
                                    ui.label(RichText::new(format!("{:?}", node.event_type)).size(11.0));
                                    ui.label(format!("{} ({})", node.id, node.execution_count));
                                    
                                    if ui.button("X").clicked() {
                                        delete_id = Some(id.clone());
                                    }
                                });
                            });
                            
                            if response.response.clicked() {
                                select_id = Some(id.clone());
                            }
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
            });
    }

    /// Renderiza la vista del grafo
    fn render_graph_view(&mut self, ui: &mut Ui) {
        let response = ui.allocate_response(ui.available_size(), Sense::click());
        
        // Renderizar nodos y conexiones
        self.render_graph_nodes_and_edges(ui, &response);
    }

    /// Renderiza nodos y conexiones del grafo
    fn render_graph_nodes_and_edges(&mut self, ui: &mut Ui, response: &Response) {
        let nodes = self.manager.get_all_nodes();
        let edges = self.manager.get_edges();
        
        // Dibujar conexiones primero (debajo de los nodos)
        for edge in edges {
            let from_node = nodes.iter().find(|n| n.id == edge.from);
            let to_node = nodes.iter().find(|n| n.id == edge.to);
            if let (Some(from_node), Some(to_node)) = (from_node, to_node) {
                self.draw_edge(
                    ui.ctx(),
                    Vec2::new(from_node.position.x, from_node.position.y),
                    Vec2::new(to_node.position.x, to_node.position.y),
                    edge.condition.as_ref().map(|_| Color32::RED).unwrap_or(Color32::WHITE),
                );
            }
        }

        // Dibujar nodos
        for node in nodes {
            let id = &node.id;
            let is_selected = self.selected_node.as_ref() == Some(id);
            let node_type_color = self.get_node_type_color(&node.event_type);
            
            let rect = egui::Rect::from_min_size(
                egui::Pos2::new(node.position.x, node.position.y),
                Vec2::new(150.0, 40.0)
            );
            
            let stroke = egui::Stroke { 
                color: if is_selected { Color32::YELLOW } else { Color32::from_rgb(50, 50, 50) }, 
                width: 1.0 
            };
            ui.painter().rect(rect, 4.0, node_type_color, stroke, egui::StrokeKind::Outside);
            let resp = ui.allocate_rect(rect, Sense::click());
            
            // Dibujar texto del nodo
            ui.allocate_ui(Vec2::new(150.0, 20.0), |ui| {
                ui.horizontal(|ui| {
                    ui.label(RichText::new(format!("{:?}", node.event_type)).size(11.0));
                });
            });
        }
    }

    /// Dibuja una conexión entre nodos
    fn draw_edge(&self, _ctx: &egui::Context, from: Vec2, to: Vec2, color: Color32) {
        let _callback = egui::epaint::CubicBezierShape::from_points_stroke(
            [
                egui::Pos2::new(from.x, from.y),
                egui::Pos2::new(from.x + 50.0, from.y),
                egui::Pos2::new(to.x - 50.0, to.y),
                egui::Pos2::new(to.x, to.y),
            ],
            false,
            Color32::TRANSPARENT,
            egui::epaint::Stroke::new(2.0, color),
        );
        
        // Note: This is a stub implementation that doesn't actually render
        // In a real implementation, you would use a custom painter or add to the context
    }

    /// Renderiza el panel de propiedades
    fn render_properties_panel(&mut self, ui: &mut Ui) {
        let collapsed = ui.ctx().data_mut(|d| {
            let val = d.get_temp_mut_or_default::<bool>(egui::Id::new("properties_collapsed"));
            *val = !*val;
            *val
        });

        egui::CollapsingHeader::new("Node Properties")
            .default_open(!collapsed)
            .show(ui, |ui| {
                if let Some(node_id) = &self.selected_node {
                    let mut node_found = None;
                    if let Some(node) = self.manager.get_node_mut(node_id) {
                        Self::render_node_properties(ui, node);
                        node_found = Some(node.id.clone());
                    }
                    
                    if let Some(id) = node_found {
                        ui.add_space(10.0);
                        // Botones de acción
                        ui.horizontal(|ui| {
                            if ui.button("Execute Node").clicked() {
                                self.manager.execute_node(id.clone());
                            }
                            if ui.button("Execute All").clicked() {
                                self.manager.execute_all();
                            }
                            if ui.button("Delete Node").clicked() {
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
                    EventType::OnStart => Self::render_on_start_properties(ui, node),
                    EventType::OnClick => Self::render_on_click_properties(ui, node),
                    EventType::OnTimer => Self::render_on_timer_properties(ui, node),
                    EventType::ShowDialog => Self::render_show_dialog_properties(ui, node),
                    EventType::PlayAnimation => Self::render_play_animation_properties(ui, node),
                    EventType::ChangeVariable => Self::render_change_variable_properties(ui, node),
                    EventType::IfCondition => Self::render_if_condition_properties(ui, node),
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

    /// Renderiza panel de estadísticas
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

