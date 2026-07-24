//! # Transform Properties UI
//! 
//! Módulo para UI completa de propiedades de transformación.

use crate::ui::Widget;
use crate::ui::WidgetType;
use eframe::egui;

/// UI de propiedades de transformación
pub struct TransformPropertiesUI {
    widgets: Vec<Widget>,
    entity_id: Option<i32>,
}

impl TransformPropertiesUI {
    /// Crea una nueva UI de propiedades de transformación
    pub fn new() -> Self {
        Self {
            widgets: Vec::new(),
            entity_id: None,
        }
    }

    /// Establece el entity_id actual
    pub fn set_entity_id(&mut self, entity_id: i32) {
        self.entity_id = Some(entity_id);
    }

    /// Obtiene el entity_id actual
    pub fn get_entity_id(&self) -> Option<i32> {
        self.entity_id
    }

    /// Crea los widgets de la UI
    pub fn create_widgets(&mut self) {
        let mut x = 10.0;
        let mut y = 10.0;
        let w = 300.0;
        let h = 50.0;

        self.widgets.push(Widget::new(WidgetType::Label, "Transform Properties", x, y, w, h));
        y += h + 10.0;
        
        // Position
        self.widgets.push(Widget::new(WidgetType::Label, "Position", x, y, w, h));
        y += h + 10.0;
        
        // Position X
        self.widgets.push(Widget::new(WidgetType::Input, "X", x, y, w, h));
        y += h + 10.0;
        
        // Position Y
        self.widgets.push(Widget::new(WidgetType::Input, "Y", x, y, w, h));
        y += h + 10.0;
        
        // Position Z
        self.widgets.push(Widget::new(WidgetType::Input, "Z", x, y, w, h));
        y += h + 10.0;
        
        // Rotation
        self.widgets.push(Widget::new(WidgetType::Label, "Rotation", x, y, w, h));
        y += h + 10.0;
        
        // Rotation X
        self.widgets.push(Widget::new(WidgetType::Input, "X", x, y, w, h));
        y += h + 10.0;
        
        // Rotation Y
        self.widgets.push(Widget::new(WidgetType::Input, "Y", x, y, w, h));
        y += h + 10.0;
        
        // Rotation Z
        self.widgets.push(Widget::new(WidgetType::Input, "Z", x, y, w, h));
        y += h + 10.0;
        
        // Scale
        self.widgets.push(Widget::new(WidgetType::Label, "Scale", x, y, w, h));
        y += h + 10.0;
        
        // Scale X
        self.widgets.push(Widget::new(WidgetType::Input, "X", x, y, w, h));
        y += h + 10.0;
        
        // Scale Y
        self.widgets.push(Widget::new(WidgetType::Input, "Y", x, y, w, h));
        y += h + 10.0;
        
        // Scale Z
        self.widgets.push(Widget::new(WidgetType::Input, "Z", x, y, w, h));
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
        Some(format!("transform_{}", item_id))
    }

    /// Maneja el drop
    pub fn on_drop(&mut self, data: &str, target_id: &str) -> bool {
        if data.starts_with("transform_") {
            let entity_id = data.strip_prefix("transform_").unwrap();
            self.set_entity_id(entity_id.parse().unwrap_or(0));
            true
        } else {
            false
        }
    }

    /// Renderiza con drag & drop
    pub fn render_with_drag(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.vertical(|ui| {
                // Área de drop para entidades
                ui.label("Drop Entity Here");
                ui.add_space(10.0);
                ui.label("Drag and drop entity here");
                ui.separator();
                
                // Transformación actual
                if let Some(entity_id) = self.entity_id {
                    ui.label(format!("Entity: {}", entity_id));
                    
                    // Position
                    ui.label("Position:");
                    ui.horizontal(|ui| {
                        let mut x = String::from("0.0");
                        let mut y = String::from("0.0");
                        let mut z = String::from("0.0");
                        ui.add(egui::TextEdit::singleline(&mut x));
                        ui.add(egui::TextEdit::singleline(&mut y));
                        ui.add(egui::TextEdit::singleline(&mut z));
                    });
                    
                    // Rotation
                    ui.label("Rotation:");
                    ui.horizontal(|ui| {
                        let mut x = String::from("0.0");
                        let mut y = String::from("0.0");
                        let mut z = String::from("0.0");
                        ui.add(egui::TextEdit::singleline(&mut x));
                        ui.add(egui::TextEdit::singleline(&mut y));
                        ui.add(egui::TextEdit::singleline(&mut z));
                    });
                    
                    // Scale
                    ui.label("Scale:");
                    ui.horizontal(|ui| {
                        let mut x = String::from("1.0");
                        let mut y = String::from("1.0");
                        let mut z = String::from("1.0");
                        ui.add(egui::TextEdit::singleline(&mut x));
                        ui.add(egui::TextEdit::singleline(&mut y));
                        ui.add(egui::TextEdit::singleline(&mut z));
                    });
                } else {
                    ui.label("No entity selected. Drag an entity here.");
                }
            });
        });
    }
}

impl Default for TransformPropertiesUI {
    fn default() -> Self {
        Self::new()
    }
}