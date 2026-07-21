//! # Scene Tree UI
//! 
//! Panel del Scene Tree con soporte para nodos, grupos y jerarquía.

use eframe::egui;
use crate::forge_scene_stub::{NodeData, EntityType, Transform, SceneTree};
use crate::{ForgeEditorApp, debugger::LogLevel};
use uuid::Uuid;
use std::sync::Arc;

/// Panel del Scene Tree
pub struct SceneTreeUI {
    pub tree: SceneTree,
}

impl SceneTreeUI {
    pub fn new(tree: SceneTree) -> Self {
        Self { tree }
    }
}

impl SceneTreeUI {
    /// Renderiza el Scene Tree
    pub fn ui(&mut self, ui: &mut egui::Ui, app: &mut ForgeEditorApp) {
        ui.group(|ui| {
            ui.heading("Scene Tree");
            ui.add_space(5.0);
            
            // Renderizar nodos raíz (los que no tienen parent_id)
            let root_nodes: Vec<Arc<NodeData>> = app.scene_tree.tree.iter()
                .filter(|n| n.parent_id.is_none())
                .cloned()
                .collect();
                
            self.render_tree_nodes(
                ui,
                app,
                &root_nodes,
                0,
            );
        });
    }
    
    /// Renderiza nodos recursivamente
    fn render_tree_nodes(
        &self,
        ui: &mut egui::Ui,
        app: &mut ForgeEditorApp,
        nodes: &[Arc<NodeData>],
        indent: usize,
    ) {
        let indent_str = "  ".repeat(indent);
        
        for node in nodes {
            // Crear label con indentación y nombre
            let label = format!("{}📄 {}", indent_str, node.name);
            
            // Determinar si está activo
            let is_active = app.scene_tree.active_node_id == Some(node.id);
            
            // Crear botón seleccionable para el nodo
            let response = ui.selectable_label(is_active, label);
            
            if response.clicked() {
                app.scene_tree.active_node_id = Some(node.id);
            }
            
            // Botones de acción
            ui.horizontal(|ui| {
                // Botón para crear grupo
                if ui.button("📁").clicked() {
                    let group_name = format!("{}Group", node.name);
                    let group_node = Arc::new(NodeData {
                        id: Uuid::new_v4(),
                        name: group_name.clone(),
                        entity_type: EntityType::Group,
                        parent_id: Some(node.id),
                        transform: Transform::default(),
                        properties: std::collections::HashMap::new(),
                        components: vec![],
                    });
                    
                    app.scene_tree.add_node_to_scene(group_node);
                    app.console.add_message(
                        LogLevel::Info,
                        &format!("Created group: {}", group_name)
                    );
                }
                
                // Botón para eliminar
                if ui.button("×").clicked() {
                    app.scene_tree.remove_node(node.id);
                    app.console.add_message(
                        LogLevel::Info,
                        &format!("Removed node: {}", node.name)
                    );
                }

                // Botón para mover
                
                ui.add_space(10.0);
                
                // Botón para mover
                if ui.button("🔄").clicked() {
                    let other_nodes: Vec<Arc<NodeData>> = app.scene_tree.tree.iter()
                        .filter(|n| n.id != node.id)
                        .cloned()
                        .collect();
                    
                    for parent_node in other_nodes {
                        if ui.selectable_label(false, &parent_node.name).clicked() {
                            if let Some(pos) = app.scene_tree.tree.iter().position(|n| n.id == node.id) {
                                app.scene_tree.tree[pos].parent_id = Some(parent_node.id);
                            }
                            app.console.add_message(
                                LogLevel::Info,
                                &format!("Moved node to: {}", parent_node.name)
                            );
                        }
                    }
                }
            });
            
            // Renderizar hijos recursivamente
            let child_nodes: Vec<Arc<NodeData>> = app.scene_tree.tree.iter()
                .filter(|n| n.parent_id == Some(node.id))
                .cloned()
                .collect();
            
            if !child_nodes.is_empty() {
                self.render_tree_nodes(
                    ui,
                    app,
                    &child_nodes,
                    indent + 1,
                );
            }
        }
    }
}

