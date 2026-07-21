use eframe::egui;

/// Transform Properties - propiedades de transformación
#[derive(Debug, Clone)]
pub struct TransformProperties {
    pub position: (f32, f32, f32),
    pub rotation: (f32, f32, f32),
    pub scale: (f32, f32, f32),
    pub visible: bool,
    pub enabled: bool,
    pub parent: Option<usize>,
}

impl Default for TransformProperties {
    fn default() -> Self {
        Self {
            position: (0.0, 0.0, 0.0),
            rotation: (0.0, 0.0, 0.0),
            scale: (1.0, 1.0, 1.0),
            visible: true,
            enabled: true,
            parent: None,
        }
    }
}

impl TransformProperties {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get_position(&self) -> (f32, f32, f32) {
        self.position
    }

    pub fn get_rotation(&self) -> (f32, f32, f32) {
        self.rotation
    }

    pub fn get_scale(&self) -> (f32, f32, f32) {
        self.scale
    }

    pub fn set_position(&mut self, x: f32, y: f32, z: f32) {
        self.position = (x, y, z);
    }

    pub fn set_rotation(&mut self, x: f32, y: f32, z: f32) {
        self.rotation = (x, y, z);
    }

    pub fn set_scale(&mut self, x: f32, y: f32, z: f32) {
        self.scale = (x, y, z);
    }

    pub fn render(&mut self, ui: &mut egui::Ui) {
        ui.label("Transform Properties");
        ui.separator();

        // Position
        ui.label("Position:");
        ui.horizontal(|ui| {
            ui.label("X:");
            ui.text_edit_singleline(&mut format!("{:.2}", self.position.0));
        });
        ui.horizontal(|ui| {
            ui.label("Y:");
            ui.text_edit_singleline(&mut format!("{:.2}", self.position.1));
        });
        ui.horizontal(|ui| {
            ui.label("Z:");
            ui.text_edit_singleline(&mut format!("{:.2}", self.position.2));
        });

        ui.add_space(5.0);

        // Rotation
        ui.label("Rotation:");
        ui.horizontal(|ui| {
            ui.label("X:");
            ui.text_edit_singleline(&mut format!("{:.2}", self.rotation.0));
        });
        ui.horizontal(|ui| {
            ui.label("Y:");
            ui.text_edit_singleline(&mut format!("{:.2}", self.rotation.1));
        });
        ui.horizontal(|ui| {
            ui.label("Z:");
            ui.text_edit_singleline(&mut format!("{:.2}", self.rotation.2));
        });

        ui.add_space(5.0);

        // Scale
        ui.label("Scale:");
        ui.horizontal(|ui| {
            ui.label("X:");
            ui.text_edit_singleline(&mut format!("{:.2}", self.scale.0));
        });
        ui.horizontal(|ui| {
            ui.label("Y:");
            ui.text_edit_singleline(&mut format!("{:.2}", self.scale.1));
        });
        ui.horizontal(|ui| {
            ui.label("Z:");
            ui.text_edit_singleline(&mut format!("{:.2}", self.scale.2));
        });

        ui.add_space(5.0);

        // Visibility
        ui.label("Visible:");
        ui.checkbox(&mut self.visible, "Visible");

        ui.add_space(5.0);

        // Enabled
        ui.label("Enabled:");
        ui.checkbox(&mut self.enabled, "Enabled");
    }
}

