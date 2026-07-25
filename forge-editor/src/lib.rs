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
pub mod event_play_integration;
pub mod event_play_integration_example;
pub mod event_forge;
pub mod export_import_panel;
pub mod export_manager;
pub mod hot_reload;
pub mod hot_reload_panel;
pub mod hot_reload_integration;
pub mod improved_hot_reload;
pub mod drag_drop_integration;
pub mod bakeforge_parser;
pub mod scene_system;
pub mod collision_system;
pub mod audio_system;
pub mod toolbar;
pub mod bitacora_ui;
pub mod excel_integration;
pub mod dialogue_ui;

pub use dialogue_ui::{
    DialogueManager,
    DialogueUI,
    OptionsManager,
    DialogueAnimationSystem,
    DialogueLineType as DialogueLine,
    DialogueChoice,
    DialogueVariable,
    DialogueContext,
};

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
pub mod explorer_context_menu;

pub use explorer_panel_integration::ExplorerPanelIntegration;
pub use explorer_context_menu::{ExplorerContextMenu, ExplorerContextAction};
pub use event_play_integration::EventPlayIntegration;
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
pub use scene_system::{SceneManager, SceneGraph};
pub use collision_system::{CollisionSystem, PhysicsCollisionSystem, CollisionType, AABB, CircleCollider};
pub use audio_system::{
    AudioManager,
    SfxManager,
    MusicManager,
    AudioMixer,
    SfxId,
    MusicTrackId,
    AudioEvent,
    PlaySessionAudioExt,
};
pub mod ui_system;
pub use ui_system::{
    UiLayerId,
    UiComponentId,
    UiComponentType,
    UiComponentState,
    UiPos,
    UiSize,
    UiColor,
    UiFont,
    UiTextAlign,
    UiVerticalAlign,
    Button,
    Panel,
    Text,
    Slider,
    UIManager,
    PlaySessionUiExt,
};
pub use bitacora_ui::{
    BitacoraPanel,
    Notification,
    NotificationType,
    NavigationDirection,
    BitacoraDateRange,
    BitacoraCategory,
    BitacoraSeverity,
    BitacoraEntry,
    BitacoraFilter,
    SearchResults,
};
pub use excel_integration::{
    Workbook,
    ExcelSheet,
    Cell,
    Row,
    CellValue,
    CellType,
    CellStyle,
    CellRange,
    SheetName,
    DataType,
    Column,
    ExcelWriter,
    SheetManager,
    DataValidator,
    ValidationRule,
    ValidationError,
};
pub mod preview_panel;
pub use preview_panel::PreviewPanel;
pub use preview_panel::AssetInfo;
pub use preview_panel::AssetType;
pub use property_panel::{PropertyPanel, EntityId};
pub use live_sync_manager::SyncEvent;
pub use delta_sync::DeltaEncoder;


