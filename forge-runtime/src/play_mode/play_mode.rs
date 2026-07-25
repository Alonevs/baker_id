//! # Play Mode Runtime
//! 
//! Integración de Play Mode con el runtime.

/// Estado del Play Mode
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PlayModeState {
    /// Detenido
    Stopped,
    /// Reproduciendo
    Playing,
    /// Pausado
    Paused,
}

/// Play Mode Runtime
#[derive(Debug)]
pub struct PlayMode {
    /// Estado actual
    pub state: PlayModeState,
    /// Delta tiempo
    pub delta: f32,
    /// Entidades activas
    pub entities: Vec<Entity>,
}

impl Default for PlayMode {
    fn default() -> Self {
        Self {
            state: PlayModeState::Stopped,
            delta: 0.0,
            entities: Vec::new(),
        }
    }
}

impl PlayMode {
    /// Crea un nuevo Play Mode
    pub fn new() -> Self {
        Self::default()
    }

    /// Inicia Play Mode
    pub fn start(&mut self) {
        self.state = PlayModeState::Playing;
    }

    /// Pausa Play Mode
    pub fn pause(&mut self) {
        self.state = PlayModeState::Paused;
    }

    /// Detiene Play Mode
    pub fn stop(&mut self) {
        self.state = PlayModeState::Stopped;
    }

    /// Actualiza Play Mode
    pub fn update(&mut self, delta: f32) {
        self.delta = delta;
        
        match self.state {
            PlayModeState::Playing => {
                // Simular físicas
                self.simulate_physics(delta);
            }
            PlayModeState::Paused => {
                // No hacer nada
            }
            PlayModeState::Stopped => {
                // No hacer nada
            }
        }
    }

    /// Simula físicas para el delta de tiempo
    fn simulate_physics(&mut self, delta: f32) {
        // Implementación futura:
        // - Mover entidades según input
        // - Aplicar colisiones
        // - Actualizar físicas
    }

    /// Obtiene el estado actual
    pub fn get_state(&self) -> PlayModeState {
        self.state
    }

    /// Obtiene el delta de tiempo
    pub fn get_delta(&self) -> f32 {
        self.delta
    }
}

/// Entidad básica
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
