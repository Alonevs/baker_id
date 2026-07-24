//! # Plugin System UI
//! 
//! Módulo para UI de gestión de plugins del editor.

use crate::ui::Widget;
use crate::ui::WidgetType;
use eframe::egui;

/// UI de gestión de plugins
pub struct PluginSystemUI {
    widgets: Vec<Widget>,
    enabled_plugins: Vec<String>,
    plugin_count: usize,
}

impl PluginSystemUI {
    /// Crea una nueva UI de plugins
    pub fn new() -> Self {
        Self {
            widgets: Vec::new(),
            enabled_plugins: Vec::new(),
            plugin_count: 0,
        }
    }

    /// Agrega un plugin
    pub fn add_plugin(&mut self, plugin_name: &str) {
        self.enabled_plugins.push(plugin_name.to_string());
        self.plugin_count += 1;
    }

    /// Elimina un plugin
    pub fn remove_plugin(&mut self, plugin_name: &str) {
        if self.enabled_plugins.iter().any(|p| p == plugin_name) {
            self.enabled_plugins.retain(|p| p != plugin_name);
            self.plugin_count -= 1;
        }
    }

    /// Obtiene la lista de plugins habilitados
    pub fn get_enabled_plugins(&self) -> &Vec<String> {
        &self.enabled_plugins
    }

    /// Obtiene el plugin_count
    pub fn get_plugin_count(&self) -> usize {
        self.plugin_count
    }

    /// Crea los widgets de la UI
    pub fn create_widgets(&mut self) {
        let x = 10.0;
        let mut y = 10.0;
        let w = 300.0;
        let h = 50.0;

        self.widgets.push(Widget::new(WidgetType::Label, "Plugin System", x, y, w, h));
        y += h + 10.0;
        
        // Plugin list
        self.widgets.push(Widget::new(WidgetType::Label, "Enabled Plugins", x, y, w, h));
        y += h + 10.0;
        
        // Plugin count
        self.widgets.push(Widget::new(WidgetType::Label, "Total Plugins", x, y, w, h));
        
        // Add plugin button
        self.widgets.push(Widget::new(WidgetType::Button, "Add Plugin", x, y + h, w, h));
        
        // Remove plugin button
        self.widgets.push(Widget::new(WidgetType::Button, "Remove Plugin", x, y + (2.0 * h), w, h));
        
        // Status
        self.widgets.push(Widget::new(WidgetType::Label, "Status", x, y + (3.0 * h), w, h));
        
        // Plugin types
        self.widgets.push(Widget::new(WidgetType::Checkbox, "Editor", x, y + (4.0 * h), w, h));
        self.widgets.push(Widget::new(WidgetType::Checkbox, "Runtime", x, y + (5.0 * h), w, h));
        self.widgets.push(Widget::new(WidgetType::Checkbox, "Export", x, y + (6.0 * h), w, h));
    }

    /// Renderiza la UI
    pub fn render(&mut self, ui: &mut egui::Ui) {
        self.render_with_drag(ui);
    }

    /// Obtiene los widgets
    pub fn get_widgets(&self) -> &Vec<Widget> {
        &self.widgets
    }

    /// Obtiene los widgets mutados
    pub fn get_widgets_mut(&mut self) -> &mut Vec<Widget> {
        &mut self.widgets
    }

    /// Maneja el inicio del drag
    pub fn on_drag_start(&self, item_id: &str, data: &str) -> Option<String> {
        Some(format!("plugin_{}", item_id))
    }

    /// Maneja el drop
    pub fn on_drop(&mut self, data: &str, target_id: &str) -> bool {
        if data.starts_with("plugin_") {
            let plugin_name = data.strip_prefix("plugin_").unwrap();
            self.add_plugin(plugin_name);
            true
        } else {
            false
        }
    }

    /// Renderiza con drag & drop
    pub fn render_with_drag(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.vertical(|ui| {
                ui.label("Plugin System");
                ui.separator();
                
                ui.label("Enabled Plugins:");
                for plugin in &self.enabled_plugins {
                    ui.label(plugin);
                }
                
                ui.label(format!("Total Plugins: {}", self.plugin_count));
                
                ui.add_space(10.0);
                
                ui.button("Add Plugin");
                ui.add_space(10.0);
                ui.button("Remove Plugin");
                
                ui.add_space(20.0);
                
                ui.checkbox(&mut self.widgets.is_empty(), "Editor");
                ui.checkbox(&mut self.widgets.is_empty(), "Runtime");
                ui.checkbox(&mut self.widgets.is_empty(), "Export");
                
                ui.separator();
                ui.label("Drag plugin here to enable");
            });
        });
    }
}

impl Default for PluginSystemUI {
    fn default() -> Self {
        Self::new()
    }
}