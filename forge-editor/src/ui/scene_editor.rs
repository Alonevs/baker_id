

/// Scene Editor UI para edición de escena 2D
#[derive(Debug, Clone)]
pub struct SceneEditor {
    pub entities: Vec<String>,
    pub selected_entity: Option<usize>,
    pub show_hierarchy: bool,
    pub show_properties: bool,
    pub scene_nodes: Vec<String>,
    pub indent_level: usize,
}

impl Default for SceneEditor {
    fn default() -> Self {
        Self {
            entities: Vec::new(),
            selected_entity: None,
            show_hierarchy: true,
            show_properties: true,
            scene_nodes: Vec::new(),
            indent_level: 0,
        }
    }
}

impl SceneEditor {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn update_entities(&mut self, entities: Vec<String>) {
        self.entities = entities;
    }

    pub fn select_entity(&mut self, index: usize) {
        self.selected_entity = Some(index);
    }

    pub fn deselect(&mut self) {
        self.selected_entity = None;
    }
}

