//! # Forge Editor
//! 
//! Editor de juego 2D con sistema de nodos, viewport, asset browser y preview de assets.

pub mod math;
pub mod ui;
pub mod timeline;
pub mod components;
pub mod timeline_integration;
pub mod animation_2d;
pub mod animation_track;
pub mod bitacora_manager;
pub mod bitacora_singleton;
pub mod bitacora_validator;
pub mod cable_system;
pub mod cable_ui;
pub mod collaboration;
pub mod collaboration_panel;
pub mod compile_panel;
pub mod compile_system;
pub mod component_editor;
pub mod component_properties_ui;
pub mod debugger;
pub mod debugger_panel;
pub mod delta_sync;
pub mod dialogue_editor;
pub mod dialogue_editor_ui;
pub mod dialogue_export;
pub mod editor_panel;
pub mod event_dialog;
pub mod event_nodes;
pub mod event_nodes_ui;
pub mod event_node_editor;
pub mod event_node_manager;
pub mod event_forge;
pub mod export_import_panel;
pub mod toolbar;
pub mod export_manager;
pub mod hot_reload;
pub mod hot_reload_panel;
pub mod hot_reload_integration;
pub mod bakeforge_parser;

pub use hot_reload::ChangeType;
pub use hot_reload_integration::SimpleFileWatcherIntegration;
pub mod import_manager;
pub mod keyframe;
pub mod map_export;
pub mod paperdoll;
pub mod particle_system;
pub mod physics_2d;
pub mod plugin_system_ui;
pub mod project_manager;
pub mod property_editor_ui;
pub mod explorer_panel;
pub mod explorer_panel_integration;

pub use explorer_panel_integration::ExplorerPanelIntegration;
pub mod property_panel;
pub mod script_executor;
pub mod script_optimizer;
pub mod play_session;
pub mod snapshot_manager;
pub mod input_capture;
pub mod timeline_integration_tests;

// Re-exportar tipos para uso externo
pub use play_session::{Entity, EntitySnapshot, SceneSnapshot, Vec2, PlaySession};
pub use snapshot_manager::SnapshotManager;
pub use input_capture::{InputCapture, KeyCode, MouseState, UserInput};
pub mod script_optimizer_panel;
pub mod script_serialization;
pub mod script_viewer;
pub mod serialization_panel;
pub mod transform_editor;
pub mod transform_properties_ui;
pub mod live_sync_manager;
pub mod integration_validation_tests;
pub mod ui_tests;

pub use timeline_integration::TimelineIntegration;
pub use ui::ForgeEditorApp;
pub use export_manager::ExportManager;
pub use import_manager::ImportManager;
pub use export_import_panel::ExportImportPanel;
pub mod preview_panel;
pub use preview_panel::PreviewPanel;
pub use preview_panel::AssetInfo;
pub use preview_panel::AssetType;
pub use property_panel::{PropertyPanel, EntityId};
pub use live_sync_manager::SyncEvent;
pub use delta_sync::DeltaEncoder;

pub mod sprite_slicer;
pub mod tile_palette_panel;
pub use sprite_slicer::{SpriteSlicer, TilesetConfig, SlicedTile};
pub use tile_palette_panel::TilePalettePanel;

pub struct VecStringDisplay(pub Vec<String>);

impl std::fmt::Display for VecStringDisplay {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?;
        for (i, item) in self.0.iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}", item)?;
        }
        write!(f, "]")
    }
}

impl std::fmt::Debug for VecStringDisplay {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "VecStringDisplay([")?;
        for (i, item) in self.0.iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}", item)?;
        }
        write!(f, "])")
    }
}

impl Clone for VecStringDisplay {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl Default for VecStringDisplay {
    fn default() -> Self {
        Self(Vec::new())
    }
}

impl VecStringDisplay {
    pub fn new() -> Self {
        Self(Vec::new())
    }
}

impl<'a> Iterator for &'a VecStringDisplay {
    type Item = &'a String;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.iter().next()
    }
}


