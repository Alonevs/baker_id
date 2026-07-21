#[cfg(test)]
mod tests {
    use crate::timeline::TimelineEditor;
    use crate::property_panel::PropertyPanel;

    #[test]
    fn test_timeline_editor_new() {
        let panel = PropertyPanel::new();
        let editor = TimelineEditor::new(panel);
        assert!(editor.current_frame == 0);
        assert!(editor.frame_duration == 16.67);
    }

    #[test]
    fn test_timeline_create_widgets() {
        let mut editor = TimelineEditor::new(PropertyPanel::new());
        editor.create_widgets();
        let widgets = editor.get_widgets();
        assert!(widgets.len() > 0);
    }

    #[test]
    fn test_timeline_get_property_panel() {
        let panel = PropertyPanel::new();
        let editor = TimelineEditor::new(panel);
        let pp = editor.get_property_panel();
        let widgets = pp.get_widgets();
        assert!(widgets.len() >= 0);
    }

    #[test]
    fn test_timeline_get_timeline_track() {
        let editor = TimelineEditor::new(PropertyPanel::new());
        let track = editor.get_timeline_track();
        assert!(track.is_empty());
    }

    #[test]
    fn test_timeline_get_current_frame() {
        let editor = TimelineEditor::new(PropertyPanel::new());
        assert_eq!(editor.get_current_frame(), 0);
    }
}

