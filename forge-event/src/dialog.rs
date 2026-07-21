use std::sync::Arc;
use crate::dialog_manager::DialogManager;
use crate::event_system::EventSystem;

/// Sistema de diálogos integrado con eventos
pub struct DialogSystem {
    pub dialog_manager: Arc<DialogManager>,
    pub event_system: Arc<EventSystem>,
    pub is_open: bool,
}

impl DialogSystem {
    /// Crear nuevo sistema de diálogos
    pub fn new() -> Self {
        Self {
            dialog_manager: Arc::new(DialogManager::default()),
            event_system: Arc::new(EventSystem::default()),
            is_open: false,
        }
    }

    /// Inicializar sistema
    pub fn init(&self) {
        // Sistema listo para usar
    }

    /// Abrir diálogo
    pub fn open_dialog(&mut self, dialog_id: &str) {
        if let Some(manager) = Arc::get_mut(&mut self.dialog_manager) {
            manager.open(dialog_id);
        }
        self.is_open = true;
    }

    /// Cerrar diálogo
    pub fn close_dialog(&mut self) {
        if let Some(manager) = Arc::get_mut(&mut self.dialog_manager) {
            manager.close();
        }
        self.is_open = false;
    }

    /// Obtener texto actual del diálogo
    pub fn get_current_text(&self) -> String {
        self.dialog_manager.get_current_text().unwrap_or_else(|| "No dialogue".to_string())
    }

    /// Obtener opciones del diálogo
    pub fn get_options(&self) -> Vec<String> {
        self.dialog_manager.get_current_options()
    }

    /// Seleccionar opción
    pub fn select_option(&mut self, option_idx: usize) {
        if let Some(manager) = Arc::get_mut(&mut self.dialog_manager) {
            if let Some(option) = manager.select_option(option_idx) {
                // Ejecutar acción de la opción
                self.execute_dialog_action(&option);
            }
        }
    }

    /// Avanzar diálogo
    pub fn next(&mut self) -> bool {
        if let Some(manager) = Arc::get_mut(&mut self.dialog_manager) {
            manager.next_option()
        } else {
            false
        }
    }

    /// Ejecutar acción de diálogo
    fn execute_dialog_action(&mut self, action: &str) {
        match action {
            "Close" => {
                self.close_dialog();
            }
            "PlayEvent" => {
                if let Some(event_id) = action.split(',').nth(1) {
                    let _actions = self.event_system.play_event(event_id);
                    if let Some(_actions) = _actions {
                        for _action in _actions {
                            // Ejecutar acción
                            println!("Executing action: {}", _action);
                        }
                    }
                }
            }
            _ => {
                println!("Unknown action: {}", action);
            }
        }
    }

    /// Verificar si hay diálogo abierto
    pub fn is_open(&self) -> bool {
        self.is_open
    }

    /// Obtener ID del diálogo actual
    pub fn get_current_dialog_id(&self) -> Option<&str> {
        self.dialog_manager.get_current_dialog_id()
    }
}
