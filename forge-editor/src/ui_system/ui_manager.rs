use std::collections::HashMap;
use std::time::Instant;

use crate::ui_system::{
    UiLayerId, UiComponentId, UiComponent, UiComponentType,
    UiPos, UiSize, UiColor, UiFont,
};

/// Capas de UI por tipo de escena
#[derive(Debug, Clone)]
pub enum UiLayerType {
    Main,
    Pause,
    Settings,
    Dialogue,
    Inventory,
    HUD,
    Menu,
    Modal,
    Tooltip,
}

impl Default for UiLayerType {
    fn default() -> Self {
        Self::Main
    }
}

/// Capa de UI
#[derive(Debug, Clone)]
pub struct UiLayer {
    pub id: UiLayerId,
    pub name: String,
    pub layer_type: UiLayerType,
    pub z_index: i32,
    pub is_active: bool,
    pub is_visible: bool,
    pub components: HashMap<UiComponentId, Box<dyn UiComponent>>,
    pub position: UiPos,
    pub size: UiSize,
    pub created_at: Instant,
    pub last_updated: Instant,
}

impl UiLayer {
    pub fn new(id: UiLayerId, name: &str, layer_type: UiLayerType) -> Self {
        Self {
            id,
            name: name.to_string(),
            layer_type,
            z_index: 0,
            is_active: true,
            is_visible: true,
            components: HashMap::new(),
            position: UiPos::zero(),
            size: UiSize::zero(),
            created_at: Instant::now(),
            last_updated: Instant::now(),
        }
    }
    
    /// Añadir componente a la capa
    pub fn add_component(&mut self, component: Box<dyn UiComponent>) {
        self.components.insert(component.id(), component);
    }
    
    /// Obtener componente por ID
    pub fn get_component(&self, component_id: UiComponentId) -> Option<&Box<dyn UiComponent>> {
        self.components.get(&component_id)
    }
    
    /// Obtener referencia mutante a componente
    pub fn get_component_mut(&mut self, component_id: UiComponentId) -> Option<&mut Box<dyn UiComponent>> {
        self.components.get_mut(&component_id)
    }
    
    /// Remover componente
    pub fn remove_component(&mut self, component_id: UiComponentId) {
        self.components.remove(&component_id);
    }
    
    /// Obtener número de componentes
    pub fn component_count(&self) -> usize {
        self.components.len()
    }
    
    /// Activar capa
    pub fn activate(&mut self) {
        self.is_active = true;
    }
    
    /// Desactivar capa
    pub fn deactivate(&mut self) {
        self.is_active = false;
    }
    
    /// Mostrar capa
    pub fn show(&mut self) {
        self.is_visible = true;
    }
    
    /// Ocultar capa
    pub fn hide(&mut self) {
        self.is_visible = false;
    }
    
    /// Obtener tiempo de vida en segundos
    pub fn age(&self) -> f32 {
        self.created_at.elapsed().as_secs_f32()
    }
}

/// Gestor central de UI para todo el juego
#[derive(Debug, Default)]
pub struct UIManager {
    ui_layers: HashMap<UiLayerId, UiLayer>,
    active_scene: Option<String>,
    ui_registry: HashMap<UiComponentType, Box<dyn UiComponent>>,
    next_layer_id: u64,
    next_component_id: u64,
    last_update_time: Instant,
    frame_count: u32,
}

impl UIManager {
    /// Crear nuevo UIManager
    pub fn new() -> Self {
        Self {
            ui_layers: HashMap::new(),
            active_scene: None,
            ui_registry: HashMap::new(),
            next_layer_id: 1,
            next_component_id: 1,
            last_update_time: Instant::now(),
            frame_count: 0,
        }
    }
    
    /// Inicializar sistema UI
    pub fn init(&mut self) {
        println!("🎨 UI System initialized");
        
        // Crear capa principal por defecto
        let main_layer_id = UiLayerId::new(self.next_layer_id);
        self.next_layer_id += 1;
        
        let main_layer = UiLayer::new(main_layer_id, "Main", UiLayerType::Main);
        self.ui_layers.insert(main_layer_id, main_layer);
    }
    
    /// Crear nueva capa de UI
    pub fn create_layer(&mut self, name: &str, layer_type: UiLayerType) -> UiLayerId {
        let id = UiLayerId::new(self.next_layer_id);
        self.next_layer_id += 1;
        
        let layer = UiLayer::new(id, name, layer_type);
        self.ui_layers.insert(id, layer);
        
        id
    }
    
    /// Obtener capa por nombre
    pub fn get_layer(&self, name: &str) -> Option<&UiLayer> {
        self.ui_layers.values().find(|layer| layer.name == name)
    }
    
    /// Obtener capa mutante por nombre
    pub fn get_layer_mut(&mut self, name: &str) -> Option<&mut UiLayer> {
        self.ui_layers.values_mut().find(|layer| layer.name == name)
    }
    
    /// Obtener capa por ID
    pub fn get_layer_by_id(&self, layer_id: UiLayerId) -> Option<&UiLayer> {
        self.ui_layers.get(&layer_id)
    }
    
    /// Mostrar capa
    pub fn show_layer(&mut self, layer_id: UiLayerId) {
        if let Some(layer) = self.ui_layers.get_mut(&layer_id) {
            layer.show();
            layer.activate();
        }
    }
    
    /// Ocultar capa
    pub fn hide_layer(&mut self, layer_id: UiLayerId) {
        if let Some(layer) = self.ui_layers.get_mut(&layer_id) {
            layer.hide();
        }
    }
    
    /// Activar capa
    pub fn activate_layer(&mut self, layer_id: UiLayerId) {
        if let Some(layer) = self.ui_layers.get_mut(&layer_id) {
            layer.activate();
        }
    }
    
    /// Desactivar capa
    pub fn deactivate_layer(&mut self, layer_id: UiLayerId) {
        if let Some(layer) = self.ui_layers.get_mut(&layer_id) {
            layer.deactivate();
        }
    }
    
    /// Destruir capa
    pub fn destroy_layer(&mut self, layer_id: UiLayerId) {
        self.ui_layers.remove(&layer_id);
    }
    
    /// Cambiar escena UI actual
    pub fn set_active_scene(&mut self, scene_name: &str) {
        self.active_scene = Some(scene_name.to_string());
        
        // Desactivar todas las capas
        for layer in self.ui_layers.values_mut() {
            layer.deactivate();
        }
        
        // Activar capas correspondientes a la escena
        if scene_name == "main_menu" {
            if let Some(layer) = self.get_layer_mut("Main Menu") {
                layer.activate();
            }
        } else if scene_name == "game" {
            if let Some(layer) = self.get_layer_mut("HUD") {
                layer.activate();
            }
        } else if scene_name == "pause" {
            if let Some(layer) = self.get_layer_mut("Pause Menu") {
                layer.activate();
            }
        }
    }
    
    /// Añadir componente a capa
    pub fn add_component_to_layer(&mut self, layer_id: UiLayerId, component: Box<dyn UiComponent>) {
        if let Some(layer) = self.ui_layers.get_mut(&layer_id) {
            layer.add_component(component);
        }
    }
    
    /// Obtener componente de capa
    pub fn get_component(&self, layer_id: UiLayerId, component_id: UiComponentId) -> Option<&Box<dyn UiComponent>> {
        self.ui_layers.get(&layer_id)?.get_component(component_id)
    }
    
    /// Obtener componente mutante de capa
    pub fn get_component_mut(&mut self, layer_id: UiLayerId, component_id: UiComponentId) -> Option<&mut Box<dyn UiComponent>> {
        self.ui_layers.get_mut(&layer_id)?.get_component_mut(component_id)
    }
    
    /// Actualizar todas las capas
    pub fn update(&mut self, dt: f32) {
        self.last_update_time = Instant::now();
        self.frame_count += 1;
        
        for layer in self.ui_layers.values_mut() {
            if layer.is_active && layer.is_visible {
                layer.update(dt);
            }
        }
    }
    
    /// Renderizar todas las capas
    pub fn render<R: Renderer>(&self, renderer: &mut R) {
        // Renderizar por orden de Z-index
        let mut sorted_layers: Vec<_> = self.ui_layers.values().collect();
        sorted_layers.sort_by(|a, b| a.z_index.cmp(&b.z_index));
        
        for layer in sorted_layers {
            if layer.is_active && layer.is_visible {
                for component in layer.components.values() {
                    component.render(renderer);
                }
            }
        }
    }
    
    /// Procesar input de todas las capas
    pub fn handle_input(&mut self, event: &crate::ui_system::InputEvent) -> Option<crate::ui_system::InputAction> {
        // Procesar capas por orden inverso de Z (más arriba primero)
        let mut sorted_layers: Vec<_> = self.ui_layers.values().collect();
        sorted_layers.sort_by(|a, b| b.z_index.cmp(&a.z_index));
        
        for layer in sorted_layers {
            if layer.is_active && layer.is_visible {
                if let Some(action) = layer.handle_input(event) {
                    return Some(action);
                }
            }
        }
        
        None
    }
    
    /// Obtener número de capas activas
    pub fn active_layer_count(&self) -> usize {
        self.ui_layers.values().filter(|layer| layer.is_active).count()
    }
    
    /// Obtener número total de componentes
    pub fn total_component_count(&self) -> usize {
        self.ui_layers.values().map(|layer| layer.component_count()).sum()
    }
    
    /// Obtener tiempo de actualización en segundos
    pub fn update_time(&self) -> f32 {
        self.last_update_time.elapsed().as_secs_f32()
    }
    
    /// Obtener contador de frames
    pub fn frame_count(&self) -> u32 {
        self.frame_count
    }
    
    /// Limpiar todas las capas
    pub fn clear_all(&mut self) {
        self.ui_layers.clear();
        self.active_scene = None;
    }
}

/// Extensiones para integración con PlaySession
pub trait PlaySessionUiExt {
    fn init_ui_system(&mut self, manager: &mut UIManager);
    fn show_main_menu(&mut self);
    fn hide_main_menu(&mut self);
    fn show_pause_menu(&mut self);
    fn hide_pause_menu(&mut self);
    fn resume_game(&mut self);
}

impl PlaySessionUiExt for crate::play_session::PlaySession {
    fn init_ui_system(&mut self, manager: &mut UIManager) {
        manager.init();
    }
    
    fn show_main_menu(&mut self) {
        if let Some(layer) = self.ui_manager.get_layer_mut("Main Menu") {
            layer.show();
            layer.activate();
        }
    }
    
    fn hide_main_menu(&mut self) {
        if let Some(layer) = self.ui_manager.get_layer_mut("Main Menu") {
            layer.hide();
            layer.deactivate();
        }
    }
    
    fn show_pause_menu(&mut self) {
        if let Some(layer) = self.ui_manager.get_layer_mut("Pause Menu") {
            layer.show();
            layer.activate();
        }
    }
    
    fn hide_pause_menu(&mut self) {
        if let Some(layer) = self.ui_manager.get_layer_mut("Pause Menu") {
            layer.hide();
            layer.deactivate();
        }
    }
    
    fn resume_game(&mut self) {
        self.hide_pause_menu();
        self.ui_manager.set_active_scene("game");
    }
}
