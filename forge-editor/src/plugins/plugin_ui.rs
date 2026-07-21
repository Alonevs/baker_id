//! # Plugin UI Module
//! 
//! UI para plugins con:
//! - Plugin panel
//! - Plugin list
//! - Plugin details
//! - Plugin settings
//! - Plugin logs
//! - Plugin actions
//! - Plugin tabs

use eframe::egui;

/// Plugin panel
#[derive(Debug, Clone)]
pub struct PluginPanel {
    pub tab: PluginTab,
    pub selected_plugin: Option<String>,
    pub search_query: String,
    pub filter_type: Option<PluginTypeFilter>,
    pub sort_by: PluginSortBy,
    pub show_errors_only: bool,
    pub show_enabled_only: bool,
    pub show_disabled_only: bool,
    pub plugin_logs: Vec<String>,
    pub plugin_errors: Vec<String>,
    pub active_tab: PluginTab,
}

impl PluginPanel {
    pub fn new() -> Self {
        Self {
            tab: PluginTab::Plugins,
            selected_plugin: None,
            search_query: String::new(),
            filter_type: None,
            sort_by: PluginSortBy::Name,
            show_errors_only: false,
            show_enabled_only: false,
            show_disabled_only: false,
            plugin_logs: Vec::new(),
            plugin_errors: Vec::new(),
            active_tab: PluginTab::Plugins,
        }
    }

    /// Set tab
    pub fn set_tab(&mut self, tab: PluginTab) {
        self.tab = tab.clone();
        self.active_tab = tab.clone();
    }

    /// Set selected plugin
    pub fn set_selected_plugin(&mut self, plugin_id: Option<String>) {
        self.selected_plugin = plugin_id;
    }

    /// Get selected plugin
    pub fn selected_plugin(&self) -> Option<&str> {
        self.selected_plugin.as_deref()
    }

    /// Get active tab
    pub fn active_tab(&self) -> PluginTab {
        self.active_tab.clone()
    }

    /// Search query
    pub fn search_query(&self) -> &str {
        &self.search_query
    }

    /// Set search query
    pub fn set_search_query(&mut self, query: String) {
        self.search_query = query;
    }

    /// Clear search query
    pub fn clear_search_query(&mut self) {
        self.search_query.clear();
    }

    /// Filter type
    pub fn filter_type(&self) -> Option<&PluginTypeFilter> {
        self.filter_type.as_ref()
    }

    /// Set filter type
    pub fn set_filter_type(&mut self, filter_type: PluginTypeFilter) {
        self.filter_type = Some(filter_type);
    }

    /// Clear filter type
    pub fn clear_filter_type(&mut self) {
        self.filter_type = None;
    }

    /// Sort by
    pub fn sort_by(&self) -> PluginSortBy {
        self.sort_by
    }

    /// Set sort by
    pub fn set_sort_by(&mut self, sort_by: PluginSortBy) {
        self.sort_by = sort_by;
    }

    /// Show errors only
    pub fn show_errors_only(&self) -> bool {
        self.show_errors_only
    }

    /// Set show errors only
    pub fn set_show_errors_only(&mut self, show: bool) {
        self.show_errors_only = show;
    }

    /// Show enabled only
    pub fn show_enabled_only(&self) -> bool {
        self.show_enabled_only
    }

    /// Set show enabled only
    pub fn set_show_enabled_only(&mut self, show: bool) {
        self.show_enabled_only = show;
    }

    /// Show disabled only
    pub fn show_disabled_only(&self) -> bool {
        self.show_disabled_only
    }

    /// Set show disabled only
    pub fn set_show_disabled_only(&mut self, show: bool) {
        self.show_disabled_only = show;
    }

    /// Clear filters
    pub fn clear_filters(&mut self) {
        self.filter_type = None;
        self.show_errors_only = false;
        self.show_enabled_only = false;
        self.show_disabled_only = false;
    }

    /// Add plugin log
    pub fn add_plugin_log(&mut self, log: String) {
        self.plugin_logs.push(log);
        if self.plugin_logs.len() > 100 {
            self.plugin_logs.remove(0);
        }
    }

    /// Add plugin error
    pub fn add_plugin_error(&mut self, error: String) {
        self.plugin_errors.push(error);
        if self.plugin_errors.len() > 50 {
            self.plugin_errors.remove(0);
        }
    }

    /// Get plugin logs
    pub fn plugin_logs(&self) -> &[String] {
        &self.plugin_logs
    }

    /// Get plugin errors
    pub fn plugin_errors(&self) -> &[String] {
        &self.plugin_errors
    }

    /// Clear plugin logs
    pub fn clear_plugin_logs(&mut self) {
        self.plugin_logs.clear();
    }

    /// Clear plugin errors
    pub fn clear_plugin_errors(&mut self) {
        self.plugin_errors.clear();
    }
}

/// Plugin tab
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PluginTab {
    Plugins,
    Settings,
    Logs,
    Extensions,
    Marketplace,
}

impl Default for PluginTab {
    fn default() -> Self {
        Self::Plugins
    }
}

/// Plugin type filter
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PluginTypeFilter {
    All,
    Editor,
    Renderer,
    Tool,
    Script,
    Asset,
    Debug,
    Plugin,
    Custom,
}

impl Default for PluginTypeFilter {
    fn default() -> Self {
        Self::All
    }
}

/// Plugin sort by
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PluginSortBy {
    Name,
    Version,
    Author,
    LastModified,
    Size,
}

impl Default for PluginSortBy {
    fn default() -> Self {
        Self::Name
    }
}

/// Plugin panel renderer
pub fn render_plugin_panel(
    ui: &mut egui::Ui,
    plugin_manager: &mut crate::plugins::PluginManager,
    panel: &mut PluginPanel,
) {
    egui::TopBottomPanel::bottom("plugin_panel")
        .default_height(200.0)
        .show_inside(ui, |ui| {
            ui.add_space(5.0);

            // Tab bar
            ui.horizontal(|ui| {
                ui.selectable_value(&mut panel.active_tab, PluginTab::Plugins, "🔌 Plugins");
                ui.selectable_value(&mut panel.active_tab, PluginTab::Settings, "⚙ Settings");
                ui.selectable_value(&mut panel.active_tab, PluginTab::Logs, "📝 Logs");
                ui.selectable_value(&mut panel.active_tab, PluginTab::Extensions, "🧩 Extensions");
                ui.selectable_value(&mut panel.active_tab, PluginTab::Marketplace, "🛒 Marketplace");
            });
            ui.separator();
            ui.add_space(5.0);

            egui::ScrollArea::vertical().show(ui, |ui| {
                match panel.active_tab {
                    PluginTab::Plugins => render_plugins_tab(ui, plugin_manager, panel),
                    PluginTab::Settings => render_settings_tab(ui, plugin_manager, panel),
                    PluginTab::Logs => render_logs_tab(ui, plugin_manager, panel),
                    PluginTab::Extensions => render_extensions_tab(ui, plugin_manager, panel),
                    PluginTab::Marketplace => render_marketplace_tab(ui, plugin_manager, panel),
                }
            });
        });
}

/// Render plugins tab
pub(crate) fn render_plugins_tab(
    ui: &mut egui::Ui,
    plugin_manager: &mut crate::plugins::PluginManager,
    _panel: &mut PluginPanel,
) {
    // Search bar
    ui.horizontal(|ui| {
        ui.add(
            egui::TextEdit::singleline(&mut _panel.search_query)
                .hint_text("Search plugins...")
                .frame(false)
                .desired_width(300.0),
        );

        // Filter buttons
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            if ui.button("Errors Only").clicked() {
                _panel.show_errors_only = !_panel.show_errors_only;
            }
            if ui.button("Enabled Only").clicked() {
                _panel.show_enabled_only = !_panel.show_enabled_only;
            }
            if ui.button("Disabled Only").clicked() {
                _panel.show_disabled_only = !_panel.show_disabled_only;
            }
        });
    });

    // Plugin list
    ui.vertical(|ui| {
        let plugins = get_filtered_plugins(plugin_manager, _panel);

        if plugins.is_empty() {
            ui.label("No plugins found");
            return;
        }

        for plugin in plugins {
            let plugin_id = plugin.id.name.clone();
            let enabled = plugin.is_enabled();
            let loaded = plugin.is_loaded();
            let has_error = plugin.has_error();

            let button_text = format!(
                "{} {} v{} - {}",
                if enabled { "✓" } else { "○" },
                if loaded { "●" } else { "○" },
                plugin.version,
                plugin.name
            );
            let button = if has_error {
                egui::Button::new(button_text).fill(egui::Color32::RED)
            } else {
                egui::Button::new(button_text)
            };
            let response = ui.add(button);

            if response.clicked() {
                _panel.selected_plugin = Some(plugin_id);
            }

            if ui.button("Load").clicked() {
                if let Err(e) = plugin_manager.load_plugin(&plugin.id) {
                    _panel.add_plugin_error(e);
                }
            }

            if ui.button("Enable").clicked() && !loaded {
                if let Err(e) = plugin_manager.enable_plugin(&plugin.id) {
                    _panel.add_plugin_error(e);
                }
            }

            if ui.button("Disable").clicked() && loaded {
                if let Err(e) = plugin_manager.disable_plugin(&plugin.id) {
                    _panel.add_plugin_error(e);
                }
            }

            ui.horizontal(|ui| {
                ui.label(format!("Author: {}", plugin.author));
                ui.label(format!("Description: {}", plugin.description));
            });

            ui.separator();
        }
    });
}

/// Render settings tab
pub(crate) fn render_settings_tab(
    ui: &mut egui::Ui,
    _plugin_manager: &mut crate::plugins::PluginManager,
    _panel: &mut PluginPanel,
) {
    ui.vertical(|ui| {
        ui.label("Plugin Settings");

        ui.horizontal(|ui| {
            ui.label("Plugin Path:");
            ui.label(_plugin_manager.plugin_path().to_string_lossy());
        });

        ui.horizontal(|ui| {
            ui.label("Cache Path:");
            ui.label(_plugin_manager.cache_path().to_string_lossy());
        });

        ui.separator();

        ui.label(format!("Total Plugins: {}", _plugin_manager.plugin_count()));
        ui.label(format!("Enabled Plugins: {}", _plugin_manager.enabled_plugin_count()));
        ui.label(format!("Loaded Plugins: {}", _plugin_manager.loaded_plugin_count()));
        ui.label(format!("Events: {}", _plugin_manager.event_count()));

        ui.separator();

        ui.horizontal(|ui| {
            if ui.button("Clear Events").clicked() {
                _plugin_manager.clear_events();
                _panel.add_plugin_log("Events cleared".to_string());
            }
        });
    });
}

/// Render logs tab
pub(crate) fn render_logs_tab(
    ui: &mut egui::Ui,
    plugin_manager: &mut crate::plugins::PluginManager,
    _panel: &mut PluginPanel,
) {
    ui.vertical(|ui| {
        ui.label("Plugin Logs");

        if _panel.plugin_logs.is_empty() {
            ui.label("No logs available");
        } else {
            egui::ScrollArea::vertical().show(ui, |ui| {
                for log in &_panel.plugin_logs {
                    ui.label(log);
                }
            });
        }

        ui.horizontal(|ui| {
            if ui.button("Clear Logs").clicked() {
                _panel.clear_plugin_logs();
            }
        });
    });
}

/// Render extensions tab
pub(crate) fn render_extensions_tab(
    ui: &mut egui::Ui,
    _plugin_manager: &mut crate::plugins::PluginManager,
    _panel: &mut PluginPanel,
) {
    ui.vertical(|ui| {
        ui.label("Extensions");

        ui.label("Extension management will be available in future updates.");
    });
}

/// Render marketplace tab
pub(crate) fn render_marketplace_tab(
    ui: &mut egui::Ui,
    plugin_manager: &mut crate::plugins::PluginManager,
    _panel: &mut PluginPanel,
) {
    ui.vertical(|ui| {
        ui.label("Marketplace");

        ui.label("Browse and install plugins from the Forge Editor marketplace.");

        ui.horizontal(|ui| {
            if ui.button("Browse Marketplace").clicked() {
                _panel.add_plugin_log("Opening marketplace...".to_string());
            }
        });
    });
}

/// Get filtered plugins
fn get_filtered_plugins(
    plugin_manager: &crate::plugins::PluginManager,
    _panel: &mut PluginPanel,
) -> Vec<crate::plugins::PluginMetadata> {
    let mut plugins = plugin_manager.get_all_plugins();

    // Apply search filter
    if !_panel.search_query.is_empty() {
        let query = _panel.search_query.to_lowercase();
        plugins.retain(|p| {
            p.name.to_lowercase().contains(&query)
                || p.description.to_lowercase().contains(&query)
                || p.author.to_lowercase().contains(&query)
                || p.version.to_lowercase().contains(&query)
        });
    }

    // Apply type filter
    if let Some(ref filter_type) = _panel.filter_type {
        match filter_type {
            PluginTypeFilter::All => {}
            PluginTypeFilter::Editor => {}
            PluginTypeFilter::Renderer => {}
            PluginTypeFilter::Tool => {}
            PluginTypeFilter::Script => {}
            PluginTypeFilter::Asset => {}
            PluginTypeFilter::Debug => {}
            PluginTypeFilter::Plugin => {}
            PluginTypeFilter::Custom => {},
        }
    }

    // Apply error filter
    if _panel.show_errors_only {
        plugins.retain(|p| p.has_error());
    }

    // Apply enabled filter
    if _panel.show_enabled_only {
        plugins.retain(|p| p.is_enabled());
    }

    // Apply disabled filter
    if _panel.show_disabled_only {
        plugins.retain(|p| !p.is_enabled());
    }

    plugins
}

