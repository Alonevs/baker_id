//! # Timeline Editor API
//! 
//! Módulo para gestionar la línea de tiempo y animaciones.

use crate::ui::Widget;
use crate::ui::WidgetType;
use eframe::egui;
use crate::property_panel::PropertyPanel;
use crate::animation_track::{Track, TrackData, InterpolationType};

/// Editor de línea de tiempo para animaciones
pub struct TimelineEditor {
    widgets: Vec<Widget>,
    property_panel: PropertyPanel,
    timeline_track: Vec<Track>,
    /// Frame actual de la reproducción
    pub current_frame: u32,
    /// Duración en ms de cada frame
    pub frame_duration: f32,
}

impl TimelineEditor {
    /// Crea un nuevo editor de línea de tiempo
    pub fn new(property_panel: PropertyPanel) -> Self {
        TimelineEditor {
            widgets: Vec::new(),
            property_panel,
            timeline_track: Vec::new(),
            current_frame: 0,
            frame_duration: 16.67, // 60 FPS
        }
    }

    /// Crea los widgets de la UI del timeline
    pub fn create_widgets(&mut self) {
        let x = 10.0;
        let y = 10.0;
        let w = 800.0;
        let h = 400.0;

        self.widgets.push(Widget::new(WidgetType::Label, "Timeline", x, y, w, h));
        self.widgets.push(Widget::new(WidgetType::Button, "Play", x, y + h, w, h));
        self.widgets.push(Widget::new(WidgetType::Button, "Stop", x + w, y + h, w, h));
        self.widgets.push(Widget::new(WidgetType::Label, "Frame: 0", x, y + (2.0 * h), w, h));
        self.widgets.push(Widget::new(WidgetType::Canvas, "Timeline Track", x, y + (2.0 * h) + h, w, h));
        self.widgets.push(Widget::new(WidgetType::Label, "Keyframe Editor", x, y + (3.0 * h), w, h));
    }

    /// Obtiene los widgets del editor
    pub fn get_widgets(&self) -> &Vec<Widget> {
        &self.widgets
    }

    /// Obtiene el panel de propiedades
    pub fn get_property_panel(&self) -> &PropertyPanel {
        &self.property_panel
    }

    /// Obtiene la pista de la línea de tiempo
    pub fn get_timeline_track(&self) -> &Vec<Track> {
        &self.timeline_track
    }

    /// Obtiene el frame actual
    pub fn get_current_frame(&self) -> u32 {
        self.current_frame
    }

    /// Establece el frame actual
    pub fn set_current_frame(&mut self, frame: u32) {
        self.current_frame = frame;
    }

    /// Obtiene la duración del frame
    pub fn get_frame_duration(&self) -> f32 {
        self.frame_duration
    }

    /// Establece la duración del frame
    pub fn set_frame_duration(&mut self, duration: f32) {
        self.frame_duration = duration;
    }

    /// Agrega un keyframe a la pista
    pub fn add_keyframe(&mut self, frame: u32, value: f32) {
        self.timeline_track.push(Track {
            name: "Timeline".to_string(),
            data: vec![TrackData {
                frame,
                value,
                properties: "Linear".to_string(),
            }],
            interpolation: InterpolationType::Linear,
            keyframes: Vec::new(),
        });
    }

    /// Elimina un keyframe por índice
    pub fn remove_keyframe(&mut self, index: usize) {
        if index < self.timeline_track.len() {
            self.timeline_track.remove(index);
        }
    }

    /// Obtiene el valor interpolado en un frame dado
    pub fn interpolate(&self, frame: f32) -> f32 {
        let mut min_frame = f32::MAX;
        let mut max_frame = 0.0;
        
        for track in &self.timeline_track {
            for data in &track.data {
                let f = data.frame as f32;
                if f < min_frame {
                    min_frame = f;
                }
                if f > max_frame {
                    max_frame = f;
                }
            }
        }
        
        if frame < min_frame {
            return self.timeline_track.first().and_then(|t| t.data.first()).map(|d| d.value).unwrap_or(0.0);
        }
        if frame > max_frame {
            return self.timeline_track.last().and_then(|t| t.data.last()).map(|d| d.value).unwrap_or(0.0);
        }

        for i in 0..self.timeline_track.len() {
            let track = &self.timeline_track[i];
            let mut found = false;
            
            for j in 0..track.data.len() - 1 {
                let data1 = &track.data[j];
                let data2 = &track.data[j + 1];
                
                let f1 = data1.frame as f32;
                let f2 = data2.frame as f32;
                
                if frame >= f1 && frame <= f2 {
                    let t = (frame - f1) / (f2 - f1);
                    return data1.value + t * (data2.value - data1.value);
                }
                found = true;
            }
            
            if !found {
                continue;
            }
        }

        self.timeline_track.last().and_then(|t| t.data.last()).map(|d| d.value).unwrap_or(0.0)
    }

    /// Actualiza el editor
    pub fn update(&mut self, ctx: &egui::Context, _ui: &egui::Ui) {
        self.widgets = Vec::new();
        self.create_widgets();
    }
}

