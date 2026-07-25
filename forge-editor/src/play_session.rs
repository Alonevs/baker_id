//! # Play Mode System
//! 
//! Sistema para ejecutar juegos en modo Play con:
//! - Snapshot de posiciones
//! - Simulación de físicas
//! - Captura de input del usuario
//! - Restauración automática al Stop

use std::collections::HashMap;

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
}

impl Entity {
    /// Crea una nueva entidad
    pub fn new(id: String, position: (f32, f32)) -> Self {
        Self {
            id,
            position,
            rotation: 0.0,
            scale: (1.0, 1.0),
        }
    }
}

/// Sesión de Play
#[derive(Debug)]
pub struct PlaySession {
    pub snapshot: SceneSnapshot,
    pub snapshot_manager: crate::snapshot_manager::SnapshotManager,
    pub input_capture: crate::input_capture::InputCapture,
    pub physics_enabled: bool,
    pub is_running: bool,
    pub last_delta: f32,
}

impl PlaySession {
    /// Crea una nueva sesión de Play
    pub fn new(entities: &[Entity]) -> Self {
        Self {
            snapshot: SceneSnapshot::from_entities(entities),
            snapshot_manager: crate::snapshot_manager::SnapshotManager::new(),
            input_capture: crate::input_capture::InputCapture::new(),
            physics_enabled: true,
            is_running: false,
            last_delta: 0.0,
        }
    }
  
    /// Inicia la sesión de Play
    pub fn start(&mut self, entities: &mut [Entity]) -> Result<(), String> {
        // Tomar snapshot actual
        self.snapshot_manager.take_snapshot(entities);
        
        // Activar físicas
        self.physics_enabled = true;
        
        // Iniciar sesión
        self.is_running = true;
        self.last_delta = 0.0;
        
        Ok(())
    }
  
    /// Detiene la sesión de Play
    pub fn stop(&mut self, entities: &mut [Entity]) -> Result<(), String> {
        // Restaurar snapshot
        self.snapshot_manager.restore_snapshot(entities);
        
        // Desactivar físicas
        self.physics_enabled = false;
        
        // Detener sesión
        self.is_running = false;
        self.last_delta = 0.0;
        
        Ok(())
    }
  
    /// Actualiza la sesión (físicas + input)
    pub fn update(&mut self, delta: f32, input: &crate::input_capture::UserInput) {
        if !self.is_running {
            return;
        }

        self.last_delta = delta;

        // Actualizar input
        self.input_capture.update(input);

        // Simular físicas (placeholder)
        self.simulate_physics(delta);
    }

    /// Simula físicas para el delta de tiempo
    #[allow(unused_variables)]
    fn simulate_physics(&mut self, delta: f32) {
        // Implementación futura:
        // - Mover entidades según input
        // - Aplicar colisiones
        // - Actualizar físicas
    }

    /// Obtiene el estado de movimiento del jugador
    pub fn get_player_movement(&self) -> (f32, f32) {
        self.input_capture.get_movement()
    }

    /// Obtiene el estado del ratón
    pub fn get_mouse_state(&self) -> &crate::input_capture::MouseState {
        &self.input_capture.mouse
    }

    /// Obtiene el snapshot actual
    pub fn get_snapshot(&self) -> &SceneSnapshot {
        &self.snapshot
    }

    /// Obtiene el snapshot mutado
    pub fn get_snapshot_mut(&mut self) -> &mut SceneSnapshot {
        &mut self.snapshot
    }

    /// Obtiene si la sesión está en ejecución
    pub fn is_active(&self) -> bool {
        self.is_running
    }

    /// Obtiene el delta de tiempo
    pub fn get_delta(&self) -> f32 {
        self.last_delta
    }
}

impl Default for PlaySession {
    fn default() -> Self {
        Self::new(&[])
    }
}
