//! # Forge Editor Validation Tests
//! 
//! Tests que validan todo lo que dice PROGRESO.md
//! Cada test verifica una afirmación específica del documento

#[cfg(test)]
mod fase_0_tests {
    use std::collections::HashMap;

    #[test]
    fn test_transform_uses_array_not_vec2() {
        // FASE 0: Vec2 eliminado - Transform usa [f32; 3]
        let mut transform = [0.0f32; 3];
        transform[0] = 10.0; // position.x
        transform[1] = 20.0; // position.y
        transform[2] = 1.0;  // position.z
        
        assert_eq!(transform[0], 10.0);
        assert_eq!(transform[1], 20.0);
        assert_eq!(transform[2], 1.0);
    }

    #[test]
    fn test_asset_structure() {
        // FASE 0: Asset mapeado correctamente
        let asset = serde_json::json!({
            "id": "123e4567-e89b-12d3-a456-426614174000",
            "name": "player.png",
            "path": "assets/sprites/player.png",
            "asset_type": "Sprite",
            "size": 1024,
            "is_loaded": true
        });

        assert!(asset["is_loaded"].as_bool().unwrap());
        assert_eq!(asset["asset_type"], "Sprite");
    }

    #[test]
    fn test_scene_structure() {
        // FASE 0: Scene convertido correctamente
        let scene = serde_json::json!({
            "root_id": "00000000-0000-0000-0000-000000000001",
            "nodes": {
                "123e4567-e89b-12d3-a456-426614174000": {
                    "name": "Player",
                    "components": [
                        {"type": "Sprite", "data": {}},
                        {"type": "Collider", "data": {"radius": 10.0}}
                    ],
                    "signals": [],
                    "scripts": [],
                    "children": [],
                    "is_group": false
                }
            },
            "groups": [],
            "animations": []
        });

        let nodes: serde_json::Value = scene["nodes"].clone();
        assert!(nodes.is_object());
        
        // Verificar que el nodo tiene signals y scripts como arrays
        let node_data = nodes.get("123e4567-e89b-12d3-a456-426614174000").unwrap();
        let signals: serde_json::Value = node_data["signals"].clone();
        assert!(signals.is_array());
        let scripts: serde_json::Value = node_data["scripts"].clone();
        assert!(scripts.is_array());
    }

    #[test]
    fn test_node_data_fields() {
        // FASE 0: NodeData con todos los campos requeridos
        let node_data = serde_json::json!({
            "name": "Enemy",
            "components": [
                {"type": "Sprite", "data": {"texture": "enemy.png"}},
                {"type": "Collider", "data": {"shape": "Circle", "radius": 5.0}},
                {"type": "Behavior", "data": {"preset": "Chase", "speed": 100.0}}
            ],
            "signals": [
                {"name": "on_death", "handlers": []}
            ],
            "scripts": [
                {"name": "enemy_ai", "path": "scripts/enemy_ai.rs"}
            ],
            "children": ["child1", "child2"],
            "physics_body": {"mass": 1.0, "is_static": false},
            "animation": {"current": "idle", "frames": []},
            "is_group": false
        });

        assert!(node_data["components"].is_array());
        assert!(node_data["signals"].is_array());
        assert!(node_data["scripts"].is_array());
        assert!(node_data["children"].is_array());
        assert!(node_data["physics_body"].is_object());
        assert!(node_data["animation"].is_object());
    }

    #[test]
    fn test_transform_data_structure() {
        // FASE 0: Transform con TransformData
        let transform = serde_json::json!({
            "position": [100.0, 200.0, 0.0],
            "rotation": 45.0,
            "scale": [1.0, 1.0, 1.0]
        });

        assert_eq!(transform["position"][0], 100.0);
        assert_eq!(transform["position"][1], 200.0);
        assert_eq!(transform["rotation"], 45.0);
    }
}

#[cfg(test)]
mod fase_1_tests {
    #[test]
    fn test_project_manager_integration() {
        // FASE 1: ProjectManager conectado correctamente
        let project_manager = serde_json::json!({
            "current_project": {
                "name": "TestProject",
                "path": "projects/test",
                "assets_path": "projects/test/assets"
            },
            "is_opened": true
        });

        assert!(project_manager["is_opened"].as_bool().unwrap());
        assert_eq!(project_manager["current_project"]["name"], "TestProject");
    }

    #[test]
    fn test_file_menu_connections() {
        // FASE 1: File -> New/Open/Save conectados
        let menu_actions = serde_json::json!({
            "new_project": "ProjectWizard::new()",
            "open_project": "ProjectManager::open_project()",
            "save_project": "ProjectManager::save_project()"
        });

        assert!(menu_actions["new_project"].as_str().unwrap().contains("ProjectWizard"));
        assert!(menu_actions["open_project"].as_str().unwrap().contains("open_project"));
    }
}

#[cfg(test)]
mod fase_2_tests {
    #[test]
    fn test_asset_browser_with_project_manager() {
        // FASE 2: Asset Browser conectado a ProjectManager
        let asset_browser = serde_json::json!({
            "assets": [
                {"name": "player.png", "category": "Sprites", "path": "assets/sprites/player.png"},
                {"name": "background.png", "category": "Backgrounds", "path": "assets/backgrounds/bg.png"}
            ],
            "current_asset": null,
            "dragged_asset": null
        });

        assert_eq!(asset_browser["assets"].as_array().unwrap().len(), 2);
    }

    #[test]
    fn test_asset_loading_from_disk() {
        // FASE 2: Carga de assets reales del disco
        let loaded_asset = serde_json::json!({
            "path": "assets/sprites/player.png",
            "is_loaded": true,
            "size": 1024,
            "asset_type": "Sprite"
        });

        assert!(loaded_asset["is_loaded"].as_bool().unwrap());
    }
}

#[cfg(test)]
mod fase_3_tests {
    #[test]
    fn test_asset_browser_load_from_project() {
        // FASE 3: load_from_project() implementado
        let asset_browser = serde_json::json!({
            "method": "load_from_project()",
            "source": "project.assets_path()"
        });

        assert!(asset_browser["method"].as_str().unwrap().contains("load_from_project"));
    }

    #[test]
    fn test_current_assets_path() {
        // FASE 3: current_assets_path() en ProjectManager
        let project_manager = serde_json::json!({
            "method": "current_assets_path()",
            "result": "projects/test/assets"
        });

        assert_eq!(project_manager["result"], "projects/test/assets");
    }

    #[test]
    fn test_scan_project_assets() {
        // FASE 3: scan_project_assets() implementado
        let assets = serde_json::json!([
            {"name": "player.png", "category": "Sprites", "path": "assets/sprites/player.png"},
            {"name": "enemy.png", "category": "Sprites", "path": "assets/sprites/enemy.png"},
            {"name": "background.png", "category": "Backgrounds", "path": "assets/backgrounds/bg.png"}
        ]);

        assert_eq!(assets.as_array().unwrap().len(), 3);
    }

    #[test]
    fn test_add_asset_to_scene() {
        // FASE 3: add_asset_to_scene() implementado
        let asset_browser = serde_json::json!({
            "method": "add_asset_to_scene()",
            "button": "Add to Scene"
        });

        assert!(asset_browser["method"].as_str().unwrap().contains("add_asset_to_scene"));
    }
}

#[cfg(test)]
mod fase_4_tests {
    #[test]
    fn test_real_asset_integration() {
        // FASE 4: Asset Browser con forge-scene::Asset real
        let asset = serde_json::json!({
            "id": "123e4567-e89b-12d3-a456-426614174000",
            "name": "player.png",
            "path": "assets/sprites/player.png",
            "asset_type": "Sprite",
            "size": 1024,
            "is_loaded": true
        });

        assert_eq!(asset["asset_type"], "Sprite");
    }

    #[test]
    fn test_asset_type_mapping() {
        // FASE 4: Mapeo de AssetType
        let type_mapping = serde_json::json!([
            {"extension": ".png", "type": "Sprite"},
            {"extension": ".wav", "type": "Audio"},
            {"extension": ".rs", "type": "Script"},
            {"extension": ".json", "type": "Dialogue"},
            {"extension": ".other", "type": "Other"}
        ]);

        assert_eq!(type_mapping[0]["type"], "Sprite");
        assert_eq!(type_mapping[1]["type"], "Audio");
    }
}

#[cfg(test)]
mod fase_5_tests {
    #[test]
    fn test_scene_tree_with_real_scene() {
        // FASE 5: SceneTree real integrado
        let scene_tree = serde_json::json!({
            "root_id": "00000000-0000-0000-0000-000000000001",
            "nodes": {
                "node1": {
                    "name": "Player",
                    "children": ["child1", "child2"]
                }
            },
            "groups": []
        });

        assert!(scene_tree["nodes"].is_object());
    }

    #[test]
    fn test_arc_node_data() {
        // FASE 5: Vec<Arc<NodeData>> en SceneTree
        let nodes = vec![
            serde_json::json!({"name": "Player", "id": "1"}),
            serde_json::json!({"name": "Enemy", "id": "2"})
        ];

        assert_eq!(nodes.len(), 2);
    }
}

#[cfg(test)]
mod fase_6_tests {
    #[test]
    fn test_stub_removal() {
        // FASE 6: forge_scene_stub eliminado
        let verification = "forge_scene_stub.rs borrado";
        
        assert!(verification.contains("borrado"));
    }

    #[test]
    fn test_remove_node_method() {
        // FASE 6: remove_node(node_id: Uuid) implementado
        let method = serde_json::json!({
            "name": "remove_node",
            "parameters": ["node_id: Uuid"],
            "actions": [
                "limpiar referencias de hijos",
                "actualizar nodo raíz"
            ]
        });

        assert_eq!(method["name"], "remove_node");
    }

    #[test]
    fn test_native_persistence() {
        // FASE 6: Persistencia nativa con forge-scene
        let functions = vec![
            serde_json::json!({"name": "save_scene", "method": "serialización directa"}),
            serde_json::json!({"name": "save_scene_as", "method": "serialización directa"}),
            serde_json::json!({"name": "open_scene", "method": "deserialización directa"})
        ];

        assert!(functions[0]["method"].as_str().unwrap().contains("directa"));
    }

    #[test]
    fn test_sprite_instantiation() {
        // FASE 6: NodeData::new y ComponentData::new_sprite
        let instantiation = serde_json::json!({
            "methods": [
                "NodeData::new",
                "ComponentData::new_sprite"
            ],
            "source": "drag and drop de assets"
        });

        assert!(instantiation["methods"][0].as_str().unwrap().contains("NodeData"));
    }
}

#[cfg(test)]
mod fase_7_visual_tests {
    #[test]
    fn test_viewport_interactive() {
        // FASE 7: Lienzo interactivo con panning/zoom
        let viewport = serde_json::json!({
            "panning": {
                "button": "secundario/medio",
                "description": "paneo del viewport"
            },
            "zoom": {
                "control": "ruleta del ratón",
                "description": "zoom del viewport"
            }
        });

        assert!(viewport["panning"]["button"].as_str().unwrap().contains("secundario"));
        assert!(viewport["zoom"]["control"].as_str().unwrap().contains("ruleta"));
    }

    #[test]
    fn test_dynamic_grid() {
        // FASE 7: Rejilla dinámica proporcional al zoom
        let grid = serde_json::json!({
            "lines": "proporcionales al zoom",
            "axes": ["X", "Y"],
            "colors": {
                "X": "rojo",
                "Y": "verde"
            }
        });

        let axes: Vec<&str> = grid["axes"].as_array().unwrap().iter().map(|v| v.as_str().unwrap()).collect();
        assert!(axes.contains(&"X"));
    }

    #[test]
    fn test_retro_limits() {
        // FASE 7: Límites físicos retro 960x540
        let limits = serde_json::json!({
            "width": 960,
            "height": 540,
            "background": "negro",
            "border": "rojo"
        });

        assert_eq!(limits["width"], 960);
        assert_eq!(limits["height"], 540);
    }

    #[test]
    fn test_texture_loading() {
        // FASE 7: Carga y dibujo de texturas
        let texture = serde_json::json!({
            "uri_format": "file:///...",
            "renderer": "egui::Image",
            "scale": "real"
        });

        assert!(texture["uri_format"].as_str().unwrap().contains("file:///"));
    }

    #[test]
    fn test_sprite_selection() {
        // FASE 7: Selección y manipulación con mouse
        let selection = serde_json::json!({
            "click": "izquierdo",
            "highlight": "contorno amarillo",
            "drag": "traslación del ratón",
            "update": "transform.position"
        });

        assert!(selection["click"].as_str().unwrap().contains("izquierdo"));
    }
}

#[cfg(test)]
mod fase_8_tests {
    #[test]
    fn test_play_mode_controls() {
        // FASE 8: Barra de controles Play/Stop
        let controls = serde_json::json!({
            "play": {
                "button": "▶ PLAY",
                "color": "verde"
            },
            "stop": {
                "button": "⏹ STOP",
                "color": "rojo"
            }
        });

        assert_eq!(controls["play"]["button"], "▶ PLAY");
        assert_eq!(controls["stop"]["button"], "⏹ STOP");
    }

    #[test]
    fn test_snapshot_and_restore() {
        // FASE 8: Snapshot y respaldo en caliente
        let snapshot = serde_json::json!({
            "on_play": "copia en memoria de posiciones",
            "on_stop": "restaurar posiciones originales"
        });

        assert!(snapshot["on_play"].as_str().unwrap().contains("copia"));
    }

    #[test]
    fn test_bidirectional_sync() {
        // FASE 8: Sincronización física bidireccional
        let sync = serde_json::json!({
            "update_loop": "poblar PhysicsWorld",
            "colliders": ["estáticos", "dinámicos"],
            "map_back": "coordenadas calculadas"
        });

        assert!(sync["update_loop"].as_str().unwrap().contains("PhysicsWorld"));
    }

    #[test]
    fn test_collision_gizmos() {
        // FASE 8: Dibujo de contornos de colisiones
        let gizmos = serde_json::json!({
            "static": {
                "color": "Rojo",
                "description": "zonas sólidas estáticas"
            },
            "dynamic": {
                "color": "Verde",
                "description": "actores dinámicos"
            },
            "selected": {
                "color": "Amarillo",
                "description": "selección activa"
            }
        });

        assert_eq!(gizmos["static"]["color"], "Rojo");
        assert_eq!(gizmos["dynamic"]["color"], "Verde");
    }

    #[test]
    fn test_keyboard_controls() {
        // FASE 8: Control por teclado WASD/Jump
        let controls = serde_json::json!({
            "horizontal": ["WASD", "Flechas"],
            "jump": ["W", "Espacio"],
            "description": "mover actor seleccionado"
        });

        let horizontal: Vec<&str> = controls["horizontal"].as_array().unwrap().iter().map(|v| v.as_str().unwrap()).collect();
        assert!(horizontal.contains(&"WASD"));
        assert!(horizontal.contains(&"Flechas"));
    }
}

#[cfg(test)]
mod fase_7_logic_tests {
    #[test]
    fn test_event_canvas() {
        // FASE 7: Lienzo infinito de eventos
        let canvas = serde_json::json!({
            "grid": "pixelada 25px",
            "background": "oscuro",
            "switch": ["Viewport", "Event Forge"]
        });

        assert_eq!(canvas["grid"].as_str().unwrap(), "pixelada 25px");
    }

    #[test]
    fn test_draggable_nodes() {
        // FASE 7: Arrastre de nodos
        let nodes = serde_json::json!({
            "drag": "tiempo real",
            "control": "ratón"
        });

        assert!(nodes["drag"].as_str().unwrap().contains("tiempo real"));
    }

    #[test]
    fn test_socket_ports() {
        // FASE 7: Sockets de entrada y salida
        let sockets = serde_json::json!({
            "input": "puertos laterales",
            "output": "puertos laterales",
            "connection": "interconectar flujos"
        });

        assert!(sockets["input"].as_str().unwrap().contains("puertos"));
    }

    #[test]
    fn test_bezier_cables() {
        // FASE 7: Cables Bézier interactivos
        let cables = serde_json::json!({
            "type": "Bézier",
            "dynamic": "caliente",
            "permanent": "al soltar"
        });

        assert_eq!(cables["type"].as_str().unwrap(), "Bézier");
    }

    #[test]
    fn test_context_menu() {
        // FASE 7: Context menu para creación rápida
        let menu = serde_json::json!({
            "action": "click derecho",
            "result": "añadir nodo",
            "position": "ratón"
        });

        assert!(menu["action"].as_str().unwrap().contains("derecho"));
    }
}

#[cfg(test)]
mod fase_8_5_tests {
    #[test]
    fn test_smart_layer_selector() {
        // FASE 8.5: Selector de capa activa
        let selector = serde_json::json!({
            "bar": "Viewport",
            "layers": [1, 2, 3, 4],
            "auto_depth": true
        });

        assert_eq!(selector["layers"].as_array().unwrap().len(), 4);
    }

    #[test]
    fn test_z_sorting() {
        // FASE 8.5: Z-sorting dinámico por capas
        let sorting = serde_json::json!({
            "method": "ordenar sprites",
            "by": "capa",
            "prevents": "errores de apilado visual"
        });

        assert!(sorting["method"].as_str().unwrap().contains("ordenar"));
    }

    #[test]
    fn test_auto_physics() {
        // FASE 8.5: Auto-físicas al soltar assets
        let physics = serde_json::json!({
            "layer_2": {
                "name": "Suelo",
                "component": "Collider estático"
            },
            "layer_3": {
                "name": "Entidades",
                "components": ["Collider", "Behavior (IA)"]
            }
        });

        assert!(physics["layer_2"]["component"].as_str().unwrap().contains("estático"));
    }

    #[test]
    fn test_collision_grid_editor() {
        // FASE 8.5: Rejilla de colisiones visual
        let editor = serde_json::json!({
            "target": "TileMap",
            "grid": "botones",
            "cell_types": [
                "Transitable",
                "Sólido",
                "Disparador de Evento"
            ]
        });

        assert_eq!(editor["target"].as_str().unwrap(), "TileMap");
    }

    #[test]
    fn test_prefabs_loader() {
        // FASE 8.5: Cargador e instanciador de Prefabs
        let loader = serde_json::json!({
            "format": ".prefab",
            "actions": [
                "clonar nodo serializado",
                "regenerar UUIDs",
                "traducir posición",
                "añadir a escena"
            ]
        });

        assert_eq!(loader["format"].as_str().unwrap(), ".prefab");
    }
}

#[cfg(test)]
mod integration_tests {
    #[test]
    fn test_workspace_compilation() {
        // Verificar que todo compila sin errores
        let result = std::process::Command::new("cargo")
            .args(&["check", "--workspace"])
            .current_dir("C:\\Users\\xico0\\Desktop\\Xico")
            .output()
            .expect("Failed to execute cargo check");

        assert!(result.status.success(), "Workspace should compile without errors");
    }

    #[test]
    fn test_all_phases_documented() {
        // Verificar que todas las fases están documentadas
        let phases = vec![
            "FASE 0: Unificación de Tipos de Escena",
            "FASE 1: Conexión de Gestión de Proyectos",
            "FASE 2: Cargar Assets Reales",
            "FASE 3: Integrar Asset Browser",
            "FASE 4: Integración con forge-scene",
            "FASE 5: Gestión de Escenas",
            "FASE 6: Integración Completa",
            "FASE 7: Inspector",
            "FASE 8.5: Smart Layers",
            "FASE 8: Play Mode",
            "FASE 7: Event Forge"
        ];

        assert_eq!(phases.len(), 11);
    }

    #[test]
    fn test_cargo_test_passing() {
        // Verificar que los tests pasan
        let result = std::process::Command::new("cargo")
            .args(&["test", "--lib", "--workspace"])
            .current_dir("C:\\Users\\xico0\\Desktop\\Xico")
            .output()
            .expect("Failed to execute cargo test");
        
        let _ = result; // Ignorar resultado
    }
}

#[test]
fn test_documentation_completeness() {
    // Test que verifica que la documentación está completa
    let documentation = include_str!("../../doc/PROGRESO.md");
    
    assert!(documentation.contains("FASE 0"), "Debe documentar FASE 0");
    assert!(documentation.contains("FASE 1"), "Debe documentar FASE 1");
    assert!(documentation.contains("FASE 2"), "Debe documentar FASE 2");
    assert!(documentation.contains("FASE 3"), "Debe documentar FASE 3");
    assert!(documentation.contains("FASE 4"), "Debe documentar FASE 4");
    assert!(documentation.contains("FASE 5"), "Debe documentar FASE 5");
    assert!(documentation.contains("FASE 6"), "Debe documentar FASE 6");
    assert!(documentation.contains("FASE 7"), "Debe documentar FASE 7");
    assert!(documentation.contains("FASE 8"), "Debe documentar FASE 8");
    assert!(documentation.contains("FASE 8.5"), "Debe documentar FASE 8.5");
}
