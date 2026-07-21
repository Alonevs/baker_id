use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use serde::{Deserialize, Serialize};

/// Sistema de eventos con triggers y acciones
pub struct EventSystem {
    pub events: Arc<RwLock<HashMap<String, HashMap<String, HashMap<String, String>>>>>,
    pub active_triggers: Arc<RwLock<HashMap<String, bool>>>,
    pub event_history: Arc<RwLock<Vec<String>>>,
}

impl Default for EventSystem {
    fn default() -> Self {
        Self {
            events: Arc::new(RwLock::new(HashMap::new())),
            active_triggers: Arc::new(RwLock::new(HashMap::new())),
            event_history: Arc::new(RwLock::new(Vec::new())),
        }
    }
}

impl EventSystem {
    /// Inicializar sistema de eventos
    pub fn init(&self) {
        // Sistema listo para usar
    }

    /// Activar trigger para un evento
    pub fn activate_trigger(&self, event_id: &str, trigger: &str) {
        let mut active_triggers = self.active_triggers.write().unwrap();
        let events = self.events.read().unwrap();
        if let Some(event) = events.get(event_id) {
            if let Some(triggers) = event.get("triggers") {
                if let Some(triggers_list) = triggers.get("list") {
                    let triggers_vec: Vec<&str> = triggers_list.split(",").collect();
                    for t in triggers_vec {
                        if t == trigger {
                            active_triggers.insert(event_id.to_string(), true);
                            self.log_event(event_id, trigger);
                            return;
                        }
                    }
                }
            }
        }
    }

    /// Desactivar trigger para un evento
    pub fn deactivate_trigger(&self, event_id: &str, trigger: &str) {
        let mut active_triggers = self.active_triggers.write().unwrap();
        let events = self.events.read().unwrap();
        if let Some(event) = events.get(event_id) {
            if let Some(triggers) = event.get("triggers") {
                if let Some(triggers_list) = triggers.get("list") {
                    let triggers_vec: Vec<&str> = triggers_list.split(",").collect();
                    for t in triggers_vec {
                        if t == trigger {
                            active_triggers.insert(event_id.to_string(), false);
                            return;
                        }
                    }
                }
            }
        }
    }

    /// Verificar si un trigger está activo
    pub fn is_trigger_active(&self, event_id: &str, _trigger: &str) -> bool {
        let active = self.active_triggers.read().unwrap();
        active.get(event_id) == Some(&true)
    }

    /// Verificar si hay algún trigger activo para un evento
    pub fn has_active_triggers(&self, event_id: &str) -> bool {
        let active = self.active_triggers.read().unwrap();
        active.get(event_id) == Some(&true)
    }

    /// Obtener lista de triggers activos para un evento
    pub fn get_active_triggers(&self, event_id: &str) -> Vec<String> {
        let active = self.active_triggers.read().unwrap();
        let is_active = active.get(event_id);
        if let Some(is_active) = is_active {
            if *is_active {
                return vec![event_id.to_string()];
            }
        }
        Vec::new()
    }

    /// Log de evento
    fn log_event(&self, event_id: &str, trigger: &str) {
        let mut history = self.event_history.write().unwrap();
        history.push(format!("Event {} triggered by {}", event_id, trigger));
    }

    /// Reproducir evento (ejecutar acciones)
    pub fn play_event(&self, event_id: &str) -> Option<Vec<String>> {
        let events = self.events.read().unwrap();
        let event_data = events.get(event_id)?;
        
        let actions = event_data.get("actions")?;
        let actions_list = actions.get("list")?;
        let actions_vec: Vec<&str> = actions_list.split(",").collect();
        
        let action_list: Vec<String> = actions_vec.iter().map(|a| a.to_string()).collect();
        return Some(action_list)
    }

    /// Verificar si hay eventos activos que deben ejecutarse
    pub fn check_active_events(&self) -> Vec<String> {
        let event_ids: Vec<String> = self.events.read().unwrap().keys().cloned().collect();
        let mut executed = Vec::new();

        for event_id in event_ids {
            if self.has_active_triggers(&event_id) {
                executed.push(event_id.clone());
                // Aquí podrías ejecutar el evento automáticamente
            }
        }

        executed
    }

    /// Obtener historial de eventos
    pub fn get_event_history(&self) -> Vec<String> {
        self.event_history.read().unwrap().clone()
    }

    /// Limpiar historial
    pub fn clear_history(&self) {
        let mut history = self.event_history.write().unwrap();
        history.clear();
    }
}

/// Tipo de trigger para eventos
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum TriggerType {
    /// Al iniciar el juego
    OnGameStart,
    /// Al cargar escena
    OnSceneLoad(String),
    /// Al tocar objeto
    OnObjectTouch(String),
    /// Al hablar con NPC
    OnNPCDialogue(String),
    /// Al recoger objeto
    OnItemCollect(String),
    /// Al completar misión
    OnMissionComplete(String),
    /// Tiempo
    OnTimeElapsed(i32),
    /// Personalizado
    Custom(String),
}

impl TriggerType {
    /// Crear trigger desde string
    pub fn from_string(s: &str) -> Option<Self> {
        match s {
            "OnGameStart" => Some(TriggerType::OnGameStart),
            "OnSceneLoad" => Some(TriggerType::OnSceneLoad("".to_string())),
            "OnObjectTouch" => Some(TriggerType::OnObjectTouch("".to_string())),
            "OnNPCDialogue" => Some(TriggerType::OnNPCDialogue("".to_string())),
            "OnItemCollect" => Some(TriggerType::OnItemCollect("".to_string())),
            "OnMissionComplete" => Some(TriggerType::OnMissionComplete("".to_string())),
            "OnTimeElapsed" => Some(TriggerType::OnTimeElapsed(0)),
            _ => Some(TriggerType::Custom(s.to_string())),
        }
    }

    /// Obtener nombre del trigger
    pub fn name(&self) -> &str {
        match self {
            TriggerType::OnGameStart => "OnGameStart",
            TriggerType::OnSceneLoad(_) => "OnSceneLoad",
            TriggerType::OnObjectTouch(_) => "OnObjectTouch",
            TriggerType::OnNPCDialogue(_) => "OnNPCDialogue",
            TriggerType::OnItemCollect(_) => "OnItemCollect",
            TriggerType::OnMissionComplete(_) => "OnMissionComplete",
            TriggerType::OnTimeElapsed(_) => "OnTimeElapsed",
            TriggerType::Custom(s) => s.as_str(),
        }
    }
}
