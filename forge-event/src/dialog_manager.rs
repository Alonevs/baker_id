use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use serde::{Deserialize, Serialize};

/// Sistema de gestión de diálogos con soporte multilingüe
pub struct DialogManager {
    pub dialogs: Arc<RwLock<HashMap<String, HashMap<String, HashMap<String, String>>>>>,
    pub current_dialog: Option<String>,
    pub current_option_idx: usize,
    pub is_open: bool,
}

impl Default for DialogManager {
    fn default() -> Self {
        Self {
            dialogs: Arc::new(RwLock::new(HashMap::new())),
            current_dialog: None,
            current_option_idx: 0,
            is_open: false,
        }
    }
}

impl DialogManager {
    /// Abrir diálogo
    pub fn open(&mut self, dialog_id: &str) {
        if let Some(ref mut current) = self.current_dialog {
            *current = dialog_id.to_string();
        } else {
            self.current_dialog = Some(dialog_id.to_string());
        }
        self.is_open = true;
        self.current_option_idx = 0;
    }

    /// Cerrar diálogo
    pub fn close(&mut self) {
        self.current_dialog = None;
        self.is_open = false;
    }

    /// Obtener texto actual del diálogo
    pub fn get_current_text(&self) -> Option<String> {
        let dialog_id = self.current_dialog.as_ref()?;
        let dialogs = self.dialogs.read().unwrap();
        let dialog = dialogs.get(dialog_id)?;
        
        let texts = dialog.get("texts")?;
        let current_lang = self.get_current_language();
        
        let texts_list = texts.get(current_lang)?;
        for text in texts_list.split(",") {
            if let Some(first) = text.strip_prefix('[').and_then(|s| s.strip_suffix(']')) {
                if !first.is_empty() {
                    return Some(first.to_string());
                }
            }
        }
        None
    }

    /// Obtener opciones actuales
    pub fn get_current_options(&self) -> Vec<String> {
        let dialog_id = self.current_dialog.as_ref();
        let dialogs = self.dialogs.read().unwrap();
        
        if let Some(id) = dialog_id {
            let _id: &str = id;
            if let Some(dialog) = dialogs.get(_id) {
                let _options = dialog.get("options");
                let _options_list = _options.and_then(|opts| opts.get("list"));
                if let (Some(_options), Some(_options_list)) = (_options, _options_list) {
                    return _options_list.split(",").map(|s| s.to_string()).collect();
                }
            }
        }
        Vec::new()
    }

    /// Obtener idioma actual
    fn get_current_language(&self) -> &'static str {
        // Por defecto español, cambiar desde UI
        "es"
    }

    /// Cambiar idioma
    pub fn set_language(&mut self, _language: &str) {
        // El idioma se usará para obtener textos
    }

    /// Avanzar a siguiente opción
    pub fn next_option(&mut self) -> bool {
        if let Some(ref dialog_id) = self.current_dialog {
            let dialogs = self.dialogs.read().unwrap();
            let _dialog_id: &str = dialog_id;
            let dialog = dialogs.get(_dialog_id);
            let options = dialog.and_then(|d| d.get("options"));
            let options_list = options.and_then(|opts| opts.get("list"));
            let options_vec = options_list.map(|list| list.split(",").collect::<Vec<_>>());
            
            if let Some(options_vec) = options_vec {
                if self.current_option_idx + 1 < options_vec.len() {
                    self.current_option_idx += 1;
                    return true;
                }
            }
        }
        false
    }

    /// Volver a opción anterior
    pub fn prev_option(&mut self) -> bool {
        if self.current_option_idx > 0 {
            self.current_option_idx -= 1;
            true
        } else {
            false
        }
    }

    /// Seleccionar opción
    pub fn select_option(&mut self, option_idx: usize) -> Option<String> {
        if let Some(ref dialog_id) = self.current_dialog {
            let dialogs = self.dialogs.read().unwrap();
            let _dialog_id: &str = dialog_id;
            let dialog = dialogs.get(_dialog_id);
            let options = dialog.and_then(|d| d.get("options"));
            let options_list = options.and_then(|opts| opts.get("list"));
            let options_vec = options_list.map(|list| list.split(",").collect::<Vec<_>>());
            
            if let Some(options_vec) = options_vec {
                if option_idx < options_vec.len() {
                    let selected = options_vec.get(option_idx).map(|s| s.to_string());
                    return selected;
                }
            }
        }
        None
    }

    /// Obtener ID del diálogo actual
    pub fn get_current_dialog_id(&self) -> Option<&str> {
        self.current_dialog.as_deref()
    }
}

/// Acción que se ejecuta al seleccionar una opción de diálogo
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DialogAction {
    /// Cerrar diálogo
    Close,
    /// Reproducir evento
    PlayEvent(String),
    /// Reproducir animación
    PlayAnimation(String),
    /// Reproducir sonido
    PlaySound(String),
    /// Ejecutar código personalizado
    ExecuteScript(String),
}

impl DialogAction {
    /// Ejecutar acción
    pub fn execute(&self) {
        match self {
            DialogAction::Close => {
                // Cerrar diálogo
            }
            DialogAction::PlayEvent(_event_id) => {
                // Reproducir evento
            }
            DialogAction::PlayAnimation(_anim_name) => {
                // Reproducir animación
            }
            DialogAction::PlaySound(_sound_name) => {
                // Reproducir sonido
            }
            DialogAction::ExecuteScript(_script) => {
                // Ejecutar script
            }
        }
    }
}
