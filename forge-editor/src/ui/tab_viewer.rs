use crate::ui::collaboration_panel::CollaborationPanel;
use crate::ui::test_panel::TestPanel;
use crate::ui::editor_panel::EditorPanel;

use crate::ui::compile_panel::CompilePanel;
use crate::ui::debugger_panel::DebuggerPanel;
use crate::ui::hot_reload_panel::HotReloadPanel;
use crate::ui::script_optimizer_panel::ScriptOptimizerPanel;
use crate::ui::PluginPanel;
use eframe::egui;

pub const TAB_COLLABORATION: Tab = Tab::Collaboration;
pub const TAB_TEST: Tab = Tab::Test;
pub const TAB_PLUGINS: Tab = Tab::Plugins;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum Tab {
    Editor,
    AssetBrowser,
    Compile,
    Debugger,
    HotReload,
    ScriptOptimizer,
    Collaboration,
    Test,
    Plugins,
}

impl std::fmt::Display for Tab {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Tab::Editor => write!(f, "Editor"),
            Tab::AssetBrowser => write!(f, "Assets"),
            Tab::Compile => write!(f, "Compile"),
            Tab::Debugger => write!(f, "Debugger"),
            Tab::HotReload => write!(f, "Hot Reload"),
            Tab::ScriptOptimizer => write!(f, "Optimizer"),
            Tab::Collaboration => write!(f, "Collaboration"),
            Tab::Test => write!(f, "Tests"),
            Tab::Plugins => write!(f, "Plugins"),
        }
    }
}

pub struct TabViewer {
    pub tabs: Vec<Tab>,
    pub selected_tab: Option<Tab>,
    pub editor_panel: Option<EditorPanel>,
    pub compile_panel: Option<CompilePanel>,
    pub debugger_panel: Option<DebuggerPanel>,
    pub hot_reload_panel: Option<HotReloadPanel>,
    pub script_optimizer_panel: Option<ScriptOptimizerPanel>,
    pub collaboration_panel: Option<CollaborationPanel>,
    pub test_panel: Option<TestPanel>,
    pub plugin_panel: Option<PluginPanel>,
}

impl Default for TabViewer {
    fn default() -> Self {
        Self::new()
    }
}

impl TabViewer {
    pub fn new() -> Self {
        Self {
            tabs: vec![
                Tab::Editor,
                Tab::AssetBrowser,
                Tab::Compile,
                Tab::Debugger,
                Tab::HotReload,
                Tab::ScriptOptimizer,
                Tab::Collaboration,
                Tab::Test,
                Tab::Plugins,
            ],
            selected_tab: Some(Tab::Editor),
            editor_panel: None,
            compile_panel: None,
            debugger_panel: None,
            hot_reload_panel: None,
            script_optimizer_panel: None,
            collaboration_panel: Some(CollaborationPanel::new()),
            test_panel: Some(TestPanel::new()),
            plugin_panel: Some(PluginPanel::new()),
        }
    }

    pub fn selected_tab(&self) -> Option<Tab> {
        self.selected_tab
    }

    pub fn set_selected_tab(&mut self, tab: Tab) {
        self.selected_tab = Some(tab);
    }

    pub fn editor_panel(&self) -> Option<&EditorPanel> {
        self.editor_panel.as_ref()
    }

    pub fn editor_panel_mut(&mut self) -> Option<&mut EditorPanel> {
        self.editor_panel.as_mut()
    }

    pub fn compile_panel(&self) -> Option<&CompilePanel> {
        self.compile_panel.as_ref()
    }

    pub fn compile_panel_mut(&mut self) -> Option<&mut CompilePanel> {
        self.compile_panel.as_mut()
    }

    pub fn debugger_panel(&self) -> Option<&DebuggerPanel> {
        self.debugger_panel.as_ref()
    }

    pub fn debugger_panel_mut(&mut self) -> Option<&mut DebuggerPanel> {
        self.debugger_panel.as_mut()
    }

    pub fn hot_reload_panel(&self) -> Option<&HotReloadPanel> {
        self.hot_reload_panel.as_ref()
    }

    pub fn hot_reload_panel_mut(&mut self) -> Option<&mut HotReloadPanel> {
        self.hot_reload_panel.as_mut()
    }

    pub fn script_optimizer_panel(&self) -> Option<&ScriptOptimizerPanel> {
        self.script_optimizer_panel.as_ref()
    }

    pub fn script_optimizer_panel_mut(&mut self) -> Option<&mut ScriptOptimizerPanel> {
        self.script_optimizer_panel.as_mut()
    }

    pub fn collaboration_panel(&self) -> Option<&CollaborationPanel> {
        self.collaboration_panel.as_ref()
    }

    pub fn collaboration_panel_mut(&mut self) -> Option<&mut CollaborationPanel> {
        self.collaboration_panel.as_mut()
    }

    pub fn test_panel(&self) -> Option<&TestPanel> {
        self.test_panel.as_ref()
    }

    pub fn test_panel_mut(&mut self) -> Option<&mut TestPanel> {
        self.test_panel.as_mut()
    }

    pub fn plugin_panel(&self) -> Option<&PluginPanel> {
        self.plugin_panel.as_ref()
    }

    pub fn plugin_panel_mut(&mut self) -> Option<&mut PluginPanel> {
        self.plugin_panel.as_mut()
    }

    pub fn tabs(&self) -> &[Tab] {
        &self.tabs
    }

    pub fn tabs_mut(&mut self) -> &mut Vec<Tab> {
        &mut self.tabs
    }
}

impl eframe::App for TabViewer {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    // File menu
                });
                ui.menu_button("Edit", |ui| {
                    // Edit menu
                });
                ui.menu_button("View", |ui| {
                    // View menu
                });
                ui.menu_button("Help", |ui| {
                    // Help menu
                });
            });
        });

        egui::SidePanel::left("left_panel").show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |_ui| {
                // Left panel content
            });
        });

        egui::SidePanel::right("right_panel").show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                if let Some(ref panel) = self.collaboration_panel {
                    panel.ui(ctx);
                }
                if let Some(ref mut panel) = self.test_panel {
                    panel.ui(ctx);
                }
                if let Some(ref mut panel) = self.plugin_panel {
                    let mut pm = crate::plugins::PluginManager::new(None, None);
                    crate::ui::plugin_panel::render_plugin_panel(ui, &mut pm, panel);
                }
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::both().show(ui, |_ui| {
                // Main editor area
            });
        });
    }
}

