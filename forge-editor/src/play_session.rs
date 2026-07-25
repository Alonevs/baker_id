//! # Play Mode System - Improved
//! 
//! Sistema mejorado para ejecutar juegos en modo Play con:
//! - Snapshot de posiciones
//! - Simulación de físicas
//! - Captura de input del usuario
//! - Restauración automática al Stop
//! - Entidades reales con componentes
//! - Integración con Event Forge
//! - UI mejorada para control de play

use std::collections::HashMap;
use std::time::Instant;

use crate::audio_system::AudioManager;

/// Vector 2D
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn zero() -> Self {
        Self::new(0.0, 0.0)
    }
    
    /// Suma otro vector
    pub fn add(&self, other: Vec2) -> Vec2 {
        Vec2::new(self.x + other.x, self.y + other.y)
    }
    
    /// Resta otro vector
    pub fn sub(&self, other: Vec2) -> Vec2 {
        Vec2::new(self.x - other.x, self.y - other.y)
    }
    
    /// Multiplica por escalar
    pub fn mul(&self, scalar: f32) -> Vec2 {
        Vec2::new(self.x * scalar, self.y * scalar)
    }
}

impl Vec2 {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn zero() -> Self {
        Self::new(0.0, 0.0)
    }
}

impl std::fmt::Display for Vec2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl std::ops::Index<usize> for Vec2 {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            _ => panic!("Index out of bounds"),
        }
    }
}

/// Snapshot de una entidad en el tiempo
#[derive(Debug, Clone)]
pub struct EntitySnapshot {
    pub id: String,
    pub position: Vec2,
    pub rotation: f32,
    pub scale: (f32, f32),
}

/// Snapshot completo de la escena
#[derive(Debug, Clone)]
pub struct SceneSnapshot {
    pub entities: HashMap<String, EntitySnapshot>,
    pub timestamp: f64,
}

impl SceneSnapshot {
    /// Crea un nuevo snapshot vacío
    pub fn new() -> Self {
        Self {
            entities: HashMap::new(),
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs_f64(),
        }
    }

    /// Crea un snapshot desde entidades
    pub fn from_entities(entities: &[crate::Entity]) -> Self {
        let mut entities_map = HashMap::new();
        for entity in entities {
            entities_map.insert(
                entity.id.clone(),
                EntitySnapshot {
                    id: entity.id.clone(),
                    position: Vec2 {
                        x: entity.position.0,
                        y: entity.position.1,
                    },
                    rotation: entity.rotation,
                    scale: (entity.scale.0, entity.scale.1),
                },
            );
        }
        Self {
            entities: entities_map,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs_f64(),
        }
    }

    /// Restaura snapshot a entidades
    pub fn restore_to_entities(&self, entities: &mut [crate::Entity]) {
        // Implementación futura
    }
}

/// Entidad en el juego
#[derive(Debug, Clone)]
pub struct Entity {
    pub id: String,
    pub position: (f32, f32),
    pub rotation: f32,
    pub scale: (f32, f32),
    pub velocity: (f32, f32),
    pub velocity_y: f32,
    pub is_active: bool,
    pub name: String,
    pub component_type: String,
    pub script_path: Option<String>,
}

impl Entity {
    /// Crea una nueva entidad
    pub fn new(id: String, position: (f32, f32)) -> Self {
        Self {
            id,
            position,
            rotation: 0.0,
            scale: (1.0, 1.0),
            velocity: (0.0, 0.0),
            velocity_y: 0.0,
            is_active: true,
            name: "Unnamed".to_string(),
            component_type: "Transform".to_string(),
            script_path: None,
        }
    }
    
    /// Crea entidad con nombre
    pub fn with_name(id: String, position: (f32, f32), name: String) -> Self {
        Self {
            id,
            position,
            rotation: 0.0,
            scale: (1.0, 1.0),
            velocity: (0.0, 0.0),
            velocity_y: 0.0,
            is_active: true,
            name,
            component_type: "Transform".to_string(),
            script_path: None,
        }
    }
    
    /// Actualiza posición con física simple
    pub fn update_physics(&mut self, delta: f32, gravity: f32) {
        if !self.is_active {
            return;
        }
        
        // Aplicar velocidad horizontal
        self.position.0 += self.velocity.0 * delta;
        
        // Aplicar gravedad y velocidad vertical
        self.velocity_y += gravity * delta;
        self.position.1 += self.velocity_y * delta;
        
        // Reducir velocidad vertical (air resistance)
        self.velocity_y *= 0.99;
    }
    
    /// Cambia estado activo/inactivo
    pub fn set_active(&mut self, active: bool) {
        self.is_active = active;
    }
}

/// Sesión de Play mejorada
#[derive(Debug)]
pub struct PlaySession {
    pub entities: Vec<Entity>,
    pub snapshot: SceneSnapshot,
    pub snapshot_manager: crate::snapshot_manager::SnapshotManager,
    pub input_capture: crate::input_capture::InputCapture,
    pub physics_enabled: bool,
    pub gravity: f32,
    pub is_running: bool,
    pub last_delta: f32,
    pub last_change_time: Option<Instant>,
    pub change_count: u32,
    pub event_integration: Option<Box<crate::event_play_integration::EventPlayIntegration>>,
    pub audio_manager: crate::audio_system::AudioManager,
    pub dialogue_ui: crate::dialogue_ui::DialogueUI,
}

impl PlaySession {
    /// Crea una nueva sesión de Play mejorada
    pub fn new(entities: &[Entity], event_integration: Option<crate::event_play_integration::EventPlayIntegration>) -> Self {
        Self {
            entities: entities.to_vec(),
            snapshot: SceneSnapshot::from_entities(&entities),
            snapshot_manager: crate::snapshot_manager::SnapshotManager::new(),
            input_capture: crate::input_capture::InputCapture::new(),
            physics_enabled: true,
            gravity: 9.8,
            is_running: false,
            last_delta: 0.0,
            last_change_time: None,
            change_count: 0,
            event_integration,
            audio_manager: crate::audio_system::AudioManager::new(),
            dialogue_ui: crate::dialogue_ui::DialogueUI::new(),
        }
    }
    
    /// Inicia la sesión de Play mejorada
    pub fn start(&mut self, entities: &mut [Entity]) -> Result<(), String> {
        // Tomar snapshot actual
        self.snapshot_manager.take_snapshot(entities);
        
        // Activar físicas
        self.physics_enabled = true;
        
        // Iniciar sesión
        self.is_running = true;
        self.last_delta = 0.0;
        self.last_change_time = Some(Instant::now());
        self.change_count = 0;
        
        // Inicializar integración de eventos si existe
        if let Some(ref mut integration) = self.event_integration {
            integration.start();
        }
        
        println!("[PLAY SESSION] Started with {} entities", entities.len());
        
        Ok(())
    }
    
    /// Detiene la sesión de Play mejorada
    pub fn stop(&mut self, entities: &mut [Entity]) -> Result<(), String> {
        // Restaurar snapshot
        self.snapshot_manager.restore_snapshot(entities);
        
        // Detener sesión
        self.is_running = false;
        self.last_delta = 0.0;
        
        // Detener integración de eventos si existe
        if let Some(ref mut integration) = self.event_integration {
            integration.stop();
        }
        
        println!("[PLAY SESSION] Stopped, changes: {}", self.change_count);
        
        Ok(())
    }
    
    /// Actualiza la sesión con el delta de tiempo mejorado
    pub fn update(&mut self, delta: f32, input: &crate::UserInput) {
        if !self.is_running {
            return;
        }
        
        self.last_delta = delta;
        self.simulate_physics(delta, input);
        
        // Ejecutar eventos de la integración si existe
        if let Some(ref mut integration) = self.event_integration {
            integration.update(delta);
        }
        
        // Incrementar contador de cambios
        self.change_count += 1;
        self.last_change_time = Some(Instant::now());
    }
    
    /// Simula físicas para el delta de tiempo mejorado
    fn simulate_physics(&mut self, delta: f32, input: &crate::UserInput) {
        let mut player_velocity = Vec2::zero();
        
        // Obtener movimiento del jugador
        if input.keys_pressed.contains(&crate::KeyCode::KeyW) {
            player_velocity.y -= 1.0;
        }
        if input.keys_pressed.contains(&crate::KeyCode::KeyS) {
            player_velocity.y += 1.0;
        }
        if input.keys_pressed.contains(&crate::KeyCode::KeyA) {
            player_velocity.x -= 1.0;
        }
        if input.keys_pressed.contains(&crate::KeyCode::KeyD) {
            player_velocity.x += 1.0;
        }
        
        // Normalizar velocidad
        if player_velocity.x.abs() > 1.0 || player_velocity.y.abs() > 1.0 {
            let length = player_velocity.length();
            if length > 0.0 {
                player_velocity = player_velocity.mul(1.0 / length);
            }
        }
        
        // Aplicar a entidades activas
        for entity in &mut self.entities {
            if !entity.is_active {
                continue;
            }
            
            // Aplicar gravedad
            entity.velocity_y += self.gravity * delta;
            
            // Aplicar movimiento horizontal
            entity.position.0 += entity.velocity.0 * delta;
            
            // Aplicar movimiento vertical
            entity.position.1 += entity.velocity_y * delta;
            
            // Limitar velocidad vertical (terminal velocity)
            if entity.velocity_y > 15.0 {
                entity.velocity_y = 15.0;
            }
        }
    }
    
    /// Obtiene movimiento del jugador
    fn get_player_movement(&self) -> Vec2 {
        let mut movement = Vec2::zero();
        
        if self.input_capture.keys_pressed.contains(&crate::KeyCode::KeyW) {
            movement.y -= 1.0;
        }
        if self.input_capture.keys_pressed.contains(&crate::KeyCode::KeyS) {
            movement.y += 1.0;
        }
        if self.input_capture.keys_pressed.contains(&crate::KeyCode::KeyA) {
            movement.x -= 1.0;
        }
        if self.input_capture.keys_pressed.contains(&crate::KeyCode::KeyD) {
            movement.x += 1.0;
        }
        
        movement
    }
    
    /// Simula físicas para el delta de tiempo
    fn simulate_physics(&mut self, delta: f32) {
        let movement = self.get_player_movement();
        
        for entity in &mut self.snapshot.entities.values_mut() {
            // Aplicar movimiento basado en input (WASD)
            if movement.0 != 0.0 || movement.1 != 0.0 {
                entity.position.x += movement.0 * 100.0 * delta;
                entity.position.y += movement.1 * 100.0 * delta;
            }
            
            // Límites del viewport (opcional)
            // entity.position.x = entity.position.x.clamp(0.0, 1000.0);
            // entity.position.y = entity.position.y.clamp(0.0, 1000.0);
        }
    }
    
    /// Obtiene el movimiento del jugador basado en input
    pub fn get_player_movement(&self) -> (f32, f32) {
        self.input_capture.get_movement()
    }
    
    /// Obtiene el estado del mouse
    pub fn get_mouse_state(&self) -> crate::MouseState {
        self.input_capture.mouse.clone()
    }
    
    /// Obtiene el snapshot actual
    pub fn get_snapshot(&self) -> &SceneSnapshot {
        &self.snapshot
    }
    
    /// Obtiene el último delta de tiempo
    pub fn get_delta(&self) -> f32 {
        self.last_delta
    }

    /// Inicializa integración de eventos con EventNodeManager
    pub fn init_event_integration(&mut self, event_manager: crate::event_node_manager::EventNodeManager) {
        use crate::event_play_integration::EventPlayIntegration;
        
        let integration = EventPlayIntegration::new();
        integration.init_with_play_session(self.clone());
        
        self.event_integration = Some(Box::new(integration));
        println!("[PLAY SESSION] Event integration initialized with {} nodes", 
            event_manager.nodes.len());
    }
    
    /// Verifica si la sesión está activa
    pub fn is_active(&self) -> bool {
        self.is_running
    }
    
    /// Obtiene referencia al AudioManager
    pub fn audio_manager(&self) -> &crate::audio_system::AudioManager {
        &self.audio_manager
    }
    
    /// Obtiene referencia mutante al AudioManager
    pub fn audio_manager_mut(&mut self) -> &mut crate::audio_system::AudioManager {
        &mut self.audio_manager
    }
    
    /// Obtiene referencia al DialogueUI
    pub fn dialogue_ui(&self) -> &crate::dialogue_ui::DialogueUI {
        &self.dialogue_ui
    }
    
    /// Obtiene referencia mutante al DialogueUI
    pub fn dialogue_ui_mut(&mut self) -> &mut crate::dialogue_ui::DialogueUI {
        &mut self.dialogue_ui
    }
    
    /// Mostrar diálogo en el juego
    pub fn show_dialogue(&mut self, speaker: String, text: String) {
        self.dialogue_ui.show(speaker, text);
        println!("[PLAY SESSION] Dialogo mostrado: {}", text);
    }
    
    /// Ocultar diálogo actual
    pub fn hide_dialogue(&mut self) {
        self.dialogue_ui.hide();
        println!("[PLAY SESSION] Dialogo oculto");
    }
    
    /// Mostrar diálogo de entidad
    pub fn show_entity_dialogue(&mut self, entity: &crate::Entity, text: String) {
        let speaker = entity.name.clone();
        self.show_dialogue(speaker, text);
    }
    
    /// Actualizar diálogo UI con delta de tiempo
    pub fn update_dialogue_ui(&mut self, delta: f32) {
        self.dialogue_ui.update(delta);
    }
    
    /// Verificar si hay diálogo activo
    pub fn has_active_dialogue(&self) -> bool {
        self.dialogue_ui.state == crate::dialogue_ui::DialogueState::Visible
    }
    
    /// Obtener/establecer variable de diálogo
    pub fn dialogue_variable(&mut self, key: &str, value: String) -> Option<String> {
        self.dialogue_ui.manager.set_variable(key, value)
    }
    
    /// Obtener valor de variable de diálogo
    pub fn get_dialogue_variable(&self, key: &str) -> Option<String> {
        self.dialogue_ui.manager.get_variable(key)
    }
    
    /// Obtener todas las variables de diálogo
    pub fn get_all_dialogue_variables(&self) -> &HashMap<String, String> {
        &self.dialogue_ui.manager.variables
    }
    
    /// Limpiar todas las variables de diálogo
    pub fn clear_dialogue_variables(&mut self) {
        self.dialogue_ui.manager.variables.clear();
        println!("[PLAY SESSION] Variables de diálogo limpiadas");
    }
    
    /// Seleccionar opción de diálogo
    pub fn select_dialogue_option(&mut self, choice_index: usize) {
        if let Some(ref mut ui) = self.dialogue_ui {
            ui.select_option(choice_index);
        }
        println!("[PLAY SESSION] Opción de diálogo seleccionada: {}", choice_index);
    }
    
    /// Verificar si hay opciones de diálogo disponibles
    pub fn has_dialogue_options(&self) -> bool {
        self.dialogue_ui.manager.has_options()
    }
    
    /// Obtener número de opciones disponibles
    pub fn get_option_count(&self) -> usize {
        self.dialogue_ui.manager.options.len()
    }
    
    /// Obtener texto de opción por índice
    pub fn get_option_text(&self, index: usize) -> Option<String> {
        self.dialogue_ui.manager.options.get(index).map(|o| o.text.clone())
    }
    
    /// Ejecutar evento al seleccionar opción
    pub fn execute_dialogue_option(&mut self, choice_index: usize) {
        if let Some(ref mut ui) = self.dialogue_ui {
            let selected = ui.manager.options.get(choice_index);
            if let Some(choice) = selected {
                // Ejecutar evento si existe
                if let Some(event_id) = &choice.event_id {
                    println!("[PLAY SESSION] Ejecutando evento: {}", event_id);
                }
                
                // Cambiar variable si existe
                if let Some(variable_key) = &choice.variable_key {
                    if let Some(variable_value) = &choice.variable_value {
                        self.dialogue_variable(variable_key, variable_value.clone());
                    }
                }
            }
        }
    }
}

impl Default for PlaySession {
    fn default() -> Self {
        Self::new(&[])
    }
}

/// Alias para compatibilidad con código existente
pub type EventGraph = crate::event_node_manager::EventNodeManager;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::input_capture::{UserInput, KeyCode};

    #[test]
    fn test_play_session_new() {
        let entities = vec![
            Entity::new("player1".to_string(), (0.0, 0.0)),
            Entity::new("player2".to_string(), (100.0, 100.0)),
        ];
        
        let session = PlaySession::new(&entities);
        
        assert_eq!(session.snapshot.entities.len(), 2);
        assert!(!session.is_active());
        assert_eq!(session.get_delta(), 0.0);
    }

    #[test]
    fn test_play_session_start() {
        let entities = vec![
            Entity::new("player1".to_string(), (0.0, 0.0)),
        ];
        
        let mut session = PlaySession::new(&entities);
        let mut entities_copy = entities.clone();
        
        assert!(!session.is_active());
        
        session.start(&mut entities_copy).unwrap();
        
        assert!(session.is_active());
        assert_eq!(session.get_delta(), 0.0);
    }

    #[test]
    fn test_play_session_stop() {
        let entities = vec![
            Entity::new("player1".to_string(), (0.0, 0.0)),
        ];
        
        let mut session = PlaySession::new(&entities);
        let mut entities_copy = entities.clone();
        
        session.start(&mut entities_copy).unwrap();
        session.stop(&mut entities_copy).unwrap();
        
        assert!(!session.is_active());
        assert_eq!(session.get_delta(), 0.0);
    }

    #[test]
    fn test_play_session_get_movement() {
        let entities = vec![
            Entity::new("player1".to_string(), (0.0, 0.0)),
        ];
        
        let session = PlaySession::new(&entities);
        
        let movement = session.get_player_movement();
        
        assert_eq!(movement, (0.0, 0.0));
    }

    #[test]
    fn test_play_session_get_mouse_state() {
        let entities = vec![
            Entity::new("player1".to_string(), (0.0, 0.0)),
        ];
        
        let session = PlaySession::new(&entities);
        
        let mouse_state = session.get_mouse_state();
        
        assert_eq!(mouse_state.position, (0.0, 0.0));
        assert!(!mouse_state.left_button);
        assert!(!mouse_state.right_button);
    }

    #[test]
    fn test_play_session_get_snapshot() {
        let entities = vec![
            Entity::new("player1".to_string(), (0.0, 0.0)),
        ];
        
        let session = PlaySession::new(&entities);
        
        let snapshot = session.get_snapshot();
        
        assert_eq!(snapshot.entities.len(), 1);
    }

    #[test]
    fn test_play_session_delta() {
        let entities = vec![
            Entity::new("player1".to_string(), (0.0, 0.0)),
        ];
        
        let mut session = PlaySession::new(&entities);
        let mut entities_copy = entities.clone();
        
        session.start(&mut entities_copy).unwrap();
        
        let mut input = UserInput::default();
        input.keyboard.insert(KeyCode::W, true);
        
        session.update(0.016, &input);
        
        assert_eq!(session.get_delta(), 0.016);
    }

    #[test]
    fn test_play_session_multiple_entities() {
        let entities = vec![
            Entity::new("player1".to_string(), (0.0, 0.0)),
            Entity::new("player2".to_string(), (100.0, 100.0)),
            Entity::new("enemy1".to_string(), (200.0, 200.0)),
        ];
        
        let session = PlaySession::new(&entities);
        
        let snapshot = session.get_snapshot();
        
        assert_eq!(snapshot.entities.len(), 3);
    }

    #[test]
    fn test_play_session_default() {
        let session = PlaySession::default();
        
        assert!(!session.is_active());
        assert_eq!(session.get_delta(), 0.0);
        assert_eq!(session.get_player_movement(), (0.0, 0.0));
    }

    #[test]
    fn test_play_session_audio_manager_integration() {
        let entities = vec![
            Entity::new("player1".to_string(), (0.0, 0.0)),
        ];
        
        let session = PlaySession::new(&entities);
        
        // Verificar que audio_manager está integrado
        let audio_mgr = session.audio_manager();
        assert!(audio_mgr.is_some());
        
        // Verificar que dialogue_ui está integrado
        let dialogue_ui = session.dialogue_ui();
        assert!(dialogue_ui.is_some());
    }

    #[test]
    fn test_play_session_audio_manager_mut() {
        let entities = vec![
            Entity::new("player1".to_string(), (0.0, 0.0)),
        ];
        
        let mut session = PlaySession::new(&entities);
        
        // Verificar que podemos acceder mutante al audio_manager
        let audio_mgr = session.audio_manager_mut();
        assert!(audio_mgr.is_some());
    }

    #[test]
    fn test_play_session_dialogue_ui_integration() {
        let entities = vec![
            Entity::new("player1".to_string(), (0.0, 0.0)),
        ];
        
        let session = PlaySession::new(&entities);
        
        // Verificar que dialogue_ui está integrado
        let dialogue_ui = session.dialogue_ui();
        assert!(dialogue_ui.is_some());
        
        // Verificar que dialogue_ui está inicializado
        assert_eq!(dialogue_ui.state, crate::dialogue_ui::DialogueState::Hidden);
    }

    #[test]
    fn test_play_session_show_dialogue() {
        let entities = vec![
            Entity::new("player1".to_string(), (0.0, 0.0)),
        ];
        
        let mut session = PlaySession::new(&entities);
        
        // Mostrar diálogo
        session.show_dialogue("Personaje1".to_string(), "¡Hola!".to_string());
        
        // Verificar que el diálogo fue mostrado
        assert_eq!(session.dialogue_ui().state, crate::dialogue_ui::DialogueState::Showing);
    }

    #[test]
    fn test_play_session_hide_dialogue() {
        let entities = vec![
            Entity::new("player1".to_string(), (0.0, 0.0)),
        ];
        
        let mut session = PlaySession::new(&entities);
        
        // Mostrar y ocultar diálogo
        session.show_dialogue("Personaje1".to_string(), "¡Hola!".to_string());
        session.hide_dialogue();
        
        // Verificar que el diálogo fue ocultado
        assert_eq!(session.dialogue_ui().state, crate::dialogue_ui::DialogueState::Hidden);
    }

    #[test]
    fn test_play_session_show_entity_dialogue() {
        let entities = vec![
            Entity::with_name("player1".to_string(), (0.0, 0.0), "Jugador".to_string()),
        ];
        
        let mut session = PlaySession::new(&entities);
        
        // Mostrar diálogo de entidad
        let entity = &entities[0];
        session.show_entity_dialogue(entity, "¡Hola!".to_string());
        
        // Verificar que el diálogo fue mostrado
        assert_eq!(session.dialogue_ui().state, crate::dialogue_ui::DialogueState::Showing);
    }

    #[test]
    fn test_play_session_dialogue_variables() {
        let entities = vec![
            Entity::new("player1".to_string(), (0.0, 0.0)),
        ];
        
        let mut session = PlaySession::new(&entities);
        
        // Establecer variable
        let value = session.dialogue_variable("has_sword".to_string(), "true".to_string());
        
        // Verificar que la variable fue establecida
        assert_eq!(value, Some("true".to_string()));
        
        // Obtener variable
        let retrieved = session.get_dialogue_variable("has_sword");
        assert_eq!(retrieved, Some("true".to_string()));
    }

    #[test]
    fn test_play_session_dialogue_has_active() {
        let entities = vec![
            Entity::new("player1".to_string(), (0.0, 0.0)),
        ];
        
        let session = PlaySession::new(&entities);
        
        // Verificar que no hay diálogo activo inicialmente
        assert!(!session.has_active_dialogue());
        
        // Mostrar diálogo y verificar que está activo
        session.show_dialogue("Personaje1".to_string(), "¡Hola!".to_string());
        assert!(session.has_active_dialogue());
        
        // Ocultar diálogo y verificar que no está activo
        session.hide_dialogue();
        assert!(!session.has_active_dialogue());
    }
}
