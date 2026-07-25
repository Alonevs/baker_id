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
    pub event_graph: Option<crate::event_node_manager::EventNodeManager>,
}

impl PlaySession {
    /// Crea una nueva sesión de Play mejorada
    pub fn new(entities: &[Entity], event_graph: Option<crate::event_node_manager::EventNodeManager>) -> Self {
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
            event_graph,
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
        
        println!("[PLAY SESSION] Stopped, changes: {}", self.change_count);
        
        Ok(())
    }
        
        // Desactivar físicas
        self.physics_enabled = false;
        
        // Detener sesión
        self.is_running = false;
        self.last_delta = 0.0;
        
        Ok(())
    }
    
    /// Actualiza la sesión con el delta de tiempo mejorado
    pub fn update(&mut self, delta: f32, input: &crate::UserInput) {
        if !self.is_running {
            return;
        }
        
        self.last_delta = delta;
        self.simulate_physics(delta, input);
        
        // Ejecutar eventos del grafo si existe
        if let Some(ref mut graph) = self.event_graph {
            graph.execute_all();
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
    
    /// Verifica si la sesión está activa
    pub fn is_active(&self) -> bool {
        self.is_running
    }
}

impl Default for PlaySession {
    fn default() -> Self {
        Self::new(&[])
    }
}

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
}
