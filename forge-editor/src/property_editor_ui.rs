//! # Property Editor UI
//! 
//! Módulo para UI unificada de propiedades de entidades.

use crate::ui::Widget;
use crate::ui::WidgetType;
use eframe::egui;

/// UI unificada de propiedades
pub struct PropertyEditorUI {
    widgets: Vec<Widget>,
    selected_entity: Option<i32>,
    show_transform: bool,
    show_component: bool,
    show_script: bool,
}

impl PropertyEditorUI {
    /// Crea una nueva UI de propiedades
    pub fn new() -> Self {
        Self {
            widgets: Vec::new(),
            selected_entity: None,
            show_transform: true,
            show_component: true,
            show_script: true,
        }
    }

    /// Establece la entidad seleccionada
    pub fn set_selected_entity(&mut self, entity_id: i32) {
        self.selected_entity = Some(entity_id);
    }

    /// Obtiene la entidad seleccionada
    pub fn get_selected_entity(&self) -> Option<i32> {
        self.selected_entity
    }

    /// Crea los widgets de la UI
    pub fn create_widgets(&mut self) {
        let x = 10.0;
        let mut y = 10.0;
        let w = 300.0;
        let h = 50.0;

        self.widgets.push(Widget::new(WidgetType::Label, "Property Editor", x, y, w, h));
        y += h + 10.0;
        
        // Tabs
        self.widgets.push(Widget::new(WidgetType::Checkbox, "Transform", x, y, w, h));
        y += h + 10.0;
        self.widgets.push(Widget::new(WidgetType::Checkbox, "Component", x, y, w, h));
        y += h + 10.0;
        self.widgets.push(Widget::new(WidgetType::Checkbox, "Script", x, y, w, h));
        
        // Entity ID
        self.widgets.push(Widget::new(WidgetType::Label, "Selected Entity", x, y + (3.0 * h), w, h));
        
        // Transform properties
        self.widgets.push(Widget::new(WidgetType::Label, "Position X", x, y + (4.0 * h), w, h));
        self.widgets.push(Widget::new(WidgetType::Label, "Position Y", x, y + (5.0 * h), w, h));
        self.widgets.push(Widget::new(WidgetType::Label, "Position Z", x, y + (6.0 * h), w, h));
        self.widgets.push(Widget::new(WidgetType::Label, "Rotation X", x, y + (7.0 * h), w, h));
        self.widgets.push(Widget::new(WidgetType::Label, "Rotation Y", x, y + (8.0 * h), w, h));
        self.widgets.push(Widget::new(WidgetType::Label, "Rotation Z", x, y + (9.0 * h), w, h));
        self.widgets.push(Widget::new(WidgetType::Label, "Scale X", x, y + (10.0 * h), w, h));
        self.widgets.push(Widget::new(WidgetType::Label, "Scale Y", x, y + (11.0 * h), w, h));
        self.widgets.push(Widget::new(WidgetType::Label, "Scale Z", x, y + (12.0 * h), w, h));
        
        // Component properties
        self.widgets.push(Widget::new(WidgetType::Label, "Component Type", x, y + (13.0 * h), w, h));
        self.widgets.push(Widget::new(WidgetType::Label, "Keys", x, y + (14.0 * h), w, h));
        self.widgets.push(Widget::new(WidgetType::Label, "Values", x, y + (15.0 * h), w, h));
        self.widgets.push(Widget::new(WidgetType::Label, "Enabled", x, y + (16.0 * h), w, h));
        
        // Script properties
        self.widgets.push(Widget::new(WidgetType::Label, "Script Name", x, y + (17.0 * h), w, h));
        self.widgets.push(Widget::new(WidgetType::Label, "Script Path", x, y + (18.0 * h), w, h));
        self.widgets.push(Widget::new(WidgetType::Label, "Current Line", x, y + (19.0 * h), w, h));
        self.widgets.push(Widget::new(WidgetType::Label, "Is Compiled", x, y + (20.0 * h), w, h));
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
        Some(format!("property_{}", item_id))
    }

    /// Maneja el drop
    pub fn on_drop(&mut self, data: &str, target_id: &str) -> bool {
        if data.starts_with("property_") {
            let property_id = data.strip_prefix("property_").unwrap();
            // Process property drop
            true
        } else {
            false
        }
    }

    /// Renderiza con drag & drop
    pub fn render_with_drag(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.vertical(|ui| {
                ui.label("Property Editor");
                ui.separator();
                
                ui.checkbox(&mut self.show_transform, "Transform");
                ui.checkbox(&mut self.show_component, "Component");
                ui.checkbox(&mut self.show_script, "Script");
                
                ui.add_space(20.0);
                
                if let Some(entity_id) = self.selected_entity {
                    ui.label(format!("Selected Entity: {}", entity_id));
                } else {
                    ui.label("No entity selected. Drag entity here.");
                }
                
                if self.show_transform {
                    ui.label("Transform:");
                    ui.horizontal(|ui| {
                        let mut text = String::from("0.0");
                        ui.add(egui::TextEdit::singleline(&mut text));
                        let mut text = String::from("0.0");
                        ui.add(egui::TextEdit::singleline(&mut text));
                        let mut text = String::from("0.0");
                        ui.add(egui::TextEdit::singleline(&mut text));
                    });
                    
                    ui.label("Rotation:");
                    ui.horizontal(|ui| {
                        let mut text = String::from("0.0");
                        ui.add(egui::TextEdit::singleline(&mut text));
                        let mut text = String::from("0.0");
                        ui.add(egui::TextEdit::singleline(&mut text));
                        let mut text = String::from("0.0");
                        ui.add(egui::TextEdit::singleline(&mut text));
                    });
                    
                    ui.label("Scale:");
                    ui.horizontal(|ui| {
                        let mut text = String::from("1.0");
                        ui.add(egui::TextEdit::singleline(&mut text));
                        let mut text = String::from("1.0");
                        ui.add(egui::TextEdit::singleline(&mut text));
                        let mut text = String::from("1.0");
                        ui.add(egui::TextEdit::singleline(&mut text));
                    });
                }
                
                if self.show_component {
                    ui.label("Component:");
                    let mut text = String::from("None");
                    ui.add(egui::TextEdit::singleline(&mut text));
                    
                    ui.label("Keys:");
                    let mut text = String::from("");
                    ui.add(egui::TextEdit::singleline(&mut text));
                    
                    ui.label("Values:");
                    let mut text = String::from("");
                    ui.add(egui::TextEdit::singleline(&mut text));
                    
                    ui.checkbox(&mut self.show_component, "Enabled");
                }
                
                if self.show_script {
                    ui.label("Script:");
                    let mut text = String::from("Untitled");
                    ui.add(egui::TextEdit::singleline(&mut text));
                    
                    let mut text = String::from("");
                    ui.add(egui::TextEdit::singleline(&mut text));
                    
                    let mut text = String::from("0");
                    ui.add(egui::TextEdit::singleline(&mut text));
                    
                    ui.checkbox(&mut self.show_script, "Is Compiled");
                }
                
                ui.separator();
                ui.label("Drag properties here to edit");
            });
        });
    }
}

impl Default for PropertyEditorUI {
    fn default() -> Self {
        Self::new()
    }
}