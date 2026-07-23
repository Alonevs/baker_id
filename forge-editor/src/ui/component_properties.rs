//! # Component Properties API
//! 
//! Módulo para visualización y edición interactiva de propiedades y componentes del Inspector.

use eframe::egui;
use crate::debugger::LogLevel;
use serde_json;

/// Component Properties - propiedades de componentes
#[derive(Debug, Clone)]
pub struct ComponentProperties {
    pub component_type: String,
    pub keys: Vec<String>,
    pub values: Vec<String>,
    pub enabled: bool,
}

impl Default for ComponentProperties {
    fn default() -> Self {
        Self {
            component_type: "None".to_string(),
            keys: Vec::new(),
            values: Vec::new(),
            enabled: true,
        }
    }
}

impl ComponentProperties {
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Renderiza el editor de componentes en la UI
    pub fn render(ui: &mut egui::Ui, app: &mut crate::ForgeEditorApp) {
        ui.heading("Inspector");
        ui.add_space(5.0);
        ui.separator();
        ui.add_space(5.0);
        
        let node_id = match app.active_node_id {
            Some(id) => id,
            None => {
                ui.colored_label(egui::Color32::from_rgb(150, 150, 150), "Selecciona una entidad en el Scene Tree o Viewport para ver y editar sus propiedades.");
                return;
            }
        };
        
        let entity = match app.scene_tree.get_node(&node_id) {
            Some(n) => n,
            None => {
                ui.colored_label(egui::Color32::from_rgb(220, 80, 80), "Error: La entidad seleccionada no existe.");
                return;
            }
        };
        
        // 1. Identificación y Renombrado del Nodo
        ui.horizontal(|ui| {
            ui.label("Nombre:");
            let mut name = entity.name.clone();
            if ui.text_edit_singleline(&mut name).changed() {
                let mut updated = entity.as_ref().clone();
                updated.name = name;
                let arc_node = std::sync::Arc::new(updated);
                app.scene_tree.nodes.insert(node_id, arc_node.clone());
                if app.scene_tree.root.as_ref().map(|n| n.id) == Some(node_id) {
                    app.scene_tree.root = Some(arc_node.clone());
                }
                if let Some(pos) = app.scene_tree.active_nodes.iter().position(|n| n.id == node_id) {
                    app.scene_tree.active_nodes[pos] = arc_node;
                }
            }
        });
        
        ui.label(format!("Tipo de Entidad: {:?}", entity.entity_type));
        ui.add_space(5.0);
        
        // 2. Exportación a Plantilla (Prefab)
        if ui.button("💾 Guardar como Plantilla (Prefab)").clicked() {
            let serialized = serde_json::to_string_pretty(&entity.as_ref()).unwrap();
            let prefabs_dir = std::path::Path::new("assets/prefabs");
            if !prefabs_dir.exists() {
                let _ = std::fs::create_dir_all(prefabs_dir);
            }
            
            let file_name = format!("{}.prefab", entity.name.to_lowercase().replace(' ', "_"));
            let file_path = prefabs_dir.join(&file_name);
            if std::fs::write(&file_path, serialized).is_ok() {
                app.console.add_message(
                    LogLevel::Info,
                    &format!("Plantilla guardada con éxito en 'assets/prefabs/{}'", file_name)
                );
            } else {
                app.console.add_message(
                    LogLevel::Error,
                    &format!("Error al escribir plantilla en 'assets/prefabs/{}'", file_name)
                );
            }
        }
        
        ui.add_space(5.0);
        ui.separator();
        ui.add_space(5.0);
        
        // 3. Edición de Transform
        ui.collapsing("Transform", |ui| {
            let mut pos = entity.transform.transform.position;
            let mut scale = entity.transform.transform.scale;
            let mut changed = false;
            
            ui.horizontal(|ui| {
                ui.label("Posición");
                ui.label("X:");
                changed |= ui.add(egui::DragValue::new(&mut pos[0]).speed(1.0)).changed();
                ui.label("Y:");
                changed |= ui.add(egui::DragValue::new(&mut pos[1]).speed(1.0)).changed();
            });
            
            ui.horizontal(|ui| {
                ui.label("Escala   ");
                ui.label("X:");
                changed |= ui.add(egui::DragValue::new(&mut scale[0]).speed(0.1).range(0.01..=100.0)).changed();
                ui.label("Y:");
                changed |= ui.add(egui::DragValue::new(&mut scale[1]).speed(0.1).range(0.01..=100.0)).changed();
            });
            
            if changed {
                let mut updated = entity.as_ref().clone();
                updated.transform.transform.position = pos;
                updated.transform.transform.scale = scale;
                let arc_node = std::sync::Arc::new(updated);
                app.scene_tree.nodes.insert(node_id, arc_node.clone());
                if app.scene_tree.root.as_ref().map(|n| n.id) == Some(node_id) {
                    app.scene_tree.root = Some(arc_node.clone());
                }
                if let Some(pos) = app.scene_tree.active_nodes.iter().position(|n| n.id == node_id) {
                    app.scene_tree.active_nodes[pos] = arc_node;
                }
            }
        });
        
        ui.add_space(5.0);
        ui.separator();
        ui.add_space(5.0);

        // 3.5. Rejilla de Colisiones para TileMaps (Opción B)
        if entity.entity_type == ::forge_scene::EntityType::TileMap || entity.properties.contains_key("collision_grid") {
            ui.collapsing("Rejilla de Colisiones (TileMap)", |ui| {
                let mut cols = entity.properties.get("grid_cols")
                    .and_then(|v| v.as_u64())
                    .unwrap_or(8) as usize;
                let mut rows = entity.properties.get("grid_rows")
                    .and_then(|v| v.as_u64())
                    .unwrap_or(8) as usize;
                
                let mut grid_changed = false;
                
                ui.horizontal(|ui| {
                    ui.label("Columnas:");
                    grid_changed |= ui.add(egui::DragValue::new(&mut cols).range(2..=32)).changed();
                    ui.label("Filas:");
                    grid_changed |= ui.add(egui::DragValue::new(&mut rows).range(2..=32)).changed();
                });
                
                let total_cells = cols * rows;
                let mut grid_data = vec!["E"; total_cells];
                if let Some(arr_val) = entity.properties.get("collision_grid").and_then(|v| v.as_array()) {
                    for (i, val) in arr_val.iter().enumerate() {
                        if i < total_cells {
                            if let Some(s) = val.as_str() {
                                grid_data[i] = s;
                            }
                        }
                    }
                }
                
                ui.add_space(5.0);
                ui.label("Haz click para conmutar: 🟩 Vacío | 🟥 Sólido | 🟦 Evento");
                
                egui::Grid::new("collision_grid_editor")
                    .spacing(egui::vec2(4.0, 4.0))
                    .show(ui, |ui| {
                        for r in 0..rows {
                            for c in 0..cols {
                                let idx = r * cols + c;
                                let cell_state = grid_data[idx];
                                
                                let (label, _color) = match cell_state {
                                    "S" => ("🟥", egui::Color32::from_rgb(220, 50, 50)),
                                    "T" => ("🟦", egui::Color32::from_rgb(50, 50, 220)),
                                    _ => ("🟩", egui::Color32::from_rgb(50, 180, 50)),
                                };
                                
                                if ui.button(label).clicked() {
                                    let next_state = match cell_state {
                                        "E" => "S",
                                        "S" => "T",
                                        _ => "E",
                                    };
                                    grid_data[idx] = next_state;
                                    grid_changed = true;
                                }
                            }
                            ui.end_row();
                        }
                    });
                
                if grid_changed {
                    let mut updated = entity.as_ref().clone();
                    updated.properties.insert("grid_cols".to_string(), serde_json::Value::Number(serde_json::Number::from(cols)));
                    updated.properties.insert("grid_rows".to_string(), serde_json::Value::Number(serde_json::Number::from(rows)));
                    
                    let json_arr: Vec<serde_json::Value> = grid_data.iter()
                        .map(|s| serde_json::Value::String(s.to_string()))
                        .collect();
                    updated.properties.insert("collision_grid".to_string(), serde_json::Value::Array(json_arr));
                    
                    let arc_node = std::sync::Arc::new(updated);
                    app.scene_tree.nodes.insert(node_id, arc_node.clone());
                    if app.scene_tree.root.as_ref().map(|n| n.id) == Some(node_id) {
                        app.scene_tree.root = Some(arc_node.clone());
                    }
                    if let Some(pos) = app.scene_tree.active_nodes.iter().position(|n| n.id == node_id) {
                        app.scene_tree.active_nodes[pos] = arc_node;
                    }
                }
            });
            ui.add_space(5.0);
            ui.separator();
            ui.add_space(5.0);
        }
        
        // 4. Componentes y propiedades JSON internas
        ui.label(egui::RichText::new("Componentes").strong());
        ui.add_space(5.0);
        
        let mut comp_to_remove = None;
        let mut comp_to_update = None;
        
        for (i, component) in entity.components.iter().enumerate() {
            let mut comp = component.clone();
            let mut comp_changed = false;
            
            let header_title = format!("{:?}", comp.component_type);
            
            ui.group(|ui| {
                ui.horizontal(|ui| {
                    ui.label(egui::RichText::new(&header_title).strong());
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if ui.button("❌").clicked() {
                            comp_to_remove = Some(i);
                        }
                    });
                });
                ui.separator();
                
                match comp.component_type {
                    ::forge_scene::ComponentType::Sprite => {
                        if let Some(obj) = comp.data.as_object_mut() {
                            if let Some(path) = obj.get_mut("texture_path") {
                                if let Some(path_str) = path.as_str() {
                                    let mut path_val = path_str.to_string();
                                    
                                    ui.horizontal(|ui| {
                                        ui.label("Textura:");
                                        let response = ui.text_edit_singleline(&mut path_val);
                                        if response.changed() {
                                            *path = serde_json::Value::String(path_val.clone());
                                            comp_changed = true;
                                        }
                                        
                                        // Soporte de Drag & Drop para asignar textura
                                        if response.hovered() && matches!(app.drag_operation, Some(crate::DragOperation::Asset { .. })) {
                                            if let Some(asset) = &app.dragged_asset {
                                                ui.label(egui::RichText::new("👈 Suelta para asignar").color(egui::Color32::YELLOW));
                                                if ui.input(|i| i.pointer.any_released()) {
                                                    *path = serde_json::Value::String(asset.path.clone());
                                                    comp_changed = true;
                                                }
                                            }
                                        }
                                    });
                                }
                            }
                            if let Some(w) = obj.get_mut("width") {
                                if let Some(w_val) = w.as_f64() {
                                    let mut w_mut = w_val as f32;
                                    ui.horizontal(|ui| {
                                        ui.label("Ancho:");
                                        if ui.add(egui::DragValue::new(&mut w_mut).speed(1.0).range(1.0..=1024.0)).changed() {
                                            *w = serde_json::Value::Number(serde_json::Number::from_f64(w_mut as f64).unwrap());
                                            comp_changed = true;
                                        }
                                    });
                                }
                            }
                            if let Some(h) = obj.get_mut("height") {
                                if let Some(h_val) = h.as_f64() {
                                    let mut h_mut = h_val as f32;
                                    ui.horizontal(|ui| {
                                        ui.label("Alto:");
                                        if ui.add(egui::DragValue::new(&mut h_mut).speed(1.0).range(1.0..=1024.0)).changed() {
                                            *h = serde_json::Value::Number(serde_json::Number::from_f64(h_mut as f64).unwrap());
                                            comp_changed = true;
                                        }
                                    });
                                }
                            }
                        }
                    }
                    ::forge_scene::ComponentType::Collider => {
                        if let Some(obj) = comp.data.as_object_mut() {
                            if let Some(shape) = obj.get_mut("shape") {
                                if let Some(shape_str) = shape.as_str() {
                                    let mut shape_val = shape_str.to_string();
                                    ui.horizontal(|ui| {
                                        ui.label("Forma:");
                                        egui::ComboBox::from_id_salt(format!("shape_{}", i))
                                            .selected_text(&shape_val)
                                            .show_ui(ui, |ui| {
                                                if ui.selectable_value(&mut shape_val, "box".to_string(), "Box").clicked() {
                                                    *shape = serde_json::Value::String("box".to_string());
                                                    comp_changed = true;
                                                }
                                                if ui.selectable_value(&mut shape_val, "circle".to_string(), "Circle").clicked() {
                                                    *shape = serde_json::Value::String("circle".to_string());
                                                    comp_changed = true;
                                                }
                                            });
                                    });
                                }
                            }
                            if let Some(size) = obj.get_mut("size") {
                                if let Some(size_arr) = size.as_array_mut() {
                                    if size_arr.len() == 2 {
                                        let mut w = size_arr[0].as_f64().unwrap_or(1.0) as f32;
                                        let mut h = size_arr[1].as_f64().unwrap_or(1.0) as f32;
                                        let mut size_changed = false;
                                        ui.horizontal(|ui| {
                                            ui.label("Tamaño:");
                                            ui.label("W:");
                                            size_changed |= ui.add(egui::DragValue::new(&mut w).speed(0.1).range(0.01..=100.0)).changed();
                                            ui.label("H:");
                                            size_changed |= ui.add(egui::DragValue::new(&mut h).speed(0.1).range(0.01..=100.0)).changed();
                                        });
                                        if size_changed {
                                            size_arr[0] = serde_json::Value::Number(serde_json::Number::from_f64(w as f64).unwrap());
                                            size_arr[1] = serde_json::Value::Number(serde_json::Number::from_f64(h as f64).unwrap());
                                            comp_changed = true;
                                        }
                                    }
                                }
                            }
                            if let Some(enabled) = obj.get_mut("enabled") {
                                if let Some(mut enabled_bool) = enabled.as_bool() {
                                    if ui.checkbox(&mut enabled_bool, "Habilitado").changed() {
                                        *enabled = serde_json::Value::Bool(enabled_bool);
                                        comp_changed = true;
                                    }
                                }
                            }
                        }
                    }
                    ::forge_scene::ComponentType::Audio => {
                        if let Some(obj) = comp.data.as_object_mut() {
                            if let Some(src) = obj.get_mut("source") {
                                let mut src_str = src.as_str().unwrap_or("").to_string();
                                ui.horizontal(|ui| {
                                    ui.label("Archivo sonido:");
                                    if ui.text_edit_singleline(&mut src_str).changed() {
                                        *src = serde_json::Value::String(src_str);
                                        comp_changed = true;
                                    }
                                });
                            }
                            if let Some(vol) = obj.get_mut("volume") {
                                if let Some(vol_val) = vol.as_f64() {
                                    let mut vol_mut = vol_val as f32;
                                    ui.horizontal(|ui| {
                                        ui.label("Volumen:");
                                        if ui.add(egui::Slider::new(&mut vol_mut, 0.0..=2.0)).changed() {
                                            *vol = serde_json::Value::Number(serde_json::Number::from_f64(vol_mut as f64).unwrap());
                                            comp_changed = true;
                                        }
                                    });
                                }
                            }
                            if let Some(pitch) = obj.get_mut("pitch") {
                                if let Some(pitch_val) = pitch.as_f64() {
                                    let mut pitch_mut = pitch_val as f32;
                                    ui.horizontal(|ui| {
                                        ui.label("Tono (Pitch):");
                                        if ui.add(egui::Slider::new(&mut pitch_mut, 0.5..=2.0)).changed() {
                                            *pitch = serde_json::Value::Number(serde_json::Number::from_f64(pitch_mut as f64).unwrap());
                                            comp_changed = true;
                                        }
                                    });
                                }
                            }
                            if let Some(is_loop) = obj.get_mut("is_loop") {
                                if let Some(mut is_loop_bool) = is_loop.as_bool() {
                                    if ui.checkbox(&mut is_loop_bool, "Bucle (Loop)").changed() {
                                        *is_loop = serde_json::Value::Bool(is_loop_bool);
                                        comp_changed = true;
                                    }
                                }
                            }
                        }
                    }
                    ::forge_scene::ComponentType::Script => {
                        if let Some(obj) = comp.data.as_object_mut() {
                            if let Some(path) = obj.get_mut("script_path") {
                                if let Some(path_str) = path.as_str() {
                                    let mut path_val = path_str.to_string();
                                    ui.horizontal(|ui| {
                                        ui.label("Script Path:");
                                        if ui.text_edit_singleline(&mut path_val).changed() {
                                            *path = serde_json::Value::String(path_val);
                                            comp_changed = true;
                                        }
                                    });
                                }
                            }
                            if let Some(enabled) = obj.get_mut("is_enabled") {
                                if let Some(mut enabled_bool) = enabled.as_bool() {
                                    if ui.checkbox(&mut enabled_bool, "Habilitado").changed() {
                                        *enabled = serde_json::Value::Bool(enabled_bool);
                                        comp_changed = true;
                                    }
                                }
                            }
                        }
                    }
                    ::forge_scene::ComponentType::Behavior => {
                        if let Some(obj) = comp.data.as_object_mut() {
                            if let Some(btype) = obj.get_mut("behavior_type") {
                                if let Some(btype_str) = btype.as_str() {
                                    let mut btype_val = btype_str.to_string();
                                    ui.horizontal(|ui| {
                                        ui.label("IA Preset:");
                                        egui::ComboBox::from_id_salt(format!("btype_{}", i))
                                            .selected_text(match btype_val.as_str() {
                                                "patrol" => "Patrullar (Patrol)",
                                                "chase" => "Perseguir (Chase)",
                                                "flee" => "Huir (Flee)",
                                                "attack" => "Atacar (Attack)",
                                                _ => &btype_val,
                                            })
                                            .show_ui(ui, |ui| {
                                                if ui.selectable_value(&mut btype_val, "patrol".to_string(), "Patrullar (Patrol)").clicked() {
                                                    *btype = serde_json::Value::String("patrol".to_string());
                                                    comp_changed = true;
                                                }
                                                if ui.selectable_value(&mut btype_val, "chase".to_string(), "Perseguir (Chase)").clicked() {
                                                    *btype = serde_json::Value::String("chase".to_string());
                                                    comp_changed = true;
                                                }
                                                if ui.selectable_value(&mut btype_val, "flee".to_string(), "Huir (Flee)").clicked() {
                                                    *btype = serde_json::Value::String("flee".to_string());
                                                    comp_changed = true;
                                                }
                                                if ui.selectable_value(&mut btype_val, "attack".to_string(), "Atacar (Attack)").clicked() {
                                                    *btype = serde_json::Value::String("attack".to_string());
                                                    comp_changed = true;
                                                }
                                            });
                                    });
                                }
                            }
                            if let Some(spd) = obj.get_mut("speed") {
                                if let Some(spd_val) = spd.as_f64() {
                                    let mut spd_mut = spd_val as f32;
                                    ui.horizontal(|ui| {
                                        ui.label("Velocidad:");
                                        if ui.add(egui::Slider::new(&mut spd_mut, 0.0..=500.0)).changed() {
                                            *spd = serde_json::Value::Number(serde_json::Number::from_f64(spd_mut as f64).unwrap());
                                            comp_changed = true;
                                        }
                                    });
                                }
                            }
                            if let Some(rng) = obj.get_mut("range") {
                                if let Some(rng_val) = rng.as_f64() {
                                    let mut rng_mut = rng_val as f32;
                                    ui.horizontal(|ui| {
                                        ui.label("Rango IA:");
                                        if ui.add(egui::Slider::new(&mut rng_mut, 10.0..=1000.0)).changed() {
                                            *rng = serde_json::Value::Number(serde_json::Number::from_f64(rng_mut as f64).unwrap());
                                            comp_changed = true;
                                        }
                                    });
                                }
                            }
                        }
                    }
                    _ => {
                        ui.label(format!("Data: {}", comp.data));
                    }
                }
            });
            
            if comp_changed {
                comp_to_update = Some((i, comp));
            }
        }
        
        // Aplicar cambios diferidos en componentes
        if let Some(index) = comp_to_remove {
            let mut updated = entity.as_ref().clone();
            updated.components.remove(index);
            let arc_node = std::sync::Arc::new(updated);
            app.scene_tree.nodes.insert(node_id, arc_node.clone());
            if app.scene_tree.root.as_ref().map(|n| n.id) == Some(node_id) {
                app.scene_tree.root = Some(arc_node.clone());
            }
            if let Some(pos) = app.scene_tree.active_nodes.iter().position(|n| n.id == node_id) {
                app.scene_tree.active_nodes[pos] = arc_node;
            }
        }
        
        if let Some((index, comp)) = comp_to_update {
            let mut updated = entity.as_ref().clone();
            updated.components[index] = comp;
            let arc_node = std::sync::Arc::new(updated);
            app.scene_tree.nodes.insert(node_id, arc_node.clone());
            if app.scene_tree.root.as_ref().map(|n| n.id) == Some(node_id) {
                app.scene_tree.root = Some(arc_node.clone());
            }
            if let Some(pos) = app.scene_tree.active_nodes.iter().position(|n| n.id == node_id) {
                app.scene_tree.active_nodes[pos] = arc_node;
            }
        }
        
        // 5. Menú para añadir nuevos componentes
        ui.add_space(10.0);
        ui.menu_button("➕ Añadir Componente", |ui| {
            if ui.button("Sprite Renderer").clicked() {
                let mut updated = entity.as_ref().clone();
                if !updated.components.iter().any(|c| c.component_type == ::forge_scene::ComponentType::Sprite) {
                    updated.components.push(::forge_scene::ComponentData::new_sprite("assets/sprites/char.png".to_string()));
                    updated.entity_type = ::forge_scene::EntityType::Sprite;
                    updated.properties.insert("sprite_path".to_string(), serde_json::Value::String("assets/sprites/char.png".to_string()));
                    
                    let arc_node = std::sync::Arc::new(updated);
                    app.scene_tree.nodes.insert(node_id, arc_node.clone());
                    if app.scene_tree.root.as_ref().map(|n| n.id) == Some(node_id) {
                        app.scene_tree.root = Some(arc_node.clone());
                    }
                    if let Some(pos) = app.scene_tree.active_nodes.iter().position(|n| n.id == node_id) {
                        app.scene_tree.active_nodes[pos] = arc_node;
                    }
                    app.console.add_message(LogLevel::Info, "Componente Sprite Renderer añadido.");
                }
                ui.close();
            }
            if ui.button("Collider 2D").clicked() {
                let mut updated = entity.as_ref().clone();
                if !updated.components.iter().any(|c| c.component_type == ::forge_scene::ComponentType::Collider) {
                    updated.components.push(::forge_scene::ComponentData::new_collider());
                    let arc_node = std::sync::Arc::new(updated);
                    app.scene_tree.nodes.insert(node_id, arc_node.clone());
                    if app.scene_tree.root.as_ref().map(|n| n.id) == Some(node_id) {
                        app.scene_tree.root = Some(arc_node.clone());
                    }
                    if let Some(pos) = app.scene_tree.active_nodes.iter().position(|n| n.id == node_id) {
                        app.scene_tree.active_nodes[pos] = arc_node;
                    }
                    app.console.add_message(LogLevel::Info, "Componente Collider 2D añadido.");
                }
                ui.close();
            }
            if ui.button("Audio Source").clicked() {
                let mut updated = entity.as_ref().clone();
                if !updated.components.iter().any(|c| c.component_type == ::forge_scene::ComponentType::Audio) {
                    updated.components.push(::forge_scene::ComponentData::new_audio());
                    let arc_node = std::sync::Arc::new(updated);
                    app.scene_tree.nodes.insert(node_id, arc_node.clone());
                    if app.scene_tree.root.as_ref().map(|n| n.id) == Some(node_id) {
                        app.scene_tree.root = Some(arc_node.clone());
                    }
                    if let Some(pos) = app.scene_tree.active_nodes.iter().position(|n| n.id == node_id) {
                        app.scene_tree.active_nodes[pos] = arc_node;
                    }
                    app.console.add_message(LogLevel::Info, "Componente Audio Source añadido.");
                }
                ui.close();
            }
            if ui.button("Behavior (No-Code)").clicked() {
                let mut updated = entity.as_ref().clone();
                if !updated.components.iter().any(|c| c.component_type == ::forge_scene::ComponentType::Behavior) {
                    updated.components.push(::forge_scene::ComponentData::new_behavior());
                    let arc_node = std::sync::Arc::new(updated);
                    app.scene_tree.nodes.insert(node_id, arc_node.clone());
                    if app.scene_tree.root.as_ref().map(|n| n.id) == Some(node_id) {
                        app.scene_tree.root = Some(arc_node.clone());
                    }
                    if let Some(pos) = app.scene_tree.active_nodes.iter().position(|n| n.id == node_id) {
                        app.scene_tree.active_nodes[pos] = arc_node;
                    }
                    app.console.add_message(LogLevel::Info, "Componente Behavior No-Code añadido.");
                }
                ui.close();
            }
            if ui.button("Script").clicked() {
                let mut updated = entity.as_ref().clone();
                if !updated.components.iter().any(|c| c.component_type == ::forge_scene::ComponentType::Script) {
                    updated.components.push(::forge_scene::ComponentData::new_script("assets/scripts/player.lua".to_string()));
                    let arc_node = std::sync::Arc::new(updated);
                    app.scene_tree.nodes.insert(node_id, arc_node.clone());
                    if app.scene_tree.root.as_ref().map(|n| n.id) == Some(node_id) {
                        app.scene_tree.root = Some(arc_node.clone());
                    }
                    if let Some(pos) = app.scene_tree.active_nodes.iter().position(|n| n.id == node_id) {
                        app.scene_tree.active_nodes[pos] = arc_node;
                    }
                    app.console.add_message(LogLevel::Info, "Componente Script añadido.");
                }
                ui.close();
            }
        });
    }
    
    /// Renderiza el panel de componentes obsoleto (se mantiene firma para compatibilidad)
    pub fn ui(&mut self, ui: &mut egui::Ui) {
        ui.heading("Component Properties");
        ui.add_space(5.0);
        ui.separator();
        ui.label(format!("Obsoleto: use render()"));
    }
    
    /// Renderiza propiedades de sprite (se mantiene firma para compatibilidad)
    pub fn render_sprite_properties(_ui: &mut egui::Ui, _app: &mut crate::ForgeEditorApp) {}

    pub fn set_component_type(&mut self, ty: &str) {
        self.component_type = ty.to_string();
    }

    pub fn add_property(&mut self, key: &str, value: &str) {
        self.keys.push(key.to_string());
        self.values.push(value.to_string());
    }

    pub fn remove_property(&mut self, index: usize) {
        if index < self.keys.len() {
            self.keys.remove(index);
            self.values.remove(index);
        }
    }
}
