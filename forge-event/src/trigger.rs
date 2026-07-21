use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use crate::event_system::{EventSystem, TriggerType};

/// Disparador de eventos con configuración
#[derive(Clone)]
pub struct EventTrigger {
    pub event_id: String,
    pub trigger_type: TriggerType,
    pub parameters: HashMap<String, String>,
    pub is_active: bool,
}

impl EventTrigger {
    /// Crear nuevo trigger
    pub fn new(event_id: &str, trigger_type: TriggerType) -> Self {
        Self {
            event_id: event_id.to_string(),
            trigger_type,
            parameters: HashMap::new(),
            is_active: false,
        }
    }

    /// Activar trigger
    pub fn activate(&mut self, event_system: &EventSystem) {
        self.is_active = true;
        event_system.activate_trigger(&self.event_id, self.trigger_type.name());
    }

    /// Desactivar trigger
    pub fn deactivate(&mut self, event_system: &EventSystem) {
        self.is_active = false;
        event_system.deactivate_trigger(&self.event_id, self.trigger_type.name());
    }

    /// Verificar si está activo
    pub fn is_active(&self) -> bool {
        self.is_active
    }

    /// Obtener nombre del trigger
    pub fn name(&self) -> &str {
        self.trigger_type.name()
    }

    /// Añadir parámetro
    pub fn set_parameter(&mut self, key: &str, value: &str) {
        self.parameters.insert(key.to_string(), value.to_string());
    }

    /// Obtener parámetro
    pub fn get_parameter(&self, key: &str) -> Option<&String> {
        self.parameters.get(key)
    }

    /// Serializar trigger a string
    pub fn to_string(&self) -> String {
        format!(
            "{}[{}|{}|{:?}]",
            self.event_id,
            self.trigger_type.name(),
            self.is_active,
            self.parameters
        )
    }

    /// Parsear string a trigger
    pub fn from_string(s: &str) -> Option<Self> {
        // Parseo simple, mejorar según necesidad
        Some(Self {
            event_id: s.to_string(),
            trigger_type: TriggerType::Custom(s.to_string()),
            parameters: HashMap::new(),
            is_active: false,
        })
    }
}

/// Gestor de triggers para múltiples eventos
pub struct TriggerManager {
    pub triggers: Arc<RwLock<HashMap<String, EventTrigger>>>,
    pub event_system: Arc<EventSystem>,
}

impl Default for TriggerManager {
    fn default() -> Self {
        Self {
            triggers: Arc::new(RwLock::new(HashMap::new())),
            event_system: Arc::new(EventSystem::default()),
        }
    }
}

impl TriggerManager {
    /// Registrar nuevo trigger
    pub fn register(&self, trigger: EventTrigger) {
        let mut triggers = self.triggers.write().unwrap();
        triggers.insert(trigger.event_id.clone(), trigger);
    }

    /// Obtener trigger por ID
    pub fn get(&self, event_id: &str) -> Option<EventTrigger> {
        self.triggers.read().unwrap().get(event_id).cloned()
    }

    /// Activar trigger
    pub fn activate(&self, event_id: &str) {
        let mut triggers = self.triggers.write().unwrap();
        if let Some(mut trigger) = triggers.remove(event_id) {
            trigger.activate(&self.event_system);
            triggers.insert(event_id.to_string(), trigger);
        }
    }

    /// Desactivar trigger
    pub fn deactivate(&self, event_id: &str) {
        let mut triggers = self.triggers.write().unwrap();
        if let Some(mut trigger) = triggers.remove(event_id) {
            trigger.deactivate(&self.event_system);
            triggers.insert(event_id.to_string(), trigger);
        }
    }

    /// Verificar si trigger está activo
    pub fn is_active(&self, event_id: &str) -> bool {
        self.get(event_id).map(|t| t.is_active()).unwrap_or(false)
    }

    /// Limpiar todos los triggers
    pub fn clear_all(&self) {
        let mut triggers = self.triggers.write().unwrap();
        for trigger in triggers.values_mut() {
            trigger.deactivate(&self.event_system);
        }
        triggers.clear();
    }

    /// Obtener lista de triggers activos
    pub fn get_active_triggers(&self) -> Vec<String> {
        self.triggers.read().unwrap()
            .values()
            .filter(|t| t.is_active())
            .map(|t| t.event_id.clone())
            .collect()
    }
}
