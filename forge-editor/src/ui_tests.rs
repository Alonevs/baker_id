//! UI Tests - Pruebas de interfaz de usuario

use crate::ui::ForgeEditorApp;
use crate::play_session::Entity;
use crate::hot_reload_panel::HotReloadPanel;

pub fn test_editor_log() {
    let mut app = ForgeEditorApp {
        logs: Vec::new(),
        timeline_integration: None,
        play_session: None,
        snapshot_manager: crate::snapshot_manager::SnapshotManager::new(),
        user_input: crate::input_capture::UserInput::default(),
        is_playing: false,
        hot_reload_panel: HotReloadPanel::new(),
    };
    app.log("Test log message".to_string());
    assert_eq!(app.logs.len(), 1);
    assert_eq!(app.logs[0], "Test log message");
}

pub fn test_play_session() {
    let entities = vec![
        Entity::new("entity1".to_string(), (100.0, 100.0)),
    ];
    let session = crate::play_session::PlaySession::new(&entities);
    assert!(!session.is_active());
}
