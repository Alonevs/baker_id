use std::collections::HashMap;
use std::sync::{Arc, RwLock};


/// Sistema de localización para diálogos y eventos
pub struct Localization {
    pub current_language: String,
    pub available_languages: Vec<String>,
    pub translations: Arc<RwLock<HashMap<String, HashMap<String, HashMap<String, String>>>>>,
}

impl Default for Localization {
    fn default() -> Self {
        Self {
            current_language: "es".to_string(),
            available_languages: vec!["es".to_string(), "en".to_string()],
            translations: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

impl Localization {
    /// Cambiar idioma actual
    pub fn set_language(&mut self, language: &str) {
        self.current_language = language.to_string();
    }

    /// Obtener idioma actual
    pub fn get_language(&self) -> &str {
        &self.current_language
    }

    /// Obtener lista de idiomas disponibles
    pub fn get_available_languages(&self) -> &Vec<String> {
        &self.available_languages
    }

    /// Cargar traducciones desde Excel/CSV
    pub fn load_translations(&self, _path: &str) -> Result<(), String> {
        // Aquí cargarías las traducciones
        // Por ahora, usaríamos el mismo parser de Excel
        Ok(())
    }

    /// Obtener texto traducido para una clave específica
    pub fn get(&self, key: &str) -> Option<String> {
        let translations = self.translations.read().unwrap();
        let lang_translations = translations.get(key)?;
        let lang = &self.current_language;
        
        lang_translations.get(lang).and_then(|t| t.get(lang).and_then(|s: &String| Some(s.clone())))
    }

    /// Guardar traducciones en Excel/CSV
    pub fn save_translations(&self, path: &str) -> Result<(), String> {
        let translations = self.translations.read().unwrap();
        let mut content = String::new();
        
        for (dialog_id, lang_data) in translations.iter() {
            for (lang, texts) in lang_data.iter() {
                for (key, text) in texts.iter() {
                    content.push_str(&format!("{},{},{},{}\n", dialog_id, lang, key, text));
                }
            }
        }
        
        std::fs::write(path, &content).map_err(|e| e.to_string())?;
        Ok(())
    }
}

/// Sistema de mensajes para logging y notificaciones
pub struct MessageSystem {
    pub messages: Arc<RwLock<Vec<(String, String, String)>>>, // (timestamp, level, message)
    pub message_level: String,
}

impl Default for MessageSystem {
    fn default() -> Self {
        Self {
            messages: Arc::new(RwLock::new(Vec::new())),
            message_level: "info".to_string(),
        }
    }
}

impl MessageSystem {
    /// Añadir mensaje
    pub fn add(&self, level: &str, message: &str) {
        let timestamp = chrono::Utc::now().format("%H:%M:%S%.f").to_string();
        let mut messages = self.messages.write().unwrap();
        messages.push((timestamp, level.to_string(), message.to_string()));
    }

    /// Obtener último mensaje
    pub fn last_message(&self) -> Option<String> {
        let messages = self.messages.read().unwrap();
        messages.last().map(|m| format!("{} - {} - {}", m.0, m.1, m.2))
    }

    /// Limpiar mensajes
    pub fn clear(&self) {
        let mut messages = self.messages.write().unwrap();
        messages.clear();
    }

    /// Obtener mensajes por nivel
    pub fn get_messages_by_level(&self, level: &str) -> Vec<(String, String, String)> {
        self.messages.read().unwrap()
            .iter()
            .filter(|(_, l, _)| l == level)
            .cloned()
            .collect()
    }
}
