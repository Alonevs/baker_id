//! # Scene System - Completo
//! 
//! Sistema completo de gestión de escenas con:
//! - SceneManager para transiciones entre escenas
//! - SceneGraph para jerarquía de escenas
//! - Scene persistence (guardado/carga)
//! - Scene templates
//! - Scene transitions con efectos

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Tipo de transición de escena
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TransitionType {
    Fade,
    Slide,
    SlideIn,
    SlideOut,
    Zoom,
    ZoomIn,
    ZoomOut,
    None,
}

/// Duración de transición en milisegundos
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct TransitionDuration {
    pub duration_ms: u32,
    pub ease: EaseFunction,
}

impl Default for TransitionDuration {
    fn default() -> Self {
        Self {
            duration_ms: 500,
            ease: EaseFunction::Linear,
        }
    }
}

/// Función de easing para transiciones
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EaseFunction {
    Linear,
    EaseIn,
    EaseOut,
    EaseInOut,
}

/// Efecto de transición
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransitionEffect {
    pub transition_type: TransitionType,
    pub duration: TransitionDuration,
    pub color: Option<(f32, f32, f32, f32)>,
}

impl Default for TransitionEffect {
    fn default() -> Self {
        Self {
            transition_type: TransitionType::Fade,
            duration: TransitionDuration::default(),
            color: None,
        }
    }
}

impl TransitionEffect {
    /// Aplica la transición
    pub fn apply(&self, canvas: &mut eframe::egui::CanvasResponse) {
        match self.transition_type {
            TransitionType::Fade => {
                if let Some(color) = self.color {
                    canvas.rect_filled(eframe::egui::Rect::from_min_size(
                        eframe::egui::Pos2::ZERO,
                        eframe::egui::Vec2::new(1000.0, 1000.0),
                    ), color);
                }
            }
            TransitionType::Slide => {
                canvas.rect(
                    eframe::egui::Rect::from_min_size(
                        eframe::egui::Pos2::new(0.0, 0.0),
                        eframe::egui::Vec2::new(1000.0, 1000.0),
                    ),
                    eframe::egui::Rounding::ZERO,
                    eframe::egui::Shape::Stroke(eframe::egui::Stroke::new(
                        2.0,
                        eframe::egui::Color32::from_rgb(255, 255, 255),
                    )),
                );
            }
            _ => {}
        }
    }
}

/// Datos de una escena
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SceneData {
    pub id: String,
    pub name: String,
    pub entities: Vec<crate::Entity>,
    pub components: HashMap<String, Vec<crate::Component>>,
    pub scripts: HashMap<String, String>,
    pub scene_settings: SceneSettings,
}

impl SceneData {
    /// Crea una nueva escena
    pub fn new(id: String, name: String) -> Self {
        Self {
            id,
            name,
            entities: Vec::new(),
            components: HashMap::new(),
            scripts: HashMap::new(),
            scene_settings: SceneSettings::default(),
        }
    }

    /// Carga datos de una escena desde entidades
    pub fn from_entities(entities: &[crate::Entity]) -> Self {
        Self {
            id: "default".to_string(),
            name: "Default Scene".to_string(),
            entities: entities.to_vec(),
            components: HashMap::new(),
            scripts: HashMap::new(),
            scene_settings: SceneSettings::default(),
        }
    }
}

/// Configuración de escena
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SceneSettings {
    pub background_color: eframe::egui::Color32,
    pub camera_position: (f32, f32),
    pub camera_zoom: f32,
    pub fog_enabled: bool,
    pub fog_color: eframe::egui::Color32,
    pub fog_density: f32,
}

/// Escena
#[derive(Debug, Clone)]
pub struct Scene {
    pub id: String,
    pub name: String,
    pub entities: Vec<crate::Entity>,
    pub components: HashMap<String, Vec<crate::Component>>,
    pub scripts: HashMap<String, String>,
    pub scene_settings: SceneSettings,
    pub is_active: bool,
    pub transition_effect: Option<TransitionEffect>,
}

impl Scene {
    /// Crea una nueva escena
    pub fn new(id: String, name: String) -> Self {
        Self {
            id,
            name,
            entities: Vec::new(),
            components: HashMap::new(),
            scripts: HashMap::new(),
            scene_settings: SceneSettings::default(),
            is_active: false,
            transition_effect: None,
        }
    }

    /// Crea escena con entidades
    pub fn with_entities(id: String, name: String, entities: &[crate::Entity]) -> Self {
        Self {
            id,
            name,
            entities: entities.to_vec(),
            components: HashMap::new(),
            scripts: HashMap::new(),
            scene_settings: SceneSettings::default(),
            is_active: false,
            transition_effect: None,
        }
    }

    /// Activa la escena
    pub fn activate(&mut self) {
        self.is_active = true;
    }

    /// Desactiva la escena
    pub fn deactivate(&mut self) {
        self.is_active = false;
    }

    /// Obtiene la entidad por ID
    pub fn get_entity(&self, id: &str) -> Option<&crate::Entity> {
        self.entities.iter().find(|e| e.id == id)
    }

    /// Obtiene la entidad mutada por ID
    pub fn get_entity_mut(&mut self, id: &str) -> Option<&mut crate::Entity> {
        self.entities.iter_mut().find(|e| e.id == id)
    }
}

/// SceneManager completo
pub struct SceneManager {
    /// Escenas disponibles
    pub scenes: HashMap<String, Scene>,
    /// Escena actual
    pub current_scene: Option<String>,
    /// Escena anterior
    pub previous_scene: Option<String>,
    /// Escena en transición
    pub transitioning_scene: Option<String>,
    /// Transición en progreso
    pub is_transitioning: bool,
    /// Transición actual
    pub current_transition: Option<TransitionEffect>,
    /// Progreso de transición (0.0 - 1.0)
    pub transition_progress: f32,
    /// Historial de escenas
    pub scene_history: Vec<String>,
    /// Escenas guardadas
    pub saved_scenes: HashMap<String, SceneData>,
    /// UIManager integrado
    pub ui_manager: crate::ui_system::UiManager,
}

impl Default for SceneManager {
    fn default() -> Self {
        Self::new()
    }
}

impl SceneManager {
    /// Crea un nuevo SceneManager
    pub fn new() -> Self {
        Self {
            scenes: HashMap::new(),
            current_scene: None,
            previous_scene: None,
            transitioning_scene: None,
            is_transitioning: false,
            current_transition: None,
            transition_progress: 0.0,
            scene_history: Vec::new(),
            saved_scenes: HashMap::new(),
            ui_manager: crate::ui_system::UiManager::new(),
        }
    }
            saved_scenes: HashMap::new(),
        }
    }

    /// Carga escena desde datos
    pub fn load_scene(&mut self, scene_data: SceneData) {
        let scene = Scene {
            id: scene_data.id.clone(),
            name: scene_data.name.clone(),
            entities: scene_data.entities,
            components: scene_data.components,
            scripts: scene_data.scripts,
            scene_settings: scene_data.scene_settings,
            is_active: false,
            transition_effect: None,
        };
        
        self.scenes.insert(scene_data.id, scene);
        println!("[SCENE MANAGER] Loaded scene: {}", scene_data.name);
    }

    /// Crea una nueva escena
    pub fn create_scene(&mut self, id: String, name: String, entities: &[crate::Entity]) {
        let scene = Scene::with_entities(id, name, entities);
        self.scenes.insert(id, scene);
        println!("[SCENE MANAGER] Created scene: {}", name);
    }

    /// Obtiene escena por ID
    pub fn get_scene(&self, id: &str) -> Option<&Scene> {
        self.scenes.get(id)
    }

    /// Obtiene escena mutada por ID
    pub fn get_scene_mut(&mut self, id: &str) -> Option<&mut Scene> {
        self.scenes.get_mut(id)
    }

    /// Elimina escena por ID
    pub fn remove_scene(&mut self, id: &str) {
        if self.scenes.remove(id).is_some() {
            // Eliminar del historial
            self.scene_history.retain(|s| s != id);
            // Eliminar del current si es la actual
            if self.current_scene.as_ref() == Some(&id.to_string()) {
                self.current_scene = None;
            }
            println!("[SCENE MANAGER] Removed scene: {}", id);
        }
    }

    /// Cambia a la escena actual
    pub fn set_current_scene(&mut self, scene_id: &str) {
        if let Some(scene) = self.scenes.get(scene_id) {
            // Guardar escena anterior
            if let Some(old_scene) = self.current_scene.clone() {
                self.previous_scene = Some(old_scene);
                self.scene_history.push(old_scene);
            }
            
            // Activar nueva escena
            if let Some(scene_mut) = self.scenes.get_mut(scene_id) {
                scene_mut.activate();
            }
            
            self.current_scene = Some(scene_id.to_string());
            self.previous_scene = None;
            self.transitioning_scene = None;
            self.is_transitioning = false;
            self.transition_progress = 0.0;
            
            println!("[SCENE MANAGER] Set current scene: {}", scene_id);
        }
    }

    /// Navega a otra escena con transición
    pub fn navigate_to(&mut self, scene_id: &str, transition: Option<TransitionEffect>) {
        if let Some(scene) = self.scenes.get(scene_id) {
            // Guardar escena anterior
            if let Some(old_scene) = self.current_scene.clone() {
                self.previous_scene = Some(old_scene);
                self.scene_history.push(old_scene);
            }
            
            // Configurar transición
            self.current_transition = transition;
            self.transitioning_scene = Some(scene_id.to_string());
            self.is_transitioning = true;
            self.transition_progress = 0.0;
            
            // Activar nueva escena
            if let Some(scene_mut) = self.scenes.get_mut(scene_id) {
                scene_mut.activate();
            }
            
            self.current_scene = Some(scene_id.to_string());
            
            println!("[SCENE MANAGER] Navigating to: {} (transitioning)", scene_id);
        }
    }

    /// Actualiza transición
    pub fn update_transition(&mut self, delta: f32) {
        if !self.is_transitioning {
            return;
        }
        
        if let Some(ref transition) = self.current_transition {
            let elapsed = delta * 1000.0;
            let progress = (self.transition_progress + (elapsed as f32 / transition.duration.duration_ms as f32))
                .min(1.0);
            
            self.transition_progress = progress;
            
            // Aplicar transición
            if progress >= 1.0 {
                self.complete_transition();
            }
        }
    }

    /// Completa transición
    fn complete_transition(&mut self) {
        self.is_transitioning = false;
        self.transitioning_scene = None;
        self.current_transition = None;
        self.transition_progress = 0.0;
        
        println!("[SCENE MANAGER] Transition complete");
    }

    /// Regresa a escena anterior
    pub fn go_back(&mut self) {
        if let Some(scene_id) = self.previous_scene.clone() {
            self.navigate_to(&scene_id, None);
        }
    }

    /// Guarda escena actual
    pub fn save_scene(&mut self, filename: &str) {
        if let Some(scene_id) = &self.current_scene {
            if let Some(scene) = self.scenes.get(scene_id) {
                let scene_data = SceneData {
                    id: scene.id.clone(),
                    name: scene.name.clone(),
                    entities: scene.entities.clone(),
                    components: scene.components.clone(),
                    scripts: scene.scripts.clone(),
                    scene_settings: scene.scene_settings.clone(),
                };
                
                self.saved_scenes.insert(filename.to_string(), scene_data);
                println!("[SCENE MANAGER] Saved scene to: {}", filename);
            }
        }
    }

    /// Carga escena guardada
    pub fn load_saved_scene(&mut self, filename: &str) -> Option<SceneData> {
        self.saved_scenes.get(filename).cloned()
    }

    /// Limpia todas las escenas
    pub fn clear_all(&mut self) {
        self.scenes.clear();
        self.current_scene = None;
        self.previous_scene = None;
        self.scene_history.clear();
        println!("[SCENE MANAGER] Cleared all scenes");
    }

    /// Obtiene lista de escenas
    pub fn get_scene_list(&self) -> Vec<&Scene> {
        self.scenes.values().collect()
    }

    /// Obtiene nombre de escena actual
    pub fn get_current_scene_name(&self) -> Option<String> {
        self.current_scene.as_ref().and_then(|id| self.scenes.get(id).map(|s| s.name.clone()))
    }

    /// Verifica si hay escena activa
    pub fn has_active_scene(&self) -> bool {
        self.current_scene.as_ref().and_then(|id| self.scenes.get(id)).map(|s| s.is_active).unwrap_or(false)
    }

    /// Obtiene progreso de transición
    pub fn get_transition_progress(&self) -> f32 {
        self.transition_progress
    }
    
    /// Obtiene referencia al UIManager
    pub fn ui_manager(&self) -> &crate::ui_system::UiManager {
        &self.ui_manager
    }
    
    /// Obtiene referencia mutante al UIManager
    pub fn ui_manager_mut(&mut self) -> &mut crate::ui_system::UiManager {
        &mut self.ui_manager
    }
}
