//! # Menu Bar con menús contextuales
//! 
//! Menú de File con soporte de guardar/cargar escena y proyectos.

use eframe::egui;
use rfd::AsyncFileDialog;
use std::fs;
use std::path::Path;
use crate::forge_scene_stub::{Scene};
use serde::{Deserialize, Serialize};
use crate::project_manager::{ProjectManager, ProjectWizard, GameType};

/// Nodos de escena para exportación
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SceneNode {
    pub id: String,
    pub name: String,
    pub entity_type: String,
    pub parent_id: Option<String>,
    pub transform: Transform,
    pub properties: std::collections::HashMap<String, serde_json::Value>,
    pub components: Vec<Component>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transform {
    pub position: [f32; 3],
    pub rotation: f32,
    pub scale: [f32; 3],
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Component {
    pub name: String,
    pub data: serde_json::Value,
}

/// Menu Bar con menús contextuales
#[derive(Debug, Clone)]
pub struct MenuBar {
    pub menus: Vec<String>,
    pub selected_menu: usize,
    pub active_submenu: Option<usize>,
    pub show_menu: bool,
}

impl Default for MenuBar {
    fn default() -> Self {
        Self {
            menus: vec![
                "File".to_string(),
                "Edit".to_string(),
                "View".to_string(),
                "Physics".to_string(),
                "Particles".to_string(),
                "Animation".to_string(),
                "Help".to_string(),
            ],
            selected_menu: 0,
            active_submenu: None,
            show_menu: true,
        }
    }
}

impl MenuBar {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn ui(&mut self, ui: &mut egui::Ui, app: &mut crate::ForgeEditorApp) {
        ui.menu_button("File", |ui| {
            // New Project
            if ui.button("New Project...").clicked() {
                let new_project = ProjectWizard::new(
                    "Nuevo Proyecto".to_string(),
                    GameType::Isometric,
                    std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from("assets"))
                );
                new_project.execute();
            }

            // Open Project
            if ui.button("Open Project...").clicked() {
                if let Some(path) = pollster::block_on(AsyncFileDialog::new().pick_file()) {
                    let path_buf = path.path().to_path_buf();
                    if let Err(e) = app.project_manager.open_project(&path_buf) {
                        app.console.add_message(
                            crate::debugger::LogLevel::Error,
                            &format!("Error opening project: {}", e)
                        );
                    } else {
                        app.console.add_message(
                            crate::debugger::LogLevel::Info,
                            &format!("Project opened from: {}", path_buf.display())
                        );
                        // Cargar assets del proyecto en el Asset Browser
                        app.load_project_assets();
                    }
                }
            }

            // New Project (also loads default assets)
            if ui.button("New Project").clicked() {
                let mut new_project = ProjectWizard::new(
                    "Nuevo Proyecto".to_string(),
                    GameType::Isometric,
                    std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from("assets"))
                );
                new_project.execute();
                // Cargar assets del nuevo proyecto
                app.load_project_assets();
            }

            // New Scene
            if ui.button("New Scene").clicked() {
                app.scene_tree.tree.clear();
                app.scene_tree.nodes.clear();
                app.scene_tree.active_nodes.clear();
                app.console.add_message(
                    crate::debugger::LogLevel::Info,
                    "Created new empty scene"
                );
            }

            // Save Scene
            if ui.button("Save Scene").clicked() {
                if let Some(path) = save_scene(app) {
                    app.console.add_message(
                        crate::debugger::LogLevel::Info,
                        &format!("Scene saved to: {}", path.display())
                    );
                }
            }

            // Save Project
            if ui.button("Save Project").clicked() {
                if let Err(e) = app.project_manager.save_project() {
                    app.console.add_message(
                        crate::debugger::LogLevel::Error,
                        &format!("Error saving project: {}", e)
                    );
                } else {
                    app.console.add_message(
                        crate::debugger::LogLevel::Info,
                        "Project saved successfully"
                    );
                }
            }

            // Save As
            if ui.button("Save Scene As...").clicked() {
                if let Some(path) = save_scene_as(app) {
                    app.console.add_message(
                        crate::debugger::LogLevel::Info,
                        &format!("Scene saved to: {}", path.display())
                    );
                }
            }

            // Open Scene
            if ui.button("Open Scene...").clicked() {
                if let Some(path) = open_scene(app) {
                    app.console.add_message(
                        crate::debugger::LogLevel::Info,
                        &format!("Scene loaded from: {}", path.display())
                    );
                }
            }

            ui.separator();

            // Exit
            if ui.button("Exit").clicked() {
                std::process::exit(0);
            }
        });
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SceneFile {
    pub nodes: Vec<SceneNode>,
    pub root_id: Option<String>,
    pub groups: std::collections::HashMap<String, Vec<String>>,
    pub animations: std::collections::HashMap<String, Vec<String>>,
}

/// Guarda la escena actual como archivo JSON
fn save_scene(app: &mut crate::ForgeEditorApp) -> Option<std::path::PathBuf> {
    let folder = pollster::block_on(rfd::AsyncFileDialog::new().pick_folder());
    let _save_dir = folder.map(|d| d.path().to_path_buf())?;

    let file = pollster::block_on(rfd::AsyncFileDialog::new().save_file());
    let path = file.map(|f| f.path().to_path_buf())?;

    // Convertir nodos de la escena a formato JSON
    let nodes: Vec<SceneNode> = app
        .scene_tree
        .get_tree()
        .iter()
        .map(|node| {
            let entity_type = format!("{:?}", node.entity_type);
            let mut properties = std::collections::HashMap::new();
            for (k, v) in &node.properties {
                properties.insert(k.clone(), serde_json::Value::String(v.clone()));
            }

            SceneNode {
                id: node.id.to_string(),
                name: node.name.clone(),
                entity_type,
                parent_id: node.parent_id.map(|id| id.to_string()),
                transform: Transform {
                    position: [node.transform.position[0], node.transform.position[1], 0.0],
                    rotation: node.transform.rotation,
                    scale: [node.transform.scale[0], node.transform.scale[1], 1.0],
                },
                properties,
                components: node
                    .components
                    .iter()
                    .map(|comp| Component {
                        name: comp.component_type.to_string(),
                        data: serde_json::Value::Null,
                    })
                    .collect(),
            }
        })
        .collect();

    let scene_data = SceneFile {
        nodes,
        root_id: None,
        groups: std::collections::HashMap::new(),
        animations: std::collections::HashMap::new(),
    };

    let json = serde_json::to_string_pretty(&scene_data).unwrap();
    fs::write(&path, json).unwrap();

    Some(path)
}

/// Guarda la escena como nuevo archivo
fn save_scene_as(app: &mut crate::ForgeEditorApp) -> Option<std::path::PathBuf> {
    let file = pollster::block_on(rfd::AsyncFileDialog::new().save_file());
    let path = file.map(|f| f.path().to_path_buf())?;

    // Convertir nodos de la escena a formato JSON
    let nodes: Vec<SceneNode> = app
        .scene_tree
        .get_tree()
        .iter()
        .map(|node| {
            let entity_type = format!("{:?}", node.entity_type);
            let mut properties = std::collections::HashMap::new();
            for (k, v) in &node.properties {
                properties.insert(k.clone(), serde_json::Value::String(v.clone()));
            }

            SceneNode {
                id: node.id.to_string(),
                name: node.name.clone(),
                entity_type,
                parent_id: node.parent_id.map(|id| id.to_string()),
                transform: Transform {
                    position: [node.transform.position[0], node.transform.position[1], 0.0],
                    rotation: node.transform.rotation,
                    scale: [node.transform.scale[0], node.transform.scale[1], 1.0],
                },
                properties,
                components: node
                    .components
                    .iter()
                    .map(|comp| Component {
                        name: comp.component_type.to_string(),
                        data: serde_json::Value::Null,
                    })
                    .collect(),
            }
        })
        .collect();

    let scene_data = SceneFile {
        nodes,
        root_id: None,
        groups: std::collections::HashMap::new(),
        animations: std::collections::HashMap::new(),
    };

    let json = serde_json::to_string_pretty(&scene_data).unwrap();
    fs::write(&path, json).unwrap();

    Some(path)
}

/// Abre una escena desde archivo JSON
fn open_scene(app: &mut crate::ForgeEditorApp) -> Option<std::path::PathBuf> {
    let file = pollster::block_on(rfd::AsyncFileDialog::new().pick_file());
    let path = file.map(|f| f.path().to_path_buf())?;

    // Limpiar escena actual
    app.scene_tree.tree.clear();
    app.scene_tree.nodes.clear();
    app.scene_tree.active_nodes.clear();

    // Cargar escena desde archivo
    let content = fs::read_to_string(&path).unwrap();
    let scene_data: SceneFile = serde_json::from_str(&content).unwrap();

    // Recargar nodos
    for node in scene_data.nodes {
        let entity_type = match node.entity_type.as_str() {
            "GameObject" => crate::forge_scene_stub::EntityType::GameObject,
            "Sprite" => crate::forge_scene_stub::EntityType::Sprite,
            "Group" => crate::forge_scene_stub::EntityType::Group,
            _ => crate::forge_scene_stub::EntityType::Empty,
        };

        let node_id = uuid::Uuid::parse_str(&node.id).unwrap_or_default();
        let parent_id = node.parent_id.and_then(|id_str| uuid::Uuid::parse_str(&id_str).ok());

        let mut properties = std::collections::HashMap::new();
        for (k, v) in node.properties {
            properties.insert(k, v.as_str().unwrap_or("").to_string());
        }

        let loaded_node = crate::forge_scene_stub::NodeData {
            id: node_id,
            name: node.name.clone(),
            entity_type,
            parent_id,
            transform: crate::forge_scene_stub::Transform {
                position: [node.transform.position[0], node.transform.position[1], 0.0],
                rotation: node.transform.rotation,
                scale: [node.transform.scale[0], node.transform.scale[1], 1.0],
            },
            properties,
            components: node.components.iter().map(|c| crate::forge_scene_stub::ComponentData {
                component_type: crate::forge_scene_stub::ComponentType::Transform,
                data: serde_json::Value::Null,
            }).collect(),
        };

        app.scene_tree.tree.push(loaded_node.clone());
        app.scene_tree.nodes.push(loaded_node);
    }

    Some(path)
}
