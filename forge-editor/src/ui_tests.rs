//! UI Tests - Pruebas de interfaz de usuario

use crate::ui::ForgeEditorApp;
use crate::play_session::Entity;
use crate::hot_reload_panel::HotReloadPanel;
use crate::hot_reload::HotReloadManager;

pub fn test_editor_log() {
    let mut app = ForgeEditorApp::default();
    app.log("Test log message".to_string());
    assert!(app.logs.iter().any(|l| l.contains("Test log message")));
}

pub fn test_play_session() {
    let entities = vec![
        Entity::new("entity1".to_string(), (100.0, 100.0)),
    ];
    let session = crate::play_session::PlaySession::new(&entities);
    assert!(!session.is_active());
}

#[test]
fn test_event_forge_execution_in_play_mode() {
    let mut app = ForgeEditorApp::default();
    app.event_forge_widget.add_node(); // Añade nodo TriggerZone
    app.event_forge_widget.add_node(); // Añade nodo Action (speed_x = 30.0)

    assert_eq!(app.event_forge_widget.manager.graph.nodes.len(), 2);

    app.start_play();
    assert!(app.is_playing);

    let initial_x = app.entities[0].position.0;
    app.update_logic(0.1);

    // La entidad 0 debió desplazarse hacia la derecha por la acción del Event Forge
    assert!(app.entities[0].position.0 > initial_x);
    assert!(app.logs.iter().any(|log| log.contains("EventForge")));

    app.stop_play();
    assert!(!app.is_playing);
}

#[test]
fn test_sprite_slicer_and_tile_palette() {
    let mut slicer = crate::sprite_slicer::SpriteSlicer::new();
    slicer.update_grid_dimensions(4, 4, 32, 32);
    assert_eq!(slicer.sliced_tiles.len(), 16);
    assert_eq!(slicer.sliced_tiles[0].name, "Tile_0");

    let mut palette_panel = crate::tile_palette_panel::TilePalettePanel::new();
    palette_panel.select_tile(5);
    assert_eq!(palette_panel.selected_tile_index, 5);

    let selected_tile = palette_panel.get_selected_tile().unwrap();
    assert_eq!(selected_tile.id, 5);
}
