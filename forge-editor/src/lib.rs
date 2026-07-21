//! # Forge Editor
//! 
//! Editor de juego 2D con sistema de nodos, viewport, asset browser y preview de assets.

pub mod math;

pub mod ui;

/// Console log structure
#[derive(Debug, Clone)]
pub struct ConsoleLog {
    pub messages: Vec<String>,
}

impl ConsoleLog {
    pub fn new() -> Self {
        Self {
            messages: Vec::new(),
        }
    }
    
    pub fn add_message(&mut self, level: LogLevel, message: &str) {
        self.messages.push(format!("[{:?}] {}", level, message));
    }
    
    pub fn clear(&mut self) {
        self.messages.clear();
    }
    
    pub fn len(&self) -> usize {
        self.messages.len()
    }
    
    pub fn is_empty(&self) -> bool {
        self.messages.is_empty()
    }
}

pub mod physics_2d;
pub mod particle_system;
pub mod animation_2d;
pub mod map_export;
pub mod property_panel;
pub mod property_types;
pub mod transform_editor;
pub mod component_editor;
pub mod timeline;
pub mod keyframe;
pub mod animation_track;
pub mod forge_scene_stub;

pub use forge_scene_stub::{
    Scene, Asset, SceneTree, TransformProps, NodeType, Transform,
};
pub use ::forge_scene::{ComponentData, ComponentType, TransformData};
pub mod export_manager;
pub mod import_manager;
pub mod dialogue_editor;
pub mod export_import_panel;
pub mod event_node_editor;
pub mod script_viewer;
pub mod script_serialization;
pub mod serialization_panel;
pub mod editor_panel;

pub mod script_executor;
pub mod compile_system;
pub mod compile_panel;
pub mod debugger;
pub mod debugger_panel;
pub mod hot_reload;
pub mod hot_reload_panel;
pub mod script_optimizer;
pub mod script_optimizer_panel;
pub mod collaboration;
pub mod collaboration_panel;
pub mod testing;
pub mod plugins;
pub mod event_node_manager;
pub mod event_nodes;
pub mod cable_system;
pub mod project_manager;

pub use math::*;
pub use physics_2d::{Physics2D, PhysicsBlock, ColliderType};
pub use particle_system::{ParticleSystem};
pub use animation_2d::{Animation2DManager};
pub use property_panel::{PropertyPanel, TransformProperties, ComponentProperties, ScriptProperties};
pub use property_panel::EntityId;
pub use transform_editor::TransformEditor;
pub use component_editor::ComponentEditor;
pub use keyframe::{Keyframe, KeyframeEditor};
pub use event_node_manager::EventNodeManager;
pub use editor_panel::EditorPanel;

pub use animation_track::{AnimationTrack, TrackData, InterpolationType};
pub use ui::timeline::TimelineEditor;
pub use export_manager::ExportManager;
pub use import_manager::ImportManager;
pub use map_export::ExportManager as MapExport;
pub use dialogue_editor::DialogueManager;
use serde::{Deserialize, Serialize};
use std::fs;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use uuid::Uuid;
use crate::debugger::LogLevel;

pub use ui::*;
use ui::AssetType;

pub use script_serialization::{
    SerializedScript, SerializedVariable, SerializedScriptError,
    SerializedEventGraph, SerializedEventNode, SerializedEdge,
    EditorConfig, CompleteProjectExport,
};



pub use script_executor::{
    ScriptExecutor,
};

pub use compile_system::{
    CompileResult, CompileError, CompileWarning, SourceLocation, ASTNode,
    DataType, ValueType, ErrorKind,
};

pub use debugger::{
    Debugger, Breakpoint, StackFrame, DebugState,
    DebugLog, DebuggerConfig,
};

pub use hot_reload::{
    HotReloadManager, HotReloadConfig,
    FileChange, ChangeType, FileVersion,
    LivePreview, PreviewMode, HotReloadState,
};

pub use script_optimizer::{
    ScriptOptimizer, OptimizationLevel, OptimizationType,
    OptimizationResult, OptimizationChange,
    PerformanceMetrics, ExpressionOptimizer,
    ASTOptimizer,
};

pub use collaboration::{
    CollaborationManager, CollaborationConfig, UserInfo, UserStatus,
    CursorPosition, UserCursor, ChatMessage, SharedClipboard,
    SelectionRange, SyncChange, PresenceUpdate, CollaborationSession,
};

pub use testing::{
    TestRunner, TestSuite, TestResult, TestStatus, TestLocation,
    TestFixture, TestFilter, TestReport, CoverageReport, CoverageCollector,
    TestReporter, MockError, Stub, StubMap, MockClock, MockTimer,
    MockLogger, LogEntry, FixtureBuilder, Fixture, FixtureConfig,
    Value, CoverageLine, CoverageFunction, CoverageBranch, CoverageAnalyzer,
};

pub use plugins::{
    PluginId, PluginMetadata, PluginHook, PluginEvent, Plugin,
    PluginManager, PluginLoader, PluginValidator, PluginHotReload,
    PluginSandbox, PluginSandboxExecutor, HookManager, HookLifecycleManager,
    PluginHookRegistry, EventSystem, EventManager, EventBus,
    ExtensionType, ExtensionMetadata, ExtensionLoader, ExtensionRegistry,
    ExtensionUpdateCheck, PluginConfig, PluginManifest,
    PluginTab, PluginTypeFilter, PluginSortBy,
};

pub use project_manager::{
    Project, GameType, PhysicsConfig, ProjectWizard, ProjectManager,
};

use egui_dock::{DockState, DockArea, Style};
use pollster::FutureExt;
use crate::forge_scene_stub as forge_scene;

/// Aplicación principal del Forge Editor
pub struct ForgeEditorApp {
    /// Escena actual
    pub scene: Scene,
    
    /// Assets cargados
    pub assets: HashMap<String, Asset>,
    
    /// Preview de asset seleccionado
    pub asset_preview: AssetPreview,
    
    /// Estado de la ventana
    pub window_pos: egui::Pos2,
    pub window_size: egui::Vec2,
    
    /// Dock layout
    pub dock_state: DockState<ui::Tab>,
    
    /// Timeline actual
    pub current_timeline: Option<String>,
    
    /// Zoom del viewport
    pub viewport_zoom: f32,
    
    /// Offset de cámara
    pub camera_offset: [f32; 3],
    
    /// Viewport 2D
    pub viewport: ui::Viewport,
    
    /// Drag operation
    pub drag_operation: Option<DragOperation>,
    
    /// Asset arrastrado (usando forge-scene::Asset real)
    pub dragged_asset: Option<crate::forge_scene::Asset>,
    
    /// Timeline playing
    pub timeline_playing: bool,
    
    /// Timeline current time
    pub timeline_time: f32,
    
    /// Audio playing
    pub audio_playing: bool,
    
    /// Asset cargado actualmente (usando forge-scene::Asset real)
    pub current_asset: Option<crate::forge_scene::Asset>,
    
    /// Physics manager
    pub physics: Physics2D,
    
    /// Particle system
    pub particles: particle_system::ParticleSystem,
    
    /// Animation manager
    pub animation: animation_2d::Animation2DManager,
    
    /// Export manager
    pub export_mgr: ExportManager,
    
    /// Import manager
    pub import_mgr: ImportManager,
    
    /// Timeline editor
    pub timeline: crate::ui::timeline::TimelineEditor,
    
    /// Animation tracks
    pub animation_tracks: Vec<crate::animation_track::Track>,
    
    /// Dialogue manager
    pub dialogue_manager: crate::dialogue_editor::DialogueManager,
    
    /// Scene tree
    pub scene_tree: SceneTree,
    
    /// Console output
    pub console: ConsoleLog,
    
    /// Asset browser
    pub asset_browser: ui::AssetBrowser,

    /// Pending asset import path and name
    pub pending_import: Option<(std::path::PathBuf, String)>,
    
    /// Drop target
    pub drop_target: Option<String>,
    
    /// Viewport rect
    pub viewport_rect: egui::Rect,
    
    /// Scene tree selected
    pub scene_tree_selected: Vec<usize>,
    
    /// Transform properties
    pub transform_props: TransformProps,

    /// Component properties
    pub component_props: crate::ui::component_properties::ComponentProperties,

    /// Script properties
    pub script_props: crate::ui::script_properties::ScriptProperties,
    
    /// Selected entity sprite path
    pub selected_entity_sprite_path: Option<String>,
    
    /// Entity type
    pub entity_type: NodeType,
    
    /// Entity name
    pub entity_name: String,
    
    /// Entity ID
    pub entity_id: Uuid,
    
    /// Entity parent ID
    pub entity_parent_id: Option<Uuid>,
    
    /// Entity transform
    pub entity_transform: Transform,
    
    /// Event node manager
    pub event_node_manager: crate::event_node_manager::EventNodeManager,
    
    /// Script viewer
    pub script_viewer: crate::ui::script_viewer::ScriptViewer,
    
    /// Serialization panel
    pub serialization_panel: crate::ui::serialization_panel::SerializationPanel,
    
    /// Compile system
    pub compile_system: crate::compile_system::CompileSystem,
    
    /// Script executor
    pub script_executor: crate::script_executor::ScriptExecutor,
    
    /// Compile panel
    pub compile_panel: crate::ui::compile_panel::CompilePanel,
    
    /// Debugger
    pub debugger: crate::debugger::Debugger,
    
    /// Debugger panel
    pub debugger_panel: crate::ui::debugger_panel::DebuggerPanel,
    
    /// Hot reload manager
    pub hot_reload_manager: crate::hot_reload::HotReloadManager,
    
    /// Hot reload panel
    pub hot_reload_panel: crate::ui::hot_reload_panel::HotReloadPanel,
    
    /// Script optimizer
    pub script_optimizer: crate::script_optimizer::ScriptOptimizer,
    
    /// Script optimizer panel
    pub script_optimizer_panel: crate::ui::script_optimizer_panel::ScriptOptimizerPanel,
    
    /// Collaboration manager
    pub collaboration_manager: crate::collaboration::CollaborationManager,
    
    /// Collaboration panel
    pub collaboration_panel: crate::ui::collaboration_panel::CollaborationPanel,
    
    /// Test runner
    pub test_runner: crate::testing::test_runner::TestRunner,
    
    /// Test reporter
    pub test_reporter: crate::testing::test_runner::TestReporter,
    
    /// Coverage collector
    pub coverage_collector: crate::testing::coverage::CoverageCollector,
    
    /// Plugin manager
    pub plugin_manager: crate::plugins::PluginManager,
    
    /// Plugin loader
    pub plugin_loader: crate::plugins::PluginLoader,
    
    /// Plugin validator
    pub plugin_validator: crate::plugins::PluginValidator,
    
    /// Extension loader
    pub extension_loader: crate::plugins::ExtensionLoader,
    
    /// Extension registry
    pub extension_registry: crate::plugins::ExtensionRegistry,
    
    /// Event system
    pub event_system: crate::plugins::EventSystem,
    
    /// Event manager
    pub event_manager: crate::plugins::EventManager,
    
    /// Plugin UI panel
    pub plugin_panel: crate::ui::plugin_panel::PluginPanel,
    
    /// Project manager
    pub project_manager: crate::project_manager::ProjectManager,
    
    /// Current project path
    pub current_project_path: Option<PathBuf>,
    
    /// Game type
    pub game_type: GameType,
}

impl Default for ForgeEditorApp {
    fn default() -> Self {
        // Default construye usando los mismos defaults que new() pero sin CreationContext
        // Solo usado internamente donde no hay contexto eframe disponible
        Self {
            scene: Scene::default(),
            assets: HashMap::new(),
            asset_preview: AssetPreview::default(),
            window_pos: egui::Pos2::ZERO,
            window_size: egui::Vec2::new(1280.0, 720.0),
            dock_state: DockState::new(vec![ui::Tab::Editor]),
            current_timeline: None,
            viewport_zoom: 1.0,
            camera_offset: [0.0, 0.0, 0.0],
            viewport: ui::Viewport::default(),
            drag_operation: None,
            timeline_playing: false,
            timeline_time: 0.0,
            audio_playing: false,
            physics: Physics2D::default(),
            particles: particle_system::ParticleSystem::default(),
            animation: animation_2d::Animation2DManager::default(),
            export_mgr: ExportManager::default(),
            import_mgr: ImportManager::default(),
            timeline: crate::ui::timeline::TimelineEditor::new(),
            animation_tracks: vec![],
            dialogue_manager: crate::dialogue_editor::DialogueManager::new(),
            scene_tree: SceneTree::default(),
            console: ConsoleLog::new(),
            asset_browser: ui::AssetBrowser::default(),
            pending_import: None,
            dragged_asset: None,
            drop_target: None,
            current_asset: None,
            viewport_rect: egui::Rect::NOTHING,
            scene_tree_selected: vec![],
            transform_props: TransformProps::default(),
            component_props: crate::ui::component_properties::ComponentProperties::new(),
            script_props: crate::ui::script_properties::ScriptProperties::new(),
            selected_entity_sprite_path: None,
            entity_type: NodeType::default(),
            entity_name: String::new(),
            entity_id: Uuid::new_v4(),
            entity_parent_id: None,
            entity_transform: Transform::default(),
            event_node_manager: crate::event_node_manager::EventNodeManager::new(),
            script_viewer: crate::ui::script_viewer::ScriptViewer::new(),
            serialization_panel: crate::ui::serialization_panel::SerializationPanel::new(),
            compile_system: crate::compile_system::CompileSystem::new(),
            script_executor: crate::script_executor::ScriptExecutor::new(),
            compile_panel: crate::ui::compile_panel::CompilePanel::new(),
            debugger: crate::debugger::Debugger::new(None),
            debugger_panel: crate::ui::debugger_panel::DebuggerPanel::new(),
            hot_reload_manager: crate::hot_reload::HotReloadManager::new(None),
            hot_reload_panel: crate::ui::hot_reload_panel::HotReloadPanel::new(),
            script_optimizer: crate::script_optimizer::ScriptOptimizer::new(crate::script_optimizer::OptimizationLevel::O1),
            script_optimizer_panel: crate::ui::script_optimizer_panel::ScriptOptimizerPanel::new(),
            collaboration_manager: crate::collaboration::CollaborationManager::new(None),
            collaboration_panel: crate::ui::collaboration_panel::CollaborationPanel::new(),
            test_runner: crate::testing::test_runner::TestRunner::new("Forge Editor Tests".to_string(), false, 5000),
            test_reporter: crate::testing::test_runner::TestReporter::new(),
            coverage_collector: crate::testing::coverage::CoverageCollector::new(),
            plugin_manager: crate::plugins::PluginManager::new(None, None),
            plugin_loader: crate::plugins::PluginLoader::new(None, None),
            plugin_validator: crate::plugins::PluginValidator::new(
                vec![".json".to_string(), ".cs".to_string(), ".dll".to_string()],
                vec!["plugin.json".to_string()],
                100.0,
                "0.0.0".to_string(),
                "999.999.999".to_string(),
            ),
            extension_loader: crate::plugins::ExtensionLoader::new(None, None, true, 60000),
            extension_registry: crate::plugins::ExtensionRegistry::new(None, 300000),
            event_system: crate::plugins::EventSystem::new(1000),
            event_manager: crate::plugins::EventManager::new(),
            plugin_panel: crate::ui::plugin_panel::PluginPanel::new(),
            project_manager: crate::project_manager::ProjectManager::new(),
            current_project_path: None,
            game_type: crate::project_manager::GameType::default(),
        }
    }
}

impl ForgeEditorApp {
    /// Crea una nueva instancia de la aplicación
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            timeline: crate::ui::timeline::TimelineEditor::new(),
            animation_tracks: vec![],
            dialogue_manager: crate::dialogue_editor::DialogueManager::new(),
            event_node_manager: crate::event_node_manager::EventNodeManager::new(),
            script_viewer: crate::ui::script_viewer::ScriptViewer::new(),
            serialization_panel: crate::ui::serialization_panel::SerializationPanel::new(),
            compile_system: crate::compile_system::CompileSystem::new(),
            script_executor: crate::script_executor::ScriptExecutor::new(),
            compile_panel: crate::ui::compile_panel::CompilePanel::new(),
            debugger: crate::debugger::Debugger::new(None),
            debugger_panel: crate::ui::debugger_panel::DebuggerPanel::new(),
            hot_reload_manager: crate::hot_reload::HotReloadManager::new(None),
            hot_reload_panel: crate::ui::hot_reload_panel::HotReloadPanel::new(),
            script_optimizer: crate::script_optimizer::ScriptOptimizer::new(crate::script_optimizer::OptimizationLevel::O1),
            script_optimizer_panel: crate::ui::script_optimizer_panel::ScriptOptimizerPanel::new(),
            collaboration_manager: crate::collaboration::CollaborationManager::new(None),
            collaboration_panel: crate::ui::collaboration_panel::CollaborationPanel::new(),
            test_runner: crate::testing::test_runner::TestRunner::new("Forge Editor Tests".to_string(), false, 5000),
            test_reporter: crate::testing::test_runner::TestReporter::new(),
            coverage_collector: crate::testing::coverage::CoverageCollector::new(),
            plugin_manager: crate::plugins::PluginManager::new(None, None),
            plugin_loader: crate::plugins::PluginLoader::new(None, None),
            plugin_validator: crate::plugins::PluginValidator::new(
                vec![".json".to_string(), ".cs".to_string(), ".dll".to_string()],
                vec!["plugin.json".to_string()],
                100.0,
                "0.0.0".to_string(),
                "999.999.999".to_string(),
            ),
            extension_loader: crate::plugins::ExtensionLoader::new(None, None, true, 60000),
            extension_registry: crate::plugins::ExtensionRegistry::new(None, 300000),
            event_system: crate::plugins::EventSystem::new(1000),
            event_manager: crate::plugins::EventManager::new(),
            plugin_panel: crate::ui::plugin_panel::PluginPanel::new(),
            component_props: crate::ui::component_properties::ComponentProperties::new(),
            script_props: crate::ui::script_properties::ScriptProperties::new(),
            pending_import: None,
            ..Default::default()
        }
    }
    
    /// Carga una escena desde un archivo
    pub fn load_scene(&mut self, path: &str) -> Result<(), String> {
        let content = fs::read_to_string(path)
            .map_err(|e| format!("Error leyendo archivo: {}", e))?;
        
        let scene: Scene = serde_json::from_str(&content)
            .map_err(|e| format!("Error parseando JSON: {}", e))?;
        
        self.scene = scene;
        Ok(())
    }
    
    /// Guarda la escena actual
    pub fn save_scene(&self, path: &str) -> Result<(), String> {
        let content = serde_json::to_string_pretty(&self.scene)
            .map_err(|e| format!("Error serializando JSON: {}", e))?;
        
        fs::write(path, content)
            .map_err(|e| format!("Error escribiendo archivo: {}", e))?;
        
        Ok(())
    }
    
    /// Actualiza el timeline con el delta de tiempo
    pub fn update_timeline(&mut self, dt: f32) {
        if self.timeline.is_playing {
            self.timeline.animation_manager.update(dt);
            
            // Actualizar tiempo de animación
            self.animation.update(dt);
        }
    }
    
    /// Actualiza el sistema de física
    pub fn update_physics(&mut self, dt: f32) {
        self.physics.apply_gravity();
        self.physics.update();
    }
    
    /// Actualiza el sistema de partículas
    pub fn update_particles(&mut self, dt: f32) {
        self.particles.update();
    }
    
    /// Actualiza todos los sistemas del editor
    pub fn update(&mut self, dt: f32) {
        self.update_physics(dt);
        self.update_particles(dt);
        self.update_timeline(dt);
    }
    
    /// Renderiza el editor de diálogos
    pub fn render_dialogue_editor(&mut self) {
        self.dialogue_manager.render();
    }

    /// Exporta el proyecto actual a un archivo .map
    pub fn export_project(&mut self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        self.export_mgr.set_name(&self.scene.name);
        self.export_mgr.export(path)?;
        Ok(())
    }

    /// Importa un proyecto desde un archivo .map
    pub fn import_project(&mut self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        self.import_mgr.import(path)?;
        self.import_mgr.validate()?;
        Ok(())
    }

    /// Exporta el proyecto en formato JSON
    pub fn export_json(&mut self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let export_data = self.export_mgr.get_project_mut().clone();
        let content = serde_json::to_string_pretty(&export_data)?;
        std::fs::write(path, content)?;
        Ok(())
    }

    /// Importa un proyecto desde JSON
    pub fn import_json(&mut self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let content = std::fs::read_to_string(path)?;
        let project: export_manager::ProjectData = serde_json::from_str(&content)?;
        
        self.import_mgr.project = project;
        Ok(())
    }

    /// Obtiene el manager de exportación
    pub fn get_export_mgr(&self) -> &ExportManager {
        &self.export_mgr
    }

    /// Obtiene el manager de importación
    pub fn get_import_mgr(&self) -> &ImportManager {
        &self.import_mgr
    }

    /// Obtiene el manager de nodos de evento
    pub fn get_event_node_manager(&mut self) -> &mut crate::event_node_manager::EventNodeManager {
        &mut self.event_node_manager
    }

    /// Renderiza el script viewer
    pub fn render_script_viewer(&mut self, ui: &mut egui::Ui) {
        self.script_viewer.render(ui, self);
    }

    /// Obtiene el panel de serialización
    pub fn get_serialization_panel(&self) -> &crate::ui::serialization_panel::SerializationPanel {
        &self.serialization_panel
    }

    /// Compila un script desde código fuente
    pub fn compile_script(&mut self, source: &str) -> CompileResult {
        self.compile_system.compile(source)
    }

    /// Ejecuta un AST
    pub fn execute_ast(&mut self, ast: &ASTNode) -> Result<ValueType, CompileError> {
        self.script_executor.execute_from_ast(ast)
    }

    /// Ejecuta un script desde código fuente
    pub fn run_script(&mut self, source: &str) -> Result<ValueType, CompileError> {
        self.compile_script(source);
        if self.compile_system.last_result.is_success() {
            self.script_executor.execute_from_source(source)
        } else {
            Err(self.compile_system.last_result.errors.first().cloned().unwrap_or_else(|| {
                CompileError::new(
                    SourceLocation::default(),
                    "Compilation failed".to_string(),
                    crate::compile_system::ErrorKind::ParseError("Unknown error".to_string())
                )
            }))
        }
    }

    /// Renderiza el panel de compilación
    pub fn render_compile_panel(&mut self) {
        self.compile_panel.ui(&egui::Context::default());
    }

    /// Agrega breakpoint en línea
    pub fn add_breakpoint(&mut self, line: usize, column: usize, conditional: Option<String>) {
        self.debugger.add_breakpoint(line, column, conditional);
    }

    /// Elimina breakpoint
    pub fn remove_breakpoint(&mut self, line: usize, column: usize) {
        self.debugger.remove_breakpoint(line, column);
    }

    /// Toggles breakpoint
    pub fn toggle_breakpoint(&mut self, line: usize, column: usize) -> bool {
        self.debugger.toggle_breakpoint(line, column)
    }

    /// Agrega variable al watch
    pub fn add_watch(&mut self, var_name: String) {
        self.debugger.add_watch(var_name);
    }

    /// Elimina variable del watch
    pub fn remove_watch(&mut self, var_name: &str) {
        self.debugger.remove_watch(var_name);
    }

    /// Paso adelante
    pub fn step_next(&mut self) {
        self.debugger.step_next();
    }

    /// Paso dentro
    pub fn step_into(&mut self) {
        self.debugger.step_into();
    }

    /// Paso sobre
    pub fn step_over(&mut self) {
        self.debugger.step_over();
    }

    /// Paso fuera
    pub fn step_out(&mut self) {
        self.debugger.step_out();
    }

    /// Resume
    pub fn resume(&mut self) {
        self.debugger.resume();
    }

    /// Stop
    pub fn stop(&mut self) {
        self.debugger.stop();
    }

    /// Evalúa expresión
    pub fn evaluate_expression(&mut self, expr: &str) -> Result<ValueType, CompileError> {
        self.debugger.evaluate_expression(expr)
    }

    /// Obtiene valor de variable
    pub fn get_variable_value(&self, var_name: &str) -> Option<ValueType> {
        self.debugger.get_variable_value(var_name)
    }

    /// Obtiene frame actual
    pub fn current_frame(&self) -> Option<&StackFrame> {
        self.debugger.current_frame()
    }

    /// Obtiene estado
    pub fn state(&self) -> &DebugState {
        self.debugger.state()
    }

    /// Renderiza el panel de debugger
    pub fn render_debugger_panel(&mut self) {
        self.debugger_panel.ui(&egui::Context::default());
    }

    /// Renderiza el panel de hot reload
    pub fn render_hot_reload_panel(&mut self) {
        self.hot_reload_panel.ui(&egui::Context::default());
    }

    /// Renderiza el panel de script optimizer
    pub fn render_script_optimizer_panel(&mut self) {
        self.script_optimizer_panel.ui(&egui::Context::default());
    }

    /// Registra archivo en hot reload
    pub fn register_file(&mut self, file_path: &str, content: &str) {
        self.hot_reload_manager.register_file(file_path, content);
    }

    /// Actualiza archivo
    pub fn update_file(&mut self, file_path: &str, content: &str) {
        self.hot_reload_manager.update_file(file_path, content);
    }

    /// Revertir archivo
    pub fn revert_file(&mut self, file_path: &str) -> Option<String> {
        self.hot_reload_manager.revert_file(file_path)
    }

    /// Obtiene estado de hot reload
    pub fn hot_reload_state(&self) -> &HotReloadState {
        self.hot_reload_manager.state()
    }

    /// Obtiene live preview
    pub fn get_live_preview(&self) -> &LivePreview {
        self.hot_reload_manager.get_live_preview()
    }

    /// Obtiene pending changes
    pub fn get_pending_changes(&self) -> &HashMap<String, FileChange> {
        self.hot_reload_manager.get_pending_changes()
    }

    /// Obtiene manager
    pub fn hot_reload_manager(&self) -> &HotReloadManager {
        &self.hot_reload_manager
    }

    /// Obtiene config
    pub fn config(&self) -> &HotReloadConfig {
        self.hot_reload_manager.config()
    }

    /// Obtiene reload count
    pub fn reload_count(&self) -> u64 {
        self.hot_reload_manager.reload_count()
    }

    /// Obtiene execution time
    pub fn execution_time(&self) -> f64 {
        self.hot_reload_manager.execution_time()
    }

    /// Obtiene version map
    pub fn version_map(&self) -> &HashMap<String, FileVersion> {
        self.hot_reload_manager.version_map()
    }

    /// Obtiene script executor
    pub fn script_executor(&self) -> &ScriptExecutor {
        self.hot_reload_manager.script_executor()
    }

    /// Obtiene current script
    pub fn get_current_script(&self) -> Option<&str> {
        self.hot_reload_manager.get_current_script()
    }

    /// Obtiene last error
    pub fn get_last_error(&self) -> Option<&CompileError> {
        self.hot_reload_manager.get_last_error()
    }

    /// Obtiene debounce count
    pub fn debounce_count(&self) -> u64 {
        self.hot_reload_panel.debounce_count
    }

    /// Obtiene preview result
    pub fn preview_result(&self) -> Option<&String> {
        self.hot_reload_panel.preview_result.as_ref()
    }

    /// Obtiene preview error
    pub fn preview_error(&self) -> Option<&String> {
        self.hot_reload_panel.preview_error.as_ref()
    }

    /// Obtiene selected file
    pub fn selected_file(&self) -> Option<&String> {
        self.hot_reload_panel.selected_file.as_ref()
    }

    /// Obtiene diff view
    pub fn diff_view(&self) -> &DiffView {
        &self.hot_reload_panel.diff_view
    }

    /// Obtiene show version history
    pub fn show_version_history(&self) -> bool {
        self.hot_reload_panel.show_version_history
    }

    /// Obtiene watch extensions
    pub fn watch_extensions(&self) -> &[String] {
        self.hot_reload_manager.watch_extensions()
    }

    /// Obtiene debounce ms
    pub fn debounce_ms(&self) -> u64 {
        self.hot_reload_manager.debounce_ms()
    }

    /// Obtiene preview mode
    pub fn preview_mode(&self) -> &PreviewMode {
        self.hot_reload_manager.preview_mode()
    }

    /// Cambia preview mode
    pub fn set_preview_mode(&mut self, mode: PreviewMode) {
        self.hot_reload_manager.set_preview_mode(mode);
    }

    /// Obtiene script optimizer
    pub fn script_optimizer(&self) -> &ScriptOptimizer {
        &self.script_optimizer
    }

    /// Obtiene optimization level
    pub fn optimization_level(&self) -> &OptimizationLevel {
        self.script_optimizer.level()
    }

    /// Obtiene performance metrics
    pub fn performance_metrics(&self) -> &PerformanceMetrics {
        self.script_optimizer.performance_metrics()
    }

    /// Obtiene optimization results
    pub fn optimization_results(&self) -> &[OptimizationResult] {
        self.script_optimizer.optimization_results()
    }

    /// Obtiene optimization suggestions
    pub fn optimization_suggestions(&self) -> &[String] {
        self.script_optimizer_panel.optimization_suggestions.as_slice()
    }

    /// Obtiene selected result
    pub fn selected_result(&self) -> Option<usize> {
        self.script_optimizer_panel.selected_result
    }

    /// Obtiene optimization history
    pub fn optimization_history(&self) -> &[OptimizationResult] {
        self.script_optimizer_panel.optimization_history.as_slice()
    }

    /// Obtiene auto optimize
    pub fn auto_optimize(&self) -> bool {
        self.script_optimizer_panel.auto_optimize
    }

    /// Obtiene selected level
    pub fn selected_level(&self) -> &OptimizationLevel {
        &self.script_optimizer_panel.selected_level
    }

    /// Obtiene selected types
    pub fn selected_types(&self) -> &[OptimizationType] {
        &self.script_optimizer_panel.selected_types
    }

    /// Obtiene show performance
    pub fn show_performance(&self) -> bool {
        self.script_optimizer_panel.show_performance
    }

    /// Obtiene show diff
    pub fn show_diff(&self) -> bool {
        self.script_optimizer_panel.show_diff
    }

    /// Obtiene diff view
    pub fn optimizer_diff_view(&self) -> &DiffView {
        &self.script_optimizer_panel.diff_view
    }

    /// Obtiene optimization time
    pub fn optimization_time(&self) -> f64 {
        self.script_optimizer.optimization_time()
    }

    /// Obtiene operation count
    pub fn operation_count(&self) -> u64 {
        self.script_optimizer.operation_count()
    }

    /// Obtiene function calls
    pub fn function_calls(&self) -> u64 {
        self.script_optimizer.function_calls()
    }

    /// Obtiene loop iterations
    pub fn loop_iterations(&self) -> u64 {
        self.script_optimizer.loop_iterations()
    }

    /// Obtiene cache misses
    pub fn cache_misses(&self) -> u64 {
        self.script_optimizer.cache_misses()
    }

    /// Obtiene branch mispredictions
    pub fn branch_mispredictions(&self) -> u64 {
        self.script_optimizer.branch_mispredictions()
    }

    /// Obtiene total changes
    pub fn total_changes(&self) -> usize {
        self.script_optimizer.total_changes()
    }

    /// Obtiene total size reduction
    pub fn total_size_reduction(&self) -> f64 {
        self.script_optimizer.total_size_reduction()
    }

    /// Obtiene total complexity reduction
    pub fn total_complexity_reduction(&self) -> f64 {
        self.script_optimizer.total_complexity_reduction()
    }

    /// Obtiene warning count
    pub fn warning_count(&self) -> usize {
        self.script_optimizer.warning_count()
    }

    /// Obtiene result count
    pub fn result_count(&self) -> usize {
        self.script_optimizer.result_count()
    }

    /// Obtiene last result
    pub fn last_result(&self) -> Option<&OptimizationResult> {
        self.script_optimizer.last_result()
    }

    /// Obtiene last result mutable
    pub fn last_result_mut(&mut self) -> Option<&mut OptimizationResult> {
        self.script_optimizer.last_result_mut()
    }

    /// Obtienes enabled types
    pub fn enabled_types(&self) -> &[OptimizationType] {
        self.script_optimizer.enabled_types()
    }

    /// Cambia level de optimización
    pub fn set_optimization_level(&mut self, level: OptimizationLevel) {
        self.script_optimizer.level = level;
        self.script_optimizer_panel.selected_level = level;
    }

    /// Set enabled types
    pub fn set_enabled_types(&mut self, types: Vec<OptimizationType>) {
        self.script_optimizer.enabled_types = types.clone();
        self.script_optimizer_panel.selected_types = types;
    }

    /// Set auto optimize
    pub fn set_auto_optimize(&mut self, auto: bool) {
        self.script_optimizer_panel.auto_optimize = auto;
    }

    /// Set show performance
    pub fn set_show_performance(&mut self, show: bool) {
        self.script_optimizer_panel.show_performance = show;
    }

    /// Set show diff
    pub fn set_show_diff(&mut self, show: bool) {
        self.script_optimizer_panel.show_diff = show;
    }

    /// Clear results
    pub fn clear_optimization_results(&mut self) {
        self.script_optimizer.clear_results();
        self.script_optimizer_panel.optimization_history.clear();
    }

    /// Start optimization
    pub fn start_optimization(&mut self) {
        self.script_optimizer.start_optimization();
    }

    /// End optimization
    pub fn end_optimization(&mut self) {
        self.script_optimizer.end_optimization();
    }

    /// Add optimization result
    pub fn add_optimization_result(&mut self, result: OptimizationResult) {
        self.script_optimizer.add_result(result.clone());
        self.script_optimizer_panel.optimization_history.push(result);
    }

    /// Analyze script
    pub fn analyze_script(&mut self) {
        self.script_optimizer.start_optimization();
        // Simulate analysis
        self.script_optimizer.end_optimization();
        let mut result = OptimizationResult::new();
        result.optimized = true;
        result.size_before = 1000;
        result.size_after = 800;
        result.complexity_before = 10.0;
        result.complexity_after = 8.0;
        result.time_before = 50.0;
        result.time_after = 30.0;
        self.add_optimization_result(result);
    }

    /// Conecta usuario en colaboración
    pub fn connect_collaboration(&mut self, user_id: String, user_name: String) -> Option<String> {
        self.collaboration_manager.connect(String::new(), user_id, user_name)
    }

    /// Conecta como guest
    pub fn connect_collaboration_guest(&mut self, user_id: String) -> Option<String> {
        self.collaboration_manager.connect_as_guest(String::new(), user_id)
    }

    /// Desconecta colaboración
    pub fn disconnect_collaboration(&mut self) {
        self.collaboration_manager.disconnect();
    }

    /// Obtiene collaboration manager
    pub fn collaboration_manager(&self) -> &CollaborationManager {
        &self.collaboration_manager
    }

    /// Obtiene collaboration manager mutable
    pub fn collaboration_manager_mut(&mut self) -> &mut CollaborationManager {
        &mut self.collaboration_manager
    }

    /// Obtiene collaboration panel
    pub fn collaboration_panel(&self) -> &CollaborationPanel {
        &self.collaboration_panel
    }

    /// Obtiene collaboration panel mutable
    pub fn collaboration_panel_mut(&mut self) -> &mut CollaborationPanel {
        &mut self.collaboration_panel
    }

    /// Obtiene is connected
    pub fn is_collaboration_connected(&self) -> bool {
        self.collaboration_manager.is_connected()
    }

    /// Obtiene user count
    pub fn collaboration_user_count(&self) -> usize {
        self.collaboration_manager.user_count()
    }

    /// Obtiene chat history
    pub fn collaboration_chat_history(&self) -> &[ChatMessage] {
        self.collaboration_manager.get_chat_history()
    }

    /// Obtiene pending changes
    pub fn collaboration_pending_changes(&self) -> &[SyncChange] {
        self.collaboration_manager.pending_changes()
    }

    /// Obtiene local user
    pub fn local_user(&self) -> Option<&UserInfo> {
        self.collaboration_manager.local_user()
    }

    /// Obtiene local cursor
    pub fn local_cursor(&self) -> Option<&UserCursor> {
        self.collaboration_manager.local_cursor()
    }

    /// Obtiene connected users
    pub fn connected_users(&self) -> Vec<UserInfo> {
        self.collaboration_manager.connected_users()
            .iter()
            .filter_map(|id| self.collaboration_manager.session().and_then(|s| s.get_user(id)).cloned())
            .collect()
    }

    /// Obtiene clipboard
    pub fn clipboard(&self) -> Option<&SharedClipboard> {
        self.collaboration_manager.get_clipboard()
    }

    /// Actualiza clipboard
    pub fn update_clipboard(&mut self, content: String) {
        self.collaboration_manager.update_clipboard(content);
    }

    /// Obtiene session
    pub fn session(&self) -> Option<&CollaborationSession> {
        self.collaboration_manager.session()
    }

    /// Obtiene session mutable
    pub fn session_mut(&mut self) -> Option<&mut CollaborationSession> {
        self.collaboration_manager.session_mut()
    }

    /// Obtiene local user name
    pub fn local_user_name(&self) -> Option<&str> {
        self.collaboration_manager.local_user_name()
    }

    /// Obtiene local user status
    pub fn local_user_status(&self) -> Option<UserStatus> {
        self.collaboration_manager.local_user_status()
    }

    /// Obtiene local user joined at
    pub fn local_user_joined_at(&self) -> Option<u64> {
        self.collaboration_manager.local_user_joined_at()
    }

    /// Obtiene local user is owner
    pub fn local_user_is_owner(&self) -> bool {
        self.collaboration_manager.local_user_is_owner().unwrap_or(false)
    }

    /// Obtiene local user avatar
    pub fn local_user_avatar(&self) -> Option<&str> {
        self.collaboration_manager.local_user_avatar()
    }

    /// Obtiene local user status
    pub fn my_local_user_status(&self) -> Option<UserStatus> {
        self.collaboration_manager.local_user_status()
    }

    /// Obtiene local cursor position
    pub fn local_cursor_position(&self) -> Option<CursorPosition> {
        self.collaboration_manager.local_cursor_position()
    }

    /// Obtiene local cursor selection
    pub fn local_cursor_selection(&self) -> Option<SelectionRange> {
        self.collaboration_manager.local_cursor_selection()
    }

    /// Obtiene local cursor timestamp
    pub fn local_cursor_timestamp(&self) -> Option<u64> {
        self.collaboration_manager.local_cursor_timestamp()
    }

    /// Envía mensaje de chat
    pub fn send_chat_message(&mut self, message: String) {
        self.collaboration_manager.send_chat_message(message);
    }

    /// Obtiene chat message count
    pub fn chat_message_count(&self) -> usize {
        self.collaboration_manager.chat_message_count()
    }

    /// Obtiene cursor count
    pub fn cursor_count(&self) -> usize {
        self.collaboration_manager.cursor_count()
    }

    /// Obtiene last sync
    pub fn last_sync(&self) -> u64 {
        self.collaboration_manager.last_sync()
    }

    /// Obtiene sync counter
    pub fn sync_counter(&self) -> u64 {
        self.collaboration_manager.sync_counter()
    }

    /// Obtiene is host
    pub fn is_collaboration_host(&self) -> bool {
        self.collaboration_manager.is_host()
    }

    /// Obtiene config
    pub fn collaboration_config(&self) -> &CollaborationConfig {
        self.collaboration_manager.config()
    }

    /// Obtiene is active
    pub fn is_collaboration_active(&self) -> bool {
        self.collaboration_manager.is_active()
    }

    /// Obtiene project_id
    pub fn project_id(&self) -> Option<&str> {
        self.collaboration_manager.project_id()
    }

    /// Obtiene session_id
    pub fn session_id(&self) -> Option<&str> {
        self.collaboration_manager.session_id()
    }

    /// Obtiene host_id
    pub fn host_id(&self) -> Option<&str> {
        self.collaboration_manager.host_id()
    }

    /// Obtiene created_at
    pub fn created_at(&self) -> Option<u64> {
        self.collaboration_manager.created_at()
    }

    /// Obtiene owner
    pub fn owner(&self) -> Option<&UserInfo> {
        self.collaboration_manager.owner()
    }

    /// Obtiene presence updates
    pub fn presence_updates(&self) -> Vec<PresenceUpdate> {
        self.collaboration_manager.presence_updates().iter().cloned().collect()
    }

    /// Obtiene pending changes count
    pub fn pending_changes_count(&self) -> usize {
        self.collaboration_manager.pending_changes().len()
    }

    /// Obtiene clipboard user
    pub fn clipboard_user(&self) -> Option<&str> {
        self.collaboration_manager.clipboard_user()
    }

    /// Obtiene show presence
    pub fn show_presence(&self) -> bool {
        self.collaboration_panel.show_presence
    }

    /// Obtiene show cursor
    pub fn show_cursor(&self) -> bool {
        self.collaboration_panel.show_cursor
    }

    /// Obtiene show chat
    pub fn show_chat(&self) -> bool {
        self.collaboration_panel.show_chat
    }

    /// Obtiene show clipboard
    pub fn show_clipboard(&self) -> bool {
        self.collaboration_panel.show_clipboard
    }

    /// Obtiene chat input
    pub fn chat_input(&self) -> &str {
        &self.collaboration_panel.chat_input
    }

    /// Obtiene chat history
    pub fn chat_history(&self) -> &[ChatMessage] {
        &self.collaboration_panel.chat_history
    }

    /// Obtiene user list
    pub fn user_list(&self) -> &[UserInfo] {
        &self.collaboration_panel.user_list
    }

    /// Obtiene cursor list
    pub fn cursor_list(&self) -> &[UserCursor] {
        &self.collaboration_panel.cursor_list
    }

    /// Obtiene clipboard content
    pub fn clipboard_content(&self) -> &str {
        &self.collaboration_panel.clipboard_content
    }

    /// Obtiene last sync time
    pub fn last_sync_time(&self) -> &str {
        &self.collaboration_panel.last_sync_time
    }

    /// Obtiene sync interval ms
    pub fn sync_interval_ms(&self) -> u64 {
        self.collaboration_panel.sync_interval_ms
    }

    /// Set show presence
    pub fn set_show_presence(&mut self, show: bool) {
        self.collaboration_panel.show_presence = show;
    }

    /// Set show cursor
    pub fn set_show_cursor(&mut self, show: bool) {
        self.collaboration_panel.show_cursor = show;
    }

    /// Set show chat
    pub fn set_show_chat(&mut self, show: bool) {
        self.collaboration_panel.show_chat = show;
    }

    /// Set show clipboard
    pub fn set_show_clipboard(&mut self, show: bool) {
        self.collaboration_panel.show_clipboard = show;
    }

    /// Set chat input
    pub fn set_chat_input(&mut self, input: String) {
        self.collaboration_panel.chat_input = input;
    }

    /// Set clipboard content
    pub fn set_clipboard_content(&mut self, content: String) {
        self.collaboration_panel.clipboard_content = content;
    }

    /// Set last sync time
    pub fn set_last_sync_time(&mut self, time: String) {
        self.collaboration_panel.last_sync_time = time;
    }

    /// Set sync interval ms
    pub fn set_sync_interval_ms(&mut self, ms: u64) {
        self.collaboration_panel.sync_interval_ms = ms;
    }

    /// Update local cursor
    pub fn update_local_cursor(&mut self, position: CursorPosition, selection: Option<SelectionRange>) {
        self.collaboration_manager.update_local_cursor(position, selection);
    }

    /// Add remote user
    pub fn add_remote_user(&mut self, user_id: String, user_info: UserInfo) {
        self.collaboration_manager.add_remote_user(user_id, user_info);
    }

    /// Remove remote user
    pub fn remove_remote_user(&mut self, user_id: &str) {
        self.collaboration_manager.remove_remote_user(user_id);
    }

    /// Add remote cursor
    pub fn add_remote_cursor(&mut self, user_id: String, cursor: UserCursor) {
        self.collaboration_manager.add_remote_cursor(user_id, cursor);
    }

    /// Remove remote cursor
    pub fn remove_remote_cursor(&mut self, user_id: &str) {
        self.collaboration_manager.remove_remote_cursor(user_id);
    }

    /// Add presence update
    pub fn add_presence_update(&mut self, update: PresenceUpdate) {
        self.collaboration_manager.add_presence_update(update);
    }

    /// Add pending change
    pub fn add_pending_change(&mut self, change: SyncChange) {
        self.collaboration_manager.add_pending_change(change);
    }

    /// Set chat message
    pub fn add_to_chat_buffer(&mut self, message: ChatMessage) {
        self.collaboration_manager.add_to_chat_buffer(message);
    }

    /// Set clipboard
    pub fn set_clipboard(&mut self, content: String) {
        self.collaboration_manager.update_clipboard(content);
    }

    /// Obtiene test runner
    pub fn test_runner(&self) -> &TestRunner {
        &self.test_runner
    }

    /// Obtiene test runner mutable
    pub fn test_runner_mut(&mut self) -> &mut TestRunner {
        &mut self.test_runner
    }

    /// Obtiene test reporter
    pub fn test_reporter(&self) -> &TestReporter {
        &self.test_reporter
    }

    /// Obtiene test reporter mutable
    pub fn test_reporter_mut(&mut self) -> &mut TestReporter {
        &mut self.test_reporter
    }

    /// Obtiene coverage collector
    pub fn coverage_collector(&self) -> &CoverageCollector {
        &self.coverage_collector
    }

    /// Obtiene coverage collector mutable
    pub fn coverage_collector_mut(&mut self) -> &mut CoverageCollector {
        &mut self.coverage_collector
    }

    /// Ejecuta todos los tests
    pub fn run_all_tests(&mut self) -> TestReport {
        self.test_runner.run()
    }

    /// Ejecuta tests con filtro
    pub fn run_tests_with_filter(&mut self, filter: &str) -> TestReport {
        let test_runner_clone = self.test_runner.clone();
        test_runner_clone.with_filter(TestFilter::new().with_name_pattern(filter.to_string())).run()
    }

    /// Ejecuta tests en paralelo
    pub fn run_tests_parallel(&mut self) -> TestReport {
        let mut test_runner_clone = self.test_runner.clone();
        test_runner_clone.run()
    }

    /// Ejecuta tests con reporte verbose
    pub fn run_tests_verbose(&mut self) -> TestReport {
        let test_runner_clone = self.test_runner.clone();
        test_runner_clone.with_verbose(true).run()
    }

    /// Obtiene suite de tests
    pub fn test_suite(&self) -> &TestSuite {
        self.test_runner.suite()
    }

    /// Obtiene suite de tests mutable
    pub fn test_suite_mut(&mut self) -> &mut TestSuite {
        self.test_runner.suite_mut()
    }

    /// Obtiene count de tests pasados
    pub fn test_passed_count(&self) -> usize {
        self.test_suite().passed_count()
    }

    /// Obtiene count de tests fallidos
    pub fn test_failed_count(&self) -> usize {
        self.test_suite().failed_count()
    }

    /// Obtiene count de tests panic
    pub fn test_panicked_count(&self) -> usize {
        self.test_suite().panicked_count()
    }

    /// Obtiene count de tests skipped
    pub fn test_skipped_count(&self) -> usize {
        self.test_suite().skipped_count()
    }

    /// Obtiene count de tests ignored
    pub fn test_ignored_count(&self) -> usize {
        self.test_suite().ignored_count()
    }

    /// Obtiene count total de tests
    pub fn test_total_count(&self) -> usize {
        self.test_suite().test_count()
    }

    /// Obtiene si todos los tests pasaron
    pub fn tests_success(&self) -> bool {
        self.test_suite().success()
    }

    /// Obtiene duration total de tests
    pub fn test_total_duration(&self) -> f64 {
        self.test_suite().total_duration()
    }

    /// Agrega test manual
    pub fn add_test(&mut self, name: String, location: TestLocation) {
        self.test_runner.add_test(name, location);
    }

    /// Agrega fixture
    pub fn add_fixture(&mut self, fixture: TestFixture) {
        self.test_runner.add_fixture(fixture);
    }

    /// Obtiene coverage line coverage
    pub fn coverage_line_coverage(&self) -> f64 {
        self.coverage_collector.overall_line_coverage()
    }

    /// Obtiene coverage function coverage
    pub fn coverage_function_coverage(&self) -> f64 {
        self.coverage_collector.overall_function_coverage()
    }

    /// Obtiene coverage branch coverage
    pub fn coverage_branch_coverage(&self) -> f64 {
        self.coverage_collector.overall_branch_coverage()
    }

    /// Obtiene coverage overall
    pub fn coverage_overall(&self) -> f64 {
        self.coverage_collector.overall_coverage()
    }

    /// Obtiene report de coverage
    pub fn coverage_report(&self) -> String {
        self.coverage_collector.summary()
    }

    /// Renderiza el panel de tests
    pub fn render_test_panel(&mut self) {
        // Placeholder for test panel UI
    }

    /// Obtiene plugin manager
    pub fn plugin_manager(&self) -> &PluginManager {
        &self.plugin_manager
    }

    /// Obtiene plugin manager mutable
    pub fn plugin_manager_mut(&mut self) -> &mut PluginManager {
        &mut self.plugin_manager
    }

    /// Obtiene plugin loader
    pub fn plugin_loader(&self) -> &PluginLoader {
        &self.plugin_loader
    }

    /// Obtiene plugin validator
    pub fn plugin_validator(&self) -> &PluginValidator {
        &self.plugin_validator
    }

    /// Obtiene extension loader
    pub fn extension_loader(&self) -> &ExtensionLoader {
        &self.extension_loader
    }

    /// Obtiene extension registry
    pub fn extension_registry(&self) -> &ExtensionRegistry {
        &self.extension_registry
    }

    /// Obtiene event system
    pub fn event_system(&self) -> &EventSystem {
        &self.event_system
    }

    /// Obtiene event manager
    pub fn event_manager(&self) -> &EventManager {
        &self.event_manager
    }

    /// Obtiene plugin panel
    pub fn plugin_panel(&self) -> &crate::ui::plugin_panel::PluginPanel {
        &self.plugin_panel
    }

    /// Obtiene plugin panel mutable
    pub fn plugin_panel_mut(&mut self) -> &mut crate::ui::plugin_panel::PluginPanel {
        &mut self.plugin_panel
    }

    /// Carga todos los plugins
    pub fn load_all_plugins(&mut self) -> Result<(), String> {
        self.plugin_manager.load_all()?;
        Ok(())
    }

    /// Habilita plugin
    pub fn enable_plugin(&mut self, plugin_id: &PluginId) -> Result<(), String> {
        self.plugin_manager.enable_plugin(plugin_id)
    }

    /// Deshabilita plugin
    pub fn disable_plugin(&mut self, plugin_id: &PluginId) -> Result<(), String> {
        self.plugin_manager.disable_plugin(plugin_id)
    }

    /// Carga plugin
    pub fn load_plugin(&mut self, plugin_id: &PluginId) -> Result<(), String> {
        self.plugin_manager.load_plugin(plugin_id)
    }

    /// Descarga plugin
    pub fn unload_plugin(&mut self, plugin_id: &PluginId) -> Result<(), String> {
        self.plugin_manager.unload_plugin(plugin_id)
    }

    /// Renderiza el panel de plugins
    pub fn render_plugin_panel(&mut self, ui: &mut egui::Ui) {
        crate::ui::plugin_panel::render_plugin_panel(ui, &mut self.plugin_manager, &mut self.plugin_panel);
    }
}

/// Convierte scene a project data para export manager
impl ForgeEditorApp {
    fn scene_to_project_data(&self, scene: &Scene) -> 
crate::export_manager::ProjectData {
        crate::export_manager::ProjectData {
            name: "Forge Project".to_string(),
            version: "1.0.0".to_string(),
            entities: self.export_mgr.get_project().entities.clone(),
            physics: self.export_mgr.get_project().physics.clone(),
            particles: self.export_mgr.get_project().particles.clone(),
            animations: self.export_mgr.get_project().animations.clone(),
            dialogues: self.export_mgr.get_project().dialogues.clone(),
            events: self.export_mgr.get_project().events.clone(),
        }
    }

    /// Cargar assets del proyecto en el Asset Browser
    pub fn load_project_assets(&mut self) {
        if let Some(ref project) = self.project_manager.current_project {
            let assets_path = project.assets_path();
            self.asset_browser.load_from_current_assets_path(&assets_path);
            self.console.add_message(
                LogLevel::Info,
                &format!("Loaded {} assets from project: {}", self.asset_browser.assets.len(), project.name),
            );
        }
    }

    /// Agregar un asset a la escena actual
    pub fn add_asset_to_scene(&mut self, asset_name: &str) -> bool {
        self.asset_browser.add_asset_to_scene(self, asset_name)
    }

    /// Obtener asset por path (usando forge-scene::Asset real)
    pub fn get_asset(&self, path: &str) -> Option<crate::forge_scene::Asset> {
        // Buscar en los assets cargados
        for asset in self.asset_browser.assets.iter() {
            if asset.ends_with(path) {
                return Some(crate::forge_scene::Asset {
                    id: uuid::Uuid::new_v4().to_string(),
                    name: asset.to_string(),
                    path: path.to_string(),
                    asset_type: crate::forge_scene_stub::AssetType::Sprite,
                    size: 0,
                    is_loaded: false,
                });
            }
        }
        None
    }

    /// Crear asset desde path (usando forge-scene::Asset real)
    pub fn create_asset(&mut self, path: &str) -> crate::forge_scene::Asset {
        let asset_name = path.split('/').last().unwrap_or(path);
        let asset_type = self.get_asset_type_from_path(path);
        
        crate::forge_scene::Asset {
            id: uuid::Uuid::new_v4().to_string(),
            name: asset_name.to_string(),
            path: path.to_string(),
            asset_type,
            size: 0,
            is_loaded: false,
        }
    }

    /// Obtener tipo de asset desde path
    fn get_asset_type_from_path(&self, path: &str) -> crate::forge_scene_stub::AssetType {
        let extension = path.split('.').last().unwrap_or("").to_lowercase();
        match extension.as_str() {
            "png" | "jpg" | "jpeg" | "gif" | "bmp" | "webp" | "tga" => {
                crate::forge_scene_stub::AssetType::Sprite
            }
            "mp3" | "wav" | "ogg" | "flac" | "aiff" => {
                crate::forge_scene_stub::AssetType::Audio
            }
            "rs" | "lua" | "gdscript" | "js" | "ts" | "json" => {
                crate::forge_scene_stub::AssetType::Script
            }
            "csv" => {
                crate::forge_scene_stub::AssetType::Dialogue
            }
            _ => {
                crate::forge_scene_stub::AssetType::Other
            }
        }
    }
}

/// Implementación del trait App de eframe para ForgeEditorApp
impl eframe::App for ForgeEditorApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let dt: f32 = ctx.input(|i| i.stable_dt).min(0.1);
        self.update(dt);

        // Procesar importación pendiente
        if let Some((_path, asset_name)) = self.pending_import.take() {
            self.console.add_message(
                LogLevel::Info,
                &format!("Asset importado: {}", asset_name),
            );
        }

        // Panel superior
        egui::TopBottomPanel::top("menu_bar").show(ctx, |_ui| {
            // Menu renderizado desde tab_viewer cuando se usa como app independiente
        });

        // Panel izquierdo — Scene Tree
        egui::SidePanel::left("scene_tree_panel").resizable(true).show(ctx, |ui| {
            ui.heading("Scene Tree");
            ui.separator();
            for (i, entity) in self.scene.entities.iter().enumerate() {
                let selected = self.scene_tree_selected.contains(&i);
                if ui.selectable_label(selected, &entity.name).clicked() {
                    self.scene_tree_selected = vec![i];
                }
            }
        });

        // Panel derecho — Inspector
        egui::SidePanel::right("inspector_panel").resizable(true).show(ctx, |ui| {
            ui.heading("Inspector");
            ui.separator();
            if let Some(&idx) = self.scene_tree_selected.first() {
                if let Some(entity) = self.scene.entities.get(idx) {
                    ui.label(format!("Nombre: {}", entity.name));
                    ui.label(format!("Tipo: {:?}", entity.entity_type));
                }
            }
        });

        // Panel inferior — Asset Browser
        egui::TopBottomPanel::bottom("asset_browser_panel").resizable(true).show(ctx, |ui| {
            ui::AssetBrowser::render(ui, self);
        });

        // Panel central — Viewport
        egui::CentralPanel::default().show(ctx, |ui| {
            self.viewport.ui(ui, self);
        });

        ctx.request_repaint();
    }
}


