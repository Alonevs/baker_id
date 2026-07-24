//! LiveSync Manager - Sistema de sincronizaciÃ³n en tiempo real
//!
//! Permite sincronizar cambios de escena, assets y estado del juego en tiempo real
//! con pub/sub, delta sync y resoluciÃ³n de conflictos.

use crate::EntityId;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use uuid::Uuid;

// Re-export scene types from forge-scene
pub use ::forge_scene::{Scene, SceneTree, Transform, EntityType, NodeData};

// Import DeltaEncoder
use crate::delta_sync::DeltaEncoder;

/// Evento de sincronizaciÃ³n de LiveSync
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum SyncEvent {
    /// Escena cargada
    SceneLoaded {
        scene_id: Uuid,
        version: u64,
    },
    /// Entidad aÃ±adida
    EntityAdded {
        entity_id: EntityId,
        entity_type: EntityType,
        data: Option<serde_json::Value>,
    },
    /// Entidad removida
    EntityRemoved {
        entity_id: EntityId,
    },
    /// TransformaciÃ³n cambiada
    TransformChanged {
        entity_id: EntityId,
        transform: Transform,
    },
    /// Componente cambiado
    ComponentChanged {
        entity_id: EntityId,
        component_type: String,
        data: Option<serde_json::Value>,
    },
}

impl SyncEvent {
    /// Crea un evento SceneLoaded
    pub fn scene_loaded(scene_id: Uuid, version: u64) -> Self {
        Self::SceneLoaded { scene_id, version }
    }

    /// Crea un evento EntityAdded
    pub fn entity_added(entity_id: EntityId, entity_type: EntityType, data: Option<serde_json::Value>) -> Self {
        Self::EntityAdded { entity_id, entity_type, data }
    }

    /// Crea un evento EntityRemoved
    pub fn entity_removed(entity_id: EntityId) -> Self {
        Self::EntityRemoved { entity_id }
    }

    /// Crea un evento TransformChanged
    pub fn transform_changed(entity_id: EntityId, transform: Transform) -> Self {
        Self::TransformChanged { entity_id, transform }
    }

    /// Crea un evento ComponentChanged
    pub fn component_changed(entity_id: EntityId, component_type: String, data: Option<serde_json::Value>) -> Self {
        Self::ComponentChanged { entity_id, component_type, data }
    }

    /// Obtiene el ID de la entidad (si aplica)
    pub fn entity_id(&self) -> Option<EntityId> {
        match self {
            Self::EntityAdded { entity_id, .. } => Some(*entity_id),
            Self::EntityRemoved { entity_id } => Some(*entity_id),
            Self::TransformChanged { entity_id, .. } => Some(*entity_id),
            Self::ComponentChanged { entity_id, .. } => Some(*entity_id),
            Self::SceneLoaded { .. } => None,
        }
    }

    /// Obtiene el tipo de entidad (si aplica)
    pub fn entity_type(&self) -> Option<EntityType> {
        match self {
            Self::EntityAdded { entity_type, .. } => Some(*entity_type),
            _ => None,
        }
    }
}

/// Subscriber para LiveSync
pub type SyncCallback = Box<dyn Fn(&SyncEvent) + Send + Sync>;

/// Gestor de sincronizaciÃ³n en tiempo real
pub struct LiveSyncManager {
    /// Subscribers por entidad
    subscribers: HashMap<EntityId, Vec<SyncCallback>>,
    /// Subscribers globales (sin filtro de entidad)
    global_subscribers: Vec<SyncCallback>,
    /// Versión de la escena actual
    scene_version: u64,
    /// ID de la escena actual
    current_scene_id: Option<Uuid>,
    /// SceneTree actual
    scene_tree: SceneTree,
    /// Entidades modificadas en este frame (para delta sync)
    pending_changes: HashSet<EntityId>,
    /// Encoder de delta para sincronización eficiente
    delta_encoder: DeltaEncoder,
}

impl LiveSyncManager {
    /// Crea un nuevo LiveSyncManager
    pub fn new() -> Self {
        Self {
            subscribers: HashMap::new(),
            global_subscribers: Vec::new(),
            scene_version: 0,
            current_scene_id: None,
            scene_tree: SceneTree::new(),
            pending_changes: HashSet::new(),
            delta_encoder: DeltaEncoder::new(),
        }
    }

    /// Suscribe a eventos de una entidad especÃ­fica
    pub fn subscribe_entity(&mut self, entity_id: EntityId, callback: SyncCallback) {
        self.subscribers
            .entry(entity_id)
            .or_insert_with(Vec::new)
            .push(callback);
    }

    /// Suscribe a todos los eventos (global)
    pub fn subscribe_global(&mut self, callback: SyncCallback) {
        self.global_subscribers.push(callback);
    }

    /// Desuscribe de una entidad
    pub fn unsubscribe_entity(&mut self, entity_id: EntityId, callback: SyncCallback) {
        if let Some(callbacks) = self.subscribers.get_mut(&entity_id) {
            callbacks.retain(|c| !std::ptr::eq(c, &callback));
        }
    }

    /// Desuscribe global
    pub fn unsubscribe_global(&mut self, callback: SyncCallback) {
        self.global_subscribers.retain(|c| !std::ptr::eq(c, &callback));
    }

    /// Publica un evento de sincronizaciÃ³n
    pub fn publish(&self, event: &SyncEvent) {
        // Notificar subscribers globales
        for callback in &self.global_subscribers {
            callback(event);
        }

        // Notificar subscribers por entidad
        if let Some(entity_id) = event.entity_id() {
            if let Some(callbacks) = self.subscribers.get(&entity_id) {
                for callback in callbacks {
                    callback(event);
                }
            }
        }
    }

    /// Sincroniza una escena completa usando delta sync
    pub fn sync_scene(&mut self, old_scene: &Scene, new_scene: &Scene) {
        let scene_id = new_scene.root_id.unwrap_or(Uuid::new_v4());
        self.current_scene_id = Some(scene_id);
        self.scene_version += 1;
        self.scene_tree = SceneTree::new();

        // Copiar todos los nodos de la escena al SceneTree
        for (node_id, node) in new_scene.nodes.iter() {
            self.scene_tree.nodes.insert(*node_id, Arc::new(node.clone()));
        }

        // Generar deltas entre escenas
        let deltas = self.delta_encoder.encode_deltas(old_scene, new_scene);
        for delta in deltas {
            self.publish(&delta);
        }

        // Limpiar pending_changes despuÃ©s de sincronizar
        self.pending_changes.clear();
    }

    /// Registra un cambio de transformaciÃ³n
    pub fn register_transform_change(&mut self, entity_id: EntityId, transform: Transform) {
        self.pending_changes.insert(entity_id);
        self.publish(&SyncEvent::transform_changed(entity_id, transform));
    }

    /// Registra un cambio de componente
    pub fn register_component_change(&mut self, entity_id: EntityId, component_type: String, data: Option<serde_json::Value>) {
        self.pending_changes.insert(entity_id);
        self.publish(&SyncEvent::component_changed(entity_id, component_type, data));
    }

    /// Genera deltas entre dos escenas
    pub fn encode_deltas(&mut self, old_scene: &Scene, new_scene: &Scene) -> Vec<SyncEvent> {
        self.delta_encoder.encode_deltas(old_scene, new_scene)
    }

    /// Registra una entidad como nueva
    pub fn register_entity_added(&mut self, entity_id: EntityId, entity_type: EntityType, data: Option<serde_json::Value>) {
        self.pending_changes.insert(entity_id);
        self.publish(&SyncEvent::entity_added(entity_id, entity_type, data));
    }

    /// Registra una entidad como removida
    pub fn register_entity_removed(&mut self, entity_id: EntityId) {
        self.pending_changes.insert(entity_id);
        self.publish(&SyncEvent::entity_removed(entity_id));
    }

    /// Obtiene la versiÃ³n actual de la escena
    pub fn scene_version(&self) -> u64 {
        self.scene_version
    }

    /// Obtiene el ID actual de la escena
    pub fn current_scene_id(&self) -> Option<&Uuid> {
        self.current_scene_id.as_ref()
    }

    /// Obtiene el SceneTree actual
    pub fn scene_tree(&self) -> &SceneTree {
        &self.scene_tree
    }

    /// Obtiene las entidades modificadas en este frame
    pub fn pending_changes(&self) -> &HashSet<EntityId> {
        &self.pending_changes
    }

    /// Obtiene el encoder de delta
    pub fn delta_encoder(&self) -> &DeltaEncoder {
        &self.delta_encoder
    }

    /// Obtiene el nÃºmero de subscribers globales
    pub fn global_subscriber_count(&self) -> usize {
        self.global_subscribers.len()
    }

    /// Obtiene el nÃºmero de subscribers por entidad
    pub fn entity_subscriber_count(&self) -> usize {
        self.subscribers.values().map(|v| v.len()).sum()
    }
}

impl Default for LiveSyncManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex;

    #[test]
    fn test_subscribe_publish() {
        let mut sync = LiveSyncManager::new();
        let received_events = Arc::new(Mutex::new(Vec::new()));

        // Suscribirse
        {
            let value = received_events.clone();
            sync.subscribe_global(Box::new(move |event: &SyncEvent| {
                let mut events = value.lock().unwrap();
                events.push(event.clone());
            }));
        }

        // Publicar evento
        sync.publish(&SyncEvent::scene_loaded(Uuid::new_v4(), 1));

        // Verificar que se recibiÃ³ el evento
        let events = received_events.lock().unwrap();
        assert_eq!(events.len(), 1);
        match &events[0] {
            SyncEvent::SceneLoaded { version, .. } => {
                assert_eq!(*version, 1);
            }
            _ => panic!("Evento incorrecto"),
        }
    }

    #[test]
    fn test_entity_subscription() {
        let mut sync = LiveSyncManager::new();
        let received_events = Arc::new(Mutex::new(Vec::new()));
        let entity_id = EntityId(1);

        // Suscribirse a entidad especÃ­fica
        {
            let value = received_events.clone();
            sync.subscribe_entity(entity_id.clone(), Box::new(move |event: &SyncEvent| {
                let mut events = value.lock().unwrap();
                events.push(event.clone());
            }));
        }

        // Publicar evento para esa entidad
        sync.publish(&SyncEvent::transform_changed(entity_id, Transform::default()));

        // Verificar que se recibiÃ³ el evento
        let events = received_events.lock().unwrap();
        assert_eq!(events.len(), 1);
    }

    #[test]
    fn test_scene_sync() {
        let mut sync = LiveSyncManager::new();
        let entity_ids = Arc::new(Mutex::new(Vec::new()));

        // Suscribirse a EntityAdded
        {
            let value = entity_ids.clone();
            sync.subscribe_global(Box::new(move |event: &SyncEvent| {
                if let SyncEvent::EntityAdded { entity_id, .. } = event {
                    let mut ids = value.lock().unwrap();
                    ids.push(entity_id.clone());
                }
            }));
        }

        // Agregar 2 entidades
        for i in 0..2 {
            let entity_id = EntityId(i as i32);
            sync.register_entity_added(entity_id.clone(), EntityType::default(), None);
        }

        // Verificar que se recibieron los eventos
        let ids = entity_ids.lock().unwrap();
        assert_eq!(ids.len(), 2);
    }

    #[test]
    fn test_transform_change() {
        let mut sync = LiveSyncManager::new();
        let received_events = Arc::new(Mutex::new(Vec::new()));

        // Suscribirse
        {
            let value = received_events.clone();
            sync.subscribe_global(Box::new(move |event: &SyncEvent| {
                if let SyncEvent::TransformChanged { transform, .. } = event {
                    let mut received = value.lock().unwrap();
                    received.push(transform.clone());
                }
            }));
        }

        let entity_id = EntityId(1);
        let transform = Transform {
            transform: ::forge_scene::TransformData {
                position: [10.0, 20.0, 0.0],
                rotation: 45.0,
                scale: [1.0, 1.0, 1.0],
            },
        };

        // Registrar cambio
        sync.register_transform_change(entity_id, transform.clone());

        // Verificar
        let received = received_events.lock().unwrap();
        assert_eq!(received.len(), 1);
        assert_eq!(received[0].transform.position, transform.transform.position);
    }
}
