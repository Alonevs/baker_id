//! # Toolbar
//! 
//! Barra de herramientas con botones Play/Stop y controles de Play Mode.

use crate::play_mode::{PlayMode, PlayModeState, Entity};

/// Barra de herramientas
#[derive(Debug, Clone)]
pub struct Toolbar {
    /// Estado del Play Mode
    pub play_mode_state: PlayModeState,
    /// Play Mode
    pub play_mode: Option<PlayMode>,
    /// Botones de la toolbar
    pub buttons: ToolbarButtons,
}

impl Default for Toolbar {
    fn default() -> Self {
        Self {
            play_mode_state: PlayModeState::Stopped,
            play_mode: None,
            buttons: ToolbarButtons::new(),
        }
    }
}

impl Toolbar {
    /// Crea una nueva toolbar
    pub fn new() -> Self {
        Self::default()
    }

    /// Inicia Play Mode
    pub fn start_play(&mut self, entities: &[Entity]) {
        self.play_mode = Some(PlayMode::new());
        self.play_mode_state = PlayModeState::Playing;
    }

    /// Detiene Play Mode
    pub fn stop_play(&mut self) {
        self.play_mode = None;
        self.play_mode_state = PlayModeState::Stopped;
    }

    /// Actualiza la toolbar
    pub fn update(&mut self, delta: f32) {
        if let Some(ref mut mode) = self.play_mode {
            mode.update(delta);
        }
    }

    /// Obtiene el estado actual
    pub fn get_state(&self) -> PlayModeState {
        self.play_mode_state
    }

    /// Obtiene el Play Mode
    pub fn get_play_mode(&self) -> Option<&PlayMode> {
        self.play_mode.as_ref()
    }

    /// Obtiene el Play Mode mutado
    pub fn get_play_mode_mut(&mut self) -> Option<&mut PlayMode> {
        self.play_mode.as_mut()
    }
}

/// Botones de la toolbar
#[derive(Debug, Clone)]
pub struct ToolbarButtons {
    /// Botón Play
    pub play_button: PlayButton,
    /// Botón Stop
    pub stop_button: StopButton,
    /// Botón Pause
    pub pause_button: PauseButton,
}

impl ToolbarButtons {
    /// Crea nuevos botones
    pub fn new() -> Self {
        Self {
            play_button: PlayButton::new(),
            stop_button: StopButton::new(),
            pause_button: PauseButton::new(),
        }
    }

    /// Verifica si se debe mostrar botón Play
    pub fn should_show_play(&self) -> bool {
        self.play_button.visible
    }

    /// Verifica si se debe mostrar botón Stop
    pub fn should_show_stop(&self) -> bool {
        self.stop_button.visible
    }
}

/// Botón Play
#[derive(Debug, Clone)]
pub struct PlayButton {
    /// Visible
    pub visible: bool,
    /// Estado del click
    pub is_pressed: bool,
}

impl PlayButton {
    /// Crea un nuevo botón Play
    pub fn new() -> Self {
        Self {
            visible: true,
            is_pressed: false,
        }
    }
}

/// Botón Stop
#[derive(Debug, Clone)]
pub struct StopButton {
    /// Visible
    pub visible: bool,
    /// Estado del click
    pub is_pressed: bool,
}

impl StopButton {
    /// Crea un nuevo botón Stop
    pub fn new() -> Self {
        Self {
            visible: true,
            is_pressed: false,
        }
    }
}

/// Botón Pause
#[derive(Debug, Clone)]
pub struct PauseButton {
    /// Visible
    pub visible: bool,
    /// Estado del click
    pub is_pressed: bool,
}

impl PauseButton {
    /// Crea un nuevo botón Pause
    pub fn new() -> Self {
        Self {
            visible: true,
            is_pressed: false,
        }
    }
}
