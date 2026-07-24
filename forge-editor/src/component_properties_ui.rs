//! # Component Properties UI
//! 
//! Módulo para UI completa de propiedades de componentes.

use crate::ui::Widget;
use crate::ui::WidgetType;
use eframe::egui;

/// UI de propiedades de componentes
pub struct ComponentPropertiesUI {
    widgets: Vec<Widget>,
}

impl ComponentPropertiesUI {
    /// Crea una nueva UI de propiedades de componentes
    pub fn new() -> Self {
        Self {
            widgets: Vec::new(),
        }
    }

    /// Crea los widgets de la UI
    pub fn create_widgets(&mut self) {
        let mut x = 10.0;
        let mut y = 10.0;
        let w = 300.0;
        let h = 50.0;

        self.widgets.push(Widget::new(WidgetType::Label, "Component Properties", x, y, w, h));
        y += h + 10.0;
        
        // Component Type
        self.widgets.push(Widget::new(WidgetType::Label, "Component Type", x, y, w, h));
        y += h + 10.0;
        
        // Component Keys
        self.widgets.push(Widget::new(WidgetType::Label, "Keys", x, y, w, h));
        y += h + 10.0;
        
        // Component Values
        self.widgets.push(Widget::new(WidgetType::Label, "Values", x, y, w, h));
        y += h + 10.0;
        
        // Enabled
        self.widgets.push(Widget::new(WidgetType::Checkbox, "Enabled", x, y, w, h));
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
        Some(format!("component_{}", item_id))
    }

    /// Maneja el drop
    pub fn on_drop(&mut self, data: &str, target_id: &str) -> bool {
        if data.starts_with("component_") {
            let component_id = data.strip_prefix("component_").unwrap();
            // Dropped component: {}
            true
        } else {
            false
        }
    }

    /// Renderiza con drag & drop
    pub fn render_with_drag(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.vertical(|ui| {
                ui.label("Component Properties");
                ui.separator();
                
                ui.label("Component Type:");
                let mut text = String::from("None");
                ui.add(egui::TextEdit::singleline(&mut text));
                
                ui.label("Keys:");
                let mut text = String::from("");
                ui.add(egui::TextEdit::singleline(&mut text));
                
                ui.label("Values:");
                let mut text = String::from("");
                ui.add(egui::TextEdit::singleline(&mut text));
                
                ui.checkbox(&mut self.widgets.is_empty(), "Enabled");
                
                ui.separator();
                ui.label("Drag component here to add");
            });
        });
    }
}

impl Default for ComponentPropertiesUI {
    fn default() -> Self {
        Self::new()
    }
}