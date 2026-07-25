//! UI Tests - Pruebas de interfaz de usuario

use crate::ui::ForgeEditorApp;

pub fn test_editor_log() {
    // Create a minimal app instance directly
    let mut app = ForgeEditorApp {
        logs: Vec::new(),
        timeline_integration: None,
        play_session: None,
        snapshot_manager: crate::snapshot_manager::SnapshotManager::new(),
        user_input: crate::input_capture::UserInput::default(),
        is_playing: false,
    };
    app.log("Test log message".to_string());
    assert_eq!(app.logs.len(), 1);
    assert_eq!(app.logs[0], "Test log message");
}

pub fn test_drag_operation() {
    let _drag_op = crate::ui::DragOperation::None;
    let _drag_op2 = crate::ui::DragOperation::Move;
    let _drag_op3 = crate::ui::DragOperation::Resize;
}
