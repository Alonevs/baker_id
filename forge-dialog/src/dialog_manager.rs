use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use serde::{Serialize, Deserialize};

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
        self.current_dialog = Some(dialog_id.to_string());
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
        let dialogs = self.dialogs.read().unwrap();
        let dialog_id = self.current_dialog.as_deref()?;
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
        let dialog_id = self.current_dialog.as_deref().unwrap_or("");
        let dialogs = self.dialogs.read().unwrap();
        
        let dialog = if let Some(d) = dialogs.get(dialog_id) { d } else { &HashMap::new() };
        let options = if let Some(o) = dialog.get("options") { o } else { &HashMap::new() };
        let options_list = if let Some(list) = options.get("list") { list } else { &String::new() };
        let options_list_str = options_list.to_string();
        
        options_list_str.split(",").map(|s| s.to_string()).collect()
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
        if let Some(dialog_id) = self.current_dialog.clone() {
            let dialogs = self.dialogs.read().unwrap();
            let dialog = if let Some(d) = dialogs.get(&dialog_id) { d } else { &HashMap::new() };
            let options = if let Some(o) = dialog.get("options") { o } else { &HashMap::new() };
            let options_list = if let Some(list) = options.get("list") { list } else { &String::new() };
            let options_list_str = options_list.to_string();
            let options_vec: Vec<&str> = options_list_str.split(",").collect();
            
            if self.current_option_idx + 1 < options_vec.len() {
                self.current_option_idx += 1;
                return true;
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
        if let Some(dialog_id) = self.current_dialog.clone() {
            let dialogs = self.dialogs.read().unwrap();
            let dialog = if let Some(d) = dialogs.get(&dialog_id) { d } else { &HashMap::new() };
            let options = if let Some(o) = dialog.get("options") { o } else { &HashMap::new() };
            let options_list = if let Some(list) = options.get("list") { list } else { &String::new() };
            let options_list_str = options_list.to_string();
            let options_vec: Vec<&str> = options_list_str.split(",").collect();
            
            if option_idx < options_vec.len() {
                let selected = options_vec.get(option_idx).map(|s| s.to_string());
                return selected;
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
