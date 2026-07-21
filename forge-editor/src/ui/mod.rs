//! # Forge Editor UI Module
//! 
//! ## 📐 Arquitectura del Sistema UI
//! 
//! Este módulo implementa una arquitectura de **UI desacoplada** donde cada panel es independiente
//! pero se conecta con los motores de fondo a través de referencias mutables a la aplicación.
//! 
//! ### Diagrama de Conexiones
//! 
//! ```
//! +------------------+     +------------------+     +------------------+
//! |  ForgeEditorApp  | --> |  TabViewer<'a>   | --> |  Panel UI        |
//! +------------------+     +------------------+     +------------------+
//!         |                       |                           |
//!         | (motors)              | (referencia)              | (render)
//!         v                       v                           v
//! +--------+                 +--------+                   +--------+
//! | Physics|                 |  Tabs  |                   |  Data  |
//! +--------+                 +--------+                   +--------+
//! ```
//! 
//! ### Flujo de Datos
//! 1. `ForgeEditorApp` contiene todos los motores (physics, particles, animation)
//! 2. `TabViewer<'a>` recibe una referencia mutable a la App
//! 3. Cada panel UI llama a métodos de los motores a través de la referencia
//! 
//! ### Módulos Principales
//! 
//! | Módulo | Archivo | Responsabilidad | Conectado a |
//! |--------|---------|-----------------|-------------|
//! | DiffView | diff_view.rs | Comparación de diferencias | hot_reload |

//! ### Diagrama de Conexiones
//! | Viewport | viewport.rs | Renderizado 2D, cámara, grid | physics, particles |
//! | PropertyEditor | property_editor.rs | Propiedades de entidades | transform_props |
//! | ComponentProperties | component_properties.rs | Propiedades de componentes | component_editor |
//! | ScriptProperties | script_properties.rs | Editor de scripts | script_editor |
//! | Timeline | timeline.rs | Keyframes, animación | animation, keyframe |
//! | Console | console.rs | Logs, errores, warnings | event_nodes |
//! 
//! ### Principios de Diseño
//! - **Single Responsibility**: Cada panel tiene una sola responsabilidad
//! - **Dependency Injection**: Los panels reciben referencias a los motores
//! - **Pure Functions**: Los métodos `ui()` son puros y no tienen efectos secundarios
//! - **Modularización**: Cada panel es independiente y puede reemplazarse

pub mod viewport;
pub mod scene_editor;
pub mod property_editor;
pub mod script_editor;
pub mod script_viewer;
pub mod serialization_panel;
pub mod editor_panel;

pub mod dockable;
pub mod toolbar;
pub mod menu_bar;
pub mod console;
pub mod asset_browser;
pub mod transform_properties;
pub mod component_properties;
pub mod script_properties;
pub mod timeline;
pub mod tab_viewer;
pub mod scene_tree_ui;
pub mod asset_preview;
pub mod compile_panel;
pub mod debugger_panel;
pub mod hot_reload_panel;

pub mod script_optimizer_panel;
pub mod collaboration_panel;
pub mod test_panel;
pub mod plugin_panel;

// Re-exportaciones para facilitar el uso
pub use viewport::Viewport;
pub use test_panel::{TestPanel, TestTab};
pub use scene_editor::SceneEditor;
pub use property_editor::PropertyEditor;
pub use script_editor::ScriptEditor;
pub use editor_panel::EditorPanel;

pub use dockable::Dockable;
pub use toolbar::Toolbar;
pub use menu_bar::MenuBar;
pub use console::Console;
pub use asset_browser::AssetBrowser;
pub use asset_preview::AssetPreview;
pub use transform_properties::TransformProperties;
pub use component_properties::ComponentProperties;
pub use script_properties::ScriptProperties;
pub use timeline::TimelineDockable;
pub use tab_viewer::Tab;
pub use scene_tree_ui::SceneTreeUI;
pub use compile_panel::CompilePanel;
pub use debugger_panel::DebuggerPanel;
pub mod diff_view;
pub use diff_view::DiffView;
pub use hot_reload_panel::HotReloadPanel;
pub use crate::script_executor::ScriptExecutor;
pub use script_optimizer_panel::ScriptOptimizerPanel;
pub use collaboration_panel::CollaborationPanel;
pub use plugin_panel::PluginPanel;
pub use script_viewer::ScriptViewer;
pub use serialization_panel::SerializationPanel;

// Tipos de drag & drop
#[derive(Debug, Clone, PartialEq)]
pub enum DragOperation {
    None,
    Asset { asset_name: String, asset_type: AssetType },
    Node { node_id: String },
}

#[derive(Debug, Clone, PartialEq)]
pub enum AssetType {
    Sprite,
    Audio,
    Script,
    Texture,
    Dialogue,
    Unknown,
}

impl AssetType {
    pub fn from_extension(extension: &str) -> Self {
        let ext = extension.to_lowercase();
        match ext.as_str() {
            "png" | "jpg" | "jpeg" | "gif" | "bmp" | "webp" | "tga" => AssetType::Texture,
            "fnt" => AssetType::Sprite,
            "mp3" | "wav" | "ogg" | "flac" => AssetType::Audio,
            "cs" | "c#" | "cpp" | "py" | "js" | "ts" | "lua" | "gd" => AssetType::Script,
            "json" | "dialogue" => AssetType::Dialogue,
            _ => AssetType::Unknown,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum DropTarget {
    None,
    SceneTree,
    Viewport,
    SceneTreeNode { node_id: String },
    Group { group_id: String },
}

// Tipos de widget y estado UI
#[derive(Debug, Clone, PartialEq)]
pub enum WidgetType {
    Label,
    Button,
    List,
    Checkbox,
    Slider,
    TextEdit,
    ComboBox,
    ColorPicker,
    Image,
    Canvas,
    Input,
    Dropdown,
}

#[derive(Debug, Clone)]
pub struct Widget {
    pub ty: WidgetType,
    pub label: String,
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}

impl Widget {
    pub fn new(ty: WidgetType, label: &str, x: f32, y: f32, w: f32, h: f32) -> Self {
        Widget {
            ty,
            label: label.to_string(),
            x,
            y,
            w,
            h,
        }
    }
}

pub struct UiState {
    pub widgets: Vec<Widget>,
}

pub struct UiConfig {
    pub font_size: f32,
    pub window_padding: f32,
}

impl UiState {
    pub fn new() -> Self {
        UiState {
            widgets: Vec::new(),
        }
    }
}

impl Default for UiState {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for UiConfig {
    fn default() -> Self {
        UiConfig {
            font_size: 14.0,
            window_padding: 10.0,
        }
    }
}

