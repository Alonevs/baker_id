use eframe::egui;

pub struct ViewportDockable {
    pub camera_x: f32,
    pub camera_y: f32,
    pub zoom: f32,
}

impl Default for ViewportDockable {
    fn default() -> Self {
        Self {
            camera_x: 0.0,
            camera_y: 0.0,
            zoom: 1.0,
        }
    }
}

impl ViewportDockable {
    pub fn zoom_in(&mut self) {
        self.zoom = (self.zoom * 1.1).min(5.0);
    }

    pub fn zoom_out(&mut self) {
        self.zoom = (self.zoom / 1.1).max(0.1);
    }
}

