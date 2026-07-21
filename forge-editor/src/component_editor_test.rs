#[cfg(test)]
mod tests {
    use crate::component_editor::ComponentEditor;
    use crate::property_panel::PropertyPanel;

    #[test]
    fn test_component_editor_new() {
        let panel = PropertyPanel::new();
        let editor = ComponentEditor::new(panel);
        let widgets = editor.get_widgets();
        assert!(widgets.is_empty());
    }

    #[test]
    fn test_component_create_widgets() {
        let mut editor = ComponentEditor::new(PropertyPanel::new());
        editor.create_widgets();
        let widgets = editor.get_widgets();
        assert!(widgets.len() > 0);
    }

    #[test]
    fn test_component_get_widgets() {
        let editor = ComponentEditor::new(PropertyPanel::new());
        let widgets = editor.get_widgets();
        assert!(widgets.is_empty());
    }

    #[test]
    fn test_component_get_property_panel() {
        let panel = PropertyPanel::new();
        let editor = ComponentEditor::new(panel);
        let pp = editor.get_property_panel();
        let widgets = pp.get_widgets();
        assert!(widgets.is_empty());
    }

    #[test]
    fn test_component_get_component_properties() {
        let editor = ComponentEditor::new(PropertyPanel::new());
        let props = editor.get_component_properties();
        assert!(props.is_none());
    }

    #[test]
    fn test_component_get_transform_properties() {
        let editor = ComponentEditor::new(PropertyPanel::new());
        let props = editor.get_transform_properties();
        assert!(props.is_none());
    }

    #[test]
    fn test_component_get_script_properties() {
        let editor = ComponentEditor::new(PropertyPanel::new());
        let props = editor.get_script_properties();
        assert!(props.is_none());
    }
}

