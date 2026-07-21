use crate::event_nodes::{EventNode, EventType, NodeData};
use egui::{Color32, RichText, Vec2};

/// Renderizador de nodos de evento
pub struct EventNodeRenderer {
    node_size: Vec2,
    highlight_color: Color32,
    node_color: Color32,
}

impl EventNodeRenderer {
    pub fn new() -> Self {
        EventNodeRenderer {
            node_size: Vec2::new(150.0, 40.0),
            highlight_color: Color32::from_rgb(0, 120, 255),
            node_color: Color32::from_rgb(255, 200, 0),
        }
    }
    
    pub fn render_node_list(&mut self, nodes: &HashMap<String, EventNode>, selected: Option<&String>) -> egui::Response {
        let mut response = egui::ScrollArea::vertical()
            .show(egui::CentralPanel::default().frame(egui::Frame {
                fill: Color32::from_rgb(30, 30, 30),
                inner_margin: egui::Margin::same(5.0),
                rounding: egui::Rounding::same(5.0),
            }), |ctx, resp| {
                for (id, node) in nodes {
                    let is_selected = selected == Some(id);
                    let node_type_color = self.get_node_type_color(&node.event_type);
                    
                    egui::Frame::none()
                        .fill(node_type_color)
                        .stroke(egui::Stroke { color: if is_selected { self.highlight_color } else { Color32::BLACK }, width: 1.0 })
                        .response(
                            egui::Response::same()
                        )
                        .on_hover_ui(|ui| {
                            ui.label(format!("{} - {}", node.id, node.event_type));
                        })
                        .show(ctx, |ui| {
                            ui.horizontal(|ui| {
                                ui.label(RichText::new(&node.event_type).size(12.0));
                                if ui.button("X").clicked() {
                                    // Trigger delete
                                }
                            });
                        });
                }
            });
        
        response
    }
    
    pub fn render_node_editor(&mut self, node: &mut EventNode, node_type_color: Color32) -> egui::Response {
        egui::Frame::none()
            .fill(node_type_color)
            .stroke(egui::Stroke { color: Color32::BLACK, width: 1.0 })
            .inner_margin(egui::Margin::same(10.0))
            .show(egui::CentralPanel::default(), |ui| {
                ui.vertical_centered(|ui| {
                    ui.label(RichText::new(&node.event_type).size(14.0).strong());
                    ui.add_space(5.0);
                    
                    match &node.event_type {
                        EventType::OnStart => self.render_on_start(node, ui),
                        EventType::OnClick => self.render_on_click(node, ui),
                        EventType::OnTimer => self.render_on_timer(node, ui),
                        EventType::OnEnter => self.render_on_enter(node, ui),
                        EventType::OnExit => self.render_on_exit(node, ui),
                        EventType::ShowDialog => self.render_show_dialog(node, ui),
                        EventType::PlayAnimation => self.render_play_animation(node, ui),
                        EventType::ChangeVariable => self.render_change_variable(node, ui),
                        EventType::IfCondition => self.render_if_condition(node, ui),
                        EventType::AndCondition => self.render_and_condition(node, ui),
                        EventType::OrCondition => self.render_or_condition(node, ui),
                        _ => ui.label("No configuration available"),
                    }
                });
            })
    }
    
    pub fn render_node_properties(&mut self, node: &EventNode, node_type_color: Color32) -> egui::Response {
        egui::Frame::none()
            .fill(node_type_color)
            .stroke(egui::Stroke { color: Color32::BLACK, width: 1.0 })
            .inner_margin(egui::Margin::same(10.0))
            .show(egui::CentralPanel::default(), |ui| {
                ui.vertical_centered(|ui| {
                    ui.label(format!("Node: {}", node.id));
                    ui.label(format!("Type: {}", node.event_type));
                    ui.label(format!("Executions: {}", node.execution_count));
                    ui.add_space(5.0);
                    
                    egui::CollapsingHeader::new("Properties")
                        .default_open(true)
                        .show(ui, |ui| {
                            self.render_node_data(ui, &node.data, &node.event_type);
                        });
                });
            })
    }
    
    pub fn render_edge(&mut self, edge: &Edge) -> egui::Response {
        egui::Frame::none()
            .stroke(egui::Stroke { 
                color: if edge.condition.is_some() { Color32::RED } else { Color32::WHITE },
                width: 2.0 
            })
            .show(egui::CentralPanel::default(), |ui| {
                ui.horizontal(|ui| {
                    ui.label(format!("{} → {}", edge.from, edge.to));
                    if let Some(condition) = &edge.condition {
                        ui.label(format!("({})", condition));
                    }
                });
            })
    }
    
    fn get_node_type_color(&self, event_type: &EventType) -> Color32 {
        match event_type {
            EventType::OnStart | EventType::OnClick | EventType::OnTimer | EventType::OnEnter | EventType::OnExit |
            EventType::OnCollision | EventType::OnTrigger | EventType::OnKeyPress |
            EventType::OnMouseOver | EventType::OnMouseClick => Color32::GREEN,
            EventType::ShowDialog | EventType::PlayAnimation | EventType::StopAnimation | EventType::PauseAnimation | EventType::ResumeAnimation |
            EventType::ChangeVariable | EventType::CallFunction | EventType::SetFlag | EventType::ClearFlag |
            EventType::SpawnEntity | EventType::RemoveEntity | EventType::PlaySound |
            EventType::ChangeColor | EventType::ChangePosition | EventType::Rotate |
            EventType::Scale => Color32::BLUE,
            EventType::IfCondition | EventType::WhileLoop | EventType::ForLoop | EventType::Switch |
            EventType::AndCondition | EventType::OrCondition | EventType::NotCondition => Color32::YELLOW,
            EventType::Empty => Color32::GRAY,
        }
    }
    
    fn render_on_start(&self, node: &EventNode, ui: &mut egui::Ui) {
        ui.checkbox(&mut node.data, "Auto Execute");
    }
    
    fn render_on_click(&self, node: &EventNode, ui: &mut egui::Ui) {
        ui.label("Target:");
        ui.text_edit_singleline(&mut node.data).on_hover_text("Target object");
        ui.label("Button:");
        ui.add(egui::Slider::new(&mut node.data, 0..=1).text("Button"));
    }
    
    fn render_on_timer(&self, node: &EventNode, ui: &mut egui::Ui) {
        ui.label("Interval (ms):");
        ui.add(egui::Slider::new(&mut node.data, 100..=10000).text("ms"));
        ui.label("Count:");
        ui.add(egui::Slider::new(&mut node.data, 1..=1000).text("count"));
    }
    
    fn render_on_enter(&self, node: &EventNode, ui: &mut egui::Ui) {
        ui.label("Target:");
        ui.text_edit_singleline(&mut node.data).on_hover_text("Target object");
    }
    
    fn render_on_exit(&self, node: &EventNode, ui: &mut egui::Ui) {
        ui.label("Target:");
        ui.text_edit_singleline(&mut node.data).on_hover_text("Target object");
    }
    
    fn render_show_dialog(&self, node: &EventNode, ui: &mut egui::Ui) {
        ui.label("Dialog ID:");
        ui.text_edit_singleline(&mut node.data).on_hover_text("Dialog identifier");
        ui.label("Language:");
        ui.text_edit_singleline(&mut node.data).on_hover_text("Language code");
    }
    
    fn render_play_animation(&self, node: &EventNode, ui: &mut egui::Ui) {
        ui.label("Target:");
        ui.text_edit_singleline(&mut node.data).on_hover_text("Target object");
        ui.label("Animation:");
        ui.text_edit_singleline(&mut node.data).on_hover_text("Animation name");
        ui.label("Duration (ms):");
        ui.add(egui::Slider::new(&mut node.data, 100..=5000).text("ms"));
    }
    
    fn render_change_variable(&self, node: &EventNode, ui: &mut egui::Ui) {
        ui.label("Variable:");
        ui.text_edit_singleline(&mut node.data).on_hover_text("Variable name");
        ui.label("Operation:");
        ui.text_edit_singleline(&mut node.data).on_hover_text("Add, Subtract, Multiply, Divide, Assign");
        ui.label("Value:");
        ui.text_edit_singleline(&mut node.data).on_hover_text("Value to set");
    }
    
    fn render_if_condition(&self, node: &EventNode, ui: &mut egui::Ui) {
        ui.label("Condition:");
        ui.text_edit_singleline(&mut node.data).on_hover_text("Condition expression");
        ui.label("Then:");
        ui.text_edit_singleline(&mut node.data).on_hover_text("Then branch");
        ui.label("Else:");
        ui.text_edit_singleline(&mut node.data).on_hover_text("Else branch");
    }
    
    fn render_node_data(&self, ui: &mut egui::Ui, data: &NodeData, event_type: &EventType) {
        match event_type {
            EventType::OnStart => match data {
                NodeData::OnStart { auto_execute } => {
                    ui.checkbox(auto_execute, "Auto Execute");
                }
                _ => {}
            },
            EventType::OnClick => match data {
                NodeData::OnClick { target, button } => {
                    ui.label("Target:");
                    ui.text_edit_singleline(target);
                    ui.label("Button:");
                    ui.add(egui::Slider::new(button, 0..=1).text("Button"));
                }
                _ => {}
            },
            EventType::OnTimer => match data {
                NodeData::OnTimer { interval_ms, count } => {
                    ui.label("Interval (ms):");
                    ui.add(egui::Slider::new(interval_ms, 100..=10000).text("ms"));
                    ui.label("Count:");
                    ui.add(egui::Slider::new(count, 1..=1000).text("count"));
                }
                _ => {}
            },
            EventType::OnEnter => match data {
                NodeData::OnEnter { target } => {
                    ui.label("Target:");
                    ui.text_edit_singleline(target);
                }
                _ => {}
            },
            EventType::OnExit => match data {
                NodeData::OnExit { target } => {
                    ui.label("Target:");
                    ui.text_edit_singleline(target);
                }
                _ => {}
            },
            EventType::ShowDialog => match data {
                NodeData::ShowDialog { dialog_id, language } => {
                    ui.label("Dialog ID:");
                    ui.text_edit_singleline(dialog_id);
                    ui.label("Language:");
                    ui.text_edit_singleline(language);
                }
                _ => {}
            },
            EventType::PlayAnimation => match data {
                NodeData::PlayAnimation { target, animation, duration_ms } => {
                    ui.label("Target:");
                    ui.text_edit_singleline(target);
                    ui.label("Animation:");
                    ui.text_edit_singleline(animation);
                    ui.label("Duration (ms):");
                    ui.add(egui::Slider::new(duration_ms, 100..=5000).text("ms"));
                }
                _ => {}
            },
            EventType::ChangeVariable => match data {
                NodeData::ChangeVariable { variable, operation, value } => {
                    ui.label("Variable:");
                    ui.text_edit_singleline(variable);
                    ui.label("Operation:");
                    ui.text_edit_singleline(operation);
                    ui.label("Value:");
                    ui.text_edit_singleline(value);
                }
                _ => {}
            },
            EventType::IfCondition => match data {
                NodeData::IfCondition { condition, then_branch, else_branch } => {
                    ui.label("Condition:");
                    ui.text_edit_singleline(condition);
                    ui.label("Then:");
                    ui.text_edit_singleline(then_branch);
                    ui.label("Else:");
                    ui.text_edit_singleline(else_branch);
                }
                _ => {}
            },
            EventType::AndCondition => match data {
                NodeData::AndCondition { conditions } => {
                    ui.label("Conditions:");
                    ui.text_edit_singleline(conditions.join(" AND "));
                }
                _ => {}
            },
            EventType::OrCondition => match data {
                NodeData::OrCondition { conditions } => {
                    ui.label("Conditions:");
                    ui.text_edit_singleline(conditions.join(" OR "));
                }
                _ => {}
            },
            _ => ui.label("No data configuration"),
        }
    }
}

impl Default for EventNodeRenderer {
    fn default() -> Self {
        Self::new()
    }
}

