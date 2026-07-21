use crate::Widget;
use crate::WidgetType;
use crate::PropertyPanel;

#[derive(Debug, Clone)]
pub struct TrackData {
    pub frame: u32,
    pub value: f32,
    pub properties: String,
}

#[derive(Debug, Clone)]
pub struct Track {
    pub name: String,
    pub data: Vec<TrackData>,
    pub interpolation: InterpolationType,
    pub keyframes: Vec<crate::keyframe::Keyframe>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum InterpolationType {
    Linear,
    EaseIn,
    EaseOut,
    EaseInOut,
    Step,
}

#[derive(Debug, Clone)]
pub struct AnimationTrack {
    widgets: Vec<Widget>,
    property_panel: PropertyPanel,
    pub track_name: String,
    pub track_data: Vec<TrackData>,
}

impl AnimationTrack {
    pub fn new(property_panel: PropertyPanel, track_name: &str) -> Self {
        AnimationTrack {
            widgets: Vec::new(),
            property_panel,
            track_name: track_name.to_string(),
            track_data: Vec::new(),
        }
    }

    pub fn create_widgets(&mut self) {
        let x = 10.0;
        let y = 10.0;
        let w = 600.0;
        let h = 200.0;

        self.widgets.push(Widget::new(WidgetType::Label, &self.track_name, x, y, w, h));
        self.widgets.push(Widget::new(WidgetType::List, "Track Data", x, y + h, w, h));
        self.widgets.push(Widget::new(WidgetType::Button, "Add Data", x, y + (2.0 * h), w, h));
        self.widgets.push(Widget::new(WidgetType::Button, "Clear Data", x, y + (3.0 * h), w, h));
    }

    pub fn get_widgets(&self) -> &Vec<Widget> {
        &self.widgets
    }

    pub fn get_property_panel(&self) -> &PropertyPanel {
        &self.property_panel
    }

    pub fn get_track_name(&self) -> &String {
        &self.track_name
    }

    pub fn get_track_data(&self) -> &Vec<TrackData> {
        &self.track_data
    }

    pub fn add_track_data(&mut self, frame: u32, value: f32, properties: &str) {
        self.track_data.push(TrackData {
            frame,
            value,
            properties: properties.to_string(),
        });
        self.track_data.sort_by(|a, b| a.frame.cmp(&b.frame));
    }

    pub fn remove_track_data(&mut self, index: usize) {
        if index < self.track_data.len() {
            self.track_data.remove(index);
        }
    }

    pub fn clear_track_data(&mut self) {
        self.track_data.clear();
    }

    pub fn get_track_data_mut(&mut self) -> &mut Vec<TrackData> {
        &mut self.track_data
    }
}

