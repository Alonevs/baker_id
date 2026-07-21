#[cfg(test)]
mod tests {
    use crate::property_panel::{PropertyPanel, EntityId, TransformProperties, ComponentProperties, ScriptProperties};

    #[test]
    fn test_property_panel_new() {
        let panel = PropertyPanel::new();
        assert!(panel.get_selected_entity().is_none());
    }

    #[test]
    fn test_property_panel_set_selected_entity() {
        let mut panel = PropertyPanel::new();
        panel.set_selected_entity(EntityId(1));
        assert_eq!(panel.get_selected_entity(), Some(EntityId(1)));
    }

    #[test]
    fn test_property_panel_set_transform_props() {
        let mut panel = PropertyPanel::new();
        let transform = TransformProperties {
            position: (1.0, 2.0, 3.0),
            rotation: (45.0, 90.0, 180.0),
            scale: (2.0, 3.0, 4.0),
            visible: true,
        };
        panel.set_transform_props(transform);
        assert!(panel.get_transform_properties().is_some());
    }

    #[test]
    fn test_property_panel_set_component_props() {
        let mut panel = PropertyPanel::new();
        let component = ComponentProperties {
            components: vec!["PhysicsBody".to_string(), "Sprite".to_string()],
        };
        panel.set_component_props(component);
        assert!(panel.get_component_properties().is_some());
    }

    #[test]
    fn test_property_panel_set_script_props() {
        let mut panel = PropertyPanel::new();
        let script = ScriptProperties {
            scripts: vec!["MainScript".to_string()],
        };
        panel.set_script_props(script);
        assert!(panel.get_script_properties().is_some());
    }
}

