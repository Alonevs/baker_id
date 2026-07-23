//! # Transform Editor API
//! 
//! Módulo para editar propiedades de transformación de entidades.

use crate::ui::Widget;
use crate::ui::WidgetType;
use eframe::egui;
use crate::property_panel::PropertyPanel;
use crate::property_panel::PropertyTab;
use crate::property_panel::TransformProperties;
use crate::property_panel::ComponentProperties;
use crate::property_panel::ScriptProperties;
use crate::property_panel::EntityId;

/// Editor de transformaciones para entidades
#[derive(Debug, Clone)]
pub struct TransformEditor {
    widgets: Vec<Widget>,
    property_panel: PropertyPanel,
}

impl TransformEditor {
    /// Crea un nuevo editor de transformaciones
    pub fn new(property_panel: PropertyPanel) -> Self {
        TransformEditor {
            widgets: Vec::new(),
            property_panel,
        }
    }

    /// Crea los widgets de la UI del transform editor
    pub fn create_widgets(&mut self) {
        let mut x = 10.0;
        let y = 10.0;
        let w = 200.0;
        let h = 200.0;

        self.widgets.push(Widget::new(WidgetType::Label, "Position", x, y, w, h));
        x += w + 10.0;
        self.widgets.push(Widget::new(WidgetType::Input, "X", x, y, w, h));
        x += w + 10.0;
        self.widgets.push(Widget::new(WidgetType::Input, "Y", x, y, w, h));
        x += w + 10.0;
        self.widgets.push(Widget::new(WidgetType::Input, "Z", x, y, w, h));
        x += w + 10.0;

        self.widgets.push(Widget::new(WidgetType::Label, "Rotation", x, y + h + 10.0, w, h));
        x += w + 10.0;
        self.widgets.push(Widget::new(WidgetType::Input, "X", x, y + h + 10.0, w, h));
        x += w + 10.0;
        self.widgets.push(Widget::new(WidgetType::Input, "Y", x, y + h + 10.0, w, h));
        x += w + 10.0;
        self.widgets.push(Widget::new(WidgetType::Input, "Z", x, y + h + 10.0, w, h));
        x += w + 10.0;

        self.widgets.push(Widget::new(WidgetType::Label, "Scale", x, y + h + 20.0, w, h));
        x += w + 10.0;
        self.widgets.push(Widget::new(WidgetType::Input, "X", x, y + h + 20.0, w, h));
        x += w + 10.0;
        self.widgets.push(Widget::new(WidgetType::Input, "Y", x, y + h + 20.0, w, h));
        x += w + 10.0;
        self.widgets.push(Widget::new(WidgetType::Input, "Z", x, y + h + 20.0, w, h));
        x += w + 10.0;

        self.widgets.push(Widget::new(WidgetType::Checkbox, "Visible", x, y + h + 30.0, w, h));
    }

    /// Obtiene los widgets del editor
    pub fn get_widgets(&self) -> &Vec<Widget> {
        &self.widgets
    }

    /// Obtiene el panel de propiedades
    pub fn get_property_panel(&self) -> &PropertyPanel {
        &self.property_panel
    }

    /// Obtiene las propiedades de transformación
    pub fn get_transform_properties(&self) -> Option<&TransformProperties> {
        self.property_panel.get_transform_properties()
    }

    /// Obtiene las propiedades de componentes
    pub fn get_component_properties(&self) -> Option<&ComponentProperties> {
        self.property_panel.get_component_properties()
    }

    /// Obtiene las propiedades de scripts
    pub fn get_script_properties(&self) -> Option<&ScriptProperties> {
        self.property_panel.get_script_properties()
    }

    /// Establece la entidad seleccionada
    pub fn set_selected_entity(&mut self, entity_id: EntityId) {
        self.property_panel.set_selected_entity(entity_id);
    }

    /// Obtiene la entidad seleccionada
    pub fn get_selected_entity(&self) -> Option<EntityId> {
        self.property_panel.get_selected_entity()
    }

    /// Obtiene la pestaña actual
    pub fn get_tab(&self) -> &PropertyTab {
        self.property_panel.get_tab()
    }

    /// Establece la pestaña activa
    pub fn set_tab(&mut self, tab: PropertyTab) {
        self.property_panel.set_tab(tab);
    }

    /// Actualiza el editor
    pub fn update(&mut self, _ctx: &egui::Context, _ui: &egui::Ui) {
        self.widgets = Vec::new();
        self.create_widgets();
    }
}

