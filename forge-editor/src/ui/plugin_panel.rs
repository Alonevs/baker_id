//! # Plugin Panel UI Module
//! 
//! Panel UI para gestión de plugins con:
//! - Plugin panel UI
//! - Plugin list rendering
//! - Plugin actions
//! - Plugin tabs
//! - Plugin search and filters

use eframe::egui;

use crate::plugins::{PluginManager, PluginPanel as PluginPanelCore};

/// Plugin panel UI wrapper
#[derive(Debug, Clone)]
pub struct PluginPanel {
    pub core: PluginPanelCore,
}

impl PluginPanel {
    pub fn new() -> Self {
        Self {
            core: PluginPanelCore::new(),
        }
    }

    /// Set tab
    pub fn set_tab(&mut self, tab: crate::ui::Tab) {
        match tab {
            crate::ui::Tab::Plugins => self.core.set_tab(crate::plugins::PluginTab::Plugins),
            _ => {}
        }
    }

    /// Set selected plugin
    pub fn set_selected_plugin(&mut self, plugin_id: Option<String>) {
        self.core.set_selected_plugin(plugin_id);
    }

    /// Get selected plugin
    pub fn selected_plugin(&self) -> Option<&str> {
        self.core.selected_plugin()
    }

    /// Get active tab
    pub fn active_tab(&self) -> crate::plugins::PluginTab {
        self.core.active_tab()
    }

    /// Search query
    pub fn search_query(&self) -> &str {
        self.core.search_query()
    }

    /// Set search query
    pub fn set_search_query(&mut self, query: String) {
        self.core.set_search_query(query);
    }

    /// Clear search query
    pub fn clear_search_query(&mut self) {
        self.core.clear_search_query();
    }

    /// Filter type
    pub fn filter_type(&self) -> Option<&crate::plugins::PluginTypeFilter> {
        self.core.filter_type()
    }

    /// Set filter type
    pub fn set_filter_type(&mut self, filter_type: crate::plugins::PluginTypeFilter) {
        self.core.set_filter_type(filter_type);
    }

    /// Clear filter type
    pub fn clear_filter_type(&mut self) {
        self.core.clear_filter_type();
    }

    /// Sort by
    pub fn sort_by(&self) -> crate::plugins::PluginSortBy {
        self.core.sort_by()
    }

    /// Set sort by
    pub fn set_sort_by(&mut self, sort_by: crate::plugins::PluginSortBy) {
        self.core.set_sort_by(sort_by);
    }

    /// Show errors only
    pub fn show_errors_only(&self) -> bool {
        self.core.show_errors_only()
    }

    /// Set show errors only
    pub fn set_show_errors_only(&mut self, show: bool) {
        self.core.set_show_errors_only(show);
    }

    /// Show enabled only
    pub fn show_enabled_only(&self) -> bool {
        self.core.show_enabled_only()
    }

    /// Set show enabled only
    pub fn set_show_enabled_only(&mut self, show: bool) {
        self.core.set_show_enabled_only(show);
    }

    /// Show disabled only
    pub fn show_disabled_only(&self) -> bool {
        self.core.show_disabled_only()
    }

    /// Set show disabled only
    pub fn set_show_disabled_only(&mut self, show: bool) {
        self.core.set_show_disabled_only(show);
    }

    /// Clear filters
    pub fn clear_filters(&mut self) {
        self.core.clear_filters();
    }

    /// Add plugin log
    pub fn add_plugin_log(&mut self, log: String) {
        self.core.add_plugin_log(log);
    }

    /// Add plugin error
    pub fn add_plugin_error(&mut self, error: String) {
        self.core.add_plugin_error(error);
    }

    /// Get plugin logs
    pub fn plugin_logs(&self) -> &[String] {
        self.core.plugin_logs()
    }

    /// Get plugin errors
    pub fn plugin_errors(&self) -> &[String] {
        self.core.plugin_errors()
    }

    /// Clear plugin logs
    pub fn clear_plugin_logs(&mut self) {
        self.core.clear_plugin_logs();
    }

    /// Clear plugin errors
    pub fn clear_plugin_errors(&mut self) {
        self.core.clear_plugin_errors();
    }
}

impl Default for PluginPanel {
    fn default() -> Self {
        Self::new()
    }
}

/// Render plugin panel UI
pub fn render_plugin_panel(ui: &mut egui::Ui, plugin_manager: &mut PluginManager, panel: &mut PluginPanel) {
    egui::TopBottomPanel::bottom("plugin_panel")
        .default_height(200.0)
        .show_inside(ui, |ui| {
            ui.add_space(5.0);

            // Tab bar
            ui.horizontal(|ui| {
                ui.selectable_value(&mut panel.core.active_tab, crate::plugins::PluginTab::Plugins, "🔌 Plugins");
                ui.selectable_value(&mut panel.core.active_tab, crate::plugins::PluginTab::Settings, "⚙ Settings");
                ui.selectable_value(&mut panel.core.active_tab, crate::plugins::PluginTab::Logs, "📝 Logs");
                ui.selectable_value(&mut panel.core.active_tab, crate::plugins::PluginTab::Extensions, "🧩 Extensions");
                ui.selectable_value(&mut panel.core.active_tab, crate::plugins::PluginTab::Marketplace, "🛒 Marketplace");
            });
            ui.separator();
            ui.add_space(5.0);

            egui::ScrollArea::vertical().show(ui, |ui| {
                match panel.core.active_tab() {
                    crate::plugins::PluginTab::Plugins => {
                        crate::plugins::plugin_ui::render_plugins_tab(ui, plugin_manager, &mut panel.core);
                    }
                    crate::plugins::PluginTab::Settings => {
                        crate::plugins::plugin_ui::render_settings_tab(ui, plugin_manager, &mut panel.core);
                    }
                    crate::plugins::PluginTab::Logs => {
                        crate::plugins::plugin_ui::render_logs_tab(ui, plugin_manager, &mut panel.core);
                    }
                    crate::plugins::PluginTab::Extensions => {
                        crate::plugins::plugin_ui::render_extensions_tab(ui, plugin_manager, &mut panel.core);
                    }
                    crate::plugins::PluginTab::Marketplace => {
                        crate::plugins::plugin_ui::render_marketplace_tab(ui, plugin_manager, &mut panel.core);
                    }
                }
            });
        });
}

