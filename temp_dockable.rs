use std::sync::Arc;
use std::collections::HashMap;
use uuid::Uuid;

use crate::asset_browser::AssetBrowser;
use crate::timeline::TimelineDockable;
use crate::console::ConsoleDockable;

#[derive(Debug, Clone)]
pub enum DockableType {
    Viewport,
    Hierarchy,
    Inspector,
    Assets,
    Timeline,
    Console,
    Properties,
    Script,
    EventForge,
    Audio,
    Particle,
    Material,
    Blueprint,
    PluginManager,
    Scene,
    AssetsPanel,
    PluginsPanel,
}

impl From<AssetBrowser> for DockableType {
    fn from(_: AssetBrowser) -> Self {
        DockableType::Assets
    }
}

impl From<TimelineDockable> for DockableType {
    fn from(_: TimelineDockable) -> Self {
        DockableType::Timeline
    }
}

impl From<ConsoleDockable> for DockableType {
    fn from(_: ConsoleDockable) -> Self {
        DockableType::Console
    }
}

use crate::ui::DockPos;

#[derive(Debug, Clone)]
pub struct Dockable {
    pub id: String,
    pub name: String,
    pub dockable_type: DockableType,
    pub is_docked: bool,
    pub is_floatable: bool,
    pub tab_bar: Vec<String>,
    pub dock_pos: Arc<DockPos>,
}

impl Dockable {
    pub fn new(name: String, dockable_type: DockableType) -> Self {
        Self {
            id: format!(\"dock_{}\", name.to_lowercase()),
            name,
            dockable_type,
            is_docked: true,
            is_floatable: true,
            tab_bar: vec![name.clone()],
            dock_pos: Arc::new(DockPos::default()),
        }
    }

    pub fn new_tree_view() -> Self {
        Self {
            id: \"hierarchy\".to_string(),
            name: \"Hierarchy\".to_string(),
            dockable_type: DockableType::Hierarchy,
            is_docked: true,
            is_floatable: true,
            tab_bar: vec![\"Hierarchy\".to_string()],
            dock_pos: Arc::new(DockPos::default()),
        }
    }

    pub fn new_inspector() -> Self {
        Self {
            id: \"inspector\".to_string(),
            name: \"Inspector\".to_string(),
            dockable_type: DockableType::Inspector,
            is_docked: true,
            is_floatable: true,
            tab_bar: vec![\"Inspector\".to_string()],
            dock_pos: Arc::new(DockPos::default()),
        }
    }

    pub fn new_default() -> Self {
        Self {
            id: \"default\".to_string(),
            name: \"Default\".to_string(),
            dockable_type: DockableType::Viewport,
            is_docked: true,
            is_floatable: true,
            tab_bar: vec![\"Default\".to_string()],
            dock_pos: Arc::new(DockPos::default()),
        }
    }
}

pub trait DockableTrait {
    fn name(&self) -> &str;
    fn dockable_type(&self) -> DockableType;
    fn is_docked(&self) -> bool;
    fn set_docked(&mut self, docked: bool);
    fn dock_pos(&self) -> &DockPos;
    fn set_dock_pos(&mut self, pos: DockPos);
    fn is_floatable(&self) -> bool;
    fn tab_bar(&self) -> &Vec<String>;
    fn update(&mut self, delta: f32);
}

impl DockableTrait for Dockable {
    fn name(&self) -> &str {
        &self.name
    }

    fn dockable_type(&self) -> DockableType {
        self.dockable_type.clone()
    }

    fn is_docked(&self) -> bool {
        self.is_docked
    }

    fn set_docked(&mut self, docked: bool) {
        self.is_docked = docked;
    }

    fn dock_pos(&self) -> &DockPos {
        self.dock_pos.as_ref()
    }

    fn set_dock_pos(&mut self, pos: DockPos) {
        self.dock_pos = Arc::new(pos);
    }

    fn is_floatable(&self) -> bool {
        self.is_floatable
    }

    fn tab_bar(&self) -> &Vec<String> {
        &self.tab_bar
    }

    fn update(&mut self, _delta: f32) {
        // Default implementation
    }
}

#[derive(Debug, Clone)]
pub struct DockableBlock {
    pub id: String,
    pub name: String,
    pub dockable_type: DockableType,
    pub is_docked: bool,
    pub is_floatable: bool,
    pub tab_bar: Vec<String>,
    pub dock_pos: Arc<DockPos>,
}

impl DockableBlock {
    pub fn new(name: String, dockable_type: DockableType) -> Self {
        Self {
            id: format!(\"dock_{}\", name.to_lowercase()),
            name,
            dockable_type,
            is_docked: true,
            is_floatable: true,
            tab_bar: vec![name.clone()],
            dock_pos: Arc::new(DockPos::default()),
        }
    }

    pub fn new_tree_view() -> Self {
        Self {
            id: \"hierarchy\".to_string(),
            name: \"Hierarchy\".to_string(),
            dockable_type: DockableType::Hierarchy,
            is_docked: true,
            is_floatable: true,
            tab_bar: vec![\"Hierarchy\".to_string()],
            dock_pos: Arc::new(DockPos::default()),
        }
    }

    pub fn new_inspector() -> Self {
        Self {
            id: \"inspector\".to_string(),
            name: \"Inspector\".to_string(),
            dockable_type: DockableType::Inspector,
            is_docked: true,
            is_floatable: true,
            tab_bar: vec![\"Inspector\".to_string()],
            dock_pos: Arc::new(DockPos::default()),
        }
    }

    pub fn close(&mut self) {
        self.tab_bar.clear();
    }

    pub fn add_tab(&mut self, name: &str) {
        self.tab_bar.push(name.to_string());
    }
}

impl Default for Dockable {
    fn default() -> Self {
        Self::new(\"Default\".to_string(), DockableType::Viewport)
    }
}
