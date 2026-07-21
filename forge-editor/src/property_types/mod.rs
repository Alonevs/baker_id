use crate::property_panel::PropertyPanel;
use crate::property_panel::TransformProperties;
use crate::property_panel::ComponentProperties;
use crate::property_panel::ScriptProperties;
use crate::property_panel::EntityId;

pub struct PropertyPanelWrapper {
    pub panel: PropertyPanel,
}

impl PropertyPanelWrapper {
    pub fn new() -> Self {
        PropertyPanelWrapper {
            panel: PropertyPanel::new(),
        }
    }

    pub fn get_selected_entity(&self) -> Option<EntityId> {
        self.panel.get_selected_entity()
    }

    pub fn get_transform_properties(&self) -> Option<&TransformProperties> {
        self.panel.get_transform_properties()
    }

    pub fn get_component_properties(&self) -> Option<&ComponentProperties> {
        self.panel.get_component_properties()
    }

    pub fn get_script_properties(&self) -> Option<&ScriptProperties> {
        self.panel.get_script_properties()
    }

    pub fn set_selected_entity(&mut self, entity_id: EntityId) {
        self.panel.set_selected_entity(entity_id);
    }
}

