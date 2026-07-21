use std::collections::HashMap;
use std::sync::Arc;
use uuid::Uuid;

use serde::{Serialize, Deserialize};

use crate::scene_node::{NodeData, Scene};
use crate::ecs::{Entity, Signal};

#[derive(Debug, Clone)]
pub struct ScenePreview {
    pub scene_id: Uuid,
    pub thumbnail: Option<Vec<u8>>,
    pub width: u32,
    pub height: u32,
    pub fps: f32,
    pub is_playing: bool,
    pub entities: Vec<Arc<Entity>>,
}

impl ScenePreview {
    pub fn new(scene_id: Uuid) -> Self {
        Self {
            scene_id,
            thumbnail: None,
            width: 640,
            height: 480,
            fps: 60.0,
            is_playing: false,
            entities: Vec::new(),
        }
    }

    pub fn start(&mut self) {
        self.is_playing = true;
    }

    pub fn stop(&mut self) {
        self.is_playing = false;
    }

    pub fn update(&mut self, delta: f64) {
        if self.is_playing {
            // Actualizar preview
        }
    }

    pub fn capture_thumbnail(&mut self) {
        // Capturar thumbnail
    }
}

#[derive(Debug, Clone)]
pub struct ScenePreviewManager {
    pub previews: HashMap<Uuid, Arc<ScenePreview>>,
    pub active_preview: Option<Uuid>,
    pub preview_queue: Vec<Uuid>,
}

impl ScenePreviewManager {
    pub fn new() -> Self {
        Self {
            previews: HashMap::new(),
            active_preview: None,
            preview_queue: Vec::new(),
        }
    }

    pub fn add_preview(&mut self, preview: Arc<ScenePreview>) -> Uuid {
        let scene_id = preview.scene_id;
        self.previews.insert(scene_id, preview.clone());
        scene_id
    }

    pub fn get_preview(&self, scene_id: &Uuid) -> Option<Arc<ScenePreview>> {
        self.previews.get(scene_id).cloned()
    }

    pub fn get_active_preview(&self) -> Option<Arc<ScenePreview>> {
        self.active_preview.and_then(|id| self.previews.get(&id).cloned())
    }

    pub fn set_active_preview(&mut self, scene_id: &Uuid) {
        self.active_preview = Some(*scene_id);
    }

    pub fn queue_preview(&mut self, scene_id: Uuid) {
        self.preview_queue.push(scene_id);
    }

    pub fn process_queue(&mut self) {
        while let Some(scene_id) = self.preview_queue.pop() {
            // Procesar preview
        }
    }

    pub fn update(&mut self, delta: f64) {
        for (_, preview_arc) in &self.previews {
            let mut preview_clone = preview_arc.as_ref().clone();
            preview_clone.update(delta);
        }
    }
}

impl Default for ScenePreviewManager {
    fn default() -> Self {
        Self::new()
    }
}
